use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Top-level application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server binding configuration.
    pub server: ServerConfig,

    /// Storage backend configuration.
    pub storage: StorageConfig,

    /// TLS configuration.
    pub tls: TlsConfig,

    /// Authentication settings.
    pub auth: AuthConfig,

    /// Logging and observability.
    pub logging: LoggingConfig,

    /// Rate limiting.
    pub limits: LimitConfig,

    /// Extra plugin-specific settings.
    #[serde(flatten)]
    pub extra: HashMap<String, toml::Value>,
}

/// Network binding configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// SMTP server bind address.
    pub smtp_bind: String,

    /// IMAP server bind address.
    pub imap_bind: String,

    /// JMAP server bind address.
    pub jmap_bind: String,

    /// Admin API bind address.
    pub admin_bind: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            smtp_bind: "0.0.0.0:587".to_string(),
            imap_bind: "0.0.0.0:143".to_string(),
            jmap_bind: "0.0.0.0:443".to_string(),
            admin_bind: "127.0.0.1:8080".to_string(),
        }
    }
}

/// Storage backend configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "backend", rename_all = "lowercase")]
pub enum StorageConfig {
    /// RocksDB embedded storage.
    RocksDb { path: String },

    /// SQLite embedded storage.
    Sqlite { path: String },

    /// PostgreSQL server.
    Postgres {
        host: String,
        port: u16,
        database: String,
        user: String,
        password: String,
    },
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self::RocksDb {
            path: "./data".to_string(),
        }
    }
}

/// TLS certificate configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Enable TLS.
    pub enabled: bool,

    /// Path to certificate file.
    pub cert_path: Option<String>,

    /// Path to private key file.
    pub key_path: Option<String>,

    /// Auto-generate via ACME/Let's Encrypt.
    pub acme: Option<AcmeConfig>,
}

/// ACME configuration for automatic certificates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcmeConfig {
    /// ACME directory URL.
    pub directory_url: String,

    /// Contact email.
    pub email: String,

    /// Accept terms of service.
    pub accept_tos: bool,
}

/// Authentication configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Password hashing scheme.
    pub password_hash: String,

    /// Session lifetime (hours).
    pub session_ttl_hours: u64,

    /// Enable OAuth2.
    pub oauth_enabled: bool,

    /// Enable LDAP.
    pub ldap_enabled: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            password_hash: "argon2".to_string(),
            session_ttl_hours: 24,
            oauth_enabled: false,
            ldap_enabled: false,
        }
    }
}

/// Logging configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error).
    pub level: String,

    /// Output format (json, pretty).
    pub format: String,

    /// Log file path (optional).
    pub file: Option<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
            file: None,
        }
    }
}

/// Rate limiting configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitConfig {
    /// Max connections per IP.
    pub max_connections_per_ip: u32,

    /// Max messages per minute per user.
    pub max_messages_per_min: u32,

    /// Max login attempts per minute.
    pub max_logins_per_min: u32,
}

impl Default for LimitConfig {
    fn default() -> Self {
        Self {
            max_connections_per_ip: 10,
            max_messages_per_min: 60,
            max_logins_per_min: 5,
        }
    }
}

impl Config {
    /// Load configuration from a TOML file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Load from string (useful for tests).
    pub fn from_str(s: &str) -> Result<Self, ConfigError> {
        let config: Self = toml::from_str(s)?;
        config.validate()?;
        Ok(config)
    }

    /// Validate configuration values.
    fn validate(&self) -> Result<(), ConfigError> {
        // Add validation logic
        Ok(())
    }
}

/// Configuration errors.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = Config {
            server: ServerConfig::default(),
            storage: StorageConfig::default(),
            tls: TlsConfig {
                enabled: true,
                cert_path: None,
                key_path: None,
                acme: None,
            },
            auth: AuthConfig::default(),
            logging: LoggingConfig::default(),
            limits: LimitConfig::default(),
            extra: HashMap::new(),
        };

        assert_eq!(config.server.smtp_bind, "0.0.0.0:587");
    }

    #[test]
    fn parse_from_toml() {
        let toml = r#"
[server]
smtp_bind = "0.0.0.0:2525"
imap_bind = "0.0.0.0:1143"

[storage]
backend = "rocksdb"
path = "/var/lib/iris/data"

[tls]
enabled = true

[auth]
password_hash = "argon2"
session_ttl_hours = 48

[logging]
level = "debug"

[limits]
max_connections_per_ip = 20
"#;

        let config = Config::from_str(toml).unwrap();
        assert_eq!(config.server.smtp_bind, "0.0.0.0:2525");
        assert_eq!(config.auth.session_ttl_hours, 48);
    }
}
