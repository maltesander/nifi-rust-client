# AGENTS

## Project Overview

`nifi-rust-client` is an idiomatic Rust client library for the Apache NiFi 2.x REST API.

Supports multiple NiFi 2.x API versions via Cargo feature flags. Default (latest): the semver-latest version present in `crates/nifi-openapi-gen/specs/`.

## Workspace Structure

```
crates/
  nifi-openapi-gen/           # Code generator — published to crates.io as build-dependency
    src/
      bin/
        generate.rs       # Orchestrator binary — writes generated files
        openapi_diff.rs   # Standalone OpenAPI diff tool (--features cli)
      emit/               # Code emitters (api, types, tests, dynamic/)
        common.rs         # Shared emit helpers (field_type_to_rust, etc.)
        dynamic/          # Dynamic multi-version emitters
      docs/               # Documentation generators (README tables, API changes, resource accessors)
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
    build.rs              # Calls nifi-openapi-gen to generate code at build time
    src/
      lib.rs              # Hand-written: uses include!() for generated modules
      client.rs           # Hand-written: NifiClient struct and HTTP helpers
      builder.rs          # Hand-written: NifiClientBuilder
      error.rs            # Hand-written: NifiError via snafu
      config/
        auth.rs           # Hand-written: AuthProvider trait and impls
        retry.rs          # Hand-written: retry logic
      dynamic/
        strategy.rs       # Hand-written: VersionResolutionStrategy + resolve_version
        client.rs         # Hand-written: DynamicClient (copied into $OUT_DIR/dynamic/ at build time)
      # Generated at build time into $OUT_DIR (not tracked in git):
      # vx_y_z/           — per-version api/, types/, traits/
      # dynamic/          — canonical api/, types/, availability.rs, client.rs, mod.rs
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

Resource struct methods (e.g. `Flow::about`) that delegate to a generic helper do **not** add their
own `tracing::debug!` call — the helper already emits it. Only methods that send requests directly
(bypassing the helpers) need their own debug line.

#### Pagination helpers

Hand-written pagination helpers live in
`crates/nifi-rust-client/src/pagination.rs`. Currently provides
`HistoryPaginator` for `GET /flow/history` (offset/count paging) with
two constructors: `flow_history(&NifiClient, ...)` for static mode and
`flow_history_dynamic(&DynamicClient, ...)` for dynamic mode (gated on
`#[cfg(feature = "dynamic")]`). Additional paged endpoints follow the
same pattern — a new constructor in the same file, no generator work.

### Resource Struct Pattern

The API surface is split into resource structs mirroring NiFi's own API grouping.
Each resource borrows the client and adds methods for its API section:

```rust
pub struct Flow<'a> { client: &'a NifiClient }
pub struct Processors<'a> { client: &'a NifiClient }
pub struct Access<'a> { client: &'a NifiClient }
// etc.
```

`NifiClient` exposes accessor methods that return these structs:

```rust
client.flow().get_about_info().await?
client.processors().get_processor("some-id").await?
```

Resource structs, types, and wiremock tests are **fully generated** from the OpenAPI spec via
`nifi-openapi-gen`. Code generation runs automatically via `build.rs` at build time — generated files
go to `$OUT_DIR` and are not tracked in git. For repo maintenance (README tables, docker-compose, etc.),
run `cargo run -p nifi-openapi-gen --bin generate`.
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

NiFi 2.x uses JWT tokens. The client supports multiple authentication strategies
via the `AuthProvider` trait:

| Provider | Use case |
|---|---|
| `PasswordAuth` | Username/password login (single-user, LDAP) |
| `EnvPasswordAuth` | Reads `NIFI_USERNAME`/`NIFI_PASSWORD` from environment |
| `StaticTokenAuth` | Pre-obtained JWT (OIDC, vault, previous session) |
| Custom `AuthProvider` impl | Any JWT-producing mechanism (OIDC exchange, Kerberos, etc.) |

The login flow:

1. `AuthProvider::authenticate(&self, client)` obtains a JWT and calls `client.set_token(jwt)`
2. The token is sent as `Authorization: Bearer <token>` on every request
3. On 401, the client calls `authenticate()` again and retries the request once

Direct `client.login(username, password)` and `client.set_token(jwt)` remain
available as low-level primitives.

For integration tests, credentials are read from `NIFI_USERNAME` / `NIFI_PASSWORD` env vars.

#### mTLS and proxy authentication

`NifiClientBuilder` supports two transport-level auth mechanisms orthogonal to
`AuthProvider`:

- `.client_identity_pem(pem)` — mutual TLS client certificate (PEM-encoded cert+key).
- `.proxied_entities_chain("<user1><user2>")` — sets the `X-ProxiedEntitiesChain`
  header on every request, used behind trusted proxies (Knox, nginx with mTLS).

#### Cluster / load balancer deployments

NiFi 2.x clusters share signing key material across all nodes, so a JWT
issued by any node is valid on every other node. No sticky sessions or
per-node token caching is required — point the client at a round-robin
load balancer URL and the single stored token works cluster-wide.

The existing 401 retry logic (re-authenticate via `AuthProvider`, then
retry once) handles edge cases like mid-rotation key propagation delays
transparently.

### Error Handling

Errors use `snafu`. All variants are in `crate::error::NifiError`.
Use `#[snafu(display(...))]` and `.context(SomeSnafu)` at call sites.
Do not use `unwrap` or `expect` in non-test code — clippy denies it.

Only one "missing field" variant exists:

- `NifiError::MissingField { path }` — emitted by end-user code calling
  `RequireField::require` or the `require!` macro on an `Option<T>` that
  turned out to be `None`. Carries only a dotted path string.

This variant is not retryable.

### Strict parsing & content-type allow-list

`nifi-openapi-gen` refuses to silently drop OpenAPI shapes it doesn't
recognize. The parser panics at build time with an actionable message
pointing at the JSON path and the offending keys whenever it encounters:

- a schema `type` it hasn't seen (field types, query parameter types)
- a request or response content type not in the allow-list

The allow-list lives in `crates/nifi-openapi-gen/src/content_type.rs`
and is consumed by both the parser (body-kind detection) and the
emitters (return-type selection, client-helper dispatch).

**Currently recognized request bodies:** `application/json`,
`application/octet-stream`, `multipart/form-data`,
`application/x-www-form-urlencoded` (skipped — manually handled),
`*/*` (emitted as no-body).

**Currently recognized response bodies:** `application/json`,
`text/plain`, `application/octet-stream`, `application/xml`,
`*/*`, and empty (no 2xx content).

**Precedence when a response advertises multiple content types:**
`application/json` wins. This is why `/site-to-site/peers` is emitted
as JSON (`PeersEntity`) even though it also advertises
`application/xml`.

**When a future NiFi spec introduces a new content type:**

1. The build panics at `build.rs` time with
   `nifi-openapi-gen: unknown {request,response}_body_content_type at
   <json-pointer> (content keys=[...])`.
2. Add a new variant to `RequestBodyKind` or `ResponseBodyKind` in
   `crates/nifi-openapi-gen/src/content_type.rs`.
3. Add a match arm in `resolve_request_body` or
   `resolve_response_body`.
4. The Rust compiler will flag every emitter `match` site that now
   has an unhandled variant — fix each one. Do **not** merge the new
   variant into a `_ =>` catch-all; the whole point of the allow-list
   is to surface new shapes at compile time.
5. If the new variant needs a new HTTP helper in
   `crates/nifi-rust-client/src/client.rs`, add it (see
   `get_text`, `get_bytes`, `post_multipart` as references).
6. Add a wiremock test that exercises the new shape end-to-end.

**When a future NiFi spec introduces a new schema `type`:**

The parser panics at `parse_field_type` or the query-param parser
with `nifi-openapi-gen: unknown field_type` /
`unknown query_param_type`. The fix belongs in `parser.rs` — add a
match arm that returns the appropriate `FieldType` / `QueryParamType`.
If the new type has no sensible Rust representation, ask yourself
whether it's really something you want to support rather than
papering over the panic.

**Do not add unknown shapes to a catch-all.** The panic is the
feature — it surfaces NiFi API evolution the moment a new shape
lands, before it can silently corrupt generated code.

### Method name stability

Generated method names on resource structs (e.g. `Processors::update_run_status`)
are derived from the `operationId` with any trailing `_N` collision-counter
suffix stripped. The raw `operationId` is preserved on `Endpoint::raw_operation_id`
for override lookup and diagnostics.

Two build-time checks guarantee stability:

