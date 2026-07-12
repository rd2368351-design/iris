use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Id;

/// Globally unique identifier for an email message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MessageId(pub(crate) Id);

impl MessageId {
    pub fn new(id: Id) -> Self {
        Self(id)
    }

    pub fn id(&self) -> Id {
        self.0
    }

    pub fn value(&self) -> u64 {
        self.0.value()
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for MessageId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Id::from_str(s)?))
    }
}

impl From<Id> for MessageId {
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<MessageId> for Id {
    fn from(id: MessageId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_generic_id() {
        let id = Id::new(999);
        let msg = MessageId::new(id);

        assert_eq!(msg.id(), id);
        assert_eq!(msg.value(), 999);
    }

    #[test]
    fn roundtrip() {
        let id = MessageId::new(Id::new(12345));
        let text = id.to_string();
        let parsed: MessageId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}