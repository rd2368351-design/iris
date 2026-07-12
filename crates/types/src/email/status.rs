use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Represents the current status of an email message in its lifecycle.
///
/// This is a high-level abstraction over underlying mail system flags
/// (e.g., IMAP `\Seen`, `\Draft`, `\Deleted`). A message may transition
/// between states during its lifetime, but only occupies one primary
/// status at a time for API simplicity.
///
/// # Serialization
///
/// Serializes to lowercase strings: `"unread"`, `"read"`, `"draft"`, etc.
/// This matches the [`Display`] output and is case-insensitive when parsing.
///
/// # Future-Proofing
///
/// This enum is marked `#[non_exhaustive]` to allow adding new states
/// (e.g., `Pinned`, `Snoozed`) without breaking downstream consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Status {
    /// Message has not been opened by the recipient.
    Unread,
    
    /// Message has been opened.
    Read,
    
    /// Message is a draft and has not been sent.
    Draft,
    
    /// Message has been sent by the user.
    Sent,
    
    /// Message has been archived (removed from inbox but kept).
    Archived,
    
    /// Message is marked for deletion (may be in trash).
    Deleted,
    
    /// Message classified as spam/junk.
    Spam,
}

impl Status {
    /// Maximum length of a valid status string.
    pub const MAX_LEN: usize = 16;
    
    /// Returns `true` if the message has been seen by the user.
    #[inline]
    pub const fn is_read(self) -> bool {
        matches!(self, Self::Read)
    }
    
    /// Returns `true` if the message has not been seen.
    #[inline]
    pub const fn is_unread(self) -> bool {
        matches!(self, Self::Unread)
    }
    
    /// Returns `true` if the message is a draft.
    #[inline]
    pub const fn is_draft(self) -> bool {
        matches!(self, Self::Draft)
    }
    
    /// Returns `true` if the message has been sent.
    #[inline]
    pub const fn is_sent(self) -> bool {
        matches!(self, Self::Sent)
    }
    
    /// Returns `true` if the message is archived.
    #[inline]
    pub const fn is_archived(self) -> bool {
        matches!(self, Self::Archived)
    }
    
    /// Returns `true` if the message is marked as deleted.
    #[inline]
    pub const fn is_deleted(self) -> bool {
        matches!(self, Self::Deleted)
    }
    
    /// Returns `true` if the message is classified as spam.
    #[inline]
    pub const fn is_spam(self) -> bool {
        matches!(self, Self::Spam)
    }
    
    /// Returns `true` if the message is in a terminal state that
    /// typically prevents further user-driven transitions.
    ///
    /// Currently, [`Deleted`] and [`Archived`] are considered terminal.
    #[inline]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Deleted | Self::Archived)
    }
    
    /// Returns `true` if the message should appear in the user's
    /// primary inbox view.
    ///
    /// Excludes [`Deleted`], [`Archived`], and [`Spam`].
    #[inline]
    pub const fn is_inbox_visible(self) -> bool {
        matches!(self, Self::Unread | Self::Read | Self::Draft | Self::Sent)
    }
    
    /// Returns the string representation without allocating.
    ///
    /// Prefer this over [`ToString`] when you only need a `&str`.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unread => "unread",
            Self::Read => "read",
            Self::Draft => "draft",
            Self::Sent => "sent",
            Self::Archived => "archived",
            Self::Deleted => "deleted",
            Self::Spam => "spam",
        }
    }
}

impl Default for Status {
    #[inline]
    fn default() -> Self {
        Self::Unread
    }
}

impl fmt::Display for Status {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Status {
    type Err = crate::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let v = value.trim();
        
        // Prevent DoS via extremely long strings
        if v.len() > Self::MAX_LEN {
            return Err(crate::Error::InvalidStatus(value.to_string()));
        }
        
        // Zero-allocation case-insensitive comparison
        if v.eq_ignore_ascii_case("unread") {
            return Ok(Self::Unread);
        }
        if v.eq_ignore_ascii_case("read") {
            return Ok(Self::Read);
        }
        if v.eq_ignore_ascii_case("draft") {
            return Ok(Self::Draft);
        }
        if v.eq_ignore_ascii_case("sent") {
            return Ok(Self::Sent);
        }
        if v.eq_ignore_ascii_case("archived") {
            return Ok(Self::Archived);
        }
        if v.eq_ignore_ascii_case("deleted") {
            return Ok(Self::Deleted);
        }
        if v.eq_ignore_ascii_case("spam") {
            return Ok(Self::Spam);
        }
        
