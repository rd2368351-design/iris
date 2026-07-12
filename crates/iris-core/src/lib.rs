//! Core utilities and configuration for the Iris mail platform.
//!
//! This crate sits just above `iris-types` in the dependency graph
//! and provides shared configuration, validation, and utility
//! functionality used by all protocol and service crates.

pub mod config;
pub mod constants;
pub mod utils;
pub mod validation;

pub use config::Config;
pub use constants::*;
pub use validation::{Validator, ValidationError};
