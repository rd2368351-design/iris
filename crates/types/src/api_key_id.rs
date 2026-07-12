use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Unique identifier for an API key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ApiKeyId(pub(crate) crate::Id);

impl ApiKeyId {
    pub fn new(id: u64) -> Self {
        Self(crate::Id::new(id))
    }

    pub fn value(&self) -> u64 {
        self.0.value()
    }
}

impl fmt::Display for ApiKeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for ApiKeyId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(crate::Id::from_str(s)?))
    }
}

impl From<crate::Id> for ApiKeyId {
    fn from(id: crate::Id) -> Self {
        Self(id)
    }
}

impl From<ApiKeyId> for crate::Id {
    fn from(id: ApiKeyId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let id = ApiKeyId::new(12345);
        let text = id.to_string();
        let parsed: ApiKeyId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}