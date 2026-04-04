use std::{marker::PhantomData, sync::MutexGuard};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tauri::AppHandle;
use xmpp::jid::BareJid;

use tokio::sync::mpsc::Sender;

use crate::{
    appstate::AppState,
    error::TauriXMPPError,
    xmpp::{spawn_xmpp_thread, HasMessageSender, Jid, MessageTx, StateModifiedByXMPP},
};

pub struct XMPPMessager<A: HasMessageSender<A, M, S>, M, S> {
    handle_incoming_message: fn(M, &mut MutexGuard<S>),
    tx: Option<Sender<SendXMPPMessageRequest>>,
    join_handle: Option<tauri::async_runtime::JoinHandle<()>>,
    _a: PhantomData<A>,
}

impl<'de, A, M, S> XMPPMessager<A, M, S>
where
    A: HasMessageSender<A, M, S> + StateModifiedByXMPP<S>,
    M: Send + Sync + DeserializeOwned + Serialize + 'static,
    S: Send + Sync + 'static,
{
    /// Abort any running XMPP task and try to start a fresh one.
    /// Silently ignores "credentials not set" errors — those are expected when only
    /// one of JID / password has been configured so far.
    pub fn restart_xmpp(&mut self, app: &AppHandle, state: &AppState) {
        // Abort existing task and clear the outgoing channel.
        if self.join_handle.is_some() {
            self.join_handle.as_ref().unwrap().abort();
        }

        self.tx = None;

        match spawn_xmpp_thread::<A, M, S>(app.clone(), self.handle_incoming_message) {
            Ok(handle) => {
                self.join_handle = Some(handle);
                eprintln!("[xmpp] thread (re)started");
            }
            Err(e) => {
                eprintln!("[xmpp] could not start thread (credentials not ready?): {e}");
            }
        }
    }

    pub fn set_tx(&mut self, tx: MessageTx) {
        self.tx = Some(tx);
    }

    pub async fn send(&mut self, message: &M, recepients: Vec<Jid>) -> Result<(), TauriXMPPError> {
        let req = SendXMPPMessageRequest::SendMessages {
            recipients: recepients.iter().map(|j| j.bare_jid().clone()).collect(),
            body: serde_json::to_string(message).map_err(|_e| TauriXMPPError::SerdeSerialize)?,
        };

        if self.tx.is_some() {
            self.tx
                .as_mut()
                .unwrap()
                .send(req)
                .await
                .map_err(|_e| TauriXMPPError::ChannelClosed)?;
            Ok(())
        } else {
            Err(TauriXMPPError::NotConnected)
        }
    }
}

pub enum SendXMPPMessageRequest {
    SendMessages {
        recipients: Vec<BareJid>,
        body: String,
    },
}
