use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Id;

/// Strongly-typed identifier for a mail folder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FolderId(pub(crate) Id);

impl FolderId {
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

impl fmt::Display for FolderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for FolderId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Id::from_str(s)?))
    }
}

impl From<Id> for FolderId {
    #[inline]
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<FolderId> for Id {
    #[inline]
    fn from(id: FolderId) -> Self {
        id.0
    }
}

impl From<u64> for FolderId {
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
        let id = FolderId::new(8);
        assert_eq!(id.value(), 8);
        let text = id.to_string();
        let parsed: FolderId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}
