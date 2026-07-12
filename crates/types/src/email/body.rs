use serde::{Deserialize, Serialize};

/// The decoded body of an email message.
///
/// MIME parsing and transfer decoding are handled by the `email` crate.
/// This type only stores the normalized content.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Body {
    text: Option<String>,
    html: Option<String>,
}

impl Body {
    /// Creates an empty body.
    pub fn new() -> Self {
        Self {
            text: None,
            html: None,
        }
    }

    /// Creates a plain-text body.
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            html: None,
        }
    }

    /// Creates an HTML body.
    pub fn html(html: impl Into<String>) -> Self {
        Self {
            text: None,
            html: Some(html.into()),
        }
    }

    /// Creates a multipart body.
    pub fn multipart(text: impl Into<String>, html: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            html: Some(html.into()),
        }
    }

    /// Returns the plain-text body.
    pub fn text_part(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Returns the HTML body.
    pub fn html_part(&self) -> Option<&str> {
        self.html.as_deref()
    }

    /// Returns true if the body is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_none() && self.html.is_none()
    }

    /// Returns true if the body has both text and HTML parts.
    pub fn is_multipart(&self) -> bool {
        self.text.is_some() && self.html.is_some()
    }

    /// Returns true if only plain text is present.
    pub fn is_text_only(&self) -> bool {
        self.text.is_some() && self.html.is_none()
    }

    /// Returns true if only HTML is present.
    pub fn is_html_only(&self) -> bool {
        self.html.is_some() && self.text.is_none()
    }

    /// Sets the plain text content.
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = Some(text.into());
    }

    /// Sets the HTML content.
    pub fn set_html(&mut self, html: impl Into<String>) {
        self.html = Some(html.into());
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_text_body() {
        let body = Body::text("Hello");

        assert_eq!(body.text_part(), Some("Hello"));
        assert!(body.html_part().is_none());
        assert!(body.is_text_only());
    }

    #[test]
    fn creates_html_body() {
        let body = Body::html("<b>Hello</b>");

        assert_eq!(body.html_part(), Some("<b>Hello</b>"));
        assert!(body.is_html_only());
    }

    #[test]
    fn creates_multipart_body() {
        let body = Body::multipart("Hello", "<b>Hello</b>");

        assert!(body.text_part().is_some());
        assert!(body.html_part().is_some());
        assert!(body.is_multipart());
    }

    #[test]
    fn empty_body() {
        let body = Body::new();

        assert!(body.is_empty());
        assert!(!body.is_multipart());
    }

    #[test]
    fn setters() {
        let mut body = Body::new();

        body.set_text("Plain");
        assert!(body.is_text_only());

        body.set_html("<p>HTML</p>");
        assert!(body.is_multipart());
    }
}