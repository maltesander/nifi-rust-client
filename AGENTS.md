# AGENTS

## Project Overview

`nifi-rust-client` is an idiomatic Rust client library for the Apache NiFi 2.x REST API.

Supports multiple NiFi 2.x API versions via Cargo feature flags. Default (latest): the semver-latest version present in `crates/nifi-openapi-gen/specs/`.

## Workspace Structure

```
crates/
  nifi-openapi-gen/           # Code generator binary (not published)
    src/
      bin/
        generate.rs       # Orchestrator binary — writes generated files
        openapi_diff.rs   # Standalone OpenAPI diff tool (--features cli)
      emit/               # Code emitters (api, types, tests, dynamic/)
        common.rs         # Shared emit helpers (field_type_to_rust, etc.)
        dynamic/          # Dynamic multi-version emitters
      docs/               # Documentation generators (README table, API changes)
      repo/               # Repo sync (lib.rs, Cargo.toml, cleanup)
      parser.rs           # OpenAPI spec parser
      diff.rs             # Generic OpenAPI spec differ
      util.rs             # Shared utilities
    scripts/
      fetch-nifi-spec.sh  # Fetch OpenAPI spec from a running NiFi instance
    specs/
      x.y.z/nifi-api.json  # OpenAPI 3.0.1 spec per supported NiFi version
    tests/                # Unit tests for parser and emitters
  nifi-rust-client/       # Library crate — published to crates.io
    src/
      vx_y_z/             # Generated per version: api/ + types/ + traits/ + mod.rs
      dynamic/            # Generated: DynamicClient, common types, conversions (dynamic feature)
      lib.rs              # Generated: cfg-gated re-exports, auto-managed by generator
    tests/
      *.rs                        # Hand-written wiremock tests per API group
tests/                    # Integration test crate (not published, needs Docker)
  Cargo.toml
  docker-compose.yml      # NiFi 2.x for local integration testing
  run.sh                  # Test orchestration script
  tests/*.rs              # Integration tests per API group
```

## Architecture

### Client Design

`NifiClient` is the central struct. It holds the HTTP client, base URL, and auth token.
Private generic helpers handle all HTTP plumbing:

```rust
async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, NifiError>
async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T, NifiError>
async fn post_no_body<T: DeserializeOwned>(&self, path: &str) -> Result<T, NifiError>
async fn post_void_no_body(&self, path: &str) -> Result<(), NifiError>
async fn put<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T, NifiError>
async fn put_no_body<T: DeserializeOwned>(&self, path: &str) -> Result<T, NifiError>
async fn put_void_no_body(&self, path: &str) -> Result<(), NifiError>
async fn delete(&self, path: &str) -> Result<(), NifiError>
async fn post_octet_stream<T: DeserializeOwned>(&self, path: &str, filename: Option<&str>, data: Vec<u8>) -> Result<T, NifiError>
async fn post_void_octet_stream(&self, path: &str, filename: Option<&str>, data: Vec<u8>) -> Result<(), NifiError>
```

These helpers attach the JWT auth token, map HTTP errors to `NifiError`, and deserialize responses.
All public endpoint methods are one-liners that delegate to these helpers.

Every HTTP helper (`get`, `post`, `put`, `delete`) and every direct request (`login`, `connect`) must
emit a `tracing::debug!` call before sending, using this exact format:

```rust
tracing::debug!(method = "GET", path, "NiFi API request");
```

Use the literal method name string and the `path` variable. This applies to all future methods.

Resource struct methods (e.g. `FlowApi::about`) that delegate to a generic helper do **not** add their
own `tracing::debug!` call — the helper already emits it. Only methods that send requests directly
(bypassing the helpers) need their own debug line.

### Resource Struct Pattern

The API surface is split into resource structs mirroring NiFi's own API grouping.
Each resource borrows the client and adds methods for its API section:

```rust
pub struct FlowApi<'a> { client: &'a NifiClient }
pub struct ProcessorsApi<'a> { client: &'a NifiClient }
pub struct AccessApi<'a> { client: &'a NifiClient }
// etc.
```

`NifiClient` exposes accessor methods that return these structs:

```rust
client.flow_api().get_about_info().await?
client.processors_api().get_processor("some-id").await?
```

Resource structs, types, and wiremock tests are **fully generated** from the OpenAPI spec via
`nifi-openapi-gen`. Run `cargo run -p nifi-openapi-gen` to regenerate; it writes one `.rs` file per NiFi
API tag into `src/api/` and `src/types/`, plus `tests/generated_tests.rs`, and deletes stale files.
To add or change endpoints, update the spec parsing logic in `crates/nifi-openapi-gen/src/`.

### Query Parameter Enum Types

OpenAPI query parameters that declare an `"enum"` array are emitted as typed Rust enums instead of
raw `&str`. The full pipeline:

