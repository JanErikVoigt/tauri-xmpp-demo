use secrecy::ExposeSecret;
use std::marker::PhantomData;
use std::{future::Future, task::Poll};
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;
use xmpp::ClientBuilder;
pub use xmpp::{message::send::MessageSettings, Event};

use crate::{
    error::TauriXMPPError,
    xmpp::{
        secrets::{get_jid, get_password},
        send::SendXMPPMessageRequest,
        HasMessager, Jid,
    },
};

pub struct Messager<M, S> {
    // pub tx: MyTx,
    _phantom: PhantomData<(M, S)>,
}

pub enum SpawnThreadResult {}

impl Future for SpawnThreadResult {
    type Output = Result<(), TauriXMPPError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Poll::Pending //TODO???
    }
}

impl<M: 'static, S: 'static> Messager<M, S> {
    /// Establish a persistent XMPP connection and handle all incoming and outgoing
    /// messages for the lifetime of the app.
    ///
    /// `event_handler` receives raw XMPP events and the app handle; all app-specific
    /// logic (message parsing, state mutation, notifications) lives there.
    pub fn spawn_xmpp_thread<AS>(
        app: AppHandle,
        event_handler: fn(Vec<Event>, AppHandle) -> SpawnThreadResult,
    ) where
        AS: HasMessager<M, S> + Send + Sync + 'static,
    {
        let me = get_jid()?;

        let password = match get_password() {
            Ok(pw) => pw,
            Err(e) => {
                eprintln!("[xmpp] password not found in keyring: {e}");
                return;
            }
        };

        tauri::async_runtime::spawn(async move {
            eprintln!("[xmpp] spawned connection task, building client");
            let mut agent = ClientBuilder::new(me.bare_jid().clone(), password.expose_secret())
                .set_resource("here-now")
                .build();
            drop(password); // zero memory as soon as ClientBuilder has copied it

            // Wait until online, collecting any events that arrive alongside Online.
            let mut post_online_events: Vec<Event> = Vec::new();
            'online: loop {
                eprintln!("[xmpp] waiting for Online event...");
                match tokio::time::timeout(
                    std::time::Duration::from_secs(10),
                    agent.wait_for_events(),
                )
                .await
                {
                    Ok(events) => {
                        eprintln!("[xmpp] got {} event(s)", events.len());
                        let mut online = false;
                        for event in events {
                            eprintln!("[xmpp] event: {event:?}");
                            if let Event::Online = event {
                                online = true;
                            } else if online {
                                post_online_events.push(event);
                            }
                        }
                        if online {
                            eprintln!("[xmpp] online!");
                            break 'online;
                        }
                    }
                    Err(_) => {
                        eprintln!("[xmpp] connection timeout waiting for Online");
                        return;
                    }
                }
            }

            // Register the outgoing sender in AppState.
            let (tx, mut rx) = mpsc::channel::<SendXMPPMessageRequest>(32);
            eprintln!("[xmpp] registering tx in AppState");
            app.state::<AS>().set_tx(tx);

            // Process any events that arrived in the same batch as Online.
            if !post_online_events.is_empty() {
                event_handler(post_online_events, app.clone()).await;
            }

            // Main loop: interleave incoming events and outgoing sends.
            loop {
                tokio::select! {
                    events = agent.wait_for_events() => {
                        event_handler(events, app.clone()).await;
                    }

                    msg = rx.recv() => {
                        match msg {
                            Some(SendXMPPMessageRequest::SendMessages { recipients, body }) => {
                                for jid in &recipients {
                                    agent
                                        .send_message(MessageSettings::new(jid.clone(), &body))
                                        .await;
                                }
                            }
                            None => break, // channel closed, connection shut down
                        }
                    }
                }
            }
        });
    }
}
