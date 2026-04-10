//! Credential provider + retry policy.
//!
//! Configures the client to automatically re-authenticate on 401 responses
//! (via [`StaticCredentials`]) and to retry transient HTTP errors (via
//! [`RetryPolicy`]) with exponential backoff.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example credentials_and_retry
//! ```

use std::time::Duration;

use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::config::credentials::StaticCredentials;
use nifi_rust_client::config::retry::RetryPolicy;

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

    // StaticCredentials holds username+password and hands them back whenever
    // the client needs to refresh its JWT. NifiClient automatically calls
    // provider.refresh() on any 401 response and retries the failed request.
    let credentials = StaticCredentials::new(&user, &pass);

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
        .credential_provider(credentials)
        .retry_policy(retry)
        .build()?;

    // Initial login. If the token later expires, any API call will hit 401,
    // the credential provider will refresh, and the call retries automatically.
    client.login(&user, &pass).await?;

    let about = client.flow_api().get_about_info().await?;
    println!("Connected with auto-refresh + retry; server: {:?}", about.title);

    Ok(())
}
