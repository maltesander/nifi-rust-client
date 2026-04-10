//! Dynamic client with automatic version detection.
//!
//! Builds a dynamic client that detects the NiFi version at runtime via
//! `/flow/about` and dispatches API calls to the matching generated code.
//! Uses `VersionResolutionStrategy::Closest` so the client tolerates small
//! version drift from the supported set.
//!
//! This example requires the `dynamic` feature:
//!
//! ```bash
//! cargo run --example dynamic_autodetect --features dynamic
//! ```

use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::dynamic::VersionResolutionStrategy;
use nifi_rust_client::dynamic::traits::FlowApi;

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
        .version_strategy(VersionResolutionStrategy::Closest)
        .build_dynamic()?;

    // login() authenticates AND triggers version detection.
    client.login(&user, &pass).await?;

    println!("Detected NiFi version: {:?}", client.detected_version());

    let about = client.flow_api().get_about_info().await?;
    println!("Server title: {:?}", about.title);

    Ok(())
}
