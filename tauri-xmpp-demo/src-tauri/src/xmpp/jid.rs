use crate::error::TauriXMPPError;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use xmpp::jid::BareJid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Jid(BareJid);

impl Jid {
    pub fn new(from: &str) -> Result<Self, TauriXMPPError> {
        Ok(Jid(
            BareJid::new(&from).map_err(|e| TauriXMPPError::JidError(e))?
        ))
    }

    pub fn bare_jid(&self) -> &BareJid {
        return &self.0;
    }
}

impl Serialize for Jid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.bare_jid().as_str())
    }
}

impl<'de> Deserialize<'de> for Jid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        BareJid::new(&s).map(Jid).map_err(|e| D::Error::custom(e))
    }
}
