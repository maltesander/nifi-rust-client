//! Start and stop all processors in a process group via bulk helpers.
//!
//! Wraps NiFi's native `PUT /flow/process-groups/{id}` bulk-control
//! endpoint via [`nifi_rust_client::bulk::start_process_group`] and
//! [`nifi_rust_client::bulk::stop_process_group`].
//!
//! Note: the helpers return once the HTTP call resolves — they do NOT
//! poll for the resulting state transition. Pair them with the
//! [`nifi_rust_client::wait`] helpers when you need to await the
//! transition.
//!
//! Run with:
//!
//! ```bash
//! NIFI_URL=https://localhost:8443 \
//! NIFI_USERNAME=admin NIFI_PASSWORD=adminpassword123 \
//!     cargo run --example bulk_start_stop -- <process-group-id>
//! ```

// This example uses the static-mode API and cannot be run with `--features dynamic`.
// Compile with: cargo run --example bulk_start_stop  (default features)
#[cfg(feature = "dynamic")]
fn main() {
    eprintln!(
        "bulk_start_stop requires a single-version feature, not `dynamic`. The dynamic equivalents are bulk::start_process_group_dynamic / bulk::stop_process_group_dynamic."
    );
}

#[cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
#[cfg(not(feature = "dynamic"))]
use nifi_rust_client::bulk;
#[cfg(not(feature = "dynamic"))]
use nifi_rust_client::config::auth::EnvPasswordAuth;

#[cfg(not(feature = "dynamic"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nifi_rust_client=info".into()),
        )
        .init();

    let pg_id = std::env::args().nth(1).expect("usage: <process-group-id>");

    let url = std::env::var("NIFI_URL").unwrap_or_else(|_| "https://localhost:8443".into());

    let client = NifiClientBuilder::new(&url)?
        .danger_accept_invalid_certs(true)
        .auth_provider(EnvPasswordAuth::new())
        .build()?;

    client.authenticate().await?;

    bulk::start_process_group(&client, &pg_id).await?;
    eprintln!("started process group {pg_id}");

    bulk::stop_process_group(&client, &pg_id).await?;
    eprintln!("stopped process group {pg_id}");

    Ok(())
}