1. **Same-tag collision check** — if two operations in the same tag would
   generate the same method name, the generator panics with an actionable
   message pointing at `crates/nifi-openapi-gen/src/naming_overrides.rs`.
2. **Cross-version drift check** — if the same `(tag, method, path)` triple
   resolves to different method names across supported NiFi versions, the
   generator panics with the same kind of actionable message.

Both panics name the exact override entry the human should add. Overrides
live in `NAMING_OVERRIDES` (`naming_overrides.rs`), keyed by
`(spec_version, raw_operationId)`. The table ships empty and should stay
empty unless one of the panics above fires.

A committed golden file per version at
`crates/nifi-openapi-gen/specs/<version>/fn_names.txt` lists every
endpoint's resolved method name. `cargo test` reruns the table and
asserts it matches the committed file. `cargo run -p nifi-openapi-gen --
--check` (the existing check mode) also catches drift via the
`MutationPlan` pipeline.

**Do not bypass a panic by editing the golden file.** The golden is a
follow-up artifact; the source of truth is the parser + overrides. Fix
the root cause (add an override or accept the rename) and regenerate.

## Build & Test

### Commands

| When | Command |
|------|---------|
| After small changes | `cargo check`, `cargo build` |
| After changing a module | `cargo test -p <crate> <module>` |
| Before committing | `cargo test --workspace --exclude nifi-integration-tests` then `pre-commit run --all-files` |
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
directory contains `api/`, `types/`, `traits/`, and `mod.rs`. `lib.rs` re-exports `api`, `types`,
and `traits` from the active version under a stable public path.

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

The `DynamicClient` lazily detects the NiFi version via the `/flow/about` endpoint. Unlike the
legacy dispatch model that routed to per-version generated code, the canonical-superset model
emits a single set of types (one struct per DTO, fields `Option<T>` where any version omits them)
and a single set of API methods that start with a runtime `require_endpoint(Endpoint::FOO).await?`
availability check. Version detection happens automatically on `login()`, or can be triggered
explicitly via `detect_version()`.

The canonical dynamic emitter lives at
`crates/nifi-openapi-gen/src/emit/dynamic/{mod,api,types,availability,index}.rs`. It consumes
`CanonicalSpec` (built by `canonicalize_or_panic` over all spec versions) and emits:

- `$OUT_DIR/dynamic/api/<tag>.rs` — one `Flow`-style concrete struct per tag, with inherent
  methods that route through `DynamicClient::inner()` (no traits, no dispatch enums).
- `$OUT_DIR/dynamic/types/<tag>.rs` — union DTOs. Fields present in every supporting version
  stay at their natural type (String, i32, etc.); fields that only exist in some versions are
  `Option<T>`. String enums emit the union of all versions' variants.
- `$OUT_DIR/dynamic/availability.rs` — `pub enum Endpoint` (one variant per canonical endpoint)
  plus `ENDPOINT_AVAILABILITY` and `QUERY_PARAM_AVAILABILITY` const tables consulted at runtime.
- `$OUT_DIR/dynamic/mod.rs` — orchestrator that declares `api`, `types`, `availability`,
  `strategy` (hand-written), and `client` (hand-written), and re-exports `DynamicClient`,
  `VersionResolutionStrategy`, `Endpoint`, `DetectedVersion`, and `LATEST_NIFI_VERSION`.

The hand-written `DynamicClient` lives at `crates/nifi-rust-client/src/dynamic/client.rs`.
`build.rs` copies it (alongside `strategy.rs`) into `$OUT_DIR/dynamic/` so the generated
`mod.rs` can declare it via `pub mod client;`.

Both static and dynamic modes expose a flat API shape — path parameters are passed
as ordinary method arguments (first positional argument, in the same order as the
OpenAPI path placeholders):

```rust
let client = NifiClientBuilder::new("https://nifi:8443")?
    .build_dynamic()?;
// login() authenticates AND auto-detects the NiFi version.
client.login("admin", "password").await?;

// Top-level methods
let about = client.flow().get_about_info().await?;

// Path parameters as method arguments
let analysis = client
    .controller_services()
    .analyze_configuration("service-id", &body)
    .await?;
```

