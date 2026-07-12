use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Error;

/// A single RFC 5322 email header.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        let value = value.into();

        Self { name, value }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

impl FromStr for Header {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, value) = s
            .split_once(':')
            .ok_or_else(|| Error::InvalidHeaderName(s.to_string()))?;

        let name = name.trim();
        let value = value.trim();

        if name.is_empty() {
            return Err(Error::InvalidHeaderName(s.to_string()));
        }

        Ok(Self {
            name: name.to_string(),
            value: value.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_header() {
        let header = Header::new("Subject", "Hello World");

        assert_eq!(header.name(), "Subject");
        assert_eq!(header.value(), "Hello World");
    }

    #[test]
    fn updates_value() {
        let mut header = Header::new("Subject", "Old");

        header.set_value("New");

        assert_eq!(header.value(), "New");
    }

    #[test]
    fn display_format() {
        let header = Header::new("From", "user@example.com");

        assert_eq!(header.to_string(), "From: user@example.com");
    }

    #[test]
    fn parse_from_string() {
        let header: Header = "Subject: Hello World".parse().unwrap();

        assert_eq!(header.name(), "Subject");
        assert_eq!(header.value(), "Hello World");
    }

    #[test]
    fn parse_trims_whitespace() {
        let header: Header = "  X-Custom  :   value  ".parse().unwrap();

        assert_eq!(header.name(), "X-Custom");
        assert_eq!(header.value(), "value");
    }
}