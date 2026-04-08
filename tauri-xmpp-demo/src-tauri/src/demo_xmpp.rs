//! Sample use of the `xmpp` module.
//!
//! Defines a single message type (`Greet`) and the minimal state needed to
//! receive it. Wire this up via [`AppState`](crate::appstate::AppState) and
//! the commands in [`commands`](crate::commands).

use serde::{Deserialize, Serialize};
use std::sync::MutexGuard;

/// The only message type in this demo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MyMessage {
    Greet(String),
}

/// Demo application state — keeps a log of received greetings.
#[derive(Debug, Default)]
pub struct MyState {
    pub received_greetings: Vec<String>,
}

/// Called by the XMPP task whenever a message arrives.
pub fn handle_incoming_message(message: MyMessage, state: &mut MutexGuard<'_, MyState>) {
    match message {
        MyMessage::Greet(name) => {
            eprintln!("[demo] greeted by {name}");
            state.received_greetings.push(name);
        }
    }
}
