use serde::{Deserialize, Serialize};

use super::{EmailAddress, Header};
use crate::{MailboxId, MessageId};

/// A stored email message.
///
/// This type contains only the common metadata shared across the
/// workspace. MIME parsing, attachments and body decoding belong in the
/// `email` crate, not the `types` crate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Message {
    id: MessageId,
    mailbox_id: MailboxId,
    from: EmailAddress,
    to: Vec<EmailAddress>,
    cc: Vec<EmailAddress>,
    bcc: Vec<EmailAddress>,
    reply_to: Option<EmailAddress>,
    subject: String,
    date: Option<i64>,
    headers: Vec<Header>,
}

impl Message {
    pub fn new(
        id: MessageId,
        mailbox_id: MailboxId,
        from: EmailAddress,
        to: Vec<EmailAddress>,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            id,
            mailbox_id,
            from,
            to,
            cc: Vec::new(),
            bcc: Vec::new(),
            reply_to: None,
            subject: subject.into(),
            date: None,
            headers: Vec::new(),
        }
    }

    pub fn id(&self) -> MessageId {
        self.id
    }

    pub fn mailbox_id(&self) -> MailboxId {
        self.mailbox_id
    }

    pub fn from(&self) -> &EmailAddress {
        &self.from
    }

    pub fn to(&self) -> &[EmailAddress] {
        &self.to
    }

    pub fn cc(&self) -> &[EmailAddress] {
        &self.cc
    }

    pub fn bcc(&self) -> &[EmailAddress] {
        &self.bcc
    }

    pub fn reply_to(&self) -> Option<&EmailAddress> {
        self.reply_to.as_ref()
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn date(&self) -> Option<i64> {
        self.date
    }

    pub fn headers(&self) -> &[Header] {
        &self.headers
    }

    pub fn add_header(&mut self, header: Header) {
        self.headers.push(header);
    }

    pub fn add_recipient(&mut self, addr: EmailAddress) {
        self.to.push(addr);
    }

    pub fn add_cc(&mut self, addr: EmailAddress) {
        self.cc.push(addr);
    }

    pub fn set_reply_to(&mut self, addr: EmailAddress) {
        self.reply_to = Some(addr);
    }

    pub fn set_date(&mut self, timestamp: i64) {
        self.date = Some(timestamp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Id, MailboxId};

    #[test]
    fn creates_message() {
        let from = EmailAddress::parse("alice@example.com").unwrap();
        let to = EmailAddress::parse("bob@example.com").unwrap();

        let message = Message::new(
            MessageId::new(1),
            MailboxId::from(Id::new(10)),
            from,
            vec![to],
            "Hello",
        );

        assert_eq!(message.subject(), "Hello");
        assert_eq!(message.to().len(), 1);
        assert!(message.cc().is_empty());
        assert!(message.bcc().is_empty());
        assert!(message.reply_to().is_none());
    }

    #[test]
    fn adds_recipients() {
        let from = EmailAddress::parse("alice@example.com").unwrap();
        let to = EmailAddress::parse("bob@example.com").unwrap();
        let cc = EmailAddress::parse("carol@example.com").unwrap();

        let mut message = Message::new(
            MessageId::new(2),
            MailboxId::from(Id::new(11)),
            from,
            vec![to],
            "Meeting",
        );

        message.add_cc(cc);

        assert_eq!(message.cc().len(), 1);
    }
}