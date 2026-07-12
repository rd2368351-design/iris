use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Message priority.
///
/// This represents the relative importance of an email message.
/// It is independent of transport priority and is intended for
/// user-facing features such as inbox sorting and highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] // FIXED: Now JSON output matches Display ("high", "low")
pub enum Priority {
    Low,
    Normal,
    High,
}

impl Priority {
    /// Returns the numeric priority value mapped to RFC 2156 X-Priority.
    ///
    /// Low = 5
    /// Normal = 3
    /// High = 1
    pub fn value(self) -> u8 {
        match self {
            Self::High => 1,
            Self::Normal => 3,
            Self::Low => 5,
        }
    }

    pub fn is_high(self) -> bool {
        matches!(self, Self::High)
    }

    pub fn is_normal(self) -> bool {
        matches!(self, Self::Normal)
    }

    pub fn is_low(self) -> bool {
        matches!(self, Self::Low)
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self::Normal
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "low"),
            Self::Normal => write!(f, "normal"),
            Self::High => write!(f, "high"),
        }
    }
}

impl FromStr for Priority {
    type Err = crate::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "low" => Ok(Self::Low),
            "normal" => Ok(Self::Normal),
            "high" => Ok(Self::High),
            _ => Err(crate::Error::InvalidPriority(value.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_priority() {
        assert_eq!(Priority::default(), Priority::Normal);
    }

    #[test]
    fn parse_priority() {
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("normal".parse::<Priority>().unwrap(), Priority::Normal);
        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
    }

    #[test]
    fn display_priority() {
        assert_eq!(Priority::High.to_string(), "high");
        assert_eq!(Priority::Normal.to_string(), "normal");
        assert_eq!(Priority::Low.to_string(), "low");
    }
    
    #[test]
    fn serde_lowercase() {
        // This test ensures Serde matches Display
        let json = serde_json::to_string(&Priority::High).unwrap();
        assert_eq!(json, "\"high\"");
    }

    #[test]
    fn invalid_priority() {
        assert!("urgent".parse::<Priority>().is_err());
    }

    #[test]
    fn ordering() {
        assert!(Priority::High > Priority::Normal);
        assert!(Priority::Normal > Priority::Low);
    }

    #[test]
    fn numeric_values() {
        assert_eq!(Priority::High.value(), 1);
        assert_eq!(Priority::Normal.value(), 3);
        assert_eq!(Priority::Low.value(), 5);
    }
}
