use crate::{
    demo_xmpp::MyState,
    xmpp::{HasMessageSender, MessageSender, StateModifiedByXMPP},
};
pub use std::sync::Mutex;
use std::sync::MutexGuard;

pub struct AppState {
    pub mystate: Mutex<MyState>,
    pub messager: Mutex<Option<MessageSender>>,
    pub xmpp_task: Mutex<Option<tauri::async_runtime::JoinHandle<()>>>,
}

impl HasMessageSender for AppState {
    fn set_tx(&self, tx: MessageSender) {
        *self.messager.lock().unwrap_or_else(|e| e.into_inner()) = Some(tx);
    }
}

impl StateModifiedByXMPP<MyState> for AppState {
    fn xmpp_state(&self) -> MutexGuard<'_, MyState> {
        self.mystate.lock().unwrap_or_else(|e| e.into_inner())
    }
}
