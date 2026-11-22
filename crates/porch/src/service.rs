use thiserror::Error;

use crate::identity::{User, UserId};
use crate::storage::StorageError;

#[derive(Debug, Error)]
pub enum ServiceError<E = !> {
    #[error("authentication required")]
    Unauthenticated,
    #[error("user does not have the required permissions")]
    AccessDenied,
    #[error("target resource does not exist")]
    NoExist,
    #[error(transparent)]
    Specific(E),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ServiceResult<T, E = !> = Result<T, ServiceError<E>>;

impl<E> From<StorageError> for ServiceError<E> {
    fn from(value: StorageError) -> Self {
        match value {
            StorageError::NoExist => ServiceError::NoExist,
            error => ServiceError::Other(error.into()),
        }
    }
}

impl<E> From<Unauthorized> for ServiceError<E> {
    fn from(value: Unauthorized) -> Self {
        match value {
            Unauthorized::Unauthenticated => Self::Unauthenticated,
            Unauthorized::AccessDenied => Self::AccessDenied,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Actor {
    user: Option<User>,
}

impl Actor {
    pub const UNAUTHENTICATED: Self = Self { user: None };

    pub fn user(&self) -> Result<&User, Unauthorized> {
        self.authorize(|_| true)
    }

    pub fn user_id(&self) -> Result<UserId, Unauthorized> {
        self.user().map(|user| user.id)
    }

    pub fn authorize(&self, f: impl FnOnce(&User) -> bool) -> Result<&User, Unauthorized> {
        match &self.user {
            None => Err(Unauthorized::Unauthenticated),
            Some(user) => f(user).then_some(user).ok_or(Unauthorized::AccessDenied),
        }
    }

    pub fn must_be_admin(&self) -> Result<&User, Unauthorized> {
        self.authorize(|user| user.is_admin)
    }

    pub fn must_can_manage_user(&self, user_id: UserId) -> Result<&User, Unauthorized> {
        self.authorize(|user| user.id == user_id || user.is_admin)
    }
}

pub enum Unauthorized {
    Unauthenticated,
    AccessDenied,
}

impl From<User> for Actor {
    fn from(value: User) -> Self {
        Self { user: Some(value) }
    }
}
