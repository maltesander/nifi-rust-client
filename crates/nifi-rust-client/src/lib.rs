//! An idiomatic Rust client for the [Apache NiFi](https://nifi.apache.org) 2.x
//! REST API.
//!
//! This crate provides two usage modes that trade off compile-time determinism
//! against runtime flexibility:
//!
//! ## Static mode (default)
//!
//! Compile against exactly one NiFi version via a Cargo feature flag. The API
//! surface is fully typed, every endpoint is statically resolved, and the
//! compiler catches any version drift between your code and the server.
//!
//! ```no_run
//! use nifi_rust_client::NifiClientBuilder;
//!
//! # async fn example() -> Result<(), nifi_rust_client::NifiError> {
//! let client = NifiClientBuilder::new("https://nifi.example.com:8443")?.build()?;
//! client.login("admin", "adminpassword123").await?;
//!
//! # #[cfg(not(feature = "dynamic"))]
//! let about = client.flow().get_about_info().await?;
//! # #[cfg(not(feature = "dynamic"))]
//! println!("Connected to NiFi {:?}", about.version);
//! # Ok(())
//! # }
//! ```
//!
//! Enable a specific version via Cargo features:
//!
//! <!-- LIB_STATIC_FEATURE_EXAMPLE_START -->
//! ```toml
//! [dependencies]
//! nifi-rust-client = { version = "0.9", features = ["nifi-2-9-0"] }
//! ```
//! <!-- LIB_STATIC_FEATURE_EXAMPLE_END -->
//!
//! ## Dynamic mode (`dynamic` feature)
//!
//! Compile all supported versions and detect the NiFi server version at
//! runtime via `/flow/about`. Use this when your code must talk to multiple
//! server versions without recompilation.
//!
//! ```no_run
//! # #[cfg(feature = "dynamic")]
//! # async fn example() -> Result<(), nifi_rust_client::NifiError> {
//! use nifi_rust_client::NifiClientBuilder;
//! use nifi_rust_client::dynamic::VersionResolutionStrategy;
//!
//! let client = NifiClientBuilder::new("https://nifi.example.com:8443")?
//!     .version_strategy(VersionResolutionStrategy::Closest)
//!     .build_dynamic()?;
//!
//! // login() authenticates AND auto-detects the NiFi version.
//! client.login("admin", "adminpassword123").await?;
//! # Ok(())
//! # }
//! ```
//!
//! Enable via:
//!
//! <!-- LIB_DYNAMIC_FEATURE_EXAMPLE_START -->
//! ```toml
//! [dependencies]
//! nifi-rust-client = { version = "0.9", features = ["dynamic"] }
//! ```
//! <!-- LIB_DYNAMIC_FEATURE_EXAMPLE_END -->
//!
//! ## Entry points
//!
//! - [`NifiClientBuilder`] — construct a client with timeouts, proxies, TLS
//!   options, credential providers, and retry policy.
//! - [`NifiClient`] — the client handle itself; resource accessors like
//!   `.flow()`, `.processors()`, etc. return borrowed resource structs.
//! - [`NifiError`] — `#[non_exhaustive]` error type with typed variants
//!   (`Unauthorized`, `Forbidden`, `NotFound`, `Conflict`,
//!   `UnsupportedEndpoint`, etc.) and helpers like `status_code()` and
//!   `is_retryable()`.
//! - [`AuthProvider`] and its impls (`PasswordAuth`, `EnvPasswordAuth`,
//!   `StaticTokenAuth`) in the [`config::auth`] module — used with
//!   [`NifiClientBuilder::auth_provider`] to enable auto-refresh on 401.
//! - [`config::retry::RetryPolicy`] — exponential-backoff retry on transient
//!   errors, configured via [`NifiClientBuilder::retry_policy`].
//!
//! ## Running examples
//!
//! ```bash
//! NIFI_URL=https://localhost:8443 \
//! NIFI_USERNAME=admin NIFI_PASSWORD=adminpassword123 \
//! cargo run --example basic_static
//! ```
//!
//! All examples accept the same environment variables. See `examples/` in
//! the repository for the full set.
//!
//! ## Feature flags
//!
//! | Feature | Purpose |
//! |---|---|
//! <!-- NIFI_FEATURE_FLAGS_START -->
//! | `nifi-2-6-0`, `nifi-2-7-2`, `nifi-2-8-0`, `nifi-2-9-0` | Compile against a specific NiFi version. The semver-latest is the default. |
//! <!-- NIFI_FEATURE_FLAGS_END -->
//! | `dynamic` | Compile all versions and enable runtime version detection. Pulls in every version feature. |
//!
//! At least one version feature (or `dynamic`) must be enabled — builds with
//! none fail at both build-script time and compile time.

// `has_dynamic_or_version` is a rustc-cfg emitted by build.rs whenever it
// runs successfully with at least one NiFi version feature or the `dynamic`
// feature enabled. The flag is invisible to users — it isn't a Cargo
// feature and can't be set externally. If build.rs is ever bypassed
// entirely (some `cargo doc` / rust-analyzer configurations), the flag is
// unset and this compile_error! fires with an actionable message. The
// primary zero-features guard is the runtime check in build.rs itself;
// this is defence in depth.
#[cfg(not(has_dynamic_or_version))]
compile_error!(
    "nifi-rust-client requires at least one NiFi feature to be \
     enabled. Enable one of: `nifi-2-6-0`, `nifi-2-7-2`, \
     `nifi-2-8-0`, `nifi-2-9-0`, or `dynamic`."
);

/// Client builder: configure timeouts, TLS, credentials, and retry before connecting.
pub mod builder;
/// The connected client handle and resource accessor methods.
pub mod client;
/// Configuration types: credential providers and retry policy.
pub mod config;
/// Error type returned by all client operations.
pub mod error;
/// Pagination helpers for NiFi REST endpoints that support offset/count paging.
pub mod pagination;
mod require;
pub use builder::NifiClientBuilder;
pub use client::NifiClient;
pub use config::auth::AuthProvider;
pub use error::NifiError;
pub use require::RequireField;

// Generated: version modules, re-exports, dynamic module
include!(concat!(env!("OUT_DIR"), "/generated_lib.rs"));
