//! Custom TLS configuration — trusting a specific CA.
//!
//! Shows how to configure the client to trust a specific PEM-encoded CA
//! certificate instead of disabling TLS verification wholesale. This is the
//! right choice for production deployments against a NiFi cluster that uses
//! an internal or private certificate authority.
//!
//! Set `NIFI_CA_PEM` to the path of the CA certificate. If unset, falls
//! back to `danger_accept_invalid_certs(true)` for local dev convenience.
//!
//! Run with:
//!
//! ```bash
//! NIFI_CA_PEM=/path/to/ca.pem cargo run --example custom_tls
//! ```

// This example uses the static-mode API and cannot be run with `--features dynamic`.
// Compile with: cargo run --example custom_tls  (default features)
#[cfg(feature = "dynamic")]
fn main() {
    eprintln!(
        "custom_tls requires a single-version feature, not `dynamic`. Use dynamic_autodetect instead."
    );
}

#[cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;

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

    let mut builder = NifiClientBuilder::new(&url)?;

    match std::env::var("NIFI_CA_PEM") {
        Ok(pem_path) => {
            println!("Trusting CA certificate at {pem_path}");
            let pem = std::fs::read(&pem_path)?;
            builder = builder.add_root_certificate(&pem);
        }
        Err(_) => {
            println!("NIFI_CA_PEM not set; falling back to danger_accept_invalid_certs for dev.");
            builder = builder.danger_accept_invalid_certs(true);
        }
    }

    let client = builder.build()?;
    client.login(&user, &pass).await?;

    let about = client.flow().get_about_info().await?;
    println!("Connected via custom TLS config; server: {:?}", about.title);

    Ok(())
}
