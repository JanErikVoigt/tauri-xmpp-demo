use crate::error::TauriXMPPError;
use android_native_keyring_store::Store;
use keyring_core::api::CredentialStoreApi;
use secrecy::SecretString;

fn entry(user: &str, service_name: &str) -> Result<keyring_core::Entry, TauriXMPPError> {
    Store::new()
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))?
        .build(service_name, user, None)
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}

pub fn get_entry(user: &str, service_name: &str) -> Result<SecretString, TauriXMPPError> {
    entry(user, service_name)?
        .get_password()
        .map(|pw| SecretString::new(pw.into()))
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}

pub fn set_entry(user: &str, service_name: &str, value: &str) -> Result<(), TauriXMPPError> {
    entry(user, service_name)?
        .set_password(value)
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}
