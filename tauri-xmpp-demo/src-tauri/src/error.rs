#[derive(Debug, thiserror::Error)]
pub enum TauriXMPPError {
    #[error("invalid JID: {0:?}")]
    JidError(xmpp::jid::Error),

    #[error("failed to serialize message")]
    Serialize,

    #[error("keyring error: {0}")]
    KeyringError(String),

    #[error("not connected — credentials may not be set yet")]
    NotConnected,

    #[error("XMPP send channel is full")]
    ChannelFull,

    #[error("XMPP send channel closed")]
    ChannelClosed,
}

impl serde::Serialize for TauriXMPPError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
