use serde::{Deserialize, Serialize};
use bitflags::bitflags;

use super::{EmailAddress, Header};
use crate::{MailboxId, MessageId};

bitflags! {
    /// Standard IMAP/JMAP message system flags.
    /// Packed into a single `u32` for memory efficiency.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct MessageFlags: u32 {
        const SEEN     = 1 << 0;
        const ANSWERED = 1 << 1;
        const FLAGGED  = 1 << 2;
        const DELETED  = 1 << 3;
        const DRAFT    = 1 << 4;
        const RECENT   = 1 << 5;
    }
}

/// A stored email message.
///
/// Contains metadata shared across the workspace. MIME parsing,
/// attachments, and body decoding belong in the `email` crate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    flags: MessageFlags,
}

impl Message {
    /// Creates a new message with the minimum required fields.
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
            flags: MessageFlags::RECENT,
        }
    }

    // --- GETTERS ---

    #[inline]
    pub const fn id(&self) -> MessageId {
        self.id
    }

    #[inline]
    pub const fn mailbox_id(&self) -> MailboxId {
        self.mailbox_id
    }

    #[inline]
    pub fn from(&self) -> &EmailAddress {
        &self.from
    }

    #[inline]
    pub fn to(&self) -> &[EmailAddress] {
        &self.to
    }

    #[inline]
    pub fn cc(&self) -> &[EmailAddress] {
        &self.cc
    }

    #[inline]
    pub fn bcc(&self) -> &[EmailAddress] {
        &self.bcc
    }

    #[inline]
    pub fn reply_to(&self) -> Option<&EmailAddress> {
        self.reply_to.as_ref()
    }

    #[inline]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns the Unix timestamp (seconds) when available.
    #[inline]
    pub const fn date(&self) -> Option<i64> {
        self.date
    }

    #[inline]
    pub fn headers(&self) -> &[Header] {
        &self.headers
    }

    #[inline]
    pub const fn flags(&self) -> MessageFlags {
        self.flags
    }

    // --- STATE CHECKERS ---

    #[inline]
    pub fn is_seen(&self) -> bool {
        self.flags.contains(MessageFlags::SEEN)
    }

    #[inline]
    pub fn is_draft(&self) -> bool {
        self.flags.contains(MessageFlags::DRAFT)
    }

    #[inline]
    pub fn is_deleted(&self) -> bool {
        self.flags.contains(MessageFlags::DELETED)
    }

    #[inline]
    pub fn is_recent(&self) -> bool {
        self.flags.contains(MessageFlags::RECENT)
    }

    #[inline]
    pub fn is_flagged(&self) -> bool {
        self.flags.contains(MessageFlags::FLAGGED)
    }

    // --- SETTERS (Builder-style) ---

    #[inline]
    pub fn with_cc(mut self, cc: Vec<EmailAddress>) -> Self {
        self.cc = cc;
        self
    }

    #[inline]
    pub fn with_bcc(mut self, bcc: Vec<EmailAddress>) -> Self {
        self.bcc = bcc;
        self
    }

    #[inline]
    pub fn with_reply_to(mut self, reply_to: EmailAddress) -> Self {
        self.reply_to = Some(reply_to);
        self
    }

    #[inline]
    pub fn with_date(mut self, date: i64) -> Self {
        self.date = Some(date);
        self
    }

    #[inline]
    pub fn with_headers(mut self, headers: Vec<Header>) -> Self {
        self.headers = headers;
        self
    }

    // --- FLAG MUTATORS ---

    #[inline]
    pub fn set_seen(&mut self) {
        self.flags.insert(MessageFlags::SEEN);
    }

    #[inline]
    pub fn set_deleted(&mut self) {
        self.flags.insert(MessageFlags::DELETED);
    }

    #[inline]
    pub fn set_flagged(&mut self) {
        self.flags.insert(MessageFlags::FLAGGED);
    }

    #[inline]
    pub fn clear_recent(&mut self) {
        self.flags.remove(MessageFlags::RECENT);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_message() {
        let msg = Message::new(
            MessageId::new(1),
            MailboxId::new(10),
            EmailAddress::parse("from@example.com").unwrap(),
            vec![EmailAddress::parse("to@example.com").unwrap()],
            "Hello",
        );
        assert_eq!(msg.subject(), "Hello");
        assert!(msg.is_recent());
        assert!(!msg.is_seen());
    }

    #[test]
    fn builder_pattern() {
        let msg = Message::new(
            MessageId::new(1),
            MailboxId::new(1),
            EmailAddress::parse("a@b.com").unwrap(),
            vec![],
            "Subject",
        )
        .with_cc(vec![EmailAddress::parse("cc@b.com").unwrap()])
        .with_date(1_700_000_000);
        
        assert_eq!(msg.cc().len(), 1);
        assert_eq!(msg.date(), Some(1_700_000_000));
    }

    #[test]
    fn flag_operations() {
        let mut msg = Message::new(
            MessageId::new(1),
            MailboxId::new(1),
            EmailAddress::parse("a@b.com").unwrap(),
            vec![],
            "Test",
        );
        assert!(!msg.is_seen());
        msg.set_seen();
        assert!(msg.is_seen());
        msg.clear_recent();
        assert!(!msg.is_recent());
    }
}
