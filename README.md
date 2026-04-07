# nifi-rust-client

Apache NiFi 2.x REST API client library written in Rust.

## Supported NiFi Versions

<!-- SUPPORTED_VERSIONS_START -->
| NiFi Version | Feature flag | Endpoints | Types | Changes | Default |
|---|---|---|---|---|---|
| 2.8.0 | `nifi-2-8-0` | 317 | 405 | no API changes vs 2.7.2 | ✓ |
| 2.7.2 | `nifi-2-7-2` | 317 | 405 | +17 endpoints, +10 types vs 2.6.0 |  |
| 2.6.0 | `nifi-2-6-0` | 300 | 395 | — |  |
<!-- SUPPORTED_VERSIONS_END -->

## Workspace

| Crate | Description |
|-------|-------------|
| [`nifi-rust-client`](crates/nifi-rust-client/) | Published library — API types, resource methods, auth |
| [`nifi-openapi-gen`](crates/nifi-openapi-gen/) | Internal code generator — not published |

## Quick Start

### Static mode (default)

Target a specific NiFi version with full type safety and autocompletion:

```toml
[dependencies]
nifi-rust-client = "0.2"  # defaults to the latest supported NiFi version
```

```rust
use nifi_rust_client::NifiClientBuilder;

let mut client = NifiClientBuilder::new("https://nifi.example.com:8443")?
    .build()?;

client.login("admin", "password").await?;

let about = client.flow_api().get_about_info().await?;
println!("NiFi version: {:?}", about.version);
```

### Dynamic mode

Talk to any supported NiFi version — auto-detected at connect time:

```toml
[dependencies]
nifi-rust-client = { version = "0.2", features = ["dynamic"] }
```

```rust
use nifi_rust_client::NifiClientBuilder;

let mut client = NifiClientBuilder::new("https://nifi.example.com:8443")?
    .build_dynamic().await?;

client.login("admin", "password").await?;

println!("Connected to NiFi {}", client.detected_version());
let about = client.flow_api().get_about_info().await?;
println!("NiFi title: {:?}", about.title);
```

See [`crates/nifi-rust-client/README.md`](crates/nifi-rust-client/README.md) for the full API reference, builder options, token management, and error handling.

## Why nifi-rust-client?

**Full API coverage, generated directly from NiFi's own OpenAPI spec.**
Every endpoint NiFi exposes is available as a typed Rust method — nothing hand-written, nothing missing. When a new NiFi version ships, a single `cargo run -p nifi-openapi-gen` regenerates the entire client from the live spec in minutes.

**Two modes for different needs.**
Use **static mode** for full type safety and IDE autocompletion when you target a known NiFi version. Use **dynamic mode** when your tool needs to talk to multiple NiFi clusters running different versions — the client auto-detects the API version at connect time and dispatches to the right generated code.

**Tested against real NiFi instances.**
Every generated endpoint gets an auto-generated wiremock stub. Integration tests run against a Docker-hosted NiFi and cover the full request/response cycle — not just serialization. Both static and dynamic modes have dedicated test suites.

**Low-level by design: you stay in control.**
The client does not hide HTTP details behind opinionated abstractions. Token lifecycle, retry logic on expiry, and connection tuning are your responsibility — which means you can implement them exactly the way your application requires, without fighting the library.

## Build

```bash
cargo build
cargo clippy
```

## Test

Unit tests (no Docker):

```bash
cargo test --workspace
```

Integration tests (requires Docker):

```bash
./tests/run.sh                          # start NiFi, run tests, tear down
./tests/run.sh --skip-cleanup           # leave NiFi running between runs
./tests/run.sh --nifi-version=2.7.2     # test against a specific NiFi version
```

Default credentials used by the test container: `admin` / `adminpassword123`.
Override with `NIFI_URL`, `NIFI_USERNAME`, `NIFI_PASSWORD` env vars.

## Prerequisites

- Rust (version pinned in `rust-toolchain.toml`)
- Docker (for integration tests)
- `pre-commit` (for commit hooks)

## Before committing

```bash
pre-commit run --all-files
```
