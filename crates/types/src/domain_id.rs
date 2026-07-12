use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Id;

/// Strongly-typed identifier for a mail domain.
///
/// Examples:
/// - example.com
/// - company.org
/// - mail.example.net
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DomainId(Id);

impl DomainId {
    /// Creates a new domain identifier.
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

impl fmt::Display for DomainId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for DomainId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Id::from_str(s)?))
    }
}

impl From<Id> for DomainId {
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<DomainId> for Id {
    fn from(id: DomainId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_generic_id() {
        let id = Id::new(1234);
        let domain = DomainId::new(id);

        assert_eq!(domain.id(), id);
        assert_eq!(domain.value(), 1234);
    }

    #[test]
    fn roundtrip() {
        let id = DomainId::new(Id::new(12345));
        let text = id.to_string();
        let parsed: DomainId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}