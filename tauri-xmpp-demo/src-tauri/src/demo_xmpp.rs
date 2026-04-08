//! Sample use of the `xmpp` module.
//!
//! Defines a single message type (`Greet`) and the minimal state needed to
//! receive it. Wire this up via [`AppState`](crate::appstate::AppState) and
//! the commands in [`commands`](crate::commands).

use serde::{Deserialize, Serialize};
use std::sync::MutexGuard;

use crate::xmpp::IncomingMessage;

/// The only message type in this demo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MyMessage {
    Greet(String),
}

/// A received greeting stored in app state.
#[derive(Debug, Clone, Serialize)]
pub struct ReceivedGreeting {
    pub name: String,
    pub from: String,
    pub sent_at: i64,
}

/// Demo application state — keeps a log of received greetings.
#[derive(Debug, Default)]
pub struct MyState {
    pub received_greetings: Vec<ReceivedGreeting>,
}

/// Called by the XMPP task whenever a message arrives.
pub fn handle_incoming_message(
    incoming: IncomingMessage<MyMessage>,
    state: &mut MutexGuard<'_, MyState>,
) {
    match incoming.message {
        MyMessage::Greet(name) => {
            eprintln!("[demo] greeted by {name} from {} (sent at {})", incoming.from, incoming.sent_at);
            state.received_greetings.push(ReceivedGreeting {
                name,
                from: incoming.from.to_string(),
                sent_at: incoming.sent_at,
            });
        }
    }
}
