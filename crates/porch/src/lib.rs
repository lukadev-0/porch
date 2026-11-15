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
use axum::extract::{Path, Request};
use axum::http::{HeaderMap, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{Extension, middleware};
use base64::Engine;
use bytes::Bytes;
use rust_embed::Embed;
use tokio::net::TcpListener;
use webauthn_rs::WebauthnBuilder;

use crate::authentication::AuthenticationService;
use crate::http::{AppState, AppStorage};
use crate::identity::{IdentityService, UserStorage};
use crate::service::Actor;

#[derive(Clone)]
struct OpenApiJson(Bytes);

pub async fn run() -> Result<()> {
    let storage = AppStorage::default();

    let identity = Arc::new(IdentityService {});
    let authentication = Arc::new(AuthenticationService {
        identity: identity.clone(),
        webauthn: WebauthnBuilder::new("localhost", &"http://localhost:8080".parse()?)?.build()?,
    });

    storage
        .get()
        .create_user(&identity::User {
            id: Actor::DUMMY_ADMIN.user_id(),
            display_name: String::from("Admin"),
            name: String::from("admin"),
        })
        .await?;

    storage
        .get()
        .create_user(&identity::User {
            id: Actor::DUMMY_USER.user_id(),
            display_name: String::from("Bob"),
            name: String::from("bob"),
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
                .merge(authentication::router()),
        )
        .api_route("/openapi.json", get(serve_openapi_json))
        .finish_api_with(&mut api, api_docs)
        .with_state(app_state)
        .route("/", axum::routing::get(serve_asset))
        .route("/{*path}", axum::routing::get(serve_asset))
        .layer({
            let openapi_json = serde_json::to_vec(&api)?;
            Extension(OpenApiJson(Bytes::from(openapi_json)))
        })
        .layer(middleware::from_fn(add_actor));

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

async fn add_actor(mut request: Request, next: Next) -> Response {
    request
        .extensions_mut()
        .insert(Arc::new(Actor::DUMMY_ADMIN));
    next.run(request).await
}

async fn serve_openapi_json(
    Extension(openapi_json): Extension<OpenApiJson>,
) -> impl IntoApiResponse {
    ([(header::CONTENT_TYPE, "application/json")], openapi_json.0)
}

#[derive(Embed)]
#[folder = "../../frontend/build"]
struct Asset;

async fn serve_asset(path: Option<Path<String>>, headers: HeaderMap) -> Response {
    let content = path
        .as_ref()
        .and_then(|Path(path)| Asset::get(path))
        .unwrap_or(Asset::get("fallback.html").expect("should have 'fallback.html'"));

    let etag =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(content.metadata.sha256_hash());
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
