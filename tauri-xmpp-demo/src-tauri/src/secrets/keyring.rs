use crate::xmpp::XmppError;
use secrecy::SecretString;

pub fn get_entry(user: &str, service_name: &str) -> Result<SecretString, XmppError> {
    keyring::Entry::new(service_name, user)
        .map_err(|e| XmppError::Keyring(e.to_string()))?
        .get_password()
        .map(|pw| SecretString::new(pw.into()))
        .map_err(|e| XmppError::Keyring(e.to_string()))
}

pub fn set_entry(user: &str, service_name: &str, value: &str) -> Result<(), XmppError> {
    keyring::Entry::new(service_name, user)
        .map_err(|e| XmppError::Keyring(e.to_string()))?
        .set_password(value)
        .map_err(|e| XmppError::Keyring(e.to_string()))
}
