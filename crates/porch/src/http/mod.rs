mod error;
mod state;

use std::borrow::Cow;

use aide::OperationOutput;
use axum::http::StatusCode;
use axum::response::{IntoResponse, IntoResponseParts, NoContent, Response, ResponseParts};
use schemars::{JsonSchema, json_schema};
use serde::{Deserialize, Serialize};

use crate::authentication::SessionId;
use crate::identity::UserId;
use crate::service::Actor;

pub use error::*;
pub use state::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserTarget {
    Me,
    #[serde(untagged)]
    Id(UserId),
}

impl UserTarget {
    pub fn resolve(self, actor: &Actor) -> ApiResult<UserId> {
        match self {
            Self::Me => Ok(actor.user_id()?),
            Self::Id(user_id) => Ok(user_id),
        }
    }
}

impl JsonSchema for UserTarget {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("UserTarget")
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "description": "A user ID or the string `me` to refer to the authenticated user.",
            "oneOf": [
                {
                    "description": "Reference the user with the given ID.",
                    "type": "string",
                    "format": "uuid",
                },
                {
                    "description": "Reference the authenticated user.",
                    "const": "me",
                },
            ],
        })
    }
}

#[derive(Clone, Copy)]
pub struct SetSessionId(pub Option<SessionId>);

impl IntoResponseParts for SetSessionId {
    type Error = !;
    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.extensions_mut().insert(self);
        Ok(res)
    }
}

impl IntoResponse for SetSessionId {
    fn into_response(self) -> Response {
        (self, NoContent).into_response()
    }
}

impl OperationOutput for SetSessionId {
    type Inner = ();
    fn operation_response(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Option<aide::openapi::Response> {
        Some(aide::openapi::Response::default())
    }

    fn inferred_responses(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        Self::operation_response(ctx, operation)
            .into_iter()
            .map(|response| (Some(204), response))
            .collect()
    }
}
