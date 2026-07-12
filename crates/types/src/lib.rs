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
pub mod domain_id;
pub mod user_id;
pub mod session_id;
pub mod identity_id;
pub mod tenant_id;
pub mod queue_id;
pub mod attachment_id;
pub mod folder_id;
pub mod device_id;
pub mod api_key_id;
pub mod calendar_id;
pub mod contact_id;
pub mod event_id;
pub mod group_id;
pub mod role_id;
pub mod permission_id;
pub mod token_id;
pub mod upload_id;
pub mod notification_id;
pub mod job_id;
pub mod label_id;

pub use error::Error;
pub use id::Id;
pub use email::EmailAddress;
pub use message_id::MessageId;
pub use mailbox_id::MailboxId;
pub use account_id::AccountId;
pub use thread_id::ThreadId;
pub use blob_id::BlobId;
pub use domain_id::DomainId;
pub use user_id::UserId;
pub use session_id::SessionId;
pub use identity_id::IdentityId;
pub use tenant_id::TenantId;
pub use queue_id::QueueId;
pub use attachment_id::AttachmentId;
pub use folder_id::FolderId;
pub use device_id::DeviceId;
pub use api_key_id::ApiKeyId;
pub use calendar_id::CalendarId;
pub use contact_id::ContactId;
pub use event_id::EventId;
pub use group_id::GroupId;
pub use role_id::RoleId;
pub use permission_id::PermissionId;
pub use token_id::TokenId;
pub use upload_id::UploadId;
pub use notification_id::NotificationId;
pub use job_id::JobId;
pub use label_id::LabelId;