//! Auth provider + retry policy.
//!
//! Configures the client to automatically re-authenticate on 401 responses
//! (via [`PasswordAuth`]) and to retry transient HTTP errors (via
//! [`RetryPolicy`]) with exponential backoff.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example credentials_and_retry
//! ```

// This example uses the static-mode API and cannot be run with `--features dynamic`.
// Compile with: cargo run --example credentials_and_retry  (default features)
#[cfg(feature = "dynamic")]
fn main() {
    eprintln!(
        "credentials_and_retry requires a single-version feature, not `dynamic`. Use dynamic_autodetect instead."
    );
}

#[cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
#[cfg(not(feature = "dynamic"))]
use nifi_rust_client::config::auth::PasswordAuth;
#[cfg(not(feature = "dynamic"))]
use nifi_rust_client::config::retry::RetryPolicy;
#[cfg(not(feature = "dynamic"))]
use std::time::Duration;

#[cfg(not(feature = "dynamic"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nifi_rust_client=info".into()),
        )
        .init();

    let url = std::env::var("NIFI_URL").unwrap_or_else(|_| "https://localhost:8443".into());
    let user = std::env::var("NIFI_USERNAME").unwrap_or_else(|_| "admin".into());
    let pass = std::env::var("NIFI_PASSWORD").unwrap_or_else(|_| "adminpassword123".into());

    // PasswordAuth holds username+password and hands them back whenever
    // the client needs to refresh its JWT. NifiClient automatically calls
    // provider.authenticate() on any 401 response and retries the failed request.
    let credentials = PasswordAuth::new(&user, &pass);

    // RetryPolicy retries transient errors (408, 429, 500, 502, 503, 504, and
    // reqwest-level connection failures) with exponential backoff. Max 3
    // attempts, starting at 100ms, doubling each time, capped at 2s.
    let retry = RetryPolicy {
        max_retries: 3,
        initial_backoff: Duration::from_millis(100),
        max_backoff: Duration::from_secs(2),
    };

    let client = NifiClientBuilder::new(&url)?
        .danger_accept_invalid_certs(true)
        .auth_provider(credentials)
        .retry_policy(retry)
        .build()?;

    // Initial login. If the token later expires, any API call will hit 401,
    // the credential provider will refresh, and the call retries automatically.
    client.login(&user, &pass).await?;

    let about = client.flow().get_about_info().await?;
    println!(
        "Connected with auto-refresh + retry; server: {:?}",
        about.title
    );

    Ok(())
}
