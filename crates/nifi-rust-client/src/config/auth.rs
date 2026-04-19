#![deny(missing_docs)]
//! Flexible authentication providers for NiFi.
//!
//! The [`AuthProvider`] trait abstracts how a NiFi client obtains its JWT
//! token, enabling username/password login, pre-shared tokens, environment
//! variable lookups, and custom strategies (vault, mTLS exchange, etc.).

use std::fmt;
use std::sync::Arc;

use crate::NifiClient;
use crate::error::NifiError;

// ── Redacted<T> ──────────────────────────────────────────────────────────────

/// Wraps a value and masks it in [`fmt::Debug`] output as `[REDACTED]`.
///
/// Use this for sensitive fields (e.g. passwords, tokens) to prevent them from
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

// ── AuthProvider trait ───────────────────────────────────────────────────────

/// Provides authentication for a [`NifiClient`].
///
/// Implementors produce a valid JWT token and install it on the client via
/// [`NifiClient::login`] or [`NifiClient::set_token`]. The trait is
/// object-safe so it can be stored as `Arc<dyn AuthProvider>` in the client
/// and builder.
#[async_trait::async_trait]
pub trait AuthProvider: Send + Sync + fmt::Debug {
    /// Authenticate the given client.
    ///
    /// Implementations should obtain a JWT token and store it on the client
    /// (e.g. by calling [`NifiClient::login`] or [`NifiClient::set_token`]).
    async fn authenticate(&self, client: &NifiClient) -> Result<(), NifiError>;
}

// Allow `Arc<dyn AuthProvider>` to be used as an `AuthProvider` directly.
#[async_trait::async_trait]
impl AuthProvider for Arc<dyn AuthProvider> {
    async fn authenticate(&self, client: &NifiClient) -> Result<(), NifiError> {
        (**self).authenticate(client).await
    }
}

// ── PasswordAuth ─────────────────────────────────────────────────────────────

/// Authenticates with a fixed username and password via [`NifiClient::login`].
///
/// Useful for tests or simple deployments where credentials are known at
/// build time. The password is stored in a [`zeroize::Zeroizing`] wrapper
/// so it is zeroed on drop.
#[derive(Clone)]
pub struct PasswordAuth {
    username: String,
    password: zeroize::Zeroizing<String>,
}

impl PasswordAuth {
    /// Create a new `PasswordAuth` with the given username and password.
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: zeroize::Zeroizing::new(password.into()),
        }
    }
}

impl fmt::Debug for PasswordAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PasswordAuth")
            .field("username", &self.username)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

#[async_trait::async_trait]
impl AuthProvider for PasswordAuth {
    async fn authenticate(&self, client: &NifiClient) -> Result<(), NifiError> {
        client.login(&self.username, &self.password).await
    }
}

// ── EnvPasswordAuth ──────────────────────────────────────────────────────────

/// Reads username and password from environment variables, then authenticates
/// via [`NifiClient::login`].
///
/// By default reads `NIFI_USERNAME` and `NIFI_PASSWORD`. Use
/// [`EnvPasswordAuth::with_vars`] to override the variable names.
#[derive(Debug, Clone)]
pub struct EnvPasswordAuth {
    username_var: String,
    password_var: String,
}

impl EnvPasswordAuth {
    /// Create an `EnvPasswordAuth` using the default environment variable
    /// names (`NIFI_USERNAME` and `NIFI_PASSWORD`).
    pub fn new() -> Self {
        Self {
            username_var: "NIFI_USERNAME".to_string(),
            password_var: "NIFI_PASSWORD".to_string(),
        }
    }

    /// Create an `EnvPasswordAuth` using custom environment variable names.
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

impl Default for EnvPasswordAuth {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl AuthProvider for EnvPasswordAuth {
    async fn authenticate(&self, client: &NifiClient) -> Result<(), NifiError> {
        let username = std::env::var(&self.username_var).map_err(|_| NifiError::Auth {
            message: format!("environment variable {} is not set", self.username_var),
        })?;
        let password =
            zeroize::Zeroizing::new(std::env::var(&self.password_var).map_err(|_| {
                NifiError::Auth {
                    message: format!("environment variable {} is not set", self.password_var),
                }
            })?);
        client.login(&username, &password).await
    }
}

// ── StaticTokenAuth ──────────────────────────────────────────────────────────

/// Authenticates by installing a pre-obtained JWT token directly.
///
/// Useful when a token has been acquired externally (e.g. from a vault or
/// a previous session) and does not require a login round-trip. The token
/// is stored in a [`zeroize::Zeroizing`] wrapper so it is zeroed on drop.
#[derive(Clone)]
pub struct StaticTokenAuth {
    token: zeroize::Zeroizing<String>,
}

impl StaticTokenAuth {
    /// Create a new `StaticTokenAuth` with the given JWT token.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: zeroize::Zeroizing::new(token.into()),
        }
    }
}

impl fmt::Debug for StaticTokenAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StaticTokenAuth")
            .field("token", &"[REDACTED]")
            .finish()
    }
}

#[async_trait::async_trait]
impl AuthProvider for StaticTokenAuth {
    async fn authenticate(&self, client: &NifiClient) -> Result<(), NifiError> {
        client.set_token((*self.token).clone()).await;
        Ok(())
    }
}