**Forward compatibility:** All dynamic structs and enums carry `#[non_exhaustive]`. Fields in
some versions only are `Option<T>`. Endpoints not available on the detected version return
`NifiError::UnsupportedEndpoint`; query parameters set to a non-`None` value on an unsupporting
version return `NifiError::UnsupportedQueryParam` (design §7.3 "silent-when-None / error-when-set").

To extract values from dynamic-mode `Option<T>` fields, use
`RequireField::require` or the `require!` macro from the crate root
(see `crates/nifi-rust-client/src/require.rs`). Nested extractions go
through the macro so the error path reflects the full dotted identifier
chain (`"component.name"`).

#### VersionResolutionStrategy

Controls how `DynamicClient` maps a detected NiFi version to a supported API module when there is no exact match.

| Variant | Behavior |
|---------|----------|
| `Strict` | Exact major.minor match required; returns `NifiError::UnsupportedVersion` otherwise. **Default.** |
| `Closest` | Nearest supported minor within the same major. Ties go to the lower version. |
| `Latest` | Highest supported minor within the same major. |

Design decisions:

- Major version boundaries are never crossed (e.g. NiFi 1.x → 2.x is always an error).
- Non-strict resolutions emit `tracing::warn!` with both the detected and resolved versions.
- Patch component is ignored during comparison — only major.minor is matched.
- Uses the `semver` crate for ordering.

Implementation split:

- `crates/nifi-rust-client/src/dynamic/strategy.rs` — hand-written; contains the enum, `resolve_version()` function, and unit tests.
- `DynamicClient` integration — generated; `detect_version()` calls `strategy.resolve()` after fetching `/flow/about`.

Configure via `NifiClientBuilder::version_strategy(VersionResolutionStrategy)` before calling `.build_dynamic()`.

### Adding or bumping a NiFi version

```bash
# 1. Fetch spec from a running NiFi instance of the target version
NIFI_VERSION=x.y.z ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh

# 2. Run generator — updates Cargo.toml features, README versions table,
#    docker-compose default tag, and API changes doc.
#    (Code generation is handled by build.rs automatically.)
NIFI_VERSION=x.y.z cargo run -p nifi-openapi-gen

# 3. Verify all versions compile
cargo build --features dynamic
cargo build --no-default-features --features nifi-x-y-z

# 4. Commit
git add crates/nifi-openapi-gen/specs/x.y.z/ \
        crates/nifi-rust-client/Cargo.toml \
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

### Integration test override registry

Auto-generated endpoint-availability and field-presence tests assume uniform
behavior: `id` path params resolve to the `"root"` sentinel, and every newly
added field is populated on the stock test flow. Some NiFi resources break
those assumptions (e.g. `/connectors/{id}` has no `"root"` sentinel,
`ProvenanceEventDTO.connectorId` is conditional-by-design). These exceptions
live in `crates/nifi-openapi-gen/src/emit/integration/overrides.rs`.

**Two override types:**

- `ENDPOINT_OVERRIDES` — keyed by `EndpointScope::{Exact, Tag, PathPrefix}`.
  Lookup order is `Exact → PathPrefix → Tag`, first match wins, so an `Exact`
  entry can carve out a single endpoint from a tag-wide skip without deleting
  the tag entry. Only `SkipPositiveTest { reason }` exists today; add
  `CustomSetup { helper_fn, reason }` variant when a helper can create the
  required fixture and return real path param values.
- `FIELD_PRESENCE_OVERRIDES` — keyed by `(type_name, field_name)` in Rust
  PascalCase/snake_case. Marks a field as conditional-by-design so the
  positive presence test is skipped (negative test on older versions still
  emitted).

Both emitters replace the skipped positive test with a `// skipped by
overrides` comment block containing the reason, so the generated files stay
self-documenting.

**Conditional-wording scan.** On every build, `scan_new_conditional_fields`
walks newly added fields across all spec diffs and fails if any field's
description contains phrases like "will not be set", "may not be populated",
etc. and isn't in `FIELD_PRESENCE_OVERRIDES`. On a version bump, if this test
fails, read the flagged field's description and either add an override entry
(documenting why) or verify the stock test flow actually populates it.

## Releases

`nifi-openapi-gen` and `nifi-rust-client` are released **independently**.
Each crate owns its own version (no `version.workspace = true`), its own
changelog, its own tag prefix, and its own CI pipeline. This ended a
long-running pain point where a spec addition to `nifi-openapi-gen` would
force an unrelated `nifi-rust-client` release, and where the pre-update
packaging check silently broke whenever the two crates drifted between
releases.

### Crate release cadence

| Crate | Typical trigger for a release |
|---|---|
| `nifi-openapi-gen` | New NiFi spec bundled, emitter bug fix, API extension for new kinds of codegen overrides |
| `nifi-rust-client` | Picks up a newer `nifi-openapi-gen`, or ships a hand-written client change (builder, retry, tracing, etc.) |
| `nifictl` | CLI feature additions, porcelain commands, output formatting changes |

### Tag scheme

| Crate | Tag format |
|---|---|
| `nifi-openapi-gen` | `gen-vX.Y.Z` |
| `nifi-rust-client` | `client-vX.Y.Z` |
| `nifictl` | `ctl-vX.Y.Z` |

`.github/workflows/release.yml` listens for all three prefixes and dispatches
three jobs per crate: `{prefix}-test → {prefix}-publish → {prefix}-release`.
The `test` job asserts the Git tag matches the corresponding crate's
`Cargo.toml` version so mismatches fail fast.

### release.py usage

```bash
# Release nifi-openapi-gen (always runs first when gen has new commits)
python3 release/release.py gen patch --tag-message "release message"

