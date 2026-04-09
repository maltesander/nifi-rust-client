//! Credential providers for NiFi authentication.
//!
//! The [`CredentialProvider`] trait abstracts how username/password pairs are
//! obtained, enabling static credentials, environment-variable lookups, and
//! custom strategies (vault, file-based, etc.).

use std::fmt;

use crate::error::NifiError;

/// Wraps a value and masks it in [`fmt::Debug`] output as `[REDACTED]`.
///
/// Use this for sensitive fields (e.g. passwords) to prevent them from
/// leaking into logs or debug output.
#[derive(Clone)]
pub struct Redacted<T>(T);

impl<T> Redacted<T> {
    /// Wrap `value` so its debug representation is hidden.
    pub fn new(value: T) -> Self {
        Self(value)
    }

    /// Return a reference to the wrapped value.
    pub fn inner(&self) -> &T {
        &self.0
    }
}

impl<T> fmt::Debug for Redacted<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

/// Provides username/password credentials for NiFi authentication.
///
/// Implement this trait to supply credentials from any source — static values,
/// environment variables, a secrets vault, or a rotating credential store.
#[async_trait::async_trait]
pub trait CredentialProvider: Send + Sync + fmt::Debug {
    /// Returns a `(username, password)` pair, or an error if credentials
    /// cannot be resolved.
    async fn credentials(&self) -> Result<(String, String), NifiError>;
}

/// Fixed username/password credentials.
///
/// Useful for tests or simple deployments where credentials are known at
/// build time.
#[derive(Debug, Clone)]
pub struct StaticCredentials {
    username: String,
    password: Redacted<String>,
}

impl StaticCredentials {
    /// Create a new `StaticCredentials` with the given username and password.
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: Redacted::new(password.into()),
        }
    }
}

#[async_trait::async_trait]
impl CredentialProvider for StaticCredentials {
    async fn credentials(&self) -> Result<(String, String), NifiError> {
        Ok((self.username.clone(), self.password.inner().clone()))
    }
}

/// Credentials read from environment variables.
///
/// By default reads `NIFI_USERNAME` and `NIFI_PASSWORD`. Use
/// [`EnvCredentials::with_vars`] to override the variable names.
#[derive(Debug, Clone)]
pub struct EnvCredentials {
    username_var: String,
    password_var: String,
}

impl EnvCredentials {
    /// Create an `EnvCredentials` using the default environment variable
    /// names (`NIFI_USERNAME` and `NIFI_PASSWORD`).
    pub fn new() -> Self {
        Self {
            username_var: "NIFI_USERNAME".to_string(),
            password_var: "NIFI_PASSWORD".to_string(),
        }
    }

    /// Create an `EnvCredentials` using custom environment variable names.
    pub fn with_vars(username_var: impl Into<String>, password_var: impl Into<String>) -> Self {
        Self {
            username_var: username_var.into(),
            password_var: password_var.into(),
        }
    }

    /// Returns the environment variable name used for the username.
    pub fn username_var(&self) -> &str {
        &self.username_var
    }

    /// Returns the environment variable name used for the password.
    pub fn password_var(&self) -> &str {
        &self.password_var
    }
}

impl Default for EnvCredentials {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl CredentialProvider for EnvCredentials {
    async fn credentials(&self) -> Result<(String, String), NifiError> {
        let username = std::env::var(&self.username_var).map_err(|_| NifiError::Auth {
            message: format!("environment variable {} is not set", self.username_var),
        })?;
        let password = std::env::var(&self.password_var).map_err(|_| NifiError::Auth {
            message: format!("environment variable {} is not set", self.password_var),
        })?;
        Ok((username, password))
    }
}
