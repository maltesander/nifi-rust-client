# nifi-openapi-gen

Internal code generator for `nifi-rust-client`. Reads an OpenAPI 3.0.1 spec and writes:

- `src/v{version}/types/<tag>.rs` + `common.rs` + `mod.rs` — per-tag DTO/entity structs
- `src/v{version}/api/<tag>.rs` + `mod.rs` — per-tag resource structs with async methods
- `src/lib.rs` — cfg-gated re-exports (auto-managed)
- `tests/v{version}_generated_tests.rs` — wiremock stubs for every endpoint

Stale files within the active version's `api/` and `types/` dirs are deleted automatically. Other versions' directories are never touched. `cargo fmt` is run on all output.

When the `dynamic` feature is present in `Cargo.toml`, the generator also writes:

- `src/dynamic/types.rs` — common union types with all fields as `Option<T>`
- `src/dynamic/api.rs` — `DynamicClient` with version-dispatching API methods
- `src/dynamic/conversions.rs` — `From` impls converting each version's types to common types
- `src/dynamic/mod.rs` — module re-exports

These files are regenerated from scratch on every run, covering all supported versions.

In addition to generating Rust code, the generator keeps three repo-level files in sync on every run (using all spec versions discovered in `specs/`, not just the one being generated):

- `README.md` — rewrites the supported-versions table between `<!-- SUPPORTED_VERSIONS_START -->` / `<!-- SUPPORTED_VERSIONS_END -->` markers with live endpoint and type counts and per-version deltas
- `crates/nifi-rust-client/README.md` — rewrites the static-mode feature flag example between `<!-- STATIC_FEATURE_EXAMPLE_START -->` / `<!-- STATIC_FEATURE_EXAMPLE_END -->` markers
- `tests/docker-compose.yml` — updates the `${NIFI_IMAGE_TAG:-x.y.z}` default to match the semver-latest version

## Running the generator

```bash
# Generate for the default version (latest present in specs/)
cargo run -p nifi-openapi-gen --bin generate

# Generate for a specific version
NIFI_VERSION=x.y.z cargo run -p nifi-openapi-gen --bin generate
```

## Specs

Specs live in `specs/<version>/nifi-api.json`. The generator picks the version from `$NIFI_VERSION` or falls back to the semver-latest version present in `specs/`.

### Fetching a fresh spec

```bash
# Fetch from the default running NiFi instance
# Saves to specs/<version>/nifi-api.json
./scripts/fetch-nifi-spec.sh

# Custom output directory
OUT_DIR=target/specs ./scripts/fetch-nifi-spec.sh

# Non-default docker-compose file
COMPOSE_FILE=/path/to/docker-compose.yml ./scripts/fetch-nifi-spec.sh
```

The script reads the NiFi version from the running container and names the output file accordingly.

## Adding or bumping a NiFi version

```bash
# 1. Fetch spec from a running NiFi instance of the target version
NIFI_VERSION=x.y.z ./scripts/fetch-nifi-spec.sh

# 2. Run the generator — also updates the README versions table and
#    docker-compose default tag automatically
NIFI_VERSION=x.y.z cargo run -p nifi-openapi-gen --bin generate

# 3. Verify all versions compile
cargo build --no-default-features --features nifi-x-y-z
cargo build --features dynamic

# 4. Commit the generated files
git add specs/x.y.z/ \
        ../nifi-rust-client/src/vx_y_z/ \
        ../nifi-rust-client/src/lib.rs \
        ../nifi-rust-client/Cargo.toml \
        ../nifi-rust-client/tests/vx_y_z_generated_tests.rs \
        ../../tests/Cargo.toml \
        ../../README.md \
        ../../crates/nifi-rust-client/README.md \
        ../../tests/docker-compose.yml
```

The generator uses semver ordering (via the `semver` crate) to determine the latest version when setting the `default` Cargo feature.

## Generated doc comments

The generator enriches every method and struct with documentation derived from the OpenAPI spec:

**API methods** (`emit/api.rs`):

- The OpenAPI `summary` becomes the one-line method doc; `description` (if present) is appended as additional paragraphs. Multi-line descriptions are emitted verbatim — blank lines become `///` separator lines.
- A `# Returns` rustdoc section lists every documented 2xx and 3xx HTTP response code and its description, sorted by status code.
- A `# Errors` rustdoc section lists every documented 4xx/5xx HTTP response code and its description, sorted by status code.
- A `# Permissions` rustdoc section lists the NiFi policy expressions required to call the endpoint. Endpoints with no authentication requirement get "No authentication required."

**DTO/entity structs** (`emit/types.rs`):

- Struct and field docs are emitted as full multi-line rustdoc comments, preserving paragraph breaks from the spec.
- Fields marked `readOnly` in the spec receive `#[serde(skip_serializing)]` so they are never sent in request bodies. Their doc comment is appended with "Read-only — this field is ignored when serializing requests to NiFi."

## Stale file cleanup

During a full-discovery run (no `NIFI_VERSION` / `NIFI_API_SPEC` override), the generator scans for `src/v*/` directories and `tests/v*_generated_tests.rs` files whose version is no longer present in `specs/` and deletes them automatically. Single-version runs leave other version directories untouched.

## Architecture

The generator is organized into four module groups plus two binaries:

