use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Generic numeric identifier used as the foundation for all strongly-typed IDs.
///
/// This is intentionally a thin wrapper around `u64` to ensure
/// type safety at the crate boundary while remaining zero-cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(u64);

impl Id {
    /// Creates a new identifier from a raw `u64`.
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw numeric value.
    #[inline]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns true if this is the zero/invalid ID.
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Id {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .trim()
            .parse::<u64>()
            .map_err(|e| crate::Error::InvalidId {
                value: s.to_string(),
                reason: Box::new(e),
            })?;
        Ok(Self::new(value))
    }
}

impl From<u64> for Id {
    #[inline]
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<Id> for u64 {
    #[inline]
    fn from(id: Id) -> Self {
        id.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_value() {
        let id = Id::new(42);
        assert_eq!(id.value(), 42);
        assert!(!id.is_zero());
    }

    #[test]
    fn zero_id() {
        let id = Id::new(0);
        assert!(id.is_zero());
    }

    #[test]
    fn display() {
        assert_eq!(Id::new(123).to_string(), "123");
    }

    #[test]
    fn from_str() {
        assert_eq!("456".parse::<Id>().unwrap().value(), 456);
    }

    #[test]
    fn from_str_invalid() {
        assert!("abc".parse::<Id>().is_err());
        assert!("".parse::<Id>().is_err());
    }

    #[test]
    fn roundtrip() {
        let id = Id::new(789);
        let text = id.to_string();
        let parsed: Id = text.parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn from_u64() {
        let id: Id = 100.into();
        assert_eq!(id.value(), 100);
    }

    #[test]
    fn into_u64() {
        let val: u64 = Id::new(200).into();
        assert_eq!(val, 200);
    }
}
