#[derive(Debug, thiserror::Error)]
pub enum TauriXMPPError {
    #[error("Failed creating jid: {0:?}")]
    JidError(xmpp::jid::Error),

    #[error("failed serializing")]
    SerdeSerialize,

    #[error("keyring error: {0}")]
    KeyringError(String),

    #[error("not connected — call spawn_xmpp_thread first")]
    NotConnected,

    #[error("XMPP send channel closed")]
    ChannelClosed,
}

impl serde::Serialize for TauriXMPPError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
