use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

// Assuming crate::Error is defined elsewhere in your workspace
use crate::Error;

/// A single RFC 5322 email header.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Header {
    /// Using Cow prevents allocating Strings for standard headers like "Subject" or "To".
    name: Cow<'static, str>,
    value: String,
}

impl Header {
    /// Creates a new header. Returns an Error if the name violates RFC 5322.
    pub fn new(name: impl Into<Cow<'static, str>>, value: impl Into<String>) -> Result<Self, Error> {
        let name_cow = name.into();
        
        // Validate strictly on creation
        if !Self::is_valid_name(&name_cow) {
            return Err(Error::InvalidHeaderName(name_cow.into_owned()));
        }

        Ok(Self {
            name: name_cow,
            value: value.into(),
        })
    }

    /// Check if the header name strictly complies with RFC 5322 (Section 2.2).
    /// Allowed chars: printable US-ASCII (33-126) except colon (58).
    fn is_valid_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        // No spaces, no colons, only standard printable ASCII allowed
        name.bytes().all(|b| b >= 33 && b <= 126 && b != b':')
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

        // We intentionally DO NOT trim the 'name' field here. 
        // RFC 5322 does not allow spaces before the colon or inside the name.
        if !Self::is_valid_name(name) {
            return Err(Error::InvalidHeaderName(name.to_string()));
        }

        // Trimming the value is standard practice for unfolded whitespace
        let value = value.trim();

        Ok(Self {
            // Because it was parsed from a dynamic string, we use Cow::Owned
            name: Cow::Owned(name.to_string()),
            value: value.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_header() {
        // "Subject" is &'static str, so this is completely zero-allocation for the name!
        let header = Header::new("Subject", "Hello World").unwrap();

        assert_eq!(header.name(), "Subject");
        assert_eq!(header.value(), "Hello World");
    }

    #[test]
    fn rejects_invalid_header_names() {
        // Space inside the name
        assert!(Header::new("Invalid Name", "Value").is_err());
        
        // Space before the colon
        assert!("X-Custom : value".parse::<Header>().is_err());
        
        // Non-ASCII character
        assert!(Header::new("X-हिन्दी", "Namaste").is_err());
    }

    #[test]
    fn updates_value() {
        let mut header = Header::new("Subject", "Old").unwrap();
        header.set_value("New");
        assert_eq!(header.value(), "New");
    }

    #[test]
    fn display_format() {
        let header = Header::new("From", "user@example.com").unwrap();
        assert_eq!(header.to_string(), "From: user@example.com");
    }

    #[test]
    fn parse_from_string() {
        let header: Header = "Subject: Hello World".parse().unwrap();

        assert_eq!(header.name(), "Subject");
        assert_eq!(header.value(), "Hello World");
    }

    #[test]
    fn parse_trims_whitespace_on_value_only() {
        // Notice there are no spaces in "X-Custom" anymore.
        let header: Header = "X-Custom:   value  ".parse().unwrap();

        assert_eq!(header.name(), "X-Custom");
        assert_eq!(header.value(), "value");
    }
}
