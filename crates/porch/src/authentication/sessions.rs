//! Manage the user's login sessions.

use std::fmt;
use std::str::FromStr;

use base64::Engine;
use thiserror::Error;

use crate::identity::{User, UserId};
use crate::storage::{Storage, StorageResult};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SessionId {
    data: [u8; Self::BYTES],
}

impl SessionId {
    const BYTES: usize = 32;

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

#[derive(Debug, Error)]
pub enum SessionIdFromStrError {
    #[error("failed to decode session id")]
    Decode(#[from] base64::DecodeSliceError),
    #[error("session id has invalid length")]
    InvalidLength,
}

impl FromStr for SessionId {
    type Err = SessionIdFromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = [0; Self::BYTES];
        let num_bytes =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.decode_slice(s, &mut data)?;
        if num_bytes != Self::BYTES {
            return Err(SessionIdFromStrError::InvalidLength);
        }
        Ok(Self { data })
    }
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
}

pub trait SessionStorage: Storage {
    fn create_session(&mut self, session: &Session) -> impl Future<Output = StorageResult<()>>;
    fn get_session_user_by_id(
        &mut self,
        session_id: SessionId,
    ) -> impl Future<Output = StorageResult<(Session, User)>>;
}
