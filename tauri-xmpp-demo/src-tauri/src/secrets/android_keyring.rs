use crate::error::TauriXMPPError;
use crate::prelude::*;
use android_native_keyring_store::Store;
use keyring_core::api::CredentialStoreApi;
use secrecy::SecretString;

const KEYRING_SERVICE: &str = "de_janerikvoigt_here_now_location";

fn entry(user: &str, service_name: &str) -> Result<keyring_core::Entry, TauriXMPPError> {
    Store::new()
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))?
        .build(KEYRING_SERVICE, user, None)
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}

pub fn get_jid(service_name: &str) -> Result<String, TauriXMPPError> {
    entry("jid")?
        .get_password()
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}

pub fn set_jid(jid: &str) -> Result<(), TauriXMPPError> {
    entry("jid")?
        .set_password(jid)
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}

pub fn get_password() -> Result<SecretString, TauriXMPPError> {
    entry("password")?
        .get_password()
        .map(|pw| SecretString::new(pw.into()))
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}

pub fn set_password(password: &str) -> Result<(), TauriXMPPError> {
    entry("password")?
        .set_password(password)
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}
