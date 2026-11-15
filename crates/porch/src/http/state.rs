use std::sync::Arc;

use axum::extract::FromRef;

use crate::authentication::{AuthenticationService, PasskeyStorage};
use crate::identity::{IdentityService, UserStorage};
use crate::storage::memory::MemoryStorage;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub storage: AppStorage,
    pub identity: Arc<IdentityService>,
    pub authentication: Arc<AuthenticationService>,
}

#[derive(Clone, Default)]
pub struct AppStorage {
    storage: MemoryStorage,
}

impl AppStorage {
    pub fn get(&self) -> impl UserStorage + PasskeyStorage {
        self.storage.clone()
    }
}
