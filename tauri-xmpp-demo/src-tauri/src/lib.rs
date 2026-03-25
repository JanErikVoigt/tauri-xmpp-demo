use std::sync::Mutex;

use crate::{
    appstate::AppState,
    commands::restart_xmpp,
    demo_xmpp::MyState,
};

pub use tauri::Manager;

mod appstate;
mod commands;
mod demo_xmpp;
mod error;
mod secrets;
mod xmpp;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        //TODO .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::cmd_set_jid,
            commands::cmd_set_password,
            commands::cmd_send_greet,
            commands::cmd_befriend,
            commands::cmd_unfriend,
            commands::cmd_get_friends,
            commands::cmd_get_history,
            commands::cmd_get_my_jid,
        ])
        .setup(|app| {
            app.manage(AppState {
                mystate: Mutex::new(MyState::default()),
                messager: Mutex::new(None),
                xmpp_task: Mutex::new(None),
            });
            // Try to start immediately; silently skips if credentials aren't set yet.
            restart_xmpp(app.handle(), &app.state::<AppState>());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
