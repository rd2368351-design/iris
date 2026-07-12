use serde::{Deserialize, Serialize};
use std::fmt;

/// A logical mail folder (mailbox) name.
///
/// This type represents user-visible folders such as:
/// - Inbox
/// - Sent
/// - Drafts
/// - Trash
/// - Archive
/// - Spam
/// - Custom folders
///
/// It intentionally contains only the validated folder name.
/// Permissions, hierarchy, counters and synchronization metadata
/// belong to higher-level crates.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)] // FIXED: Added transparent for single-field struct
pub struct Folder {
    name: String,
}

impl Folder {
    /// Maximum length for a folder name.
    pub const MAX_LEN: usize = 255;

    /// Creates a validated folder.
    pub fn new(name: impl AsRef<str>) -> Result<Self, crate::Error> {
        let trimmed = name.as_ref().trim();

        if trimmed.is_empty() {
            // Convert to String only for the error path
            return Err(crate::Error::InvalidFolderName(name.as_ref().to_string()));
        }

        if trimmed.len() > Self::MAX_LEN {
            return Err(crate::Error::InvalidFolderName(name.as_ref().to_string()));
        }

        Ok(Self {
            name: trimmed.to_string(), // FIXED: Only ONE allocation happens here!
        })
    }

    /// Returns the folder name.
    pub fn name(&self) -> &str {
        &self.name
    }

    // --- IMAP STANDARD FOLDER CHECKS ---
    // eq_ignore_ascii_case is absolutely PERFECT here. IMAP strictly requires
    // "INBOX" to be case-insensitive in almost all contexts.

    /// Returns true if this is the Inbox.
    pub fn is_inbox(&self) -> bool {
        self.name.eq_ignore_ascii_case("Inbox")
    }

    /// Returns true if this is the Trash.
    pub fn is_trash(&self) -> bool {
        self.name.eq_ignore_ascii_case("Trash")
    }

    /// Returns true if this is the Sent folder.
    pub fn is_sent(&self) -> bool {
        self.name.eq_ignore_ascii_case("Sent")
    }

    /// Returns true if this is the Drafts folder.
    pub fn is_drafts(&self) -> bool {
        self.name.eq_ignore_ascii_case("Drafts")
    }

    /// Returns true if this is the Spam folder.
    pub fn is_spam(&self) -> bool {
        self.name.eq_ignore_ascii_case("Spam")
    }

    /// Returns true if this is the Archive folder.
    pub fn is_archive(&self) -> bool {
        self.name.eq_ignore_ascii_case("Archive")
    }
}

impl fmt::Display for Folder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_folder() {
        // Now extremely efficient with &str
        let folder = Folder::new("Inbox").unwrap();
        assert_eq!(folder.name(), "Inbox");
        assert!(folder.is_inbox());
    }

    #[test]
    fn reject_empty_folder() {
        assert!(Folder::new("").is_err());
    }

    #[test]
    fn trim_name() {
        let folder = Folder::new("  Sent  ").unwrap();
        assert_eq!(folder.name(), "Sent");
        assert!(folder.is_sent());
    }
    
    #[test]
    fn case_insensitive_standard_folders() {
        // IMAP requires INBOX to be treated case-insensitively.
        let folder = Folder::new("iNbOx").unwrap();
        assert!(folder.is_inbox());
    }
}
