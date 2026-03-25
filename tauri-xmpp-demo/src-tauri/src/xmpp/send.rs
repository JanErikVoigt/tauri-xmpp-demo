use xmpp::jid::BareJid;

use tokio::sync::mpsc::Sender;
pub type MyTx = Sender<SendXMPPMessageRequest>;

pub enum SendXMPPMessageRequest {
    SendMessages {
        recipients: Vec<BareJid>,
        body: String,
    },
}
