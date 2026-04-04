mod jid;
use std::sync::MutexGuard;

pub use jid::Jid;
mod messager;
pub use messager::spawn_xmpp_thread;

mod secrets;
pub use secrets::{get_jid, set_jid, set_password};

use crate::xmpp::send::{SendXMPPMessageRequest, XMPPMessager};
use tokio::sync::mpsc::Sender;
pub mod send;

pub type MessageTx = Sender<SendXMPPMessageRequest>;

/// Implemented by the Tauri app state to allow the XMPP layer to register its outgoing channel.
pub trait HasMessageSender<A, M, S>: Send + Sync + 'static {
    fn set_tx(&self, tx: MessageTx);
}

pub trait StateModifiedByXMPP<S> {
    // fn get_state(&self) -> S;
    // fn set_state(&mut self, new_state: S);
    fn xmpp_state(&self) -> MutexGuard<S>;
}
