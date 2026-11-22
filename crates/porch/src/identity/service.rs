use anyhow::Context;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::service::{Actor, ServiceError, ServiceResult};

use super::{User, UserId, UserStorage};

pub struct IdentityService {}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CreateUser {
    pub display_name: String,
    pub name: String,
}

impl IdentityService {
    pub async fn create_user(
        &self,
        storage: &mut impl UserStorage,
        actor: &Actor,
        body: CreateUser,
    ) -> ServiceResult<User> {
        actor.must_be_admin()?;

        let user = User {
            id: UserId::generate(),
            display_name: body.display_name,
            name: body.name,
            is_admin: false,
        };

        storage
            .create_user(&user)
            .await
            .context("could not create user in storage")?;

        Ok(user)
    }

    pub async fn get_user_by_id(
        &self,
        storage: &mut impl UserStorage,
        actor: &Actor,
        id: UserId,
    ) -> ServiceResult<User> {
        actor.must_can_manage_user(id)?;
        Ok(storage.get_user_by_id(id).await?)
    }

    pub async fn get_user_by_name(
        &self,
        storage: &mut impl UserStorage,
        name: &str,
    ) -> ServiceResult<User> {
        Ok(storage.get_user_by_name(&name).await?)
    }
}
