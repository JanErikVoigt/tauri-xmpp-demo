use std::{marker::PhantomData, sync::MutexGuard};

use secrecy::ExposeSecret;
use serde::{de::DeserializeOwned, Serialize};
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;
use xmpp::{delay::StanzaTimeInfo, message::send::MessageSettings, ClientBuilder, Event};

use crate::xmpp::{
    error::XmppError,
    secrets::{get_jid, get_password},
    HasXmppSender, Jid, MessageTx, OutgoingMessage, XmppStateAccess,
};

pub struct XMPPMessager<A, M, S> {
    message_handler: fn(M, &mut MutexGuard<'_, S>, i64),
    tx: Option<mpsc::Sender<OutgoingMessage>>,
    join_handle: Option<tauri::async_runtime::JoinHandle<()>>,
    _phantom: PhantomData<(A, S)>,
}

impl<A, M, S> XMPPMessager<A, M, S>
where
    A: HasXmppSender + XmppStateAccess<S> + Send + Sync + 'static,
    M: DeserializeOwned + Send + 'static,
    S: Send + 'static,
{
    pub fn new(message_handler: fn(M, &mut MutexGuard<'_, S>, i64)) -> Self {
        Self {
            message_handler,
            tx: None,
            join_handle: None,
            _phantom: PhantomData,
        }
    }

    /// Abort any running connection and start a fresh one.
    /// Silently returns if credentials are not yet configured.
    pub fn restart(&mut self, app: &AppHandle) {
        if let Some(h) = self.join_handle.take() {
            h.abort();
        }
        self.tx = None;

        let me = match get_jid() {
            Ok(j) => j,
            Err(e) => {
                eprintln!("[xmpp] credentials not ready: {e}");
                return;
            }
        };
        let password = match get_password() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("[xmpp] credentials not ready: {e}");
                return;
            }
        };

        let app = app.clone();
        let handler = self.message_handler;
        let handle = tauri::async_runtime::spawn(async move {
            run_xmpp_task::<A, M, S>(app, me, password, handler).await;
        });
        self.join_handle = Some(handle);
        eprintln!("[xmpp] connection task spawned");
    }

    pub fn set_tx(&mut self, tx: MessageTx) {
        self.tx = Some(tx);
    }

    /// Serialize and queue a message for delivery. Non-blocking (channel capacity 100).
    pub fn send(&self, msg: &M, recipients: Vec<Jid>) -> Result<(), XmppError>
    where
        M: Serialize,
    {
        let tx = self.tx.as_ref().ok_or(XmppError::NotConnected)?;
        let body = serde_json::to_string(msg).map_err(|_| XmppError::Serialize)?;
        let out = OutgoingMessage {
            recipients: recipients.into_iter().map(|j| j.bare_jid().clone()).collect(),
            body,
        };
        tx.try_send(out).map_err(|e| match e {
            mpsc::error::TrySendError::Full(_) => XmppError::ChannelFull,
            mpsc::error::TrySendError::Closed(_) => XmppError::ChannelClosed,
        })
    }
}

async fn run_xmpp_task<A, M, S>(
    app: AppHandle,
    me: Jid,
    password: secrecy::SecretString,
    message_handler: fn(M, &mut MutexGuard<'_, S>, i64),
) where
    A: HasXmppSender + XmppStateAccess<S> + Send + Sync + 'static,
    M: DeserializeOwned + Send + 'static,
    S: Send + 'static,
{
    eprintln!("[xmpp] building client");
    let mut agent = ClientBuilder::new(me.bare_jid().clone(), password.expose_secret())
        .set_resource("tauri-demo")
        .build();
    drop(password); // zero memory as soon as the builder has copied it

    // Wait until the connection comes online.
    let mut post_online: Vec<Event> = Vec::new();
    'wait: loop {
        eprintln!("[xmpp] waiting for Online...");
        match tokio::time::timeout(
            std::time::Duration::from_secs(10),
            agent.wait_for_events(),
        )
        .await
        {
            Ok(events) => {
                let mut got_online = false;
                for event in events {
                    if let Event::Online = event {
                        got_online = true;
                    } else if got_online {
                        post_online.push(event);
                    }
                }
                if got_online {
                    eprintln!("[xmpp] online");
                    break 'wait;
                }
            }
            Err(_) => {
                eprintln!("[xmpp] timeout waiting for Online");
                return;
            }
        }
    }

    // Register the outgoing channel in AppState.
    let (tx, mut rx) = mpsc::channel::<OutgoingMessage>(100);
    app.state::<A>().set_tx(tx);

    // Handle any events that arrived in the same batch as Online.
    if !post_online.is_empty() {
        process_events::<A, M, S>(&post_online, &app, message_handler);
    }

    // Main loop: interleave incoming events with outgoing sends.
    loop {
        tokio::select! {
            events = agent.wait_for_events() => {
                process_events::<A, M, S>(&events, &app, message_handler);
            }
            msg = rx.recv() => {
                match msg {
                    Some(OutgoingMessage { recipients, body }) => {
                        for jid in &recipients {
                            agent.send_message(MessageSettings::new(jid.clone(), &body)).await;
                        }
                    }
                    None => break, // channel closed — shut down
                }
            }
        }
    }
}

/// Extract the Unix timestamp (seconds) that best represents when a message was sent.
///
/// Uses the first XEP-0203 delay stamp if present — this is the original sent time
/// stamped by the server when storing messages for offline delivery. Falls back to
/// the library's own receive time if no delay info is available.
fn sent_timestamp(time_info: &StanzaTimeInfo) -> i64 {
    time_info
        .delays
        .first()
        .map(|d| d.stamp.0.timestamp())
        .unwrap_or_else(|| time_info.received.timestamp())
}

fn process_events<A, M, S>(
    events: &[Event],
    app: &AppHandle,
    message_handler: fn(M, &mut MutexGuard<'_, S>, i64),
) where
    A: XmppStateAccess<S> + Send + Sync + 'static,
    M: DeserializeOwned,
{
    let state = app.state::<A>();
    let mut s = state.xmpp_state();
    for event in events {
        if let Event::ChatMessage(_id, _from, body, time_info) = event {
            if let Ok(msg) = serde_json::from_str::<M>(body) {
                message_handler(msg, &mut s, sent_timestamp(time_info));
            }
        }
    }
}
