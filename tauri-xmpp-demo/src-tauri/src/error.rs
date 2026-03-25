#[derive(Debug, thiserror::Error)]
pub enum TauriXMPPError {
    #[error("Failed creating jid: {0:?}")]
    JidError(xmpp::jid::Error),

    #[error("failed serializing")]
    SerdeSerialize,

    #[error("keyring error: {0}")]
    KeyringError(String),
}
