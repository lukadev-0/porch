use std::sync::Arc;

use axum::extract::FromRef;

use crate::authentication::{AuthenticationService, PasskeyStorage, SessionStorage};
use crate::identity::{IdentityService, UserStorage};
use crate::storage::memory::MemoryStorage;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub storage: AppStorage,
    pub identity: Arc<IdentityService>,
    pub authentication: Arc<AuthenticationService>,
}

pub type AppStorage = MemoryStorage;
