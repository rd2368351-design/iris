//! Email-related shared types.

pub mod address;
pub mod body;
pub mod envelope;
pub mod header;
pub mod mailbox;
pub mod message;
pub mod message_id;

pub use address::EmailAddress;
pub use body::Body;
pub use envelope::Envelope;
pub use header::Header;
pub use mailbox::Mailbox;
pub use message::Message;
pub use message_id::MessageId;