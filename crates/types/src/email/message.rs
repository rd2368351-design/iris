use serde::{Deserialize, Serialize};
use bitflags::bitflags;

// Assuming these are defined elsewhere in your workspace
use super::{EmailAddress, Header};
use crate::{MailboxId, MessageId};

// --- BITFLAGS FOR MESSAGE STATES ---

bitflags! {
    /// Represents standard IMAP/JMAP message system flags.
    /// Uses a single `u32` (4 bytes) to store up to 32 different boolean states!
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(transparent)] // Yahan transparent ekdum perfectly kaam karega kyunki yeh secretly sirf ek u32 hai!
    pub struct MessageFlags: u32 {
        /// The message has been read (IMAP \Seen)
        const SEEN     = 1 << 0;
        /// The message has been answered (IMAP \Answered)
        const ANSWERED = 1 << 1;
        /// The message is marked as important/starred (IMAP \Flagged)
        const FLAGGED  = 1 << 2;
        /// The message is marked for deletion (IMAP \Deleted)
        const DELETED  = 1 << 3;
        /// The message has not completed composition (IMAP \Draft)
        const DRAFT    = 1 << 4;
        /// The message recently arrived in this mailbox (IMAP \Recent)
        const RECENT   = 1 << 5;
    }
}

// --- MESSAGE ENTITY ---

/// A stored email message.
///
/// This type contains only the common metadata shared across the
/// workspace. MIME parsing, attachments and body decoding belong in the
/// `email` crate, not the `types` crate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
// FIXED: Removed #[serde(transparent)] because this struct has multiple fields
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
    
    /// The current state/flags of the message
    flags: MessageFlags,
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
            // By default, a new message gets the RECENT flag, but is unread
            flags: MessageFlags::RECENT, 
        }
    }

    // --- GETTERS ---

    pub fn id(&self) -> MessageId { self.id }
    pub fn mailbox_id(&self) -> MailboxId { self.mailbox_id }
    pub fn from(&self) -> &EmailAddress { &self.from }
    pub fn to(&self) -> &[EmailAddress] { &self.to }
    pub fn cc(&self) -> &[EmailAddress] { &self.cc }
    pub fn bcc(&self) -> &[EmailAddress] { &self.bcc }
    pub fn reply_to(&self) -> Option<&EmailAddress> { self.reply_to.as_ref() }
    pub fn subject(&self) -> &str { &self.subject }
    pub fn date(&self) -> Option<i64> { self.date }
    pub fn headers(&self) -> &[Header] { &self.headers }
    pub fn flags(&self) -> MessageFlags { self.flags }

    // --- SETTERS / MUTATORS ---

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

    // --- FLAG MANAGEMENT (NEW) ---

    pub fn add_flag(&mut self, flag: MessageFlags) {
        self.flags.insert(flag);
    }
    
    pub fn remove_flag(&mut self, flag: MessageFlags) {
        self.flags.remove(flag);
    }
    
    pub fn has_flag(&self, flag: MessageFlags) -> bool {
        self.flags.contains(flag)
    }
}

// --- TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Id, MailboxId, MessageId};

    #[test]
    fn creates_message_with_default_flags() {
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
        // Check that RECENT is set by default
        assert!(message.has_flag(MessageFlags::RECENT));
        // Check that SEEN is NOT set
        assert!(!message.has_flag(MessageFlags::SEEN));
    }

    #[test]
    fn manages_flags_correctly() {
        let from = EmailAddress::parse("alice@example.com").unwrap();
        let to = EmailAddress::parse("bob@example.com").unwrap();

        let mut message = Message::new(
            MessageId::new(2),
            MailboxId::from(Id::new(11)),
            from,
            vec![to],
            "Urgent Meeting",
        );

        // Add FLAGGED and SEEN states
        message.add_flag(MessageFlags::FLAGGED | MessageFlags::SEEN);
        
        assert!(message.has_flag(MessageFlags::FLAGGED));
        assert!(message.has_flag(MessageFlags::SEEN));

        // Remove the RECENT state
        message.remove_flag(MessageFlags::RECENT);
        assert!(!message.has_flag(MessageFlags::RECENT));
    }
}
