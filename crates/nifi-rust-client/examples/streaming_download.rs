//! Stream a flowfile's content to disk without buffering the whole body.
//!
//! Demonstrates the `_stream` variant of a generated download method:
//! the call returns a [`nifi_rust_client::BytesStream`] that yields
//! [`bytes::Bytes`] chunks as they arrive, which we append to a file
//! without ever holding the full payload in memory.
//!
//! Run with:
//!
//! ```bash
//! NIFI_URL=https://localhost:8443 \
//! NIFI_USERNAME=admin NIFI_PASSWORD=adminpassword123 \
//!     cargo run --example streaming_download -- \
//!     <connection-id> <flowfile-uuid> /tmp/out.bin
//! ```

// This example uses the static-mode API and cannot be run with `--features dynamic`.
// Compile with: cargo run --example streaming_download  (default features)
#[cfg(feature = "dynamic")]
fn main() {
    eprintln!(
        "streaming_download requires a single-version feature, not `dynamic`. The same pattern applies to dynamic mode — call the _stream variant on the resource struct."
    );
}

#[cfg(not(feature = "dynamic"))]
use futures_util::StreamExt;
#[cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
#[cfg(not(feature = "dynamic"))]
use nifi_rust_client::config::auth::EnvPasswordAuth;
#[cfg(not(feature = "dynamic"))]
use std::io::Write;

#[cfg(not(feature = "dynamic"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nifi_rust_client=info".into()),
        )
        .init();

    let mut args = std::env::args().skip(1);
    let connection_id = args
        .next()
        .expect("usage: <connection-id> <flowfile-uuid> <output-path>");
    let flowfile_uuid = args
        .next()
        .expect("usage: <connection-id> <flowfile-uuid> <output-path>");
    let out_path = args
        .next()
        .expect("usage: <connection-id> <flowfile-uuid> <output-path>");

    let url = std::env::var("NIFI_URL").unwrap_or_else(|_| "https://localhost:8443".into());

    // EnvPasswordAuth reads NIFI_USERNAME / NIFI_PASSWORD lazily so the client
    // can refresh its JWT on any 401 response without redoing the login setup.
    let client = NifiClientBuilder::new(&url)?
        .danger_accept_invalid_certs(true)
        .auth_provider(EnvPasswordAuth::new())
        .build()?;

    client.authenticate().await?;

    // `_stream` variant: returns a BytesStream instead of a Vec<u8>. The
    // initial request (status-line exchange) is still retry-eligible; once
    // chunks start arriving, any transport error terminates the stream.
    let mut stream = client
        .flowfilequeues()
        .download_flow_file_content_stream(
            &connection_id,
            &flowfile_uuid,
            None, // client_id
            None, // cluster_node_id
            None, // range
        )
        .await?;

    // Append chunks to disk using blocking I/O. For larger workloads,
    // adapt the stream to `tokio::io::AsyncRead` via
    // `tokio_util::io::StreamReader` and use `tokio::fs::File` with the
    // `fs` / `io-util` features enabled.
    let mut file = std::fs::File::create(&out_path)?;
    let mut total: u64 = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
        total += chunk.len() as u64;
    }
    file.flush()?;

    eprintln!("wrote {total} bytes to {out_path}");
    Ok(())
}
