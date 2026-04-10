//! Minimal "hello NiFi" example.
//!
//! Builds a static client, logs in with single-user credentials, and prints
//! the server's `About` information. Reads configuration from environment:
//!
//! - `NIFI_URL` (default: `https://localhost:8443`)
//! - `NIFI_USERNAME` (default: `admin`)
//! - `NIFI_PASSWORD` (default: `adminpassword123`)
//!
//! Run with:
//!
//! ```bash
//! cargo run --example basic_static
//! ```

use nifi_rust_client::NifiClientBuilder;

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

    let client = NifiClientBuilder::new(&url)?
        .danger_accept_invalid_certs(true)
        .build()?;

    client.login(&user, &pass).await?;

    let about = client.flow_api().get_about_info().await?;
    println!("Connected to NiFi:");
    println!("  version:  {:?}", about.version);
    println!("  title:    {:?}", about.title);
    println!("  timezone: {:?}", about.timezone);

    Ok(())
}
