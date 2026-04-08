pub mod error;
pub use error::XmppError;

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
