use thiserror::Error;

/// Errors that can originate from the foundational `types` crate.
///
/// Higher-level crates (store, directory, smtp, ...) define their own
/// error types and wrap this one with `#[from]` where relevant, rather
/// than every crate in the workspace depending on one giant enum.
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid id: {0}")]
    InvalidId(String),

    #[error("invalid email address: {0}")]
    InvalidEmail(String),
}