use xmpp::jid::BareJid;

use tokio::sync::mpsc::Sender;
pub type MessageSender = Sender<SendXMPPMessageRequest>;

pub enum SendXMPPMessageRequest {
    SendMessages {
        recipients: Vec<BareJid>,
        body: String,
    },
}
