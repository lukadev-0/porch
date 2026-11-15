use anyhow::Context;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::service::{Actor, ServiceError, ServiceResult};
use crate::storage::StorageError;

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
        actor.is_admin().ok_or(ServiceError::AccessDenied)?;

        let user = User {
            id: UserId::generate(),
            display_name: body.display_name,
            name: body.name,
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
        actor
            .can_manage_user(id)
            .ok_or(ServiceError::AccessDenied)?;
        storage
            .get_user_by_id(id)
            .await
            .map_err(|error| match error {
                StorageError::NoExist => ServiceError::NoExist,
                StorageError::Other(error) => ServiceError::Other(error),
            })
    }

    pub async fn get_user_by_name(
        &self,
        storage: &mut impl UserStorage,
        name: &str,
    ) -> ServiceResult<User> {
        storage
            .get_user_by_name(&name)
            .await
            .map_err(|error| match error {
                StorageError::NoExist => ServiceError::NoExist,
                StorageError::Other(error) => ServiceError::Other(error),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::memory::MemoryStorage;

    use anyhow::Result;

    #[tokio::test]
    async fn test_create_and_get_user() -> Result<()> {
        let identity = IdentityService {};
        let mut storage = MemoryStorage::default();

        let create_user = CreateUser {
            display_name: String::from("Bob"),
            name: String::from("bob@example.com"),
        };

        let user = identity
            .create_user(&mut storage, &Actor::DUMMY_ADMIN, create_user.clone())
            .await?;

        let got_user = identity
            .get_user_by_id(&mut storage, &Actor::DUMMY_ADMIN, user.id)
            .await?;
        assert_eq!(user.id, got_user.id);

        assert!(matches!(
            identity
                .get_user_by_id(&mut storage, &Actor::DUMMY_USER, user.id)
                .await,
            Err(ServiceError::AccessDenied)
        ));

        assert!(matches!(
            identity
                .create_user(&mut storage, &Actor::DUMMY_USER, create_user)
                .await,
            Err(ServiceError::AccessDenied)
        ));

        assert!(matches!(
            identity
                .get_user_by_id(
                    &mut storage,
                    &Actor::DUMMY_ADMIN,
                    Actor::DUMMY_USER.user_id()
                )
                .await,
            Err(ServiceError::NoExist)
        ));

        Ok(())
    }
}
