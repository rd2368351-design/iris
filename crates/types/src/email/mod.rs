//! Email-related shared types.

pub mod address;
pub mod attachment;
pub mod body;
pub mod envelope;
pub mod flags;
pub mod header;
pub mod mailbox;
pub mod message;
pub mod message_id;
pub mod folder;

pub use address::EmailAddress;
pub use attachment::Attachment;
pub use body::Body;
pub use envelope::Envelope;
pub use flags::{Flag, Flags};
pub use header::Header;
pub use mailbox::Mailbox;
pub use message::Message;
pub use message_id::MessageId;
pub use folder::Folder;