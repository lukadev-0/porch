mod error;
mod state;

use std::borrow::Cow;

use schemars::{JsonSchema, json_schema};
use serde::{Deserialize, Serialize};

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
    pub fn resolve(self, actor: &Actor) -> UserId {
        match self {
            Self::Me => actor.user_id(),
            Self::Id(user_id) => user_id,
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
