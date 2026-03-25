use crate::{
    demo_xmpp::{MyMessage, MyState},
    xmpp::{HasMessageSender, MessageSender},
};
pub use std::sync::Mutex;

pub struct AppState {
    pub mystate: Mutex<MyState>,
    pub messager: Mutex<Option<MessageSender>>,
}

impl HasMessageSender for AppState {
    fn set_tx(&self, tx: MessageSender) {}
}
