//! Email-specific types and entities.

pub mod address;
pub mod attachment;
pub mod body;
pub mod envelope;
pub mod flags;
pub mod folder;
pub mod header;
pub mod label;
pub mod mailbox;
pub mod message;
pub mod message_id;
pub mod priority;
pub mod status;

pub use address::EmailAddress;
pub use message::Message;
pub use message::MessageFlags;
pub use priority::Priority;
pub use status::Status;
pub use folder::Folder;
