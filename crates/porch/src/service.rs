use thiserror::Error;
use uuid::uuid;

use crate::identity::UserId;

#[derive(Debug, Error)]
pub enum ServiceError<E = !> {
    #[error("access denied")]
    AccessDenied,
    #[error("target resource does not exist")]
    NoExist,
    #[error(transparent)]
    Specific(E),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ServiceResult<T, E = !> = Result<T, ServiceError<E>>;

#[derive(Debug, Clone)]
pub struct Actor {
    user_id: UserId,
    is_admin: bool,
}

impl Actor {
    pub const DUMMY_USER: Self = Self {
        user_id: UserId(uuid!("014c155d-a180-7974-89bb-0def4dc42c43")),
        is_admin: false,
    };

    pub const DUMMY_ADMIN: Self = Self {
        user_id: UserId(uuid!("017f1e83-5d80-7e02-b95d-e69f40f1166d")),
        is_admin: true,
    };

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    pub fn can_manage_user(&self, user_id: UserId) -> bool {
        self.is_admin() || self.user_id() == user_id
    }
}
