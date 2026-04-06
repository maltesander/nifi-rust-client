# nifi-openapi-gen

Internal code generator for `nifi-rust-client`. Reads an OpenAPI 3.0.1 spec and writes:

- `src/v{version}/types/<tag>.rs` + `common.rs` + `mod.rs` — per-tag DTO/entity structs
- `src/v{version}/api/<tag>.rs` + `mod.rs` — per-tag resource structs with async methods
- `src/lib.rs` — cfg-gated re-exports (auto-managed)
- `tests/v{version}_generated_tests.rs` — wiremock stubs for every endpoint

Stale files within the active version's `api/` and `types/` dirs are deleted automatically. Other versions' directories are never touched. `cargo fmt` is run on all output.

## Running the generator

```bash
# Generate for the default version (latest present in specs/)
cargo run -p nifi-openapi-gen

# Generate for a specific version
NIFI_VERSION=2.7.2 cargo run -p nifi-openapi-gen
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
NIFI_VERSION=2.9.0 ./scripts/fetch-nifi-spec.sh

# 2. Run the generator
NIFI_VERSION=2.9.0 cargo run -p nifi-openapi-gen

# 3. Verify both versions compile
cargo build --no-default-features --features nifi-2-9-0
cargo build --no-default-features --features nifi-2-8-0

# 4. Commit the generated files
git add specs/2.9.0/ \
        ../nifi-rust-client/src/v2_9_0/ \
        ../nifi-rust-client/src/lib.rs \
        ../nifi-rust-client/Cargo.toml \
        ../nifi-rust-client/tests/v2_9_0_generated_tests.rs \
        ../../tests/Cargo.toml
```

The generator uses semver ordering (via the `semver` crate) to determine the latest version when setting the `default` Cargo feature.

## Architecture

The generator is structured as:

| Module | Responsibility |
|--------|---------------|
| `src/main.rs` | Entry point, version resolution, file orchestration |
| `src/parser.rs` | Parse OpenAPI spec into `ParsedSpec` — paths, operations, schemas, query param enums |
| `src/emit_types.rs` | Emit `types/<tag>.rs` — structs, enums, wrappers |
| `src/emit_api.rs` | Emit `api/<tag>.rs` — resource structs with async methods |
| `src/emit_tests.rs` | Emit wiremock test stubs |

To add or change endpoints, update the spec parsing and emission logic in these modules rather than editing generated files directly.
