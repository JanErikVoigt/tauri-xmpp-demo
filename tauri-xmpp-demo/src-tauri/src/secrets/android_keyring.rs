use crate::error::TauriXMPPError;
use android_native_keyring_store::Store;
use keyring_core::api::CredentialStoreApi;
use secrecy::SecretString;

fn entry(user: &str, service_name: &str) -> Result<keyring_core::Entry, TauriXMPPError> {
    Store::new()
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))?
        .build(KEYRING_SERVICE, user, None)
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}

pub fn get_entry(user: &str, service_name: &str) -> Result<keyring_core::Entry, TauriXMPPError> {
    entry("jid")?
        .get_password()
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}

pub fn set_entry(
    user: &str,
    service_name: &str,
    value: &str,
) -> Result<keyring_core::Entry, TauriXMPPError> {
    entry("jid")?
        .set_password(value)
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}
