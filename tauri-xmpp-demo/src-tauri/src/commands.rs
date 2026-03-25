use tauri::{AppHandle, State};
use xmpp::jid::BareJid;

use crate::{
    appstate::AppState,
    demo_xmpp::{handle_incoming_message, MyMessage, MyState},
    error::TauriXMPPError,
    xmpp::{
        send::SendXMPPMessageRequest,
        get_jid, set_jid, set_password, spawn_xmpp_thread,
    },
};

/// Abort any running XMPP task and try to start a fresh one.
/// Silently ignores "credentials not set" errors — those are expected when only
/// one of JID / password has been configured so far.
pub fn restart_xmpp(app: &AppHandle, state: &AppState) {
    // Abort existing task and clear the outgoing channel.
    if let Some(handle) = state.xmpp_task.lock().expect("task lock").take() {
        handle.abort();
    }
    *state.messager.lock().expect("messager lock") = None;

    match spawn_xmpp_thread::<AppState, MyMessage, MyState>(app.clone(), handle_incoming_message) {
        Ok(handle) => {
            *state.xmpp_task.lock().expect("task lock") = Some(handle);
            eprintln!("[xmpp] thread (re)started");
        }
        Err(e) => {
            eprintln!("[xmpp] could not start thread (credentials not ready?): {e}");
        }
    }
}

#[tauri::command]
pub fn cmd_get_my_jid() -> Option<String> {
    get_jid().ok().map(|jid| jid.bare_jid().as_str().to_string())
}

#[tauri::command]
pub fn cmd_get_friends(state: State<'_, AppState>) -> Vec<String> {
    let mut friends: Vec<String> = state
        .mystate
        .lock()
        .expect("state lock")
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
        .expect("state lock")
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
    restart_xmpp(&app, &state);
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
    to: String,
    name: String,
    state: State<'_, AppState>,
) -> Result<(), TauriXMPPError> {
    send_message(MyMessage::Greet(name), &to, &state).await
}

#[tauri::command]
pub async fn cmd_befriend(
    jid: String,
    state: State<'_, AppState>,
) -> Result<(), TauriXMPPError> {
    send_message(MyMessage::Befriend(jid.clone()), &jid, &state).await
}

#[tauri::command]
pub async fn cmd_unfriend(
    jid: String,
    state: State<'_, AppState>,
) -> Result<(), TauriXMPPError> {
    send_message(MyMessage::Unfriend(jid.clone()), &jid, &state).await
}

async fn send_message(
    message: MyMessage,
    recipient: &str,
    state: &AppState,
) -> Result<(), TauriXMPPError> {
    let body = serde_json::to_string(&message).map_err(|_| TauriXMPPError::SerdeSerialize)?;
    let jid = BareJid::new(recipient).map_err(TauriXMPPError::JidError)?;
    let tx = state
        .messager
        .lock()
        .expect("messager lock")
        .clone()
        .ok_or(TauriXMPPError::NotConnected)?;
    tx.send(SendXMPPMessageRequest::SendMessages {
        recipients: vec![jid],
        body,
    })
    .await
    .map_err(|_| TauriXMPPError::ChannelClosed)
}
