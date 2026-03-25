mod jid;
use std::sync::MutexGuard;

pub use jid::Jid;
mod messager;
pub use messager::spawn_xmpp_thread;

pub use crate::xmpp::send::MessageSender;
mod secrets;
mod send;

/// Implemented by the Tauri app state to allow the XMPP layer to register its outgoing channel.
pub trait HasMessageSender: Send + Sync + 'static {
    fn set_tx(&self, tx: MessageSender);
}

pub trait StateModifiedByXMPP<S> {
    // fn get_state(&self) -> S;
    // fn set_state(&mut self, new_state: S);
    fn xmpp_state(&self) -> MutexGuard<S>;
}
