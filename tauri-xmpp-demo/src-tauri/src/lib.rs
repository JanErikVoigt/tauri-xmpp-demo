use std::sync::Mutex;

use crate::{
    appstate::AppState,
    demo_xmpp::{handle_incoming_message, MyMessage, MyState},
    xmpp::spawn_xmpp_thread,
};

pub use tauri::Manager;

mod appstate;
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
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            app.manage(AppState {
                mystate: Mutex::new(MyState::default()),
                messager: Mutex::new(None),
            });
            let _ = spawn_xmpp_thread::<AppState, MyMessage, MyState>(
                app.handle().clone(),
                handle_incoming_message,
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