| Module | Responsibility |
|--------|---------------|
| `src/bin/generate.rs` | Thin orchestrator — version resolution, calls emit/docs/repo modules |
| `src/bin/openapi_diff.rs` | Standalone CLI tool for diffing two OpenAPI specs (requires `--features cli`) |
| `src/parser.rs` | Parse OpenAPI spec into `ApiSpec` — paths, operations, schemas, query param enums |
| `src/diff.rs` | Structural diffing of two `ApiSpec` values |
| `src/util.rs` | Shared helpers (naming conventions, string manipulation) |
| **`src/emit/`** | **Code emitters** |
| `src/emit/types.rs` | Emit `types/<tag>.rs` — structs, enums, wrappers |
| `src/emit/api.rs` | Emit `api/<tag>.rs` — resource structs with async methods |
| `src/emit/tests.rs` | Emit wiremock test stubs |
| `src/emit/common.rs` | Shared emit helpers (doc comments, serde attributes) |
| `src/emit/dynamic/` | Emit all `dynamic/` code — traits, dispatch enums, per-version impls, common types, conversions, tests |
| **`src/docs/`** | **Documentation generation** |
| `src/docs/versions_table.rs` | Rewrite the supported-versions table in `README.md` |
| `src/docs/api_changes.rs` | Generate per-version API change summaries |
| `src/docs/readme_examples.rs` | Rewrite the static-mode feature flag example in `crates/nifi-rust-client/README.md` |
| **`src/repo/`** | **Repository sync** |
| `src/repo/lib_rs.rs` | Regenerate `lib.rs` with cfg-gated re-exports |
| `src/repo/cargo_features.rs` | Sync Cargo.toml feature flags for both crates |
| `src/repo/docker_compose.rs` | Update docker-compose default NiFi image tag |
| `src/repo/cleanup.rs` | Delete stale version directories and test files |

To add or change endpoints, update the spec parsing and emission logic in the `src/` modules rather than editing generated files directly.

## OpenAPI diff tool

The `openapi-diff` binary provides two subcommands:

```bash
# Compare two specs — shows added/removed/changed endpoints and types
cargo run -p nifi-openapi-gen --features cli --bin openapi-diff -- \
  compare specs/2.7.0/nifi-api.json specs/2.8.0/nifi-api.json

# Summarize a single spec — endpoint, type, and tag counts
cargo run -p nifi-openapi-gen --features cli --bin openapi-diff -- \
  summary specs/2.8.0/nifi-api.json
```

Add `--format json` to either subcommand for machine-readable JSON output.

## Integration Test Coverage

Auto-generated by `cargo run -p nifi-openapi-gen`. Do not edit manually.

<!-- INTEGRATION_COVERAGE_START -->
**3** NiFi versions tested · **17** added-endpoint checks · **1** enum param checks · **10** field presence checks · **1** query param checks

<details><summary>2.7.2 (vs 2.6.0)</summary>

| Category | What |
|----------|------|
| Added endpoint | `POST /controller-services/{id}/bulletins/clear-requests` |
| Added endpoint | `POST /controller/flow-analysis-rules/{id}/bulletins/clear-requests` |
| Added endpoint | `POST /controller/parameter-providers/{id}/bulletins/clear-requests` |
| Added endpoint | `POST /controller/registry-clients/{id}/bulletins/clear-requests` |
| Added endpoint | `POST /controller/registry-clients/{id}/config/analysis` |
| Added endpoint | `POST /controller/registry-clients/{id}/config/verification-requests` |
| Added endpoint | `DELETE /controller/registry-clients/{id}/config/verification-requests/{requestId}` |
| Added endpoint | `GET /controller/registry-clients/{id}/config/verification-requests/{requestId}` |
| Added endpoint | `GET /flow/flow-registry-client-definition/{group}/{artifact}/{version}/{type}` |
| Added endpoint | `GET /flow/listen-ports` |
| Added endpoint | `POST /flow/process-groups/{id}/bulletins/clear-requests` |
| Added endpoint | `POST /input-ports/{id}/bulletins/clear-requests` |
| Added endpoint | `POST /output-ports/{id}/bulletins/clear-requests` |
| Added endpoint | `POST /parameter-providers/{id}/bulletins/clear-requests` |
| Added endpoint | `POST /processors/{id}/bulletins/clear-requests` |
| Added endpoint | `POST /remote-process-groups/{id}/bulletins/clear-requests` |
| Added endpoint | `POST /reporting-tasks/{id}/bulletins/clear-requests` |
| Query param | `GET /flow/metrics/{producer}` +`flowMetricsReportingStrategy` |
| Field presence | `BulletinDto.stack_trace` |
| Field presence | `BulletinDto.timestamp_iso` |
| Field presence | `BulletinEntity.timestamp_iso` |
| Field presence | `ComponentManifest.flow_registry_clients` |
| Field presence | `ProcessorDto.physical_state` |
| Field presence | `ProcessorEntity.physical_state` |
| Field presence | `PropertyDescriptor.listen_port_definition` |
| Field presence | `VersionedPropertyDescriptor.listen_port_definition` |

</details>

<details><summary>2.8.0 (vs 2.7.2)</summary>

| Category | What |
|----------|------|
| Enum value | `IncludedRegistries::VersionInfo` accepted |
| Field presence | `ProvenanceEventDto.event_timestamp` |
| Field presence | `ProvenanceNodeDto.component_type` |

</details>

<!-- INTEGRATION_COVERAGE_END -->
