use crate::{
    demo_xmpp::{MyMessage, MyState},
    xmpp::{HasMessager, Messager},
};
pub use std::sync::Mutex;

pub struct AppState {
    pub mystate: Mutex<MyState>,
    pub messager: Mutex<Option<Messager<MyMessage, MyState>>>,
}

impl HasMessager for AppState {
    fn set_tx(&self, tx: MyTx) {}
}
