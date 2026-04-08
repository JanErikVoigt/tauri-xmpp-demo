use crate::xmpp::{Jid, XmppError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub jid: String,
    pub display_name: String,
}

pub struct ContactsState {
    pub contacts: Vec<Contact>,
    path: PathBuf,
}

impl ContactsState {
    pub fn load(path: PathBuf) -> Self {
        let contacts = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        Self { contacts, path }
    }

    /// Add or update a contact (upsert by JID).
    pub fn upsert(&mut self, jid: String, display_name: String) {
        if let Some(existing) = self.contacts.iter_mut().find(|c| c.jid == jid) {
            existing.display_name = display_name;
        } else {
            self.contacts.push(Contact { jid, display_name });
        }
        self.save();
    }

    pub fn remove(&mut self, jid: &str) {
        self.contacts.retain(|c| c.jid != jid);
        self.save();
    }

    fn save(&self) {
        if let Some(parent) = self.path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        match serde_json::to_string_pretty(&self.contacts) {
            Ok(json) => {
                if let Err(e) = std::fs::write(&self.path, json) {
                    eprintln!("[contacts] failed to save: {e}");
                }
            }
            Err(e) => eprintln!("[contacts] failed to serialize: {e}"),
        }
    }
}

// ── Commands ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_contacts(contacts: State<'_, Mutex<ContactsState>>) -> Vec<Contact> {
    contacts.lock().unwrap().contacts.clone()
}

#[tauri::command]
pub fn add_contact(
    jid: String,
    display_name: String,
    contacts: State<'_, Mutex<ContactsState>>,
) -> Result<(), XmppError> {
    Jid::new(&jid)?; // validate before storing
    contacts.lock().unwrap().upsert(jid, display_name);
    Ok(())
}

#[tauri::command]
pub fn remove_contact(jid: String, contacts: State<'_, Mutex<ContactsState>>) {
    contacts.lock().unwrap().remove(&jid);
}
