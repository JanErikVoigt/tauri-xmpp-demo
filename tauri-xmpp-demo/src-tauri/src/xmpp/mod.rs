mod jid;
pub use jid::Jid;
mod messager;
pub use messager::*;

pub use crate::xmpp::send::MessageSender;
mod secrets;
mod send;

/// Implemented by the Tauri app state to allow the XMPP layer to register its outgoing channel.
pub trait HasMessageSender: Send + Sync + 'static {
    fn set_tx(&self, tx: MessageSender);
}
