# nifi-rust-client

> Apache NiFi 2.x REST API client library written in Rust.

[![CI](https://github.com/maltesander/nifi-rust-client/actions/workflows/ci.yml/badge.svg)](https://github.com/maltesander/nifi-rust-client/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/nifi-rust-client.svg)](https://crates.io/crates/nifi-rust-client)
[![Docs.rs](https://docs.rs/nifi-rust-client/badge.svg)](https://docs.rs/nifi-rust-client)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://github.com/maltesander/nifi-rust-client/blob/main/LICENSE)
![MSRV: 1.88](https://img.shields.io/badge/MSRV-1.88-blue.svg)

## Supported NiFi Versions

<!-- SUPPORTED_VERSIONS_START -->
| NiFi Version | Feature flag | Endpoints | Types | Changes |
|---|---|---|---|---|
| 2.9.0 | `nifi-2-9-0` | 352 | 437 | +35 endpoints, +32 types, +10 fields vs 2.8.0 |
| 2.8.0 | `nifi-2-8-0` | 317 | 405 | +2 fields, +2 enum values vs 2.7.2 |
| 2.7.2 | `nifi-2-7-2` | 317 | 405 | +17 endpoints, +10 types, +8 fields vs 2.6.0 |
| 2.6.0 | `nifi-2-6-0` | 300 | 395 | — |
<!-- SUPPORTED_VERSIONS_END -->

Full changelog: [NIFI_API_CHANGES.md](NIFI_API_CHANGES.md)

## Workspace

| Crate | Description |
|-------|-------------|
| [`nifi-rust-client`](crates/nifi-rust-client/) | Published library — API types, resource methods, auth |
| [`nifictl`](crates/nifictl/) | CLI tool for Apache NiFi 2.x — operator-friendly commands for every API endpoint |
| [`nifi-openapi-gen`](crates/nifi-openapi-gen/) | Internal code, test and documentation generator |

## Quick Start

### Static mode (default)

Target a specific NiFi version with full type safety and API completeness:

<!-- STATIC_FEATURE_EXAMPLE_START -->
```toml
[dependencies]
nifi-rust-client = "0.10"  # defaults to the latest supported NiFi version
```
<!-- STATIC_FEATURE_EXAMPLE_END -->

```rust
use nifi_rust_client::NifiClientBuilder;

let client = NifiClientBuilder::new("https://nifi.example.com:8443")?
    .build()?;

client.login("admin", "password").await?;

let about = client.flow().get_about_info().await?;
println!("NiFi version: {:?}", about.version);
```

### Dynamic mode

Talk to any supported NiFi version — auto-detected at connect time:

<!-- DYNAMIC_FEATURE_EXAMPLE_START -->
```toml
[dependencies]
nifi-rust-client = { version = "0.10", features = ["dynamic"] }
```
<!-- DYNAMIC_FEATURE_EXAMPLE_END -->

```rust
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::dynamic::types::DiagnosticLevel;
use nifi_rust_client::dynamic::VersionResolutionStrategy;

let client = NifiClientBuilder::new("https://nifi.example.com:8443")?
    .version_strategy(VersionResolutionStrategy::Closest)
    .build_dynamic()?;

// login() authenticates AND auto-detects the NiFi version.
client.login("admin", "password").await?;
if let Some(version) = client.detected_version() {
    println!("Connected to NiFi {version}");
}

let diag = client.systemdiagnostics()
    .get_system_diagnostics(Some(true), Some(DiagnosticLevel::Verbose), None)
    .await?;
```

When the detected NiFi version doesn't exactly match a supported version, the configured strategy controls what happens: `Strict` (default) requires an exact major.minor match or returns an error; `Closest` picks the nearest supported minor version within the same major; `Latest` always picks the highest supported minor within the same major. All strategies refuse to cross major version boundaries.

See [`crates/nifi-rust-client/README.md`](crates/nifi-rust-client/README.md) for the full API reference, builder options, token management, and error handling.

### Streaming large binary responses

For endpoints that return potentially large binary payloads — NAR
downloads, flowfile content, provenance input/output content, asset
bytes — each buffered method also has a streaming sibling that yields
chunks as they arrive from the server:

```rust
use futures_util::StreamExt;
use nifi_rust_client::{NifiClientBuilder, Bytes};
use tokio::io::AsyncWriteExt;

async fn save_provenance_content(
    dest: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = NifiClientBuilder::new("https://nifi:8443")?.build()?;
    client.login("admin", "adminpassword123").await?;

    let mut stream = client
        .provenanceevents()
        .get_input_content_stream("event-id", None, None)
        .await?;

    let mut file = tokio::fs::File::create(dest).await?;
    while let Some(chunk) = stream.next().await {
        let chunk: Bytes = chunk?;
        file.write_all(&chunk).await?;
    }
    Ok(())
}
```

The same `Range` header semantics apply — pass `Some("bytes=0-1023")`
as the last argument for a partial download.

## Development

This repo uses [pre-commit](https://pre-commit.com) with both pre-commit
and pre-push stages. After cloning or pulling changes that modify hook
stages, re-install hooks:

```bash
pre-commit install --hook-type pre-commit --hook-type pre-push --hook-type pre-merge-commit
```

## License

Apache-2.0. See [`LICENSE`](LICENSE).
