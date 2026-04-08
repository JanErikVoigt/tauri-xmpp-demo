use crate::xmpp::error::XmppError;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use xmpp::jid::BareJid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Jid(BareJid);

impl Jid {
    pub fn new(from: &str) -> Result<Self, XmppError> {
        BareJid::new(from).map(Jid).map_err(XmppError::InvalidJid)
    }

    pub fn bare_jid(&self) -> &BareJid {
        &self.0
    }
}

impl std::fmt::Display for Jid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for Jid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.bare_jid().as_str())
    }
}

impl<'de> Deserialize<'de> for Jid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        BareJid::new(&s).map(Jid).map_err(D::Error::custom)
    }
}
