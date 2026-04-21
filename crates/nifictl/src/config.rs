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
    PlaintextSecret {
        name: String,
        kind: &'static str,
        env_alternative: &'static str,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "failed to read config file: {e}"),
            ConfigError::Parse(e) => write!(f, "failed to parse config file: {e}"),
            ConfigError::UnknownContext(name) => {
                write!(f, "current_context '{name}' not found in contexts")
            }
            ConfigError::PlaintextSecret {
                name,
                kind,
                env_alternative,
            } => write!(
                f,
                "context '{name}' has a plaintext {kind} in config; \
                 set NIFICTL_ALLOW_PLAINTEXT_SECRETS=1 or move it to {env_alternative}"
            ),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(e) => Some(e),
            ConfigError::Parse(e) => Some(e),
            ConfigError::UnknownContext(_) => None,
            ConfigError::PlaintextSecret { .. } => None,
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
    /// - Refuses plaintext `password` or `token` fields by default, returning
    ///   `ConfigError::PlaintextSecret`. Set the environment variable
    ///   `NIFICTL_ALLOW_PLAINTEXT_SECRETS=1` (or `true` / `TRUE`) to
    ///   downgrade the refusal to a stderr warning.
    /// - Returns `ConfigError::UnknownContext` when `current_context` names a
    ///   context that doesn't exist in `contexts`.
    pub fn validate(&self) -> Result<(), ConfigError> {
        let allow_plaintext = matches!(
            std::env::var("NIFICTL_ALLOW_PLAINTEXT_SECRETS").as_deref(),
            Ok("1") | Ok("true") | Ok("TRUE")
        );

        for ctx in &self.contexts {
            match &ctx.auth {
                AuthConfig::Password {
                    password: Some(_), ..
                } => {
                    if allow_plaintext {
                        eprintln!(
                            "warning: context '{}' has a plaintext password in config; \
                             allowed via NIFICTL_ALLOW_PLAINTEXT_SECRETS — prefer password_env",
                            ctx.name
                        );
                    } else {
                        return Err(ConfigError::PlaintextSecret {
                            name: ctx.name.clone(),
                            kind: "password",
                            env_alternative: "password_env",
                        });
                    }
                }
                AuthConfig::Token { token: Some(_), .. } => {
                    if allow_plaintext {
                        eprintln!(
                            "warning: context '{}' has a plaintext token in config; \
                             allowed via NIFICTL_ALLOW_PLAINTEXT_SECRETS — prefer token_env",
                            ctx.name
                        );
                    } else {
                        return Err(ConfigError::PlaintextSecret {
                            name: ctx.name.clone(),
                            kind: "token",
                            env_alternative: "token_env",
                        });
                    }
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
    use std::sync::Mutex;

    // Serializes tests that mutate NIFICTL_ALLOW_PLAINTEXT_SECRETS so they
    // don't race each other under cargo's default parallel runner.
    // `std::env::set_var` is process-global; unrelated tests don't touch
    // this variable, so a local mutex is enough.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

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
    fn parse_password_plaintext_still_deserializes() {
        let toml_str = r#"
[[contexts]]
name = "dev"
url = "https://nifi-dev:8443"

[contexts.auth]
type = "password"
username = "admin"
password = "s3cr3t"
"#;
        // Parsing a plaintext password must still succeed — validate() is
        // what enforces the refusal.
        let config: Config = toml::from_str(toml_str).expect("should parse");
        match &config.contexts[0].auth {
            AuthConfig::Password { password, .. } => {
                assert_eq!(password.as_deref(), Some("s3cr3t"));
            }
            other => panic!("expected Password auth, got {other:?}"),
        }
    }

    #[test]
    fn plaintext_password_refused_by_default() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: ENV_LOCK serializes all tests in this module that touch
        // NIFICTL_ALLOW_PLAINTEXT_SECRETS, and the config tests use no
        // threads internally.
        unsafe {
            std::env::remove_var("NIFICTL_ALLOW_PLAINTEXT_SECRETS");
        }
        let raw = r#"
current_context = "prod"
[[contexts]]
name = "prod"
url = "https://nifi:8443"
[contexts.auth]
type = "password"
username = "admin"
password = "hunter2"
"#;
        let cfg: Config = toml::from_str(raw).unwrap();
        let err = cfg.validate().unwrap_err();
        assert!(matches!(err, ConfigError::PlaintextSecret { .. }));
    }

    #[test]
    fn plaintext_token_refused_by_default() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            std::env::remove_var("NIFICTL_ALLOW_PLAINTEXT_SECRETS");
        }
        let raw = r#"
current_context = "prod"
[[contexts]]
name = "prod"
url = "https://nifi:8443"
[contexts.auth]
type = "token"
token = "abc.def.ghi"
"#;
        let cfg: Config = toml::from_str(raw).unwrap();
        let err = cfg.validate().unwrap_err();
        assert!(matches!(err, ConfigError::PlaintextSecret { .. }));
    }

    #[test]
    fn plaintext_allowed_with_env_escape_hatch() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            std::env::set_var("NIFICTL_ALLOW_PLAINTEXT_SECRETS", "1");
        }
        let raw = r#"
current_context = "prod"
[[contexts]]
name = "prod"
url = "https://nifi:8443"
[contexts.auth]
type = "password"
username = "admin"
password = "hunter2"
"#;
        let cfg: Config = toml::from_str(raw).unwrap();
        let result = cfg.validate();
        unsafe {
            std::env::remove_var("NIFICTL_ALLOW_PLAINTEXT_SECRETS");
        }
        assert!(
            result.is_ok(),
            "validate should succeed when env allows: {result:?}"
        );
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
