//! Application-wide constants.

/// Default SMTP port (submission).
pub const SMTP_SUBMISSION_PORT: u16 = 587;

/// Default SMTP port (legacy).
pub const SMTP_PORT: u16 = 25;

/// Default SMTPS port.
pub const SMTPS_PORT: u16 = 465;

/// Default IMAP port.
pub const IMAP_PORT: u16 = 143;

/// Default IMAPS port.
pub const IMAPS_PORT: u16 = 993;

/// Default JMAP port.
pub const JMAP_PORT: u16 = 443;

/// Default HTTP admin port.
pub const ADMIN_PORT: u16 = 8080;

/// Maximum message size (50 MB).
pub const MAX_MESSAGE_SIZE: usize = 50 * 1024 * 1024;

/// Maximum number of recipients per message.
pub const MAX_RECIPIENTS: usize = 100;

/// Default connection timeout (seconds).
pub const CONNECTION_TIMEOUT_SECS: u64 = 30;

/// Default idle timeout (minutes).
pub const IDLE_TIMEOUT_MINS: u64 = 30;

/// Maximum failed login attempts before lockout.
pub const MAX_LOGIN_ATTEMPTS: u32 = 5;

/// Lockout duration after failed attempts (minutes).
pub const LOCKOUT_DURATION_MINS: u64 = 30;
