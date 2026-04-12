use std::fmt;
use std::path::{Path, PathBuf};

use serde::Deserialize;

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Config {
    pub current_context: Option<String>,
    #[serde(default)]
    pub contexts: Vec<Context>,
}

#[derive(Debug, Deserialize)]
pub struct Context {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub insecure: bool,
    pub ca_cert_path: Option<String>,
    pub proxied_entities_chain: Option<String>,
    pub version_strategy: Option<String>,
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AuthConfig {
    Password {
        username: String,
        password_env: Option<String>,
        password: Option<String>,
    },
    Token {
        token_env: Option<String>,
        token: Option<String>,
    },
    Mtls {
        client_identity_path: String,
    },
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(toml::de::Error),
    UnknownContext(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "failed to read config file: {e}"),
            ConfigError::Parse(e) => write!(f, "failed to parse config file: {e}"),
            ConfigError::UnknownContext(name) => {
                write!(f, "current_context '{name}' not found in contexts")
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(e) => Some(e),
            ConfigError::Parse(e) => Some(e),
            ConfigError::UnknownContext(_) => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::Parse(e)
    }
}

// ---------------------------------------------------------------------------
// Config methods
// ---------------------------------------------------------------------------

impl Config {
    /// Load and validate a config from the given path.
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let raw = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&raw)?;
        config.validate()?;
        Ok(config)
    }

    /// Look up a context by name.
    pub fn find_context(&self, name: &str) -> Option<&Context> {
        self.contexts.iter().find(|c| c.name == name)
    }

    /// Return the active context, as determined by `current_context`.
    pub fn active_context(&self) -> Option<&Context> {
        self.current_context
            .as_deref()
            .and_then(|name| self.find_context(name))
    }

    /// Validate the config.
    ///
    /// - Warns to stderr when a plaintext `password` or `token` is present.
    /// - Returns `ConfigError::UnknownContext` when `current_context` names a
    ///   context that doesn't exist in `contexts`.
    pub fn validate(&self) -> Result<(), ConfigError> {
        for ctx in &self.contexts {
            match &ctx.auth {
                AuthConfig::Password {
                    password: Some(_), ..
                } => {
                    eprintln!(
                        "warning: context '{}' has a plaintext password in config; \
                         prefer password_env",
                        ctx.name
                    );
                }
                AuthConfig::Token { token: Some(_), .. } => {
                    eprintln!(
                        "warning: context '{}' has a plaintext token in config; \
                         prefer token_env",
                        ctx.name
                    );
                }
                _ => {}
            }
        }

        if let Some(current) = &self.current_context
            && self.find_context(current).is_none()
        {
            return Err(ConfigError::UnknownContext(current.clone()));
        }

        Ok(())
    }

    /// Return the default config file path: `$HOME/.nifictl/config.toml`.
    pub fn default_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".nifictl").join("config.toml")
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const FULL_CONFIG: &str = r#"
current_context = "prod"

[[contexts]]
name = "prod"
url = "https://nifi-prod:8443"
insecure = false
ca_cert_path = "/etc/ssl/nifi-ca.pem"
proxied_entities_chain = "<CN=admin>"
version_strategy = "closest"

[contexts.auth]
type = "password"
username = "admin"
password_env = "NIFI_PASSWORD"

[[contexts]]
name = "staging"
url = "https://nifi-staging:8443"
insecure = true

[contexts.auth]
type = "token"
token_env = "NIFI_TOKEN"

[[contexts]]
name = "mtls"
url = "https://nifi-mtls:8443"

[contexts.auth]
type = "mtls"
client_identity_path = "/etc/ssl/client.pem"
"#;

    #[test]
    fn parse_full_config() {
        let config: Config = toml::from_str(FULL_CONFIG).expect("should parse");

        assert_eq!(config.current_context.as_deref(), Some("prod"));
        assert_eq!(config.contexts.len(), 3);

        // prod — password auth via env
        let prod = &config.contexts[0];
        assert_eq!(prod.name, "prod");
        assert_eq!(prod.url, "https://nifi-prod:8443");
        assert!(!prod.insecure);
        assert_eq!(prod.ca_cert_path.as_deref(), Some("/etc/ssl/nifi-ca.pem"));
        assert_eq!(prod.proxied_entities_chain.as_deref(), Some("<CN=admin>"));
        assert_eq!(prod.version_strategy.as_deref(), Some("closest"));
        match &prod.auth {
            AuthConfig::Password {
                username,
                password_env,
                password,
            } => {
                assert_eq!(username, "admin");
                assert_eq!(password_env.as_deref(), Some("NIFI_PASSWORD"));
                assert!(password.is_none());
            }
            other => panic!("expected Password auth, got {other:?}"),
        }

        // staging — token auth via env
        let staging = &config.contexts[1];
        assert_eq!(staging.name, "staging");
        assert!(staging.insecure);
        match &staging.auth {
            AuthConfig::Token { token_env, token } => {
                assert_eq!(token_env.as_deref(), Some("NIFI_TOKEN"));
                assert!(token.is_none());
            }
            other => panic!("expected Token auth, got {other:?}"),
        }

        // mtls
        let mtls = &config.contexts[2];
        assert_eq!(mtls.name, "mtls");
        match &mtls.auth {
            AuthConfig::Mtls {
                client_identity_path,
            } => {
                assert_eq!(client_identity_path, "/etc/ssl/client.pem");
            }
            other => panic!("expected Mtls auth, got {other:?}"),
        }
    }

    #[test]
    fn parse_password_plaintext_warns() {
        let toml_str = r#"
[[contexts]]
name = "dev"
url = "https://nifi-dev:8443"

[contexts.auth]
type = "password"
username = "admin"
password = "s3cr3t"
"#;
        let config: Config = toml::from_str(toml_str).expect("should parse");
        // validate() should succeed (only warn), not return an error
        let result = config.validate();
        assert!(
            result.is_ok(),
            "plaintext password should warn but not error"
        );

        match &config.contexts[0].auth {
            AuthConfig::Password { password, .. } => {
                assert_eq!(password.as_deref(), Some("s3cr3t"));
            }
            other => panic!("expected Password auth, got {other:?}"),
        }
    }

    #[test]
    fn unknown_current_context_is_error() {
        let toml_str = r#"
current_context = "nonexistent"

[[contexts]]
name = "dev"
url = "https://nifi-dev:8443"

[contexts.auth]
type = "token"
token_env = "NIFI_TOKEN"
"#;
        let config: Config = toml::from_str(toml_str).expect("should parse");
        let result = config.validate();
        assert!(
            matches!(result, Err(ConfigError::UnknownContext(ref name)) if name == "nonexistent"),
            "expected UnknownContext error, got {result:?}"
        );
    }
}
