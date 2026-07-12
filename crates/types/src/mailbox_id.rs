use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Id;

/// Strongly-typed identifier for a mailbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MailboxId(Id);

impl MailboxId {
    /// Create a new mailbox identifier.
    pub fn new(id: Id) -> Self {
        Self(id)
    }

    /// Returns the wrapped generic identifier.
    pub fn id(self) -> Id {
        self.0
    }

    /// Returns the raw numeric value.
    pub fn value(self) -> u64 {
        self.0.value()
    }
}

impl fmt::Display for MailboxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for MailboxId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Id::from_str(s)?))
    }
}

impl From<Id> for MailboxId {
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<MailboxId> for Id {
    fn from(id: MailboxId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_generic_id() {
        let id = Id::new(100);
        let mailbox = MailboxId::new(id);

        assert_eq!(mailbox.id(), id);
        assert_eq!(mailbox.value(), 100);
    }

    #[test]
    fn roundtrip() {
        let id = MailboxId::new(Id::new(12345));
        let text = id.to_string();
        let parsed: MailboxId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}