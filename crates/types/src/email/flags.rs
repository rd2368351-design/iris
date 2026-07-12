use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use bitflags::bitflags;

use crate::Error; 

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Flags: u8 {
        const SEEN     = 1 << 0;
        const ANSWERED = 1 << 1;
        const FLAGGED  = 1 << 2;
        const DELETED  = 1 << 3;
        const DRAFT    = 1 << 4;
        const RECENT   = 1 << 5;
    }
}

impl Flags {
    fn as_keyword(self) -> Option<&'static str> {
        match self {
            Self::SEEN => Some("\\Seen"),
            Self::ANSWERED => Some("\\Answered"),
            Self::FLAGGED => Some("\\Flagged"),
            Self::DELETED => Some("\\Deleted"),
            Self::DRAFT => Some("\\Draft"),
            Self::RECENT => Some("\\Recent"),
            _ => None,
        }
    }
}

impl FromStr for Flags {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "\\seen" | "seen" => Ok(Flags::SEEN),
            "\\answered" | "answered" => Ok(Flags::ANSWERED),
            "\\flagged" | "flagged" => Ok(Flags::FLAGGED),
            "\\deleted" | "deleted" => Ok(Flags::DELETED),
            "\\draft" | "draft" => Ok(Flags::DRAFT),
            "\\recent" | "recent" => Ok(Flags::RECENT),
            _ => Err(Error::InvalidFlag(s.to_string())), 
        }
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for flag in self.iter() {
            if let Some(keyword) = flag.as_keyword() {
                if !first {
                    f.write_str(" ")?;
                }
                f.write_str(keyword)?;
                first = false;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_and_contains_flags() {
        let mut flags = Flags::empty();

        flags.insert(Flags::SEEN);
        flags.insert(Flags::FLAGGED);

        assert!(flags.contains(Flags::SEEN));
        assert!(flags.contains(Flags::FLAGGED));
        assert!(!flags.contains(Flags::DRAFT));
    }

    #[test]
    fn removes_flags() {
        let mut flags = Flags::SEEN | Flags::DRAFT;

        flags.remove(Flags::SEEN);

        assert!(!flags.contains(Flags::SEEN));
        assert!(flags.contains(Flags::DRAFT));
    }

    #[test]
    fn flag_from_str() {
        assert_eq!("\\Seen".parse::<Flags>().unwrap(), Flags::SEEN);
        assert_eq!("seen".parse::<Flags>().unwrap(), Flags::SEEN);
        assert!("\\Invalid".parse::<Flags>().is_err());
    }

    #[test]
    fn display_roundtrip() {
        let flags = Flags::SEEN | Flags::FLAGGED;
        let text = flags.to_string();

        assert_eq!(text, "\\Seen \\Flagged");
    }
}
