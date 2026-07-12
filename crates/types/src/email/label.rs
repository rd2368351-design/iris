use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A user-defined label that can be attached to one or more email messages.
///
/// Unlike folders, labels do not determine where a message is stored.
/// A single message may have multiple labels.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)] // Absolutely perfect!
pub struct Label {
    name: String,
}

impl Label {
    /// Maximum length for a label name.
    pub const MAX_LEN: usize = 100;

    /// Creates a validated label.
    pub fn new(name: impl AsRef<str>) -> Result<Self, crate::Error> {
        let trimmed = name.as_ref().trim();

        if trimmed.is_empty() {
            return Err(crate::Error::InvalidLabelName(name.as_ref().to_string()));
        }

        if trimmed.len() > Self::MAX_LEN {
            return Err(crate::Error::InvalidLabelName(name.as_ref().to_string()));
        }

        Ok(Self {
            name: trimmed.to_string(), // FIXED: Single allocation!
        })
    }

    /// Returns the label name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns true if this is a system label.
    pub fn is_system(&self) -> bool {
        // FIXED: Case-insensitive check makes it robust against different API clients
        let lower = self.name.to_ascii_lowercase();
        matches!(
            lower.as_str(),
            "important" | "starred" | "unread" | "flagged" | "junk" | "phishing"
        )
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)
    }
}

impl FromStr for Label {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_label() {
        let label = Label::new("Work").unwrap();
        assert_eq!(label.name(), "Work");
        assert!(!label.is_system());
    }

    #[test]
    fn create_system_label() {
        let label = Label::new("Important").unwrap();
        assert!(label.is_system());
        
        // Testing case-insensitivity
        let lower_label = Label::new("important").unwrap();
        assert!(lower_label.is_system());
    }

    #[test]
    fn reject_empty_label() {
        assert!(Label::new("").is_err());
    }

    #[test]
    fn trim_label() {
        let label = Label::new("  Personal  ").unwrap();
        assert_eq!(label.name(), "Personal");
    }

    #[test]
    fn from_str() {
        let label: Label = "Starred".parse().unwrap();
        assert!(label.is_system());
    }

    #[test]
    fn serde_transparent() {
        let label = Label::new("Work").unwrap();
        // Assuming serde_json is in dev-dependencies
        let json = serde_json::to_string(&label).unwrap();

        assert_eq!(json, "\"Work\"");
    }
}