1. **Parser** (`parser.rs`): detects `schema.enum` on query params, sets `QueryParam::enum_type_name`
   (e.g. `"ParameterContextHandlingStrategy"`) and `QueryParamType::Enum(variants)`, and pushes a
   `TypeDef { kind: TypeKind::StringEnum(variants) }` into `all_types`.
2. **Emit types** (`emit/types.rs`): `TypeKind::StringEnum` is handled by `emit_standalone_string_enum`,
   which generates an enum with `#[serde(rename = "WIRE_VALUE")]` per variant and a `Display` impl
   that outputs the wire value. No `Default` derive (these types always appear as `Option<T>`).
   `types_referenced_by_tag` includes query param enum names so they land in the correct per-tag file.
3. **Emit API** (`emit/api.rs`): `query_param_rust_type` returns `crate::types::TypeName` for enum
   params; the query-building code calls `.to_string()` (which uses the `Display` impl).

**Naming rules:**

- Type name: capitalize first char of the camelCase param name → `parameterContextHandlingStrategy` →
  `ParameterContextHandlingStrategy`
- Variant name: PascalCase each underscore-separated word → `KEEP_EXISTING` → `KeepExisting`
- Wire value preserved via `#[serde(rename = "KEEP_EXISTING")]` and the `Display` impl

**Current enum query params (NiFi 2.8.0):**

| Type | Variants |
|------|----------|
| `ParameterContextHandlingStrategy` | `KeepExisting`, `Replace` |
| `DiagnosticLevel` | `Basic`, `Verbose` |
| `FlowMetricsReportingStrategy` | `AllProcessGroups`, `AllComponents` |
| `IncludedRegistries` | `Nifi`, `Jvm`, `Bulletin`, `Connection`, `Cluster`, `VersionInfo` |

### Response Types

Response structs live in `src/types/<resource>.rs`, mirroring the resource grouping in `src/api/`.
Use `#[serde(rename_all = "camelCase")]` on all structs.

For endpoints that return a JSON envelope wrapping the real payload (e.g. `{ "about": { ... } }`):

- Define a `pub(crate)` wrapper struct (e.g. `AboutEntity`) used only for deserialization.
- Define a `pub` DTO struct (e.g. `AboutDto`) with the actual fields.
- The API method unwraps the envelope and returns the DTO directly.

### Authentication

NiFi 2.x uses JWT tokens obtained via single-user credentials:

1. POST `/nifi-api/access/token` with form fields `username` and `password`
2. Response body is the raw JWT token string
3. Token is stored on `NifiClient` and sent as `Authorization: Bearer <token>` on every request

`NifiClient::login(username, password)` performs this flow and stores the token.
For integration tests, credentials are read from `NIFI_USERNAME` / `NIFI_PASSWORD` env vars.

### Error Handling

Errors use `snafu`. All variants are in `crate::error::NifiError`.
Use `#[snafu(display(...))]` and `.context(SomeSnafu)` at call sites.
Do not use `unwrap` or `expect` in non-test code — clippy denies it.

## Build & Test

### Commands

| When | Command |
|------|---------|
| After small changes | `cargo check`, `cargo build` |
| After changing a module | `cargo test -p <crate> <module>` |
| Before committing | `cargo test --workspace` then `pre-commit run --all-files` |
| Test specific NiFi version | `cargo test -p nifi-rust-client --no-default-features --features nifi-2-8-0` |
| Test dynamic mode | `cargo test -p nifi-rust-client --features dynamic` |
| Clippy dynamic mode | `cargo clippy -p nifi-rust-client --features dynamic -- -D warnings` |

### Integration Tests

**Wiremock tests** live in `crates/nifi-rust-client/tests/` and run with plain `cargo test` — no Docker needed.

**Integration tests** live in `tests/tests/` and require a running NiFi instance.

```bash
./tests/run.sh                 # start NiFi → run tests → stop NiFi
./tests/run.sh --skip-cleanup  # leave NiFi running after tests (faster iteration)
./tests/run-matrix.sh                        # test all supported NiFi versions
./tests/run-matrix.sh --versions=2.6.0,2.8.0 # test specific versions only
```

Environment variables read by integration tests:

| Variable | Default | Description |
|----------|---------|-------------|
| `NIFI_URL` | `https://localhost:8443` | NiFi base URL |
| `NIFI_USERNAME` | `admin` | Single-user login |
| `NIFI_PASSWORD` | `adminpassword123` | Single-user password |

## Multi-Version Support

### How versions are structured

Generated files live in `crates/nifi-rust-client/src/v{major}_{minor}_{patch}/`. Each version
directory contains `api/`, `types/`, and `mod.rs`. `lib.rs` re-exports `api` and `types` from
the active version under a stable public path.

Naming conventions:

