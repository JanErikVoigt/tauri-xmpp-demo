use crate::error::TauriXMPPError;
use secrecy::SecretString;

pub fn get_entry(user: &str, service_name: &str) -> Result<SecretString, TauriXMPPError> {
    keyring::Entry::new(service_name, user)
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))?
        .get_password()
        .map(|pw| SecretString::new(pw.into()))
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}

pub fn set_entry(user: &str, service_name: &str, value: &str) -> Result<(), TauriXMPPError> {
    keyring::Entry::new(service_name, user)
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))?
        .set_password(value)
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}
