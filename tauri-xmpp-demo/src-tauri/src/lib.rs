use std::sync::Mutex;

use appstate::AppState;
use demo_xmpp::{handle_incoming_message, MyState};
use tauri::Manager;
use tauri_xmpp::contacts;
use tauri_xmpp::contacts::ContactsState;
use tauri_xmpp::xmpp::XMPPMessager;
mod appstate;
mod commands;
mod demo_xmpp;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_my_jid,
            commands::set_jid,
            commands::set_password,
            commands::send_greet,
            commands::get_greetings,
            contacts::get_contacts,
            contacts::add_contact,
            contacts::remove_contact,
        ])
        .setup(|app| {
            let contacts_path = app.path().app_data_dir()?.join("contacts.json");

            app.manage(AppState {
                mystate: Mutex::new(MyState::default()),
                messager: Mutex::new(XMPPMessager::new(handle_incoming_message)),
            });
            app.manage(Mutex::new(ContactsState::load(contacts_path)));

            // Try to connect immediately; silently skips if credentials aren't set yet.
            app.state::<AppState>()
                .messager
                .lock()
                .unwrap()
                .restart(app.handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
