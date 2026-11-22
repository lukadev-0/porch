//! An in-memory storage backend.
//!
//! This is intended for testing purposes only. The entire inner state is guarded by a singular
//! reader-writer lock.

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::authentication::{Passkey, PasskeyId, PasskeyRegistration, PasskeyRegistrationId};
use crate::authentication::{PasskeyAuthentication, PasskeyAuthenticationId};
use crate::authentication::{PasskeyStorage, Session, SessionId, SessionStorage};
use crate::identity::{User, UserId, UserStorage};

use super::{Storage, StorageError, StorageResult};

#[derive(Debug, Clone, Default)]
pub struct MemoryStorage {
    state: Arc<RwLock<State>>,
}

#[derive(Debug, Default)]
pub struct State {
    pub users: BTreeMap<UserId, User>,
    pub user_passkeys: BTreeMap<UserId, BTreeMap<PasskeyId, Passkey>>,
    pub user_passkey_registrations:
        BTreeMap<UserId, BTreeMap<PasskeyRegistrationId, PasskeyRegistration>>,
    pub passkey_authentications: BTreeMap<PasskeyAuthenticationId, PasskeyAuthentication>,
    pub sessions: BTreeMap<SessionId, Session>,
}

impl MemoryStorage {
    pub fn read(&self) -> RwLockReadGuard<'_, State> {
        self.state.read().unwrap()
    }

    pub fn write(&mut self) -> RwLockWriteGuard<'_, State> {
        self.state.write().unwrap()
    }
}

impl Storage for MemoryStorage {}

impl UserStorage for MemoryStorage {
    async fn create_user(&mut self, user: &User) -> StorageResult<()> {
        let mut state = self.write();
        state.user_passkeys.insert(user.id, BTreeMap::new());
        state
            .user_passkey_registrations
            .insert(user.id, BTreeMap::new());
        state.users.insert(user.id, user.clone());
        Ok(())
    }

    async fn get_user_by_id(&mut self, id: UserId) -> StorageResult<User> {
        self.read()
            .users
            .get(&id)
            .cloned()
            .ok_or(StorageError::NoExist)
    }

    async fn get_user_by_name(&mut self, name: &str) -> StorageResult<User> {
        self.read()
            .users
            .values()
            .find(|user| user.name == name)
            .cloned()
            .ok_or(StorageError::NoExist)
    }
}

impl PasskeyStorage for MemoryStorage {
    async fn create_passkey(&mut self, passkey: &Passkey) -> StorageResult<()> {
        self.write()
            .user_passkeys
            .get_mut(&passkey.user_id)
            .ok_or(StorageError::NoExist)?
            .insert(passkey.id, passkey.clone());
        Ok(())
    }

    async fn delete_passkey(
        &mut self,
        user_id: UserId,
        passkey_id: PasskeyId,
    ) -> StorageResult<()> {
        self.write()
            .user_passkeys
            .get_mut(&user_id)
            .ok_or(StorageError::NoExist)?
            .remove(&passkey_id)
            .is_none()
            .ok_or(StorageError::NoExist)
    }

    async fn get_user_passkeys(&mut self, user_id: UserId) -> StorageResult<Vec<Passkey>> {
        Ok(self
            .read()
            .user_passkeys
            .get(&user_id)
            .ok_or(StorageError::NoExist)?
            .values()
            .cloned()
            .collect())
    }

    async fn create_passkey_registration(
        &mut self,
        registration: &PasskeyRegistration,
    ) -> StorageResult<()> {
        self.write()
            .user_passkey_registrations
            .get_mut(&registration.user_id)
            .ok_or(StorageError::NoExist)?
            .insert(registration.id, registration.clone());
        Ok(())
    }

    async fn get_and_delete_passkey_registration(
        &mut self,
        user_id: UserId,
        registration_id: PasskeyRegistrationId,
    ) -> StorageResult<PasskeyRegistration> {
        self.write()
            .user_passkey_registrations
            .get_mut(&user_id)
            .ok_or(StorageError::NoExist)?
            .remove(&registration_id)
            .ok_or(StorageError::NoExist)
    }

    async fn create_passkey_authentication(
        &mut self,
        authentication: &PasskeyAuthentication,
    ) -> StorageResult<()> {
        self.write()
            .passkey_authentications
            .insert(authentication.id, authentication.clone());
        Ok(())
    }

    async fn get_and_delete_passkey_authentication(
        &mut self,
        authentication_id: PasskeyAuthenticationId,
    ) -> StorageResult<PasskeyAuthentication> {
        self.write()
            .passkey_authentications
            .remove(&authentication_id)
            .ok_or(StorageError::NoExist)
    }
}

impl SessionStorage for MemoryStorage {
    async fn create_session(&mut self, session: &Session) -> StorageResult<()> {
        self.write().sessions.insert(session.id, session.clone());
        Ok(())
    }

    async fn get_session_user_by_id(
        &mut self,
        session_id: SessionId,
    ) -> StorageResult<(Session, User)> {
        let state = self.read();
        let session = state
            .sessions
            .get(&session_id)
            .ok_or(StorageError::NoExist)?;
        let user = state
            .users
            .get(&session.user_id)
            .ok_or(StorageError::NoExist)?;
        Ok((session.clone(), user.clone()))
    }
}
