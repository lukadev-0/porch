use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;
use webauthn_rs::prelude as webauthn;

use crate::id::id_type;
use crate::identity::UserId;
use crate::storage::{Storage, StorageResult};

id_type! {
    pub struct PasskeyId(Uuid::now_v7);
    pub struct PasskeyRegistrationId(Uuid::new_v4);
    pub struct PasskeyAuthenticationId(Uuid::new_v4);
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Passkey {
    pub id: PasskeyId,
    pub user_id: UserId,
    pub name: String,
    #[serde(skip)]
    pub data: webauthn::Passkey,
}

#[derive(Debug, Clone)]
pub struct PasskeyRegistration {
    pub id: PasskeyRegistrationId,
    pub user_id: UserId,
    pub data: webauthn::PasskeyRegistration,
}

#[derive(Debug, Clone)]
pub struct PasskeyAuthentication {
    pub id: PasskeyAuthenticationId,
    pub user_id: UserId,
    pub data: webauthn::PasskeyAuthentication,
}

pub trait PasskeyStorage: Storage {
    fn create_passkey(
        &mut self,
        passkey: &Passkey,
    ) -> impl Future<Output = StorageResult<()>> + Send;

    fn delete_passkey(
        &mut self,
        user_id: UserId,
        passkey_id: PasskeyId,
    ) -> impl Future<Output = StorageResult<()>> + Send;

    fn get_user_passkeys(
        &mut self,
        user_id: UserId,
    ) -> impl Future<Output = StorageResult<Vec<Passkey>>> + Send;

    fn create_passkey_registration(
        &mut self,
        registration: &PasskeyRegistration,
    ) -> impl Future<Output = StorageResult<()>> + Send;

    fn get_and_delete_passkey_registration(
        &mut self,
        user_id: UserId,
        registration_id: PasskeyRegistrationId,
    ) -> impl Future<Output = StorageResult<PasskeyRegistration>> + Send;

    fn create_passkey_authentication(
        &mut self,
        authentication: &PasskeyAuthentication,
    ) -> impl Future<Output = StorageResult<()>> + Send;

    fn get_and_delete_passkey_authentication(
        &mut self,
        authentication_id: PasskeyAuthenticationId,
    ) -> impl Future<Output = StorageResult<PasskeyAuthentication>> + Send;
}
