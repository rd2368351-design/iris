use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Id;

/// Strongly-typed identifier for a user.
///
/// A user owns one or more mailboxes, identities, sessions,
/// application passwords, and other account-related resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub(crate) Id);

impl UserId {
    /// Creates a new user identifier from a raw `u64`.
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

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for UserId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Id::from_str(s)?))
    }
}

impl From<Id> for UserId {
    #[inline]
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<UserId> for Id {
    #[inline]
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl From<u64> for UserId {
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
        let id = UserId::new(42);
        assert_eq!(id.value(), 42);
        assert_eq!(id.id(), Id::new(42));
        assert!(!id.is_zero());
    }

    #[test]
    fn from_id() {
        let inner = Id::new(100);
        let user = UserId::from(inner);
        assert_eq!(user.value(), 100);
    }

    #[test]
    fn into_id() {
        let user = UserId::new(200);
        let inner: Id = user.into();
        assert_eq!(inner.value(), 200);
    }

    #[test]
    fn from_u64() {
        let user: UserId = 300.into();
        assert_eq!(user.value(), 300);
    }

    #[test]
    fn roundtrip() {
        let id = UserId::new(12345);
        let text = id.to_string();
        let parsed: UserId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn invalid_parse() {
        assert!("abc".parse::<UserId>().is_err());
        assert!("".parse::<UserId>().is_err());
    }
}
