use thiserror::Error;

/// Errors that can originate from the foundational `types` crate.
///
/// Higher-level crates (store, directory, smtp, ...) define their own
/// error types and wrap this one with `#[from]` where relevant, rather
/// than every crate in the workspace depending on one giant enum.
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
}