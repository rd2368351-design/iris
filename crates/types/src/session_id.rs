use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Id;

/// Strongly-typed identifier for an authenticated session.
///
/// Sessions are created after successful authentication and are used
/// across IMAP, SMTP AUTH, JMAP, WebDAV and the Admin API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(pub(crate) Id);

impl SessionId {
    /// Creates a new session identifier from a raw `u64`.
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

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for SessionId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Id::from_str(s)?))
    }
}

impl From<Id> for SessionId {
    #[inline]
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<SessionId> for Id {
    #[inline]
    fn from(id: SessionId) -> Self {
        id.0
    }
}

impl From<u64> for SessionId {
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
        let id = SessionId::new(1000);
        assert_eq!(id.value(), 1000);
        assert_eq!(id.id(), Id::new(1000));
    }

    #[test]
    fn roundtrip() {
        let id = SessionId::new(12345);
        let text = id.to_string();
        let parsed: SessionId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn from_u64() {
        let session: SessionId = 999.into();
        assert_eq!(session.value(), 999);
    }

    #[test]
    fn invalid_parse() {
        assert!("xyz".parse::<SessionId>().is_err());
    }
}
