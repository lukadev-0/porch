//! User identity.

mod routes;
mod service;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::id::id_type;
use crate::storage::{Storage, StorageResult};

pub use routes::*;
pub use service::*;

id_type! {
    pub struct UserId(Uuid::now_v7);
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub display_name: String,
    pub is_admin: bool,
}

pub trait UserStorage: Storage {
    fn create_user(&mut self, user: &User) -> impl Future<Output = StorageResult<()>> + Send;
    fn get_user_by_id(&mut self, id: UserId) -> impl Future<Output = StorageResult<User>> + Send;
    fn get_user_by_name(&mut self, name: &str) -> impl Future<Output = StorageResult<User>> + Send;
}
