mod http;
mod service;

mod authentication;
mod identity;
mod storage;

mod id;

use std::sync::Arc;

use aide::axum::routing::get;
use aide::axum::{ApiRouter, IntoApiResponse};
use aide::openapi::{self, OpenApi};
use aide::transform::TransformOpenApi;
use anyhow::Result;
use axum::extract::{Path, Request, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{Extension, middleware};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::prelude::*;
use bytes::Bytes;
use rust_embed::Embed;
use time::SignedDuration;
use tokio::net::TcpListener;
use uuid::uuid;
use webauthn_rs::WebauthnBuilder;

use crate::authentication::AuthenticationService;
use crate::http::{AppState, AppStorage, SetSessionId};
use crate::identity::{IdentityService, UserId, UserStorage};
use crate::service::Actor;

const SESSION_ID_COOKIE_NAME: &str = "porch-session";
const SESSION_ID_COOKIE_MAX_AGE: SignedDuration = SignedDuration::days(30);
const ADMIN_USER_ID: UserId = UserId(uuid!("017f1e83-5d80-792d-8ed6-240444e7c10f"));

#[derive(Clone)]
struct OpenApiJson(Bytes);

pub async fn run() -> Result<()> {
    let mut storage = AppStorage::default();

    let rp_id = std::env::var("PORCH_RP_ID").unwrap_or("localhost".into());
    let rp_origin = std::env::var("PORCH_RP_ORIGIN")
        .unwrap_or("http://localhost:8080".into())
        .parse()?;

    let identity = Arc::new(IdentityService {});
    let authentication = Arc::new(AuthenticationService {
        identity: identity.clone(),
        webauthn: WebauthnBuilder::new(&rp_id, &rp_origin)?.build()?,
    });

    storage
        .create_user(&identity::User {
            id: ADMIN_USER_ID,
            display_name: String::from("Admin"),
            name: String::from("admin"),
            is_admin: true,
        })
        .await?;

    let app_state = AppState {
        storage,
        identity,
        authentication,
    };

    let mut api = OpenApi::default();

    let router = ApiRouter::new()
        .nest(
            "/api",
            ApiRouter::new()
                .merge(identity::router())
                .merge(authentication::router())
                .route("/become-admin", axum::routing::post(become_admin))
                .layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    authenticate,
                )),
        )
        .api_route("/openapi.json", get(serve_openapi_json))
        .finish_api_with(&mut api, api_docs)
        .route("/", axum::routing::get(serve_asset))
        .route("/{*path}", axum::routing::get(serve_asset))
        .layer({
            let openapi_json = serde_json::to_vec(&api)?;
            Extension(OpenApiJson(Bytes::from(openapi_json)))
        })
        .with_state(app_state);

    let listener = TcpListener::bind("localhost:8080").await?;
    axum::serve(listener, router).await?;

    Ok(())
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Porch API")
        .summary("An identity and authentication system.")
        .server(openapi::Server {
            url: String::from("/"),
            ..Default::default()
        })
}

async fn authenticate(
    State(mut state): State<AppState>,
    mut cookies: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let session_id = cookies
        .get(SESSION_ID_COOKIE_NAME)
        .and_then(|cookie| cookie.value().parse().ok());

    let actor = Arc::new(match session_id {
        None => Actor::UNAUTHENTICATED,
        Some(session_id) => state
            .authentication
            .get_session_user_by_id(&mut state.storage, session_id)
            .await
            .map_or(Actor::UNAUTHENTICATED, |(_, user)| Actor::from(user)),
    });

    request.extensions_mut().insert(actor.clone());

    let response = next.run(request).await;

    let session_cookie = Cookie::build(SESSION_ID_COOKIE_NAME)
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(SESSION_ID_COOKIE_MAX_AGE);

    let new_session_id = response.extensions().get::<SetSessionId>().map(|v| v.0);

    if new_session_id == Some(None) {
        cookies = cookies.remove(session_cookie);
    } else if let Some(session_id) = new_session_id.flatten().or(session_id) {
        let mut cookie = session_cookie.build();
        cookie.set_value(session_id.expose_to_string());
        cookies = cookies.add(cookie);
    }

    (cookies, response).into_response()
}

async fn serve_openapi_json(
    Extension(openapi_json): Extension<OpenApiJson>,
) -> impl IntoApiResponse {
    ([(header::CONTENT_TYPE, "application/json")], openapi_json.0)
}

async fn become_admin(State(mut state): State<AppState>) -> http::ApiResult<impl IntoResponse> {
    let session = state
        .authentication
        .create_session(&mut state.storage, ADMIN_USER_ID)
        .await?;
    Ok(SetSessionId(Some(session.id)))
}

#[derive(Embed)]
#[folder = "../../frontend/build"]
struct Asset;

async fn serve_asset(path: Option<Path<String>>, headers: HeaderMap) -> Response {
    let content = path
        .as_ref()
        .and_then(|Path(path)| Asset::get(path))
        .unwrap_or(Asset::get("fallback.html").expect("should have 'fallback.html'"));

    let etag = BASE64_URL_SAFE_NO_PAD.encode(content.metadata.sha256_hash());
    let cache_control = if let Some(Path(path)) = &path
        && path.starts_with("static/_app/immutable/")
    {
        "public, max-age=31536000, immutable"
    } else {
        "public, max-age=3600"
    };

    let res_headers = [
        (header::CONTENT_TYPE, content.metadata.mimetype()),
        (header::CACHE_CONTROL, cache_control),
        (header::ETAG, &etag),
    ];

    if headers
        .get_all(header::IF_NONE_MATCH)
        .iter()
        .any(|v| *v == *etag)
    {
        return (res_headers, StatusCode::NOT_MODIFIED).into_response();
    }

    (res_headers, content.data).into_response()
}
