//! Sample use of the `xmpp` module.
//!
//! Defines a single message type (`Greet`) and the minimal state needed to
//! receive it. Wire this up via [`AppState`](crate::appstate::AppState) and
//! the commands in [`commands`](crate::commands).

use serde::{Deserialize, Serialize};
use std::sync::MutexGuard;

use crate::xmpp::XmppError;

/// The only message type in this demo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MyMessage {
    Greet(String),
}

/// A received greeting with its original sent time (Unix seconds, from XEP-0203 delay or receive time).
#[derive(Debug, Clone, Serialize)]
pub struct ReceivedGreeting {
    pub name: String,
    pub sent_at: i64,
}

/// Demo application state — keeps a log of received greetings.
#[derive(Debug, Default)]
pub struct MyState {
    pub received_greetings: Vec<ReceivedGreeting>,
}

/// Called by the XMPP task whenever a message arrives.
///
/// `sent_at` is a Unix timestamp (seconds) representing when the message was
/// originally sent, derived from the XEP-0203 delay element if present.
pub fn handle_incoming_message(
    message: MyMessage,
    state: &mut MutexGuard<'_, MyState>,
    sent_at: i64,
) {
    match message {
        MyMessage::Greet(name) => {
            eprintln!("[demo] greeted by {name} (sent at {sent_at})");
            state
                .received_greetings
                .push(ReceivedGreeting { name, sent_at });
        }
    }
}
