#[derive(Debug, thiserror::Error)]
pub enum XmppError {
    #[error("invalid JID: {0:?}")]
    InvalidJid(xmpp::jid::Error),

    #[error("failed to serialize message")]
    Serialize,

    #[error("keyring error: {0}")]
    Keyring(String),

    #[error("not connected — credentials may not be set yet")]
    NotConnected,

    #[error("XMPP send channel is full")]
    ChannelFull,

    #[error("XMPP send channel closed")]
    ChannelClosed,
}

/// Required for Tauri commands to return this error across the IPC bridge.
impl serde::Serialize for XmppError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
