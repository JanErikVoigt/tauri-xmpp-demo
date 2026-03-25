use serde::Deserialize;
use std::{collections::HashSet, sync::MutexGuard};

#[derive(Debug, Default)]
pub struct MyState {
    pub name: String,
    pub message_history: Vec<MyMessage>,
    pub friends: HashSet<String>,
}

#[derive(Debug, Deserialize)]
pub enum MyMessage {
    Greet(String),
    Befriend(String),
    Unfriend(String),
}

pub fn handle_incoming_message(message: MyMessage, state: &mut MutexGuard<MyState>) {
    match message {
        MyMessage::Greet(name) => {
            eprintln!("[demo] Greet from {name}");
            state.name = name.clone();
            state.message_history.push(MyMessage::Greet(name));
        }
        MyMessage::Befriend(jid) => {
            eprintln!("[demo] Befriend {jid}");
            state.friends.insert(jid.clone());
            state.message_history.push(MyMessage::Befriend(jid));
        }
        MyMessage::Unfriend(jid) => {
            eprintln!("[demo] Unfriend {jid}");
            state.friends.remove(&jid);
            state.message_history.push(MyMessage::Unfriend(jid));
        }
    }
}