| Artifact | Example |
|---|---|
| Version string (spec path dir) | `x.y.z` |
| Rust module name | `vx_y_z` |
| Cargo feature flag | `nifi-x-y-z` |

### Selecting a version

Users add the feature flag to their `Cargo.toml`:

```toml
nifi-rust-client = { version = "...", default-features = false, features = ["nifi-x-y-z"] }
```

The default feature is always the semver-latest version present in the repo. IDE autocompletion
resolves against the default feature automatically.

### Dynamic mode

The `dynamic` feature compiles all supported versions and enables runtime version detection:

```toml
nifi-rust-client = { version = "...", features = ["dynamic"] }
```

The `DynamicClient` lazily detects the NiFi version via the `/flow/about` endpoint and dispatches
API calls to the correct version's generated code. Version detection happens automatically
on `login()`, or can be triggered explicitly via `detect_version()`. Returns common union
types with all fields as `Option<T>`.

The dispatch layer is trait-based with a hierarchical sub-resource pattern matching the static client:

- **`dynamic::traits`** — public traits (e.g., `FlowApi`, `ProcessorsApi`) plus sub-resource traits (e.g., `ControllerServicesConfigApi`, `ProcessorsRunStatusApi`) that callers import to use dispatch enum methods or write generic code
- **`dynamic::dispatch`** — dispatch enums (e.g., `FlowApiDispatch`, `ControllerServicesConfigApiDispatch`) that implement the traits and route to per-version code
- **`dynamic::impls`** — per-version implementations generated from each spec
- **`vx_y_z::traits`** — per-version static traits mirroring the same hierarchy, enabling mockability and generic code for static-mode users

Both static and dynamic modes use the same sub-resource builder pattern for path parameters:

```rust
use nifi_rust_client::dynamic::traits::{FlowApi, ControllerServicesApi, ControllerServicesConfigApi};

let client = NifiClientBuilder::new("https://nifi:8443")?
    .build_dynamic()?;
// login() authenticates AND auto-detects the NiFi version.
client.login("admin", "password").await?;

// Top-level methods
let about = client.flow_api().get_about_info().await?;

// Sub-resource builder chain: .config(id) returns a sub-resource struct
let analysis = client.controller_services_api()
    .config("service-id")
    .analyze_configuration(&body)
    .await?;
```

**Forward compatibility:** All dynamic structs and enums carry `#[non_exhaustive]`. All fields are `Option<T>`. Trait methods for endpoints that don't exist in a given version have default impls returning `NifiError::UnsupportedEndpoint`.

### Adding or bumping a NiFi version

```bash
# 1. Fetch spec from a running NiFi instance of the target version
NIFI_VERSION=x.y.z ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh

# 2. Run generator — writes src/vx_y_z/, updates lib.rs, both Cargo.tomls,
#    the README versions table, and the docker-compose default tag.
NIFI_VERSION=x.y.z cargo run -p nifi-openapi-gen

# 3. Verify all versions compile
cargo build --features dynamic
cargo build --no-default-features --features nifi-x-y-z

# 4. Commit
git add crates/nifi-openapi-gen/specs/x.y.z/ \
        crates/nifi-rust-client/src/vx_y_z/ \
        crates/nifi-rust-client/src/lib.rs \
        crates/nifi-rust-client/Cargo.toml \
        crates/nifi-rust-client/tests/vx_y_z_generated_tests.rs \
        tests/Cargo.toml \
        README.md \
        tests/docker-compose.yml \
        NIFI_API_CHANGES.md
```

The generator uses **semver ordering** (via the `semver` crate) to determine the latest version
when setting the `default` feature and when auto-detecting which spec to use.

### Version-specific integration tests

Gate tests for endpoints that only exist in a specific NiFi version with `#[cfg(feature)]`:

```rust
#[cfg(feature = "nifi-x-y-z")]
#[tokio::test]
#[ignore]
async fn test_endpoint_only_in_x_y_z() { ... }
```

## References

| Resource | URL |
|----------|-----|
| NiFi 2.8.0 REST API docs | <https://nifi.apache.org/nifi-docs/rest-api.html> |
| NiFi OpenAPI specs (local) | `crates/nifi-openapi-gen/specs/{version}/nifi-api.json` — 237 paths each, full request/response schemas. Fetch with `./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh` (requires NiFi running). Use `grep` or `python3 -c "import json..."` to look up response schemas instead of fetching the HTML docs. |
| nipyapi (Python client — for API design reference) | <https://github.com/Chaffelson/nipyapi> |
| octocrab (Rust API client — for ergonomics reference) | <https://github.com/XAMPPRocky/octocrab> |
| kube-rs (Rust K8s client — domain-adjacent reference) | <https://github.com/kube-rs/kube> |
| snafu docs | <https://docs.rs/snafu> |
