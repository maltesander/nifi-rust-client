# nifi-rust-client

Apache NiFi 2.x REST API client library written in Rust.

## Why nifi-rust-client?

**Full API coverage, generated directly from NiFi's own OpenAPI spec.**
Every endpoint NiFi exposes is available as a typed Rust method — nothing hand-written, nothing missing. When a new NiFi version ships, a single `cargo run -p nifi-openapi-gen` regenerates the entire client from the live spec in minutes.

**Two modes for different needs.**
Use **static mode** for full type safety and IDE autocompletion when you target a known NiFi version. Use **dynamic mode** when your tool needs to talk to multiple NiFi clusters running different versions — the client auto-detects the API version at connect time and dispatches to the right generated code.

**Multi-version support with zero ceremony.**
Pin to specific NiFi version via a Cargo feature flag, or enable `dynamic` to compile all versions and let the client pick at runtime. Adding a new NiFi version is one command — the generator handles features, types, dispatch, and tests automatically.

**Tested against real NiFi instances.**
Every generated endpoint gets an auto-generated wiremock stub. Integration tests run against a Docker-hosted NiFi and cover the full request/response cycle — not just serialization. Both static and dynamic modes have dedicated test suites.

**Low-level by design: you stay in control.**
The client does not hide HTTP details behind opinionated abstractions. Token lifecycle, retry logic on expiry, and connection tuning are your responsibility — which means you can implement them exactly the way your application requires, without fighting the library.

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
nifi-rust-client = "0.1"  # defaults to latest (currently nifi-2-8-0)
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
nifi-rust-client = { version = "0.1", features = ["dynamic"] }
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

## Supported NiFi Versions

| NiFi Version | Feature Flag | Default |
|---|---|---|
| 2.8.0 | `nifi-2-8-0` | Yes |
| 2.7.2 | `nifi-2-7-2` | |
| 2.6.0 | `nifi-2-6-0` | |

The `dynamic` feature compiles all versions and enables runtime version detection.

Adding a new version requires only a running NiFi instance and two commands:

```bash
NIFI_IMAGE_TAG=2.9.0 docker compose -f tests/docker-compose.yml up -d  # start NiFi
./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh                    # fetch spec
cargo run -p nifi-openapi-gen                                           # generate everything
```

The generator creates the version module, updates feature flags, regenerates the dynamic dispatch layer, and produces wiremock tests — no manual edits needed.

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
