use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Unique identifier for a calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CalendarId(pub(crate) crate::Id);

impl CalendarId {
    pub fn new(id: u64) -> Self {
        Self(crate::Id::new(id))
    }

    pub fn value(&self) -> u64 {
        self.0.value()
    }
}

impl fmt::Display for CalendarId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for CalendarId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(crate::Id::from_str(s)?))
    }
}

impl From<crate::Id> for CalendarId {
    fn from(id: crate::Id) -> Self {
        Self(id)
    }
}

impl From<CalendarId> for crate::Id {
    fn from(id: CalendarId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let id = CalendarId::new(12345);
        let text = id.to_string();
        let parsed: CalendarId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}