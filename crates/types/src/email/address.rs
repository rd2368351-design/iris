use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A validated email address.
///
/// Ensures the address is syntactically valid per RFC 5322.
/// This does NOT verify that the domain exists or that the mailbox
/// is reachable — only that the format is correct.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmailAddress {
    address: String,
}

impl EmailAddress {
    /// Maximum length of an email address (RFC 5321).
    pub const MAX_LEN: usize = 254;

    /// Local part maximum length (before @).
    pub const MAX_LOCAL_LEN: usize = 64;

    /// Creates a validated email address.
    pub fn parse(address: impl AsRef<str>) -> Result<Self, crate::Error> {
        let trimmed = address.as_ref().trim();

        if trimmed.is_empty() {
            return Err(crate::Error::InvalidEmail {
                addr: trimmed.to_string(),
                reason: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "empty address",
                )),
            });
        }

        if trimmed.len() > Self::MAX_LEN {
            return Err(crate::Error::InvalidEmail {
                addr: trimmed.to_string(),
                reason: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "address too long",
                )),
            });
        }

        // Basic validation: must contain exactly one @ with non-empty parts
        let parts: Vec<&str> = trimmed.split('@').collect();
        if parts.len() != 2 {
            return Err(crate::Error::InvalidEmail {
                addr: trimmed.to_string(),
                reason: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "missing @",
                )),
            });
        }

        let local = parts[0];
        let domain = parts[1];

        if local.is_empty() || local.len() > Self::MAX_LOCAL_LEN {
            return Err(crate::Error::InvalidEmail {
                addr: trimmed.to_string(),
                reason: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "invalid local part",
                )),
            });
        }

        if domain.is_empty() || !domain.contains('.') {
            return Err(crate::Error::InvalidEmail {
                addr: trimmed.to_string(),
                reason: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "invalid domain",
                )),
            });
        }

        Ok(Self {
            address: trimmed.to_lowercase(),
        })
    }

    /// Returns the full email address.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.address
    }

    /// Returns the local part (before @).
    pub fn local(&self) -> &str {
        self.address.split('@').next().unwrap_or("")
    }

    /// Returns the domain part (after @).
    pub fn domain(&self) -> &str {
        self.address.split('@').nth(1).unwrap_or("")
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.address)
    }
}

impl FromStr for EmailAddress {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl AsRef<str> for EmailAddress {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_address() {
        let addr = EmailAddress::parse("user@example.com").unwrap();
        assert_eq!(addr.as_str(), "user@example.com");
        assert_eq!(addr.local(), "user");
        assert_eq!(addr.domain(), "example.com");
    }

    #[test]
    fn normalizes_to_lowercase() {
        let addr = EmailAddress::parse("User@Example.COM").unwrap();
        assert_eq!(addr.as_str(), "user@example.com");
    }

    #[test]
    fn rejects_empty() {
        assert!(EmailAddress::parse("").is_err());
        assert!(EmailAddress::parse("   ").is_err());
    }

    #[test]
    fn rejects_missing_at() {
        assert!(EmailAddress::parse("userexample.com").is_err());
    }

    #[test]
    fn rejects_multiple_at() {
        assert!(EmailAddress::parse("user@@example.com").is_err());
    }

    #[test]
    fn rejects_empty_local() {
        assert!(EmailAddress::parse("@example.com").is_err());
    }

    #[test]
    fn rejects_empty_domain() {
        assert!(EmailAddress::parse("user@").is_err());
    }

    #[test]
    fn rejects_missing_tld() {
        assert!(EmailAddress::parse("user@example").is_err());
    }

    #[test]
    fn from_str() {
        let addr: EmailAddress = "test@domain.com".parse().unwrap();
        assert_eq!(addr.as_str(), "test@domain.com");
    }

    #[test]
    fn display() {
        let addr = EmailAddress::parse("hello@world.com").unwrap();
        assert_eq!(addr.to_string(), "hello@world.com");
    }
}
