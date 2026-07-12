use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid id '{value}': {reason}")]
    InvalidId {
        value: String,
        #[source]
        reason: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("invalid email address '{addr}': {reason}")]
    InvalidEmail {
        addr: String,
        #[source]
        reason: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("invalid domain: {0}")]
    InvalidDomain(String),

    #[error("invalid tenant: {0}")]
    InvalidTenant(String),

    #[error("invalid mailbox name")]
    InvalidMailboxName,
}