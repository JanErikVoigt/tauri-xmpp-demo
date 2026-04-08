use crate::{
    secrets::{get_entry, set_entry},
    xmpp::{error::XmppError, Jid},
};
use secrecy::ExposeSecret;
use secrecy::SecretString;

const KEYRING_SERVICE: &str = "de_janerikvoigt_here_now_location";
const JID_USER: &str = "xmpp_jid";
const PASSWORD_USER: &str = "xmpp_password";

pub fn get_password() -> Result<SecretString, XmppError> {
    get_entry(PASSWORD_USER, KEYRING_SERVICE)
}

pub fn set_password(new_val: &str) -> Result<(), XmppError> {
    set_entry(PASSWORD_USER, KEYRING_SERVICE, new_val)
}

pub fn get_jid() -> Result<Jid, XmppError> {
    Jid::new(get_entry(JID_USER, KEYRING_SERVICE)?.expose_secret())
}

pub fn set_jid(new_val: &str) -> Result<(), XmppError> {
    set_entry(JID_USER, KEYRING_SERVICE, new_val)
}
