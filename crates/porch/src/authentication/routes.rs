use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with};
use aide::transform::TransformOperation;
use axum::extract::{FromRef, Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use webauthn_rs::prelude as webauthn;

use crate::authentication::{PasskeyAuthenticationId, StartPasskeyAuthenticationResponse};
use crate::http::{ApiError, ApiResult, IntoApiError, SetSessionId, UserTarget};
use crate::identity::UserId;
use crate::{http::AppStorage, service::Actor};

use super::AuthenticationService;
use super::{Passkey, PasskeyId, PasskeyRegistrationId, StartPasskeyRegistrationResponse};

pub fn router<S>() -> ApiRouter<S>
where
    S: Clone + Send + Sync + 'static,
    AppStorage: FromRef<S>,
    Arc<AuthenticationService>: FromRef<S>,
{
    ApiRouter::new()
        .api_route(
            "/users/{user_id}/passkeys/registration",
            post_with(start_passkey_registration, start_passkey_registration_docs),
        )
        .api_route(
            "/users/{user_id}/passkeys",
            get_with(get_user_passkeys, get_user_passkeys_docs).post_with(
                finish_passkey_registration,
                finish_passkey_registration_docs,
            ),
        )
        .api_route(
            "/auth/passkey/start",
            post_with(
                start_passkey_authentication,
                start_passkey_authentication_docs,
            ),
        )
        .api_route(
            "/auth/passkey/finish",
            post_with(
                finish_passkey_authentication,
                finish_passkey_authentication_docs,
            ),
        )
        .api_route("/auth/sign-out", post_with(sign_out, sign_out_docs))
        .with_path_items(|path| path.tag("Authentication"))
}

#[derive(Deserialize, JsonSchema)]
struct SelectUserPasskeys {
    user_id: UserTarget,
}

async fn start_passkey_registration(
    State(mut storage): State<AppStorage>,
    State(authentication): State<Arc<AuthenticationService>>,
    Extension(actor): Extension<Arc<Actor>>,
    Path(path): Path<SelectUserPasskeys>,
) -> ApiResult<Json<StartPasskeyRegistrationResponse>> {
    let response = authentication
        .start_passkey_registration(&mut storage, &actor, path.user_id.resolve(&actor)?)
        .await?;
    Ok(Json(response))
}

fn start_passkey_registration_docs(op: TransformOperation) -> TransformOperation {
    op.id("startPasskeyRegistration")
        .summary("Start passkey registration")
        .description("Get a registration ID and passkey credential creation options.")
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct PasskeyCreationBody {
    registration_id: PasskeyRegistrationId,
    /// The [`PublicKeyCredential`][mdn].
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential
    #[schemars(with = "serde_json::Value", example = json!({}))]
    credential: webauthn::RegisterPublicKeyCredential,
    name: String,
}

async fn finish_passkey_registration(
    State(mut storage): State<AppStorage>,
    State(authentication): State<Arc<AuthenticationService>>,
    Extension(actor): Extension<Arc<Actor>>,
    Path(path): Path<SelectUserPasskeys>,
    Json(body): Json<PasskeyCreationBody>,
) -> ApiResult<Json<Passkey>> {
    let response = authentication
        .finish_passkey_registration(
            &mut storage,
            &actor,
            path.user_id.resolve(&actor)?,
            body.registration_id,
            body.credential,
            body.name,
        )
        .await
        .into_api_error(|error| ApiError::new(StatusCode::BAD_REQUEST, error.to_string()))?;
    Ok(Json(response))
}

fn finish_passkey_registration_docs(op: TransformOperation) -> TransformOperation {
    op.id("finishPasskeyRegistration")
        .summary("Finish passkey registration")
}

#[derive(Serialize, JsonSchema)]
struct UserPasskey {
    id: PasskeyId,
    name: String,
}

async fn get_user_passkeys(
    State(mut storage): State<AppStorage>,
    State(authentication): State<Arc<AuthenticationService>>,
    Extension(actor): Extension<Arc<Actor>>,
    Path(path): Path<SelectUserPasskeys>,
) -> ApiResult<Json<Vec<UserPasskey>>> {
    let passkeys = authentication
        .get_user_passkeys(&mut storage, &actor, path.user_id.resolve(&actor)?)
        .await?
        .into_iter()
        .map(|passkey| UserPasskey {
            id: passkey.id,
            name: passkey.name,
        })
        .collect();

    Ok(Json(passkeys))
}

fn get_user_passkeys_docs(op: TransformOperation) -> TransformOperation {
    op.id("getUserPasskeys").summary("Get user passkeys")
}

#[derive(Deserialize, JsonSchema)]
struct StartPasskeyAuthenticationBody {
    username: String,
}

async fn start_passkey_authentication(
    State(mut storage): State<AppStorage>,
    State(authentication): State<Arc<AuthenticationService>>,
    Json(body): Json<StartPasskeyAuthenticationBody>,
) -> ApiResult<Json<StartPasskeyAuthenticationResponse>> {
    let response = authentication
        .start_passkey_authentication(&mut storage, &body.username)
        .await?;
    Ok(Json(response))
}

fn start_passkey_authentication_docs(op: TransformOperation) -> TransformOperation {
    op.id("startPasskeyAuthentication")
        .summary("Start passkey authentication")
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct PasskeyAuthenticationBody {
    authentication_id: PasskeyAuthenticationId,
    /// The [`PublicKeyCredential`][mdn].
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential
    #[schemars(with = "serde_json::Value", example = json!({}))]
    credential: webauthn::PublicKeyCredential,
}

#[derive(Serialize, JsonSchema)]
struct AuthenticationResponse {
    user_id: UserId,
}

async fn finish_passkey_authentication(
    State(mut storage): State<AppStorage>,
    State(authentication): State<Arc<AuthenticationService>>,
    Json(body): Json<PasskeyAuthenticationBody>,
) -> ApiResult<SetSessionId> {
    let session = authentication
        .finish_passkey_authentication(&mut storage, body.authentication_id, body.credential)
        .await
        .into_api_error(|error| ApiError::new(StatusCode::BAD_REQUEST, error.to_string()))?;

    Ok(SetSessionId(Some(session.id)))
}

fn finish_passkey_authentication_docs(op: TransformOperation) -> TransformOperation {
    op.id("finishPasskeyAuthentication")
        .summary("Finish passkey authentication")
}

async fn sign_out() -> ApiResult<SetSessionId> {
    Ok(SetSessionId(None))
}

fn sign_out_docs(op: TransformOperation) -> TransformOperation {
    op.id("signOut").summary("Sign out")
}
