use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with};
use aide::transform::TransformOperation;
use axum::extract::{FromRef, Path, State};
use axum::{Extension, Json};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::http::{ApiResult, AppStorage, UserTarget};
use crate::service::Actor;

use super::{CreateUser, IdentityService, User};

pub fn router<S>() -> ApiRouter<S>
where
    S: Clone + Send + Sync + 'static,
    AppStorage: FromRef<S>,
    Arc<IdentityService>: FromRef<S>,
{
    ApiRouter::new()
        .api_route("/users", post_with(create_user, create_user_docs))
        .api_route(
            "/users/{user_id}",
            get_with(get_user_by_id, get_user_by_id_docs),
        )
        .with_path_items(|path| path.tag("Identity"))
}

async fn create_user(
    State(mut storage): State<AppStorage>,
    State(identity): State<Arc<IdentityService>>,
    Extension(actor): Extension<Arc<Actor>>,
    Json(body): Json<CreateUser>,
) -> ApiResult<Json<User>> {
    let user = identity.create_user(&mut storage, &actor, body).await?;
    Ok(Json(user))
}

fn create_user_docs(op: TransformOperation) -> TransformOperation {
    op.id("createUser").summary("Create user")
}

#[derive(Deserialize, JsonSchema)]
struct SelectUser {
    user_id: UserTarget,
}

async fn get_user_by_id(
    State(mut storage): State<AppStorage>,
    State(identity): State<Arc<IdentityService>>,
    Extension(actor): Extension<Arc<Actor>>,
    Path(path): Path<SelectUser>,
) -> ApiResult<Json<User>> {
    let user = identity
        .get_user_by_id(&mut storage, &actor, path.user_id.resolve(&actor)?)
        .await?;
    Ok(Json(user))
}

fn get_user_by_id_docs(op: TransformOperation) -> TransformOperation {
    op.id("getUserById").summary("Get user by ID")
}