# Then — only once nifi-openapi-gen is on crates.io — pick it up on the
# client side and release the client:
#   1. Manually edit crates/nifi-rust-client/Cargo.toml to bump the
#      nifi-openapi-gen build-dep version to the freshly published one.
#   2. Commit: chore(client): bump nifi-openapi-gen build-dep to X.Y.Z
#   3. Run:
python3 release/release.py client patch --tag-message "release message"

# Release nifictl (independent of gen and client)
python3 release/release.py ctl patch --tag-message "release message"
```

`release.py client` runs a crates.io lookup as its first gate: it parses
the `nifi-openapi-gen` version from `crates/nifi-rust-client/Cargo.toml`'s
`[build-dependencies]` and hits
`https://crates.io/api/v1/crates/nifi-openapi-gen/<version>`. If the
version isn't published, the script aborts with an actionable error. This
is the only guard against accidentally releasing a client that depends on
an unpublished generator.

### Changelogs

- `crates/nifi-openapi-gen/CHANGELOG.md`
- `crates/nifi-rust-client/CHANGELOG.md`
- `crates/nifictl/CHANGELOG.md`

All three are generated from conventional commits via `git log
{old-tag}..HEAD -- <crate-paths>`, so each changelog only lists commits
that touched its crate's files. A commit that touches multiple crates
appears in each relevant changelog, which is almost always what you want.

Commit scopes that are purely internal (`ci`, `release`, `rustfmt`,
`rust-analyzer`) and the `chore` type are filtered out in all
changelogs.

### First-release-under-new-scheme fallback

When `release.py` runs for the first time after the prefixed-tag split,
there is no `gen-v<old>` or `client-v<old>` tag yet. The script falls back
to the legacy `v<old>` tag for both the git-log range and the
"Unreleased" compare link. After that, every subsequent release uses the
prefixed form.

### Skipping packaging checks

`release.py` calls `cargo package -p <crate> --allow-dirty` twice: once
before the version bump (as a sanity check on the previous state) and
once after the lockfile is regenerated. Both are gated on
`SKIP_PACKAGE_CHECK=1` for bootstrapping situations where nothing is on
crates.io yet.

## References

| Resource | URL |
|----------|-----|
| NiFi 2.8.0 REST API docs | <https://nifi.apache.org/nifi-docs/rest-api.html> |
| NiFi OpenAPI specs (local) | `crates/nifi-openapi-gen/specs/{version}/nifi-api.json` — 237 paths each, full request/response schemas. Fetch with `./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh` (requires NiFi running). Use `grep` or `python3 -c "import json..."` to look up response schemas instead of fetching the HTML docs. |
| nipyapi (Python client — for API design reference) | <https://github.com/Chaffelson/nipyapi> |
| octocrab (Rust API client — for ergonomics reference) | <https://github.com/XAMPPRocky/octocrab> |
| kube-rs (Rust K8s client — domain-adjacent reference) | <https://github.com/kube-rs/kube> |
| snafu docs | <https://docs.rs/snafu> |
