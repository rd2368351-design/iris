//! Input validation utilities.

use std::net::IpAddr;

/// Validates that a string is a valid email address (basic check).
pub fn is_valid_email(s: &str) -> bool {
    if s.len() > 254 {
        return false;
    }
    let parts: Vec<&str> = s.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    !parts[0].is_empty() && parts[0].len() <= 64 && !parts[1].is_empty() && parts[1].contains('.')
}

/// Validates that a string is a valid domain name.
pub fn is_valid_domain(s: &str) -> bool {
    if s.is_empty() || s.len() > 253 {
        return false;
    }
    s.chars()
        .all(|c| c.is_alphanumeric() || c == '.' || c == '-')
}

/// Validates IP address is not in a private/reserved range.
pub fn is_public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            // Not private ranges
            !(octets[0] == 10
                || (octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31)
                || (octets[0] == 192 && octets[1] == 168)
                || octets[0] == 127)
        }
        IpAddr::V6(_) => true, // Simplified
    }
}

/// Sanitizes a folder name to prevent path traversal.
pub fn sanitize_folder_name(name: &str) -> Option<String> {
    let sanitized = name
        .trim()
        .replace("..", "")
        .replace('/', "")
        .replace('\\', "");
    if sanitized.is_empty() || sanitized.len() > 255 {
        None
    } else {
        Some(sanitized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_emails() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("a@b.co"));
    }

    #[test]
    fn invalid_emails() {
        assert!(!is_valid_email(""));
        assert!(!is_valid_email("no-at-sign"));
        assert!(!is_valid_email("@nodomain"));
        assert!(!is_valid_email("spaces in@name.com"));
    }

    #[test]
    fn valid_domains() {
        assert!(is_valid_domain("example.com"));
        assert!(is_valid_domain("mail.example.co.uk"));
    }

    #[test]
    fn sanitize_folder() {
        assert_eq!(sanitize_folder_name("Inbox"), Some("Inbox".to_string()));
        assert_eq!(
            sanitize_folder_name("../etc/passwd"),
            Some("etcpasswd".to_string())
        );
        assert_eq!(sanitize_folder_name(""), None);
    }
}
