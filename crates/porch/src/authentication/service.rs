use std::sync::Arc;

use anyhow::Context;
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::json;
use webauthn_rs::Webauthn;
use webauthn_rs::prelude as webauthn;

use crate::identity::{IdentityService, UserId, UserStorage};
use crate::service::{Actor, ServiceError, ServiceResult};
use crate::storage::StorageError;

use super::{Passkey, PasskeyId, PasskeyStorage};
use super::{PasskeyAuthentication, PasskeyAuthenticationId};
use super::{PasskeyRegistration, PasskeyRegistrationId};

pub struct AuthenticationService {
    pub identity: Arc<IdentityService>,
    pub webauthn: Webauthn,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct StartPasskeyRegistrationResponse {
    registration_id: PasskeyRegistrationId,
    /// The options to pass into [`navigator.credentials.create`][mdn].
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create
    #[schemars(with = "PasskeyCredentialOptions")]
    credential_options: webauthn::CreationChallengeResponse,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct StartPasskeyAuthenticationResponse {
    authentication_id: PasskeyAuthenticationId,
    /// The options to pass into [`navigator.credentials.get`][mdn].
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get
    #[schemars(with = "PasskeyCredentialOptions")]
    credential_options: webauthn::RequestChallengeResponse,
}

#[derive(JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct PasskeyCredentialOptions {
    /// The [`PublicKeyCredentialRequestOptions`][mdn]. Binary buffers are encoded using Base64.
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredentialRequestOptions
    #[schemars(example = json!({}))]
    pub public_key: serde_json::Value,
}

impl AuthenticationService {
    pub async fn start_passkey_registration(
        &self,
        storage: &mut (impl PasskeyStorage + UserStorage),
        actor: &Actor,
        user_id: UserId,
    ) -> ServiceResult<StartPasskeyRegistrationResponse> {
        actor
            .can_manage_user(user_id)
            .ok_or(ServiceError::AccessDenied)?;

        let user = self
            .identity
            .get_user_by_id(storage, actor, user_id)
            .await?;

        let existing_passkeys = storage
            .get_user_passkeys(user_id)
            .await
            .context("could not fetch the user's passkeys")?;

        let exclude_credentials = existing_passkeys
            .iter()
            .map(|passkey| passkey.data.cred_id().clone())
            .collect();

        let (credential_options, data) = self
            .webauthn
            .start_passkey_registration(
                user_id.0,
                &user.name,
                &user.display_name,
                Some(exclude_credentials),
            )
            .context("failed to generate passkey registration")?;

        let registration_id = PasskeyRegistrationId::generate();
        let registration = PasskeyRegistration {
            id: registration_id,
            user_id,
            data,
        };
        let response = StartPasskeyRegistrationResponse {
            registration_id,
            credential_options,
        };

        storage
            .create_passkey_registration(&registration)
            .await
            .context("could not create passkey registration")?;

        Ok(response)
    }

    pub async fn finish_passkey_registration(
        &self,
        storage: &mut impl PasskeyStorage,
        actor: &Actor,
        user_id: UserId,
        registration_id: PasskeyRegistrationId,
        credential: webauthn::RegisterPublicKeyCredential,
        name: String,
    ) -> ServiceResult<Passkey, webauthn::WebauthnError> {
        actor
            .can_manage_user(user_id)
            .ok_or(ServiceError::AccessDenied)?;

        let registration = storage
            .get_and_delete_passkey_registration(user_id, registration_id)
            .await
            .map_err(|error| match error {
                StorageError::NoExist => ServiceError::NoExist,
                StorageError::Other(error) => ServiceError::Other(error),
            })?;

        let data = self
            .webauthn
            .finish_passkey_registration(&credential, &registration.data)
            .map_err(ServiceError::Specific)?;

        let passkey = Passkey {
            id: PasskeyId::generate(),
            user_id,
            name,
            data,
        };

        storage
            .create_passkey(&passkey)
            .await
            .context("could not create passkey in storage")?;

        Ok(passkey)
    }

    pub async fn get_user_passkeys(
        &self,
        storage: &mut impl PasskeyStorage,
        actor: &Actor,
        user_id: UserId,
    ) -> ServiceResult<Vec<Passkey>> {
        actor
            .can_manage_user(user_id)
            .ok_or(ServiceError::AccessDenied)?;

        let passkeys = storage
            .get_user_passkeys(user_id)
            .await
            .map_err(|error| match error {
                StorageError::NoExist => ServiceError::NoExist,
                StorageError::Other(error) => ServiceError::Other(error),
            })?;

        Ok(passkeys)
    }

    pub async fn start_passkey_authentication(
        &self,
        storage: &mut (impl PasskeyStorage + UserStorage),
        username: &str,
    ) -> ServiceResult<StartPasskeyAuthenticationResponse> {
        let user = self.identity.get_user_by_name(storage, username).await?;

        let passkeys: Vec<webauthn::Passkey> = storage
            .get_user_passkeys(user.id)
            .await
            .context("could not fetch the user's passkeys")?
            .into_iter()
            .map(|passkey| passkey.data)
            .collect();

        let (credential_options, data) = self
            .webauthn
            .start_passkey_authentication(&passkeys)
            .context("failed to generate passkey registration")?;

        let authentication_id = PasskeyAuthenticationId::generate();
        let registration = PasskeyAuthentication {
            id: authentication_id,
            user_id: user.id,
            data,
        };
        let response = StartPasskeyAuthenticationResponse {
            authentication_id,
            credential_options,
        };

        storage
            .create_passkey_authentication(&registration)
            .await
            .context("could not create passkey registration")?;

        Ok(response)
    }

    pub async fn finish_passkey_authentication(
        &self,
        storage: &mut impl PasskeyStorage,
        authentication_id: PasskeyAuthenticationId,
        credential: webauthn::PublicKeyCredential,
    ) -> ServiceResult<UserId, webauthn::WebauthnError> {
        let registration = storage
            .get_and_delete_passkey_authentication(authentication_id)
            .await
            .map_err(|error| match error {
                StorageError::NoExist => ServiceError::NoExist,
                StorageError::Other(error) => ServiceError::Other(error),
            })?;

        self.webauthn
            .finish_passkey_authentication(&credential, &registration.data)
            .map_err(ServiceError::Specific)?;

        Ok(registration.user_id)
    }
}
