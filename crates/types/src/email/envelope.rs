use serde::{Deserialize, Serialize};

use super::EmailAddress;

/// SMTP envelope.
///
/// The envelope is used during SMTP transport and is independent of the
/// message headers. For example, the envelope sender may differ from the
/// `From:` header.
///
/// The envelope can have a `null` sender (represented as `None`),
/// which is used for bounce messages and delivery status notifications.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Envelope {
    mail_from: Option<EmailAddress>,
    rcpt_to: Vec<EmailAddress>,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            mail_from: None,
            rcpt_to: Vec::new(),
        }
    }

    pub fn set_mail_from(&mut self, address: EmailAddress) {
        self.mail_from = Some(address);
    }

    pub fn set_null_sender(&mut self) {
        self.mail_from = None;
    }

    pub fn mail_from(&self) -> Option<&EmailAddress> {
        self.mail_from.as_ref()
    }

    pub fn is_null_sender(&self) -> bool {
        self.mail_from.is_none()
    }

    pub fn add_recipient(&mut self, address: EmailAddress) {
        self.rcpt_to.push(address);
    }

    pub fn recipients(&self) -> &[EmailAddress] {
        &self.rcpt_to
    }

    pub fn recipient_count(&self) -> usize {
        self.rcpt_to.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rcpt_to.is_empty()
    }

    pub fn clear_recipients(&mut self) {
        self.rcpt_to.clear();
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_envelope() {
        let mut env = Envelope::new();

        env.set_mail_from(
            EmailAddress::parse("alice@example.com").unwrap(),
        );

        env.add_recipient(
            EmailAddress::parse("bob@example.com").unwrap(),
        );

        env.add_recipient(
            EmailAddress::parse("carol@example.com").unwrap(),
        );

        assert_eq!(env.recipient_count(), 2);
        assert!(env.mail_from().is_some());
    }

    #[test]
    fn null_sender_for_bounces() {
        let mut env = Envelope::new();

        env.set_null_sender();

        assert!(env.is_null_sender());
        assert!(env.mail_from().is_none());
    }

    #[test]
    fn clears_recipients() {
        let mut env = Envelope::new();

        env.add_recipient(
            EmailAddress::parse("bob@example.com").unwrap(),
        );
        env.clear_recipients();

        assert!(env.is_empty());
    }
}