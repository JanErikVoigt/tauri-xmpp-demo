use std::sync::Mutex;

use appstate::AppState;
use contacts::ContactsState;
use demo_xmpp::{handle_incoming_message, MyState};
use tauri::Manager;
use xmpp::XMPPMessager;

mod appstate;
mod commands;
mod contacts;
mod demo_xmpp;
mod secrets;
mod xmpp;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::cmd_get_my_jid,
            commands::cmd_set_jid,
            commands::cmd_set_password,
            commands::cmd_send_greet,
            commands::cmd_get_greetings,
            commands::cmd_get_contacts,
            commands::cmd_add_contact,
            commands::cmd_remove_contact,
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
