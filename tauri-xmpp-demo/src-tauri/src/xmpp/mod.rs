pub mod error;
pub use error::XmppError;

/// Wrapper passed to every message handler containing routing metadata alongside
/// the application-level message.
pub struct IncomingMessage<M> {
    /// Original send time: first XEP-0203 delay stamp if present, otherwise receive time.
    pub sent_at: i64,
    /// Bare JID of the sender.
    pub from: Jid,
    /// Deserialized application message.
    pub message: M,
}

/// Tauri event names emitted by the XMPP layer to the frontend.
pub mod events {
    /// One or more new messages were received and stored. No payload.
    pub const MESSAGE: &str = "xmpp:message";
    /// The XMPP connection came online. No payload.
    pub const ONLINE: &str = "xmpp:online";
    /// The XMPP connection went offline. Payload: error reason (`String`), empty if clean.
    pub const OFFLINE: &str = "xmpp:offline";
    /// A non-fatal error occurred (e.g. unrecognised message body). Payload: description (`String`).
    pub const ERROR: &str = "xmpp:error";
}

mod jid;
pub use jid::Jid;

pub mod connection;
pub use connection::XMPPMessager;

mod secrets;
pub use secrets::{get_jid, set_jid, set_password};

use std::sync::MutexGuard;
use tokio::sync::mpsc::Sender;
use xmpp::jid::BareJid;

pub(crate) struct OutgoingMessage {
    pub recipients: Vec<BareJid>,
    pub body: String,
}

pub type MessageTx = Sender<OutgoingMessage>;

/// Implemented by AppState to let the XMPP task register its outgoing channel.
pub trait HasXmppSender: Send + Sync + 'static {
    fn set_tx(&self, tx: MessageTx);
}

/// Implemented by AppState to give the XMPP task mutable access to the inner state.
pub trait XmppStateAccess<S>: Send + Sync + 'static {
    fn xmpp_state(&self) -> MutexGuard<'_, S>;
}
