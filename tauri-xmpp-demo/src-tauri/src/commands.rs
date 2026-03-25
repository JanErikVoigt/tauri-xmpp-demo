use tauri::State;
use xmpp::jid::BareJid;

use crate::{
    appstate::AppState,
    demo_xmpp::MyMessage,
    error::TauriXMPPError,
    xmpp::{
        send::SendXMPPMessageRequest,
        set_jid, set_password,
    },
};

#[tauri::command]
pub fn cmd_get_friends(state: State<'_, AppState>) -> Vec<String> {
    let mut friends: Vec<String> = state.mystate.lock().expect("state lock").friends.iter().cloned().collect();
    friends.sort();
    friends
}

#[tauri::command]
pub fn cmd_get_history(state: State<'_, AppState>) -> Vec<MyMessage> {
    state.mystate.lock().expect("state lock").message_history.clone()
}

#[tauri::command]
pub fn cmd_set_jid(jid: String) -> Result<(), TauriXMPPError> {
    set_jid(&jid)
}

#[tauri::command]
pub fn cmd_set_password(password: String) -> Result<(), TauriXMPPError> {
    set_password(&password)
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
