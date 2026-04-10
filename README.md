# nifi-rust-client

Apache NiFi 2.x REST API client library written in Rust.

## Supported NiFi Versions

<!-- SUPPORTED_VERSIONS_START -->
| NiFi Version | Feature flag | Endpoints | Types | Changes |
|---|---|---|---|---|
| 2.9.0 | `nifi-2-9-0` | 352 | 437 | +35 endpoints, +32 types, +10 fields vs 2.8.0 |
| 2.8.0 | `nifi-2-8-0` | 317 | 405 | +2 fields, +1 enum values vs 2.7.2 |
| 2.7.2 | `nifi-2-7-2` | 317 | 405 | +17 endpoints, +10 types, +8 fields vs 2.6.0 |
| 2.6.0 | `nifi-2-6-0` | 300 | 395 | — |
<!-- SUPPORTED_VERSIONS_END -->

Full changelog: [NIFI_API_CHANGES.md](NIFI_API_CHANGES.md)

## Workspace

| Crate | Description |
|-------|-------------|
| [`nifi-rust-client`](crates/nifi-rust-client/) | Published library — API types, resource methods, auth |
| [`nifi-openapi-gen`](crates/nifi-openapi-gen/) | Internal code, test and documentation generator |

## Quick Start

### Static mode (default)

Target a specific NiFi version with full type safety and API completeness:

<!-- STATIC_FEATURE_EXAMPLE_START -->
```toml
[dependencies]
nifi-rust-client = "0.6"  # defaults to the latest supported NiFi version
```
<!-- STATIC_FEATURE_EXAMPLE_END -->

```rust
use nifi_rust_client::NifiClientBuilder;

let client = NifiClientBuilder::new("https://nifi.example.com:8443")?
    .build()?;

client.login("admin", "password").await?;

let about = client.flow_api().get_about_info().await?;
println!("NiFi version: {:?}", about.version);
```

### Dynamic mode

Talk to any supported NiFi version — auto-detected at connect time:

<!-- DYNAMIC_FEATURE_EXAMPLE_START -->
```toml
[dependencies]
nifi-rust-client = { version = "0.6", features = ["dynamic"] }
```
<!-- DYNAMIC_FEATURE_EXAMPLE_END -->

```rust
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::dynamic::traits::SystemDiagnosticsApi;
use nifi_rust_client::dynamic::types::DiagnosticLevel;
use nifi_rust_client::dynamic::VersionResolutionStrategy;

let client = NifiClientBuilder::new("https://nifi.example.com:8443")?
    .version_strategy(VersionResolutionStrategy::Closest)
    .build_dynamic()?;

// login() authenticates AND auto-detects the NiFi version.
client.login("admin", "password").await?;
println!("Connected to NiFi {}", client.detected_version());

let diag = client.systemdiagnostics_api()
    .get_system_diagnostics(Some(true), Some(DiagnosticLevel::Verbose), None)
    .await?;
```

When the detected NiFi version doesn't exactly match a supported version, the configured strategy controls what happens: `Strict` (default) requires an exact major.minor match or returns an error; `Closest` picks the nearest supported minor version within the same major; `Latest` always picks the highest supported minor within the same major. All strategies refuse to cross major version boundaries.

See [`crates/nifi-rust-client/README.md`](crates/nifi-rust-client/README.md) for the full API reference, builder options, token management, and error handling.

## Why nifi-rust-client?

**Full API coverage, generated directly from NiFi's own OpenAPI spec.**
Every endpoint NiFi exposes is available as a typed Rust method — nothing hand-written, nothing missing. When a new NiFi version ships, a single `cargo run -p nifi-openapi-gen` regenerates the entire client from the live spec in minutes.

**Two modes for different needs.**
Use **static mode** for full type safety and IDE autocompletion when you target a known NiFi version. Use **dynamic mode** when your tool needs to talk to multiple NiFi clusters running different versions — the client auto-detects the API version at connect time and dispatches to the right generated code. Both modes share the same sub-resource builder pattern and trait hierarchy.

**Tested against real NiFi instances.**
Every generated endpoint gets an auto-generated wiremock stub. Integration tests run against a Docker-hosted NiFi and cover the full request/response cycle — not just serialization. Both static and dynamic modes have dedicated test suites.

**Low-level by design: you stay in control.**
The client does not hide HTTP details behind opinionated abstractions. Token refresh, retry on transient errors, and connection tuning are opt-in — configure what you need, and the defaults stay out of your way.

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
