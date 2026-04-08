use std::sync::{Mutex, MutexGuard};

use crate::{
    demo_xmpp::{MyMessage, MyState},
    xmpp::{HasXmppSender, MessageTx, XMPPMessager, XmppStateAccess},
};

pub struct AppState {
    pub mystate: Mutex<MyState>,
    pub messager: Mutex<XMPPMessager<AppState, MyMessage, MyState>>,
}

impl HasXmppSender for AppState {
    fn set_tx(&self, tx: MessageTx) {
        self.messager.lock().unwrap().set_tx(tx);
    }
}

impl XmppStateAccess<MyState> for AppState {
    fn xmpp_state(&self) -> MutexGuard<'_, MyState> {
        self.mystate.lock().unwrap_or_else(|e| e.into_inner())
    }
}
