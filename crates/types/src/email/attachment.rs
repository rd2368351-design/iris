Use serde::{Deserialize, Serialize};

use crate::BlobId;

/// Metadata describing an email attachment.
///
/// The actual attachment bytes are stored separately in blob storage.
/// This type only contains metadata needed by the rest of the system.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Attachment {
    blob_id: BlobId,
    filename: String,
    content_type: String,
    size: u64,
    inline: bool,
    content_id: Option<String>,
}

impl Attachment {
    /// Creates a new attachment.
    pub fn new(
        blob_id: BlobId,
        filename: impl Into<String>,
        content_type: impl Into<String>,
        size: u64,
    ) -> Self {
        Self {
            blob_id,
            filename: filename.into(),
            content_type: content_type.into(),
            size,
            inline: false,
            content_id: None,
        }
    }

    /// Returns the blob id.
    pub fn blob_id(&self) -> BlobId {
        self.blob_id
    }

    /// Returns the filename.
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Returns the MIME content type.
    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    /// Returns the attachment size in bytes.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Returns true if this is an inline attachment.
    pub fn is_inline(&self) -> bool {
        self.inline
    }

    /// Marks the attachment as inline.
    pub fn set_inline(&mut self, inline: bool) {
        self.inline = inline;
    }

    /// Returns the Content-ID, if any.
    pub fn content_id(&self) -> Option<&str> {
        self.content_id.as_deref()
    }

    /// Sets the Content-ID for inline references.
    pub fn set_content_id(&mut self, cid: impl Into<String>) {
        self.content_id = Some(cid.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Id;

    #[test]
    fn creates_attachment() {
        let attachment = Attachment::new(
            BlobId::from(Id::new(1)),
            "image.png",
            "image/png",
            2048,
        );

        assert_eq!(attachment.filename(), "image.png");
        assert_eq!(attachment.content_type(), "image/png");
        assert_eq!(attachment.size(), 2048);
        assert!(!attachment.is_inline());
        assert!(attachment.content_id().is_none());
    }

    #[test]
    fn inline_with_content_id() {
        let mut attachment = Attachment::new(
            BlobId::from(Id::new(2)),
            "logo.png",
            "image/png",
            4096,
        );

        attachment.set_inline(true);
        attachment.set_content_id("<logo@example.com>");

        assert!(attachment.is_inline());
        assert_eq!(attachment.content_id(), Some("<logo@example.com>"));
    }
}