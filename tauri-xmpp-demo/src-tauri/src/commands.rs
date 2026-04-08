use crate::{
    appstate::AppState,
    demo_xmpp::{MyMessage, ReceivedGreeting},
};
use tauri::{AppHandle, State};
use tauri_xmpp::xmpp;
use tauri_xmpp::xmpp::{Jid, XmppError};

// ── XMPP connection ───────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_my_jid() -> Option<String> {
    xmpp::get_jid()
        .ok()
        .map(|j| j.bare_jid().as_str().to_string())
}

#[tauri::command]
pub fn set_jid(jid: String, app: AppHandle, state: State<'_, AppState>) -> Result<(), XmppError> {
    xmpp::set_jid(&jid)?;
    state.messager.lock().unwrap().restart(&app);
    Ok(())
}

#[tauri::command]
pub fn set_password(
    password: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), XmppError> {
    xmpp::set_password(&password)?;
    state.messager.lock().unwrap().restart(&app);
    Ok(())
}

// ── Greet ─────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn send_greet(to: Jid, name: String, state: State<'_, AppState>) -> Result<(), XmppError> {
    state
        .messager
        .lock()
        .unwrap()
        .send(&MyMessage::Greet(name), vec![to])
}

#[tauri::command]
pub fn get_greetings(state: State<'_, AppState>) -> Vec<ReceivedGreeting> {
    state
        .mystate
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .received_greetings
        .clone()
}
