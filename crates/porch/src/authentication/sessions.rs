//! Manage the user's login sessions.

use std::fmt;

use base64::Engine;

use crate::identity::UserId;
use crate::storage::{Storage, StorageResult};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SessionId {
    data: [u8; 32],
}

impl SessionId {
    pub fn generate() -> Self {
        let mut data = [0; 32];
        getrandom::fill(&mut data).expect("could not get random bytes for session id");
        Self { data }
    }

    pub fn expose_to_string(&self) -> String {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(self.data)
    }
}

impl fmt::Debug for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SessionId(***)")
    }
}

pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
}

pub trait SessionStorage: Storage {
    fn create_session(&mut self, session: &Session) -> impl Future<Output = StorageResult<()>>;
}
