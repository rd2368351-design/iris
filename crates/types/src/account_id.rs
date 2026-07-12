use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Id;

/// Strongly-typed identifier for a user account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountId(Id);

impl AccountId {
    /// Create a new account identifier.
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

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for AccountId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Id::from_str(s)?))
    }
}

impl From<Id> for AccountId {
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<AccountId> for Id {
    fn from(id: AccountId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_generic_id() {
        let id = Id::new(7);
        let account = AccountId::new(id);

        assert_eq!(account.id(), id);
        assert_eq!(account.value(), 7);
    }

    #[test]
    fn roundtrip() {
        let id = AccountId::new(Id::new(12345));
        let text = id.to_string();
        let parsed: AccountId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}