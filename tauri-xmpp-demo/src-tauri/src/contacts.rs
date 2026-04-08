use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
