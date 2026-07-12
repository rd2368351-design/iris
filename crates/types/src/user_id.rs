use serde::{Deserialize, Serialize};
use std::fmt;

use crate::Id;

/// Strongly-typed identifier for a user.
///
/// A user owns one or more mailboxes, identities, sessions,
/// application passwords, and other account-related resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UserId(Id);

impl UserId {
    /// Creates a new user identifier.
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

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Id> for UserId {
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<UserId> for Id {
    fn from(id: UserId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_generic_id() {
        let id = Id::new(42);
        let user = UserId::new(id);

        assert_eq!(user.id(), id);
        assert_eq!(user.value(), 42);
    }
}