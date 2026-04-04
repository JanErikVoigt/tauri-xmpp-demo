use tauri::App;

use crate::{
    demo_xmpp::{MyMessage, MyState},
    xmpp::{send::XMPPMessager, HasMessageSender, MessageTx, StateModifiedByXMPP},
};
pub use std::sync::Mutex;
use std::sync::MutexGuard;

pub struct AppState {
    pub mystate: Mutex<MyState>,
    pub messager: Mutex<Option<XMPPMessager<AppState, MyMessage, MyState>>>,
}

impl HasMessageSender<AppState, MyMessage, MyState> for AppState {
    fn set_tx(&self, tx: MessageTx) {
        self.messager
            .lock()
            .as_mut()
            .unwrap()
            .as_mut()
            .unwrap()
            .set_tx(tx);
    }
}

impl StateModifiedByXMPP<MyState> for AppState {
    fn xmpp_state(&self) -> MutexGuard<'_, MyState> {
        self.mystate.lock().unwrap_or_else(|e| e.into_inner())
    }
}
