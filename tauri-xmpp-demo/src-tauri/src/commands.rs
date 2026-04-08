use tauri::{AppHandle, State};

use crate::{
    appstate::AppState,
    demo_xmpp::MyMessage,
    xmpp::{get_jid, set_jid, set_password, Jid, XmppError},
};

#[tauri::command]
pub fn cmd_get_my_jid() -> Option<String> {
    get_jid().ok().map(|j| j.bare_jid().as_str().to_string())
}

#[tauri::command]
pub fn cmd_set_jid(
    jid: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), XmppError> {
    set_jid(&jid)?;
    state.messager.lock().unwrap().restart(&app);
    Ok(())
}

#[tauri::command]
pub fn cmd_set_password(
    password: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), XmppError> {
    set_password(&password)?;
    state.messager.lock().unwrap().restart(&app);
    Ok(())
}

#[tauri::command]
pub fn cmd_send_greet(
    to: Jid,
    name: String,
    state: State<'_, AppState>,
) -> Result<(), XmppError> {
    state
        .messager
        .lock()
        .unwrap()
        .send(&MyMessage::Greet(name), vec![to])
}

#[tauri::command]
pub fn cmd_get_greetings(state: State<'_, AppState>) -> Vec<String> {
    state
        .mystate
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .received_greetings
        .clone()
}
