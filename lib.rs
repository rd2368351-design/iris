//! Shared types used across every Iris crate.
//!
//! Nothing in this crate should depend on any other Iris crate — it sits
//! at the very bottom of the dependency graph so everything else can
//! depend on it safely.

pub mod error;
pub mod id;
pub mod email;

pub use error::Error;
pub use id::Id;