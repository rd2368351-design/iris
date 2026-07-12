use serde::{Deserialize, Serialize};
use std::fmt;

use crate::Error;

/// A validated email address (`local@domain`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmailAddress {
    local: String,
    domain: String,
}

impl EmailAddress {
    /// Parse and validate an email address.
    pub fn parse(input: &str) -> Result<Self, Error> {
        let input = input.trim();

        let (local, domain) = input
            .rsplit_once('@')
            .ok_or_else(|| Error::InvalidEmail(input.to_string()))?;

        if local.is_empty() || domain.is_empty() || !domain.contains('.') {
            return Err(Error::InvalidEmail(input.to_string()));
        }

        Ok(Self {
            local: local.to_string(),
            domain: domain.to_ascii_lowercase(),
        })
    }

    /// Returns the local part.
    pub fn local_part(&self) -> &str {
        &self.local
    }

    /// Returns the domain.
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Returns the full address.
    pub fn as_str(&self) -> String {
        format!("{}@{}", self.local, self.domain)
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.local, self.domain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_email() {
        let email = EmailAddress::parse("User@Example.COM").unwrap();

        assert_eq!(email.local_part(), "User");
        assert_eq!(email.domain(), "example.com");
    }

    #[test]
    fn rejects_invalid_email() {
        assert!(EmailAddress::parse("invalid-email").is_err());
    }

    #[test]
    fn rejects_domain_without_dot() {
        assert!(EmailAddress::parse("user@localhost").is_err());
    }
}