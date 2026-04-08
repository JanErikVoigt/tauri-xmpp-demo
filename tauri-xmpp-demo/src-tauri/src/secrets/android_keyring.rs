use crate::xmpp::XmppError;
use android_native_keyring_store::Store;
use keyring_core::api::CredentialStoreApi;
use secrecy::SecretString;

fn entry(user: &str, service_name: &str) -> Result<keyring_core::Entry, XmppError> {
    Store::new()
        .map_err(|e| XmppError::Keyring(e.to_string()))?
        .build(service_name, user, None)
        .map_err(|e| XmppError::Keyring(e.to_string()))
}

pub fn get_entry(user: &str, service_name: &str) -> Result<SecretString, XmppError> {
    entry(user, service_name)?
        .get_password()
        .map(|pw| SecretString::new(pw.into()))
        .map_err(|e| XmppError::Keyring(e.to_string()))
}

pub fn set_entry(user: &str, service_name: &str, value: &str) -> Result<(), XmppError> {
    entry(user, service_name)?
        .set_password(value)
        .map_err(|e| XmppError::Keyring(e.to_string()))
}
