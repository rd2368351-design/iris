use serde::{Deserialize, Serialize};
use std::fmt;

/// A validated, normalized email address (`local@domain`).
///
/// Validation here is intentionally minimal — RFC 5321/5322 syntax is
/// notoriously permissive. We check the shape that actually matters for
/// routing (exactly one `@`, non-empty local part, domain with at least
/// one dot) and leave full grammar validation to the `email` crate later,
/// which needs it anyway to parse MIME headers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmailAddress {
    local: String,
    domain: String,
}

impl EmailAddress {
    pub fn parse(input: &str) -> Result<Self, crate::Error> {
        let input = input.trim();
        let (local, domain) = input
            .rsplit_once('@')
            .ok_or_else(|| crate::Error::InvalidEmail(input.to_string()))?;

        if local.is_empty() || domain.is_empty() || !domain.contains('.') {
            return Err(crate::Error::InvalidEmail(input.to_string()));
        }

        Ok(EmailAddress {
            local: local.to_string(),
            domain: domain.to_ascii_lowercase(),
        })
    }

    pub fn local_part(&self) -> &str {
        &self.local
    }

    pub fn domain(&self) -> &str {
        &self.domain
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
    fn parses_valid_address() {
        let addr = EmailAddress::parse("User@Example.COM").unwrap();
        assert_eq!(addr.local_part(), "User");
        assert_eq!(addr.domain(), "example.com");
    }

    #[test]
    fn rejects_missing_at() {
        assert!(EmailAddress::parse("not-an-email").is_err());
    }

    #[test]
    fn rejects_domain_without_dot() {
        assert!(EmailAddress::parse("user@localhost").is_err());
    }
}
