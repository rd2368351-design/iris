use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Id;

/// Strongly-typed identifier for a sender identity.
///
/// An identity represents a "From" address that a user can send mail
/// from, such as:
///
/// - support@example.com
/// - sales@example.com
/// - jane@example.org
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct IdentityId(Id);

impl IdentityId {
    /// Creates a new identity identifier.
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

impl fmt::Display for IdentityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for IdentityId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Id::from_str(s)?))
    }
}

impl From<Id> for IdentityId {
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<IdentityId> for Id {
    fn from(id: IdentityId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_generic_id() {
        let id = Id::new(2025);
        let identity = IdentityId::new(id);

        assert_eq!(identity.id(), id);
        assert_eq!(identity.value(), 2025);
    }

    #[test]
    fn roundtrip() {
        let id = IdentityId::new(Id::new(12345));
        let text = id.to_string();
        let parsed: IdentityId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}