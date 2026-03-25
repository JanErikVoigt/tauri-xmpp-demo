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

pub fn handle_incoming_message(_message: MyMessage, _state: &mut MutexGuard<MyState>) {}
