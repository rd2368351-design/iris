use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::{Error, Id};

/// Globally unique identifier for an email message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MessageId(Id);

impl MessageId {
    /// Creates a new message id.
    pub fn new(id: u64) -> Self {
        Self(Id::new(id))
    }

    /// Returns the underlying generic Id.
    pub fn id(&self) -> Id {
        self.0
    }

    /// Returns the numeric value.
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
    type Err = Error;

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
    fn roundtrip() {
        let id = MessageId::new(42);
        let text = id.to_string();
        let parsed = MessageId::from_str(&text).unwrap();

        assert_eq!(id, parsed);
    }
}