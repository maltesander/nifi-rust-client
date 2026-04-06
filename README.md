# nifi-rust-client

Apache NiFi 2.x REST API client library written in Rust.

## Why nifi-rust-client?

**Full API coverage, generated directly from NiFi's own OpenAPI spec.**
Every endpoint NiFi exposes is available as a typed Rust method — nothing hand-written, nothing missing. When a new NiFi version ships, a single `cargo run -p nifi-openapi-gen` regenerates the entire client from the live spec in minutes.

**Multi-version support with zero ceremony.**
Pin to `nifi-2-8-0` or `nifi-2-7-2` via a Cargo feature flag. Switch versions by changing one line. The library compiles only what you need.

**Tested against real NiFi instances.**
Every generated endpoint gets an auto-generated wiremock stub. Integration tests run against a Docker-hosted NiFi and cover the full request/response cycle — not just serialization.

**Low-level by design: you stay in control.**
The client does not hide HTTP details behind opinionated abstractions. Token lifecycle, retry logic on expiry, and connection tuning are your responsibility — which means you can implement them exactly the way your application requires, without fighting the library.

## Workspace

| Crate | Description |
|-------|-------------|
| [`nifi-rust-client`](crates/nifi-rust-client/) | Published library — API types, resource methods, auth |
| [`nifi-openapi-gen`](crates/nifi-openapi-gen/) | Internal code generator — not published |

## Quick Start

Add the dependency:

```toml
[dependencies]
nifi-rust-client = "0.1"
```

To pin a specific NiFi API version (default is the latest — currently 2.8.0):

```toml
nifi-rust-client = { version = "0.1", default-features = false, features = ["nifi-2-8-0"] }
```

Connect and make a request:

```rust
use nifi_rust_client::NifiClientBuilder;

let mut client = NifiClientBuilder::new("https://nifi.example.com:8443")?
    .build()?;

client.login("admin", "password").await?;

let about = client.flow_api().get_about_info().await?;
println!("NiFi version: {}", about.version);
```

See [`crates/nifi-rust-client/README.md`](crates/nifi-rust-client/README.md) for the full API reference, builder options, token management, and error handling.

## Prerequisites

- Rust (version pinned in `rust-toolchain.toml`)
- Docker (for integration tests)
- `pre-commit` (for commit hooks)

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

## Before committing

```bash
pre-commit run --all-files
```
