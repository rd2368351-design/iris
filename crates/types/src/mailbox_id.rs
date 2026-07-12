use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Id;

/// Strongly-typed identifier for a mailbox.
///
/// A mailbox is a container for messages (e.g., Inbox, Sent, Drafts).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MailboxId(pub(crate) Id);

impl MailboxId {
    #[inline]
    pub const fn new(id: u64) -> Self {
        Self(Id::new(id))
    }

    #[inline]
    pub const fn id(self) -> Id {
        self.0
    }

    #[inline]
    pub const fn value(self) -> u64 {
        self.0.value()
    }

    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0.is_zero()
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
    #[inline]
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<MailboxId> for Id {
    #[inline]
    fn from(id: MailboxId) -> Self {
        id.0
    }
}

impl From<u64> for MailboxId {
    #[inline]
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_roundtrip() {
        let id = MailboxId::new(10);
        assert_eq!(id.value(), 10);
        let text = id.to_string();
        let parsed: MailboxId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}
