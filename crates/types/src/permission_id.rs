use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Unique identifier for a permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PermissionId(pub(crate) crate::Id);

impl PermissionId {
    pub fn new(id: u64) -> Self {
        Self(crate::Id::new(id))
    }

    pub fn value(&self) -> u64 {
        self.0.value()
    }
}

impl fmt::Display for PermissionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for PermissionId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(crate::Id::from_str(s)?))
    }
}

impl From<crate::Id> for PermissionId {
    fn from(id: crate::Id) -> Self {
        Self(id)
    }
}

impl From<PermissionId> for crate::Id {
    fn from(id: PermissionId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let id = PermissionId::new(12345);
        let text = id.to_string();
        let parsed: PermissionId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}