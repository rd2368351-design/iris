//! Shared types used across every Iris crate.
//!
//! Nothing in this crate should depend on any other Iris crate — it sits
//! at the very bottom of the dependency graph so everything else can
//! depend on it safely.

pub mod error;
pub mod id;
pub mod email;
pub mod message_id;
pub mod mailbox_id;
pub mod account_id;
pub mod thread_id;
pub mod blob_id;

pub use error::Error;
pub use id::Id;
pub use email::EmailAddress;
pub use message_id::MessageId;
pub use mailbox_id::MailboxId;
pub use account_id::AccountId;
pub use thread_id::ThreadId;
pub use blob_id::BlobId;