use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Id;

/// Strongly-typed identifier for a stored binary object (blob).
///
/// Blobs represent attachment data, message bodies, thumbnails,
/// S/MIME certificates, and other binary content stored by the mail server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BlobId(Id);

impl BlobId {
    /// Create a new blob identifier.
    pub fn new(id: Id) -> Self {
        Self(id)
    }

    /// Returns the wrapped generic identifier.
    pub fn id(self) -> Id {
        self.0
    }

    /// Returns the raw numeric value.
    pub fn value(self) -> u64 {
        self.0.value()
    }
}

impl fmt::Display for BlobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for BlobId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Id::from_str(s)?))
    }
}

impl From<Id> for BlobId {
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl From<BlobId> for Id {
    fn from(id: BlobId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_generic_id() {
        let id = Id::new(999);
        let blob = BlobId::new(id);

        assert_eq!(blob.id(), id);
        assert_eq!(blob.value(), 999);
    }

    #[test]
    fn roundtrip() {
        let id = BlobId::new(Id::new(12345));
        let text = id.to_string();
        let parsed: BlobId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }
}