use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct MyState {
    pub name: String,
    pub message_history: Vec<MyMessage>,
    pub friends: HashSet<String>,
}
// impl MyState {
//     pub
// }

#[derive(Debug)]
pub enum MyMessage {
    Greet(String),
    Befriend(String),
    Unfriend(String),
}

pub fn handle_incoming_events() {}
