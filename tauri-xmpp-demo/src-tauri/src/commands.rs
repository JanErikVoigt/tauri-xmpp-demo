use tauri::{AppHandle, State};
use xmpp::jid::BareJid;

use crate::{
    appstate::AppState,
    demo_xmpp::{handle_incoming_message, MyMessage, MyState},
    error::TauriXMPPError,
    xmpp::{
        get_jid,
        send::{SendXMPPMessageRequest, XMPPMessager},
        set_jid, set_password, spawn_xmpp_thread, Jid,
    },
};

#[tauri::command]
pub fn cmd_get_my_jid() -> Option<String> {
    get_jid()
        .ok()
        .map(|jid| jid.bare_jid().as_str().to_string())
}

#[tauri::command]
pub fn cmd_get_friends(state: State<'_, AppState>) -> Vec<String> {
    let mut friends: Vec<String> = state
        .mystate
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .friends
        .iter()
        .cloned()
        .collect();
    friends.sort();
    friends
}

#[tauri::command]
pub fn cmd_get_history(state: State<'_, AppState>) -> Vec<MyMessage> {
    state
        .mystate
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .message_history
        .clone()
}

#[tauri::command]
pub fn cmd_set_jid(
    jid: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), TauriXMPPError> {
    set_jid(&jid)?;
    let state = state.lock();

    XMPPMessager::restart_xmpp(&app, &state);
    Ok(())
}

#[tauri::command]
pub fn cmd_set_password(
    password: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), TauriXMPPError> {
    set_password(&password)?;
    restart_xmpp(&app, &state);
    Ok(())
}

#[tauri::command]
pub async fn cmd_send_greet(
    to: Jid,
    name: String,
    state: State<'_, AppState>,
) -> Result<(), TauriXMPPError> {
    send_message(&MyMessage::Greet(name), &to, &state).await
}

#[tauri::command]
pub async fn cmd_befriend(jid: Jid, state: State<'_, AppState>) -> Result<(), TauriXMPPError> {
    send_message(&MyMessage::Befriend(jid.clone()), &jid, &state).await
}

#[tauri::command]
pub async fn cmd_unfriend(jid: Jid, state: State<'_, AppState>) -> Result<(), TauriXMPPError> {
    send_message(&MyMessage::Unfriend(jid.clone()), &jid, &state).await
}

async fn send_message(
    message: &MyMessage,
    recipient: &Jid,
    state: &AppState,
) -> Result<(), TauriXMPPError> {
    state
        .messager
        .lock()
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap()
        .send(message, vec![recipient.clone()])
        .await
}
