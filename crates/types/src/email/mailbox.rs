use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::{Error, Id};

/// Identifies a mailbox (Inbox, Sent, Trash, Archive, ...).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Mailbox {
    #[serde(transparent)]
    id: Id,
    name: String,
}

impl Mailbox {
    pub fn new(id: Id, name: impl Into<String>) -> Result<Self, Error> {
        let name = name.into().trim().to_string();

        if name.is_empty() {
            return Err(Error::InvalidMailboxName);
        }

        Ok(Self { id, name })
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rename(&mut self, name: impl Into<String>) -> Result<(), Error> {
        let name = name.into().trim().to_string();

        if name.is_empty() {
            return Err(Error::InvalidMailboxName);
        }

        self.name = name;
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

    #[test]
    fn creates_mailbox() {
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
        let mailbox = Mailbox::new(Id::new(2), "  Sent  ").unwrap();

        assert_eq!(mailbox.name(), "Sent");
    }
}