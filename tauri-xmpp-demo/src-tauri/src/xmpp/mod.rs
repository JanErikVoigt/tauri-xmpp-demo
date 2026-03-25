mod jid;
pub use jid::Jid;
mod messager;
pub use messager::*;

use crate::xmpp::send::MyTx;
mod secrets;
mod send;

/// Implemented by the Tauri app state to allow the XMPP layer to register its outgoing channel.
pub trait HasMessager<M, S>: Send + Sync + 'static {
    fn set_tx(&self, tx: MyTx);
}
