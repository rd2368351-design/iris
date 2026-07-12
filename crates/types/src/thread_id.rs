use serde::{Deserialize, Serialize};
use std::fmt;

use crate::Id;

/// Strongly-typed identifier for an email thread (conversation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ThreadId(Id);

impl ThreadId {
    /// Create a new thread identifier.
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

impl fmt::Display for ThreadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Id> for ThreadId {
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<ThreadId> for Id {
    fn from(id: ThreadId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_generic_id() {
        let id = Id::new(500);
        let thread = ThreadId::new(id);

        assert_eq!(thread.id(), id);
        assert_eq!(thread.value(), 500);
    }
}