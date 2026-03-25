use secrecy::SecretString;

use crate::error::TauriXMPPError;

const KEYRING_SERVICE: &str = "de_janerikvoigt_here_now_location";

pub fn get_jid() -> Result<String, TauriXMPPError> {
    keyring::Entry::new(KEYRING_SERVICE, "jid")
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))?
        .get_password()
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}

pub fn set_jid(jid: &str) -> Result<(), TauriXMPPError> {
    keyring::Entry::new(KEYRING_SERVICE, "jid")
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))?
        .set_password(jid)
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}

pub fn get_password() -> Result<SecretString, TauriXMPPError> {
    keyring::Entry::new(KEYRING_SERVICE, "password")
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))?
        .get_password()
        .map(|pw| SecretString::new(pw.into()))
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}

pub fn set_password(password: &str) -> Result<(), TauriXMPPError> {
    keyring::Entry::new(KEYRING_SERVICE, "password")
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))?
        .set_password(password)
        .map_err(|e| TauriXMPPError::KeyringError(e.to_string()))
}
