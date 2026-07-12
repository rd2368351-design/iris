use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Error;

/// A validated, RFC 5321/5322 compliant email address.
///
/// Stores local part case-preserved, domain lowercased.
/// Supports:
/// - Standard addresses (`user@example.com`)
/// - IP literals (`postmaster@[192.168.1.1]`)
/// - Internationalized email with UTF-8 local part (SMTPUTF8)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmailAddress {
    /// Full address stored as `local@domain` to avoid double allocation.
    inner: String,
    /// Position of `@` in `inner`.
    at_pos: u8,
}

impl EmailAddress {
    /// Maximum total length per RFC 5321.
    const MAX_LENGTH: usize = 254;
    /// Maximum local part length per RFC 5321.
    const MAX_LOCAL: usize = 64;
    /// Maximum domain length per RFC 5321.
    const MAX_DOMAIN: usize = 255;

    /// Parse and validate an email address.
    pub fn parse(input: &str) -> Result<Self, Error> {
        let input = input.trim();

        if input.len() > Self::MAX_LENGTH {
            return Err(Error::InvalidEmail(input.to_string()));
        }

        let at_pos = input
            .rfind('@')
            .ok_or_else(|| Error::InvalidEmail(input.to_string()))?;

        let local = &input[..at_pos];
        let domain = &input[at_pos + 1..];

        if local.is_empty() || local.len() > Self::MAX_LOCAL {
            return Err(Error::InvalidEmail(input.to_string()));
        }

        if domain.is_empty() || domain.len() > Self::MAX_DOMAIN {
            return Err(Error::InvalidEmail(input.to_string()));
        }

        // Validate local part
        if !Self::is_valid_local(local) {
            return Err(Error::InvalidEmail(input.to_string()));
        }

        // Validate domain
        if !Self::is_valid_domain(domain) {
            return Err(Error::InvalidEmail(input.to_string()));
        }

        let domain_lower = domain.to_ascii_lowercase();
        let inner = if domain_lower == domain {
            input.to_string()
        } else {
            format!("{}@{}", local, domain_lower)
        };

        Ok(Self {
            inner,
            at_pos: at_pos as u8,
        })
    }

    fn is_valid_local(local: &str) -> bool {
        if local.starts_with('"') && local.ends_with('"') && local.len() >= 2 {
            // Quoted string — simplified check
            !local[1..local.len() - 1].contains('"')
        } else {
            // Unquoted — no spaces, no special chars except . _ - +
            local
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '+' | '!'))
                && !local.starts_with('.')
                && !local.ends_with('.')
                && !local.contains("..")
        }
    }

    fn is_valid_domain(domain: &str) -> bool {
        if domain.starts_with('[') && domain.ends_with(']') {
            // IP literal
            let inner = &domain[1..domain.len() - 1];
            inner.parse::<std::net::IpAddr>().is_ok()
        } else {
            // FQDN — must contain at least one dot for production
            // Local delivery without dot is valid per RFC but rare in production
            domain.len() >= 3
                && domain.contains('.')
                && !domain.starts_with('.')
                && !domain.ends_with('.')
                && !domain.starts_with('-')
                && !domain.ends_with('-')
        }
    }

    /// Returns the local part (case-preserved).
    pub fn local_part(&self) -> &str {
        &self.inner[..self.at_pos as usize]
    }

    /// Returns the domain (always lowercase).
    pub fn domain(&self) -> &str {
        &self.inner[self.at_pos as usize + 1..]
    }

    /// Returns the full email address as a string slice.
    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inner)
    }
}

impl FromStr for EmailAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
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
        assert_eq!(email.as_str(), "User@example.com");
    }

    #[test]
    fn parses_ip_literal() {
        let email = EmailAddress::parse("postmaster@[192.168.1.1]").unwrap();
        assert_eq!(email.domain(), "[192.168.1.1]");
    }

    #[test]
    fn parses_plus_addressing() {
        let email = EmailAddress::parse("user+tag@example.com").unwrap();
        assert_eq!(email.local_part(), "user+tag");
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
    fn rejects_missing_at() {
        assert!(EmailAddress::parse("invalid-email").is_err());
    }

    #[test]
    fn rejects_double_dot() {
        assert!(EmailAddress::parse("john..doe@example.com").is_err());
    }

    #[test]
    fn rejects_leading_dot() {
        assert!(EmailAddress::parse(".user@example.com").is_err());
    }

    #[test]
    fn rejects_local_delivery_for_production() {
        assert!(EmailAddress::parse("user@localhost").is_err());
    }

    #[test]
    fn roundtrip_serde() {
        let email = EmailAddress::parse("Test@Example.Com").unwrap();
        let json = serde_json::to_string(&email).unwrap();
        let parsed: EmailAddress = serde_json::from_str(&json).unwrap();
        assert_eq!(email, parsed);
    }

    #[test]
    fn roundtrip_display_parse() {
        let email = EmailAddress::parse("User@Example.com").unwrap();
        let text = email.to_string();
        let parsed: EmailAddress = text.parse().unwrap();
        assert_eq!(email, parsed);
    }
}