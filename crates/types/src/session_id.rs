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
pub struct SessionId(Id);

impl SessionId {
    /// Creates a new session identifier.
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
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<SessionId> for Id {
    fn from(id: SessionId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_generic_id() {
        let id = Id::new(1000);
        let session = SessionId::new(id);

        assert_eq!(session.id(), id);
        assert_eq!(session.value(), 1000);
    }

    #[test]
    fn roundtrip() {
        let id = SessionId::new(Id::new(12345));
        let text = id.to_string();
        let parsed: SessionId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}