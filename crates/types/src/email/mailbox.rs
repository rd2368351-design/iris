use serde::{Deserialize, Serialize};
use std::fmt;
// FromStr removed to avoid unused import warnings

use crate::{Error, Id};

/// Identifies a mailbox (Inbox, Sent, Trash, Archive, ...).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Mailbox {
    // Removed #[serde(transparent)] from the field
    id: Id,
    name: String,
}

impl Mailbox {
    /// Using impl AsRef<str> prevents the "Double Allocation" problem.
    pub fn new(id: Id, name: impl AsRef<str>) -> Result<Self, Error> {
        let trimmed_name = name.as_ref().trim();

        if trimmed_name.is_empty() {
            return Err(Error::InvalidMailboxName);
        }

        Ok(Self { 
            id, 
            name: trimmed_name.to_string() // Only allocated ONCE!
        })
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rename(&mut self, name: impl AsRef<str>) -> Result<(), Error> {
        let trimmed_name = name.as_ref().trim();

        if trimmed_name.is_empty() {
            return Err(Error::InvalidMailboxName);
        }

        // Overwrite the old string
        self.name = trimmed_name.to_string();
        Ok(())
    }
}

impl fmt::Display for Mailbox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Id; // Ensure Id is accessible in tests

    #[test]
    fn creates_mailbox() {
        // Testing with &str
        let mailbox = Mailbox::new(Id::new(1), "Inbox").unwrap();

        assert_eq!(mailbox.name(), "Inbox");
        assert_eq!(mailbox.id().value(), 1);
    }

    #[test]
    fn rejects_empty_name() {
        assert!(Mailbox::new(Id::new(1), "").is_err());
    }

    #[test]
    fn trims_whitespace() {
        // Testing with String directly to ensure AsRef<str> handles it
        let mailbox = Mailbox::new(Id::new(2), String::from("  Sent  ")).unwrap();

        assert_eq!(mailbox.name(), "Sent");
    }
}
