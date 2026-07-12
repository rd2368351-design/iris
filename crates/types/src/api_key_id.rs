use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Id;

/// Strongly-typed identifier for an API key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ApiKeyId(pub(crate) Id);

impl ApiKeyId {
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

impl fmt::Display for ApiKeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for ApiKeyId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Id::from_str(s)?))
    }
}

impl From<Id> for ApiKeyId {
    #[inline]
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<ApiKeyId> for Id {
    #[inline]
    fn from(id: ApiKeyId) -> Self {
        id.0
    }
}

impl From<u64> for ApiKeyId {
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
        let id = ApiKeyId::new(88);
        assert_eq!(id.value(), 88);
        let text = id.to_string();
        let parsed: ApiKeyId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}