        Err(crate::Error::InvalidStatus(value.to_string()))
    }
}

impl AsRef<str> for Status {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_unread() {
        assert_eq!(Status::default(), Status::Unread);
    }

    #[test]
    fn parse_all_variants() {
        assert_eq!("unread".parse::<Status>().unwrap(), Status::Unread);
        assert_eq!("read".parse::<Status>().unwrap(), Status::Read);
        assert_eq!("draft".parse::<Status>().unwrap(), Status::Draft);
        assert_eq!("sent".parse::<Status>().unwrap(), Status::Sent);
        assert_eq!("archived".parse::<Status>().unwrap(), Status::Archived);
        assert_eq!("deleted".parse::<Status>().unwrap(), Status::Deleted);
        assert_eq!("spam".parse::<Status>().unwrap(), Status::Spam);
    }

    #[test]
    fn parse_case_insensitive() {
        assert_eq!("ReAd".parse::<Status>().unwrap(), Status::Read);
        assert_eq!("DRAFT".parse::<Status>().unwrap(), Status::Draft);
        assert_eq!("SpAm".parse::<Status>().unwrap(), Status::Spam);
    }

    #[test]
    fn parse_trims_whitespace() {
        assert_eq!("  read  ".parse::<Status>().unwrap(), Status::Read);
        assert_eq!("\t unread \n".parse::<Status>().unwrap(), Status::Unread);
    }

    #[test]
    fn display_matches_expected() {
        assert_eq!(Status::Unread.to_string(), "unread");
        assert_eq!(Status::Read.to_string(), "read");
        assert_eq!(Status::Draft.to_string(), "draft");
        assert_eq!(Status::Sent.to_string(), "sent");
        assert_eq!(Status::Archived.to_string(), "archived");
        assert_eq!(Status::Deleted.to_string(), "deleted");
        assert_eq!(Status::Spam.to_string(), "spam");
    }

    #[test]
    fn display_matches_as_str() {
        for status in [
            Status::Unread,
            Status::Read,
            Status::Draft,
            Status::Sent,
            Status::Archived,
            Status::Deleted,
            Status::Spam,
        ] {
            assert_eq!(status.to_string(), status.as_str());
            assert_eq!(status.as_ref(), status.as_str());
        }
    }

    #[test]
    fn serde_roundtrip_all_variants() {
        for status in [
            Status::Unread,
            Status::Read,
            Status::Draft,
            Status::Sent,
            Status::Archived,
            Status::Deleted,
            Status::Spam,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            let parsed: Status = serde_json::from_str(&json).unwrap();
            assert_eq!(status, parsed);
        }
    }

    #[test]
    fn serde_matches_display() {
        // Critical: JSON API consumers must see the same string as logs
        let json = serde_json::to_string(&Status::Read).unwrap();
        assert_eq!(json, "\"read\"");
        
        let json = serde_json::to_string(&Status::Archived).unwrap();
        assert_eq!(json, "\"archived\"");
    }

    #[test]
    fn serde_deserialize_lowercase() {
        let parsed: Status = serde_json::from_str("\"deleted\"").unwrap();
        assert_eq!(parsed, Status::Deleted);
    }

    #[test]
    fn invalid_status_rejected() {
        assert!("unknown".parse::<Status>().is_err());
        assert!("".parse::<Status>().is_err());
        assert!("   ".parse::<Status>().is_err());
    }

    #[test]
    fn long_string_rejected() {
        let long = "a".repeat(100);
        assert!(long.parse::<Status>().is_err());
    }

    #[test]
    fn checkers() {
        assert!(Status::Read.is_read());
        assert!(!Status::Unread.is_read());
        assert!(Status::Unread.is_unread());
        
        assert!(Status::Deleted.is_terminal());
        assert!(Status::Archived.is_terminal());
        assert!(!Status::Unread.is_terminal());
        
        assert!(Status::Unread.is_inbox_visible());
        assert!(Status::Read.is_inbox_visible());
        assert!(!Status::Deleted.is_inbox_visible());
        assert!(!Status::Spam.is_inbox_visible());
    }

    #[test]
    fn copy_and_equality() {
        let a = Status::Draft;
        let b = a;
        assert_eq!(a, b); // Copy works
        assert!(a == b);  // PartialEq works
    }

    #[test]
    fn as_ref_str() {
        let status = Status::Sent;
        let s: &str = status.as_ref();
        assert_eq!(s, "sent");
    }
}
