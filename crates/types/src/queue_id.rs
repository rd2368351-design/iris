use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Id;

/// Unique identifier for a mail queue entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QueueId(pub(crate) Id);

impl QueueId {
    /// Creates a new queue identifier from a raw `u64`.
    #[inline]
    pub const fn new(id: u64) -> Self {
        Self(Id::new(id))
    }

    /// Returns the wrapped generic identifier.
    #[inline]
    pub const fn id(self) -> Id {
        self.0
    }

    /// Returns the raw numeric value.
    #[inline]
    pub const fn value(self) -> u64 {
        self.0.value()
    }

    /// Returns true if this is the zero/invalid ID.
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0.is_zero()
    }
}

impl fmt::Display for QueueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for QueueId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Id::from_str(s)?))
    }
}

impl From<Id> for QueueId {
    #[inline]
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<QueueId> for Id {
    #[inline]
    fn from(id: QueueId) -> Self {
        id.0
    }
}

impl From<u64> for QueueId {
    #[inline]
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_value() {
        let id = QueueId::new(42);
        assert_eq!(id.value(), 42);
        assert_eq!(id.id(), Id::new(42));
    }

    #[test]
    fn roundtrip() {
        let id = QueueId::new(12345);
        let text = id.to_string();
        let parsed: QueueId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn from_u64() {
        let queue: QueueId = 777.into();
        assert_eq!(queue.value(), 777);
    }

    #[test]
    fn invalid_parse() {
        assert!("not-a-number".parse::<QueueId>().is_err());
    }
}
