# AGENTS

## Project Overview

`nifi-rust-client` is an idiomatic Rust client library for the Apache NiFi 2.x REST API.
Multiple NiFi 2.x API versions are supported via Cargo feature flags. Default = semver-latest
spec under `crates/nifi-openapi-gen/specs/`.

## Workspace Structure

```
crates/
  nifi-openapi-gen/           # Code generator — published to crates.io as build-dependency
    src/
      bin/{generate.rs, openapi_diff.rs}   # Orchestrator + standalone diff tool
      emit/                                # Code emitters (api, types, tests, method,
                                           #   common, types_shared, cli/, dynamic/, integration/)
      docs/, repo/                         # Doc generators / repo sync
      diff/                                # Structural diff (endpoint, types, report, index)
      parser.rs, parser_strict.rs          # OpenAPI parser + strict-shape validation
      canonical.rs, plan.rs, layout.rs     # Canonicalization, build plan, on-disk layout
      naming.rs, naming_overrides.rs       # Method-name resolution + overrides
      non_additive_detector.rs,            # Cross-version drift detection + overrides
        non_additive_overrides.rs
      content_type.rs                      # Request/response content-type allow-list
      build_api.rs, util.rs, lib.rs
    scripts/fetch-nifi-spec.sh             # Fetch OpenAPI spec from a running NiFi
    specs/x.y.z/nifi-api.json              # OpenAPI 3.0.1 spec per supported version
    tests/                                 # Parser + emitter unit tests
  nifi-rust-client/           # Library crate — published to crates.io
    build.rs                  # Calls nifi-openapi-gen to generate code at build time
    src/
      lib.rs, client.rs, builder.rs, error.rs   # Hand-written core
      config/{auth,retry}.rs                    # Hand-written
      dynamic/{strategy,client}.rs              # Hand-written; client.rs copied into $OUT_DIR
      pagination.rs, wait.rs, bulk.rs           # Workflow / paging / bulk helpers
      streaming.rs, compat.rs                   # BytesStream type + FlexibleString
      require.rs, url.rs                        # require! macro + URL helpers
      # Generated into $OUT_DIR (not in git):
      #   vx_y_z/    — per-version api/, types/
      #   dynamic/   — canonical api/, types/, availability.rs, client.rs, mod.rs
    tests/*.rs                # Hand-written wiremock tests; per-endpoint stubs are generated
                              #   into $OUT_DIR/tests/<tag>_generated_tests.rs
tests/                        # Integration test crate (needs Docker; not published)
  docker-compose.yml, run.sh, tests/*.rs
```

## Architecture

### Client Design

`NifiClient` holds the HTTP client, base URL, and auth token. Private generic helpers do
all HTTP plumbing (auth, error mapping, deserialization); public endpoint methods are
one-liners that delegate.

Helpers in `client.rs` cover every body / response-type combination the generator emits
(~31 in total). Grouped by HTTP verb:

- **GET:** `get`, `get_void`, `get_with_query`, `get_void_with_query`, `get_text`,
  `get_bytes`, `get_bytes_with_query`, `get_bytes_stream`, `get_bytes_stream_with_query`.
- **POST:** `post`, `post_void`, `post_no_body`, `post_void_no_body`, `post_with_query`,
  `post_void_with_query`, `post_octet_stream`, `post_void_octet_stream`,
  `post_octet_stream_returning_text`, `post_returning_text`, `post_multipart`,
  `post_multipart_with_fields`, `post_void_multipart`.
- **PUT:** `put`, `put_void`, `put_no_body`, `put_void_no_body`, `put_with_query`.
- **DELETE:** `delete`, `delete_returning`, `delete_returning_with_query`,
  `delete_with_query`.

All public on `pub(crate)` and route through `NifiClient::build_request`.

All helpers route through `NifiClient::build_request`, which emits exactly one
`tracing::debug!(method, path, "NiFi API request")` per call and attaches auth,
`X-ProxiedEntitiesChain`, and the optional request-id header. **Helpers must NOT add their
own request-side `tracing::debug!`.** Resource struct methods that delegate to a helper also
must not add one.

Exception: `login` bypasses `build_request` (pre-auth, form-encoded, no bearer). It keeps
its own inline `tracing::debug!` and calls `apply_request_id` directly.

### Streaming binary downloads

Every endpoint with response body `application/octet-stream` or `*/*` is emitted in two
variants:

- `download_nar(id) -> Result<Vec<u8>, NifiError>` — buffered.
- `download_nar_stream(id) -> Result<BytesStream, NifiError>` — streaming.

`BytesStream = Pin<Box<dyn Stream<Item = Result<Bytes, NifiError>> + Send>>` is re-exported
from the crate root with `bytes::Bytes`. Adapt to `tokio::io::AsyncRead` via
`tokio_util::io::StreamReader`.

**Retry semantics:** the initial request (status-line) is auth-retry and transient-retry
eligible. Once chunks start flowing, transport errors terminate the stream and are not retried.

Buffered + streaming siblings share one `Endpoint::FOO` availability entry — dynamic mode's
`require_endpoint` gate applies to both.

### Pagination helpers

Live in `crates/nifi-rust-client/src/pagination.rs`. Currently `HistoryPaginator` for
`GET /flow/history` (offset/count). Two constructors: `flow_history(&NifiClient, …)` (static)
and `flow_history_dynamic(&DynamicClient, …)` (gated on `#[cfg(feature = "dynamic")]`).
New paged endpoints follow the same pattern — new constructor in the same file, no generator work.

### Workflow helpers

Live in `src/wait.rs` for state-transition / async-query polling:

State transitions:

- `wait::processor_state(&NifiClient, id, target, config)` — `Running | Stopped | Disabled`.
- `wait::controller_service_state(...)` — `Enabled | Disabled`.

Async-request polling (`POST` → `GET poll` → optional cleanup `DELETE` via `WaitConfig::cleanup`):

- `wait::parameter_context_update(...)` — async parameter-context update.
- `wait::parameter_context_validation(...)` — async parameter-context validation.
- `wait::provenance_query(...)` — async provenance query completion.
- `wait::provenance_lineage(...)` — async lineage query completion (no `failureReason`).
- `wait::flowfile_drop(...)` — async drop request for a FlowFile queue.
- `wait::flowfile_listing(...)` — async listing request for a FlowFile queue.
- `wait::empty_all_connections(...)` — process-group "empty all connections" drop request.
- `wait::processor_verify_config(...)` / `controller_service_verify_config` / `reporting_task_verify_config` / `parameter_provider_verify_config` / `flow_analysis_rule_verify_config` — async config-verification requests.
- `wait::parameter_provider_apply_parameters(...)` — async apply-parameters request.
- `wait::versioned_flow_update(...)` / `wait::versioned_flow_revert(...)` — async PG version update / revert.

Each has a `_dynamic` sibling under `#[cfg(feature = "dynamic")]`. On timeout, helpers
return `NifiError::Timeout { operation }`.

One-shot bulk actions live in `src/bulk.rs`:

- `bulk::start_process_group` / `stop_process_group` — PUT `/flow/process-groups/{id}`.
- `bulk::enable_all_controller_services` / `disable_all_controller_services` —
  PUT `/flow/process-groups/{id}/controller-services`.

Same free-fn + `_dynamic` sibling pattern as `pagination.rs`.

### Resource Struct Pattern

The API surface mirrors NiFi's tag grouping. Each resource borrows the client and adds
methods for its tag:

```rust
pub struct Flow<'a> { client: &'a NifiClient }
pub struct Processors<'a> { client: &'a NifiClient }
// …
client.flow().get_about_info().await?;
client.processors().get_processor("some-id").await?;
```

Resource structs, types, and wiremock tests are **fully generated** from the OpenAPI spec
via `nifi-openapi-gen`. Generation runs from `build.rs` into `$OUT_DIR` (not tracked in git).
For repo maintenance (README tables, docker-compose, etc.) run
`cargo run -p nifi-openapi-gen --bin generate`. To add or change endpoints, update
`crates/nifi-openapi-gen/src/`.

### Query Parameter Enum Types

OpenAPI query params with `"enum"` are emitted as typed Rust enums (not `&str`). Pipeline:

1. `parser.rs` sets `QueryParam::enum_type_name` and `QueryParamType::Enum(variants)`,
   pushes a `TypeDef { kind: TypeKind::StringEnum(variants) }` into `all_types`.
2. `emit/types.rs::emit_standalone_string_enum` generates an enum with `#[serde(rename = "WIRE")]`
   per variant + a `Display` impl. No `Default` derive (always `Option<T>`).
3. `emit/api.rs::query_param_rust_type` returns `crate::types::TypeName`; query-building
   calls `.to_string()`.

**Naming:** type = capitalize first char of camelCase param name; variants = PascalCase per
underscore-separated word; wire value preserved via `#[serde(rename)]` + `Display`.

**Current enum query params:**

| Type | Variants |
|------|----------|
| `ParameterContextHandlingStrategy` | `KeepExisting`, `Replace` |
| `DiagnosticLevel` | `Basic`, `Verbose` |
| `FlowMetricsReportingStrategy` | `AllProcessGroups`, `AllComponents` |
| `IncludedRegistries` | `Nifi`, `Jvm`, `Bulletin`, `Connection`, `Cluster`, `VersionInfo` |

### `string + format: date-time` → `FlexibleString`

NiFi 2.6.0 emits date-time fields as strings; 2.9.0+ emits the same fields as JSON integers
(Unix millis). `Option<String>` rejects integers and the whole response fails to deserialize.

Pipeline:

1. `parser.rs::parse_field_type`: `type: string` + `format: date-time` →
   `FieldType::DateTimeStr`. Other string formats fall through to plain `Str`.
2. `emit/common.rs::field_type_to_rust`: `DateTimeStr` → `crate::compat::FlexibleString`.
   Both per-version and dynamic emitters route through this single function.
3. `compat::FlexibleString` (hand-written): transparent newtype around `String`, accepts JSON
   strings and numbers on deserialize, serializes as a string. Implements
   `Deref<Target=str>` / `Display` / `AsRef<str>` / `From<&str>` / `From<String>`. Re-exported
   as `nifi_rust_client::FlexibleString`.

Soft-breaking change (`Option<String>` → `Option<FlexibleString>`): pattern matches against
`String`, owned-`String` construction, and direct `==` against `String` need
`.into()` / `.into_inner()` / `.0`. Anything going through `&str` keeps compiling.

If a future spec field drifts but lacks `format: date-time` (e.g. `StatusHistoryDTO.generated`),
add an explicit per-field override mechanism (similar shape to `non_additive_overrides.rs`) —
not yet implemented.

### Response Types

Response structs live in `src/types/<resource>.rs` mirroring `src/api/`. Always
`#[serde(rename_all = "camelCase")]`. For JSON-envelope endpoints (e.g. `{ "about": { … } }`):
define a `pub(crate)` wrapper (`AboutEntity`) and a `pub` DTO (`AboutDto`); the API method
unwraps and returns the DTO.

#### Field-level enum helpers (dynamic mode)

In dynamic mode, DTO string-enum fields keep wire type `Option<String>` (so unknown server
values still deserialize), and a sibling helper enum is generated:

```rust
let body = ScheduleComponentsEntity {
    state: Some(ScheduleComponentsEntityState::Running.into()),
    ..Default::default()
};
if let Some(state) = entity.state.as_deref()
    .and_then(ScheduleComponentsEntityState::from_wire) { /* typed match */ }
```

Helpers are `#[non_exhaustive]`, no serde derives, expose `as_str` / `from_wire` / `Display` /
`From<T> for String`. Variants = union across supported versions, sorted alphabetically by
wire value. Static mode also exposes `as_str()` for parity.

### Authentication

NiFi 2.x uses JWT. Auth strategies via `AuthProvider` trait:

| Provider | Use case |
|---|---|
| `PasswordAuth` | Username/password (single-user, LDAP) |
| `EnvPasswordAuth` | Reads `NIFI_USERNAME` / `NIFI_PASSWORD` from env |
| `StaticTokenAuth` | Pre-obtained JWT (OIDC, vault, prior session) |
| Custom impl | Any JWT-producing mechanism (OIDC exchange, Kerberos, …) |

Flow: `authenticate(&self, client)` obtains a JWT and calls `client.set_token(jwt)`. Token is
sent as `Authorization: Bearer …`. On 401, `authenticate()` is called again and the request
retried once. `client.login(...)` and `client.set_token(...)` remain as low-level primitives.

**Transport-level auth** (orthogonal to `AuthProvider`):

- `.client_identity_pem(pem)` — mTLS client cert (PEM cert+key).
- `.proxied_entities_chain("<u1><u2>")` — sets `X-ProxiedEntitiesChain` header on every
  request (Knox / nginx-mTLS proxies).

**Per-request correlation:** `NifiClientBuilder::request_id_header(Some(name))` attaches a
fresh UUIDv4 to every outgoing request under the chosen header (`"X-Request-Id"`,
`"X-Correlation-Id"`, …) and records it on the per-request span as `request_id`. Off by default.

**Cluster / load balancer:** NiFi 2.x clusters share signing keys, so a JWT from any node is
valid on every other node. Point at a round-robin LB; no sticky sessions needed. The 401
retry handles mid-rotation key-propagation delays.

### Error Handling

`snafu`-based; all variants in `crate::error::NifiError`. Use `#[snafu(display(…))]` and
`.context(SomeSnafu)` at call sites. `unwrap` / `expect` are clippy-denied in non-test code.

Only one "missing field" variant: `NifiError::MissingField { path }` — emitted by
`RequireField::require` / the `require!` macro on `Option<T> = None`. Carries a dotted path
string. **Not retryable.**

### Strict parsing & content-type allow-list

`nifi-openapi-gen` refuses to silently drop OpenAPI shapes. The parser **panics at build time**
with an actionable JSON-pointer message on:

- a schema `type` it hasn't seen;
- a request or response content type not in the allow-list (`crates/nifi-openapi-gen/src/content_type.rs`).

**Recognized request bodies:** `application/json`, `application/octet-stream`,
`multipart/form-data`, `application/x-www-form-urlencoded` (skipped — manual), `*/*` (no body).

**Recognized response bodies:** `application/json`, `text/plain`, `application/octet-stream`,
`application/xml`, `*/*`, empty.

**Multi-content-type precedence:** `application/json` wins. Hence `/site-to-site/peers` is
emitted as JSON (`PeersEntity`) even though it also advertises XML.

**When a future spec introduces a new content type:** the build panics. Add a variant to
`RequestBodyKind` / `ResponseBodyKind`, a match arm in `resolve_request_body` /
`resolve_response_body`, then fix every emitter `match` site the compiler flags. Add a new
HTTP helper in `client.rs` if needed (see `get_text`, `get_bytes`, `post_multipart`). Add a
wiremock test. **Never merge into a `_ =>` catch-all** — surfacing new shapes at compile
time is the whole point.

**When a future spec introduces a new schema `type`:** parser panics in `parse_field_type` or
the query-param parser. Add the right `FieldType` / `QueryParamType` arm in `parser.rs`.

### Method name stability

Generated method names are derived from `operationId` with any trailing `_N` collision
suffix stripped. The raw id is preserved on `Endpoint::raw_operation_id`.

Two build-time checks:

1. **Same-tag collision** — two operations in one tag generating the same name → panic.
2. **Cross-version drift** — same `(tag, method, path)` resolving to different names across
   versions → panic.

Both panics name the exact override entry to add. Overrides live in `NAMING_OVERRIDES`
(`naming_overrides.rs`), keyed by `(spec_version, raw_operationId)`. The table ships empty
and stays empty unless one of the panics fires.

A committed golden file per version at `crates/nifi-openapi-gen/specs/<version>/fn_names.txt`
lists every endpoint's resolved method name; `cargo test` re-runs the table and asserts
match. **Do not bypass a panic by editing the golden file** — fix the parser/overrides root
cause and regenerate.

### Inline-JSON responses (emitter correctness)

When an endpoint declares `application/json` with an inline schema (no `$ref`) — e.g.
`GET /process-groups/{id}/download` whose schema is `{"type": "string"}` even though NiFi
sends a JSON object — the generator emits `-> Result<serde_json::Value, NifiError>` using
the matching helper, NOT the `post_void_no_body` fallback.

Both `dynamic_response_type_for` and `emit_dispatch_dynamic` (in
`crates/nifi-openapi-gen/src/emit/method.rs`) dispatch on `response_kind`, **not** on whether
`response_type` happens to be populated. Static mode is correct via
`common.rs::response_return_type`. New emitter sites that decide between a real helper and
`post_void_no_body` must mirror this. `mod emit_fix_inline_json_tests` pins the shape.

### Multipart request-body schema coverage

When a `multipart/form-data` schema declares `properties` beyond the `file` part, the parser
populates `Endpoint::multipart_fields: Vec<MultipartField>` (sorted alphabetically by wire
name for deterministic regen). Each field becomes a typed function param (`&str`, `bool`, or
`f64`, wrapped in `Option<T>` for non-required), and dispatch routes through
`NifiClient::post_multipart_with_fields`. Empty `multipart_fields` → keeps the older
`post_multipart` dispatch.

CLI exposure of multipart endpoints is intentionally skipped via
`emit/cli/commands.rs::is_skipped_body_kind` — hand-written porcelain (e.g.
`porcelain::flow::import`) consumes the library method directly.

### Path-param URL encoding (single- vs multi-segment)

Every path-param substitution is percent-encoded so a `/` or `?` in the value can't reshape
the URL. Two helpers in `src/url.rs`, selected per param by the emitter:

- `encode_path_segment` — encodes `/` → `%2F`. Default for ordinary params (UUIDs, names).
- `encode_path_multi_segment` — splits on `/`, encodes each segment, rejoins with `/`. Slashes
  are preserved as structural delimiters; `?`/`#`/`%`/space inside a segment are still encoded.

The discriminator is `PathParam::multi_segment`, set by `parser.rs::path_param_is_multi_segment`
from the OpenAPI schema `pattern`: none → single-segment; `.+`/`.*` → multi-segment; **any other
pattern panics** (strict, like the content-type allow-list — classify new patterns explicitly).
Today the only multi-segment param is `GET /policies/{action}/{resource}`'s `{resource}`
(NiFi policy resources are paths like `process-groups/root`). The emitter wraps each param via
`emit/method.rs::path_encode_fn` in both static and dynamic modes.

### nifictl `Commands` enum — manual resource enumeration

The nifictl binary is split:

- `crates/nifictl/src/main.rs` — thin entry (~60 lines): tracing init, CLI parse, delegate
  to `dispatch::run`.
- `cli.rs` — every clap definition: `Cli`, `Commands` / `ConfigCommand` / `OpsCommand` /
  `FlowCommand` enums, `Flow*Args` structs.
- `dispatch.rs` — `run()`, the shared `dispatch_resource` helper, `handle_config`, config /
  context resolution, password-input helpers.

Historically `Commands` flattened `generated::GeneratedResource`. Phase 3 changed this:
porcelain `nifictl flow export|import|replace` shares the top-level `flow` name with the
generator's `Flow` tag and clap rejects duplicate variant names under flatten. `Commands`
now lists every generated resource explicitly (~29 variants), with `Flow` substituted by:

```rust
enum FlowCommand {
    Export(FlowExportArgs),
    Import(FlowImportArgs),
    Replace(FlowReplaceArgs),
    #[command(flatten)]
    Generated(generated::FlowCommand),  // → dispatch_resource
}
```

Each per-resource arm in `run()` routes via the `dispatch_uniform!` macro (one expansion per
variant) with the reconstructed `generated::GeneratedResource::<Variant>`. **A new tag means
both `Commands` in `cli.rs` AND a matching `dispatch_uniform!(cli, command, Variant)` arm in
`dispatch.rs` must grow by one.** The test `help_lists_all_generated_resources` catches drift.

The global `--output` flag (json/yaml/table) has `global = true` and propagates everywhere.
Per-subcommand `--output` flags collide at clap parse-time. Rename to `--output-file` (as
`flow export` does) and give the `clap::Arg` a distinct `id = "..."`.

## Build & Test

| When | Command |
|------|---------|
| After small changes | `cargo check`, `cargo build` |
| After changing a module | `cargo test -p <crate> <module>` |
| Before committing | `cargo test --workspace --exclude nifi-integration-tests` then `pre-commit run --all-files` |
| Test specific NiFi version | `cargo test -p nifi-rust-client --no-default-features --features nifi-2-8-0` |
| Test dynamic mode | `cargo test -p nifi-rust-client --features dynamic` |
| Clippy dynamic mode | `cargo clippy -p nifi-rust-client --features dynamic -- -D warnings` |

**Wiremock tests** live in `crates/nifi-rust-client/tests/` — plain `cargo test`, no Docker.

**`default-members` excludes the `tests` integration crate** (workspace `Cargo.toml`), so a
bare `cargo test` / `cargo build` skips it and "just works". This is required because
`nifictl` depends on `nifi-rust-client` with the `dynamic` feature; a whole-workspace build
unifies `dynamic` on, which removes the static `NifiClient` accessors the integration tests
(static-mode, `#![cfg(not(feature = "dynamic"))]`) compile against. `cargo test --workspace`
still includes the integration crate and will hit that unification error — use
`--exclude nifi-integration-tests`, or run the integration suite via its own `-p` invocation
(`./tests/run.sh`), which is unaffected by `default-members`.

**Integration tests** live in `tests/tests/` and require running NiFi:

```bash
./tests/run.sh                                # start → run → stop
./tests/run.sh --skip-cleanup                 # leave NiFi up after tests
./tests/run-matrix.sh                         # all supported NiFi versions
./tests/run-matrix.sh --versions=2.6.0,2.8.0  # specific versions
```

Integration env vars: `NIFI_URL` (default `https://localhost:8443`), `NIFI_USERNAME`
(default `admin`), `NIFI_PASSWORD` (default `adminpassword123`).

## Multi-Version Support

### Layout

Generated files live in `$OUT_DIR/v{major}_{minor}_{patch}/` (not in git). Each contains
`api/`, `types/`, `mod.rs`. `lib.rs` re-exports the active version under a stable public
path via `include!(concat!(env!("OUT_DIR"), "/generated_lib.rs"))`.

Naming: spec dir `x.y.z`, Rust module `vx_y_z`, Cargo feature `nifi-x-y-z`.

### Selecting a version

```toml
nifi-rust-client = { version = "...", default-features = false, features = ["nifi-x-y-z"] }
```

Default feature = semver-latest spec. IDE autocomplete uses it.

### Dynamic mode

The `dynamic` feature compiles all supported versions and detects the running NiFi version
at runtime:

```toml
nifi-rust-client = { version = "...", features = ["dynamic"] }
```

`DynamicClient` lazily detects the version via `/flow/about` (auto on `login()`, or
explicit via `detect_version()`). It emits **one** set of types (fields `Option<T>` where any
version omits them) and **one** set of API methods, each starting with a runtime
`require_endpoint(Endpoint::FOO).await?` check.

The canonical emitter (`crates/nifi-openapi-gen/src/emit/dynamic/{mod,api,types,availability,index}.rs`)
consumes `CanonicalSpec` (built by `canonicalize_or_panic` over all spec versions) and emits:

- `$OUT_DIR/dynamic/api/<tag>.rs` — one concrete struct per tag with inherent methods
  routing through `DynamicClient::inner()`. No traits, no dispatch enums.
- `$OUT_DIR/dynamic/types/<tag>.rs` — union DTOs. Fields present in every supporting version
  stay at their natural type; version-specific fields are `Option<T>`. String enums = union
  of all variants.
- `$OUT_DIR/dynamic/availability.rs` — `pub enum Endpoint` plus `ENDPOINT_AVAILABILITY` and
  `QUERY_PARAM_AVAILABILITY` const tables consulted at runtime.
- `$OUT_DIR/dynamic/mod.rs` — orchestrator that re-exports `DynamicClient`,
  `VersionResolutionStrategy`, `Endpoint`, `DetectedVersion`, `LATEST_NIFI_VERSION`.

Hand-written `DynamicClient` (`src/dynamic/client.rs`) is copied (with `strategy.rs`) into
`$OUT_DIR/dynamic/` by `build.rs`.

API shape is flat in both modes — path params are ordinary positional arguments matching
the OpenAPI placeholder order:

```rust
let client = NifiClientBuilder::new("https://nifi:8443")?.build_dynamic()?;
client.login("admin", "password").await?;          // auth + auto-detect version
let about = client.flow().get_about_info().await?;
let analysis = client.controller_services()
    .analyze_configuration("service-id", &body).await?;
```

**Forward compatibility:** all dynamic structs and enums carry `#[non_exhaustive]`. Endpoints
unavailable on the detected version → `NifiError::UnsupportedEndpoint`. Query params set to
non-`None` on an unsupporting version → `NifiError::UnsupportedQueryParam` (silent-when-None /
error-when-set, design §7.3).

To extract `Option<T>` field values, use `RequireField::require` or the `require!` macro
(crate root; see `src/require.rs`). Nested extractions go through the macro so the error
path reflects the full dotted identifier chain (`"component.name"`).

#### VersionResolutionStrategy

Maps a detected NiFi version → supported API module when no exact match exists.

| Variant | Behavior |
|---------|----------|
| `Strict` | Exact major.minor required; else `NifiError::UnsupportedVersion`. **Default.** |
| `Closest` | Nearest supported minor in same major; ties → lower. |
| `Latest` | Highest supported minor in same major. |

- Major boundaries are never crossed (NiFi 1.x → 2.x is always an error).
- Non-strict resolutions emit `tracing::warn!`.
- Patch ignored — only major.minor compared (via `semver` crate).

Implementation: `src/dynamic/strategy.rs` (hand-written enum + `resolve_version` + tests);
`detect_version()` in `DynamicClient` calls `strategy.resolve()` after `/flow/about`.
Configure via `NifiClientBuilder::version_strategy(...)` before `.build_dynamic()`.

### Adding or bumping a NiFi version

```bash
# 1. Fetch spec from a running NiFi of the target version
NIFI_VERSION=x.y.z ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh

# 2. Run generator — updates Cargo.toml features, README versions table,
#    docker-compose default tag, API changes doc. (Code gen happens in build.rs.)
NIFI_VERSION=x.y.z cargo run -p nifi-openapi-gen

# 3. Verify all versions compile
cargo build --features dynamic
cargo build --no-default-features --features nifi-x-y-z

# 4. Commit the relevant tree
git add crates/nifi-openapi-gen/specs/x.y.z/ \
        crates/nifi-rust-client/Cargo.toml tests/Cargo.toml \
        README.md tests/docker-compose.yml NIFI_API_CHANGES.md
```

The generator uses **semver ordering** (`semver` crate) to pick the latest version when
setting `default` and auto-detecting which spec to use.

### Version-specific integration tests

Gate version-only endpoints with `#[cfg(feature)]`:

```rust
#[cfg(feature = "nifi-x-y-z")]
#[tokio::test]
#[ignore]
async fn test_endpoint_only_in_x_y_z() { … }
```

### Integration test override registry

Auto-generated endpoint-availability and field-presence tests assume `id` path params resolve
to the `"root"` sentinel and that newly added fields are populated on the stock test flow.
Some resources break those assumptions (e.g. `/connectors/{id}` has no `"root"`,
`ProvenanceEventDTO.connectorId` is conditional-by-design). Exceptions live in
`crates/nifi-openapi-gen/src/emit/integration/overrides.rs`:

- `ENDPOINT_OVERRIDES` — keyed by `EndpointScope::{Exact, Tag, PathPrefix}`. Lookup order:
  `Exact → PathPrefix → Tag`, first match wins (so `Exact` can carve a single endpoint out
  of a tag-wide skip). Today only `SkipPositiveTest { reason }`; add a `CustomSetup
  { helper_fn, reason }` variant when a fixture-creating helper can return real path values.
- `FIELD_PRESENCE_OVERRIDES` — keyed by `(type_name, field_name)` (Rust PascalCase /
  snake_case). Marks a field as conditional-by-design; positive presence test skipped,
  negative test on older versions still emitted.

Both emitters replace skipped positive tests with a `// skipped by overrides` block carrying
the reason.

**Conditional-wording scan:** every build, `scan_new_conditional_fields` walks newly added
fields across spec diffs and fails if any description contains phrases like "will not be
set" / "may not be populated" without a `FIELD_PRESENCE_OVERRIDES` entry. On a version
bump, either add an override (with reason) or verify the stock flow really does populate it.

## Releases

`nifi-openapi-gen`, `nifi-rust-client`, and `nifictl` are released **independently**. Each
crate owns its version, changelog, tag prefix, and CI pipeline.

### Cadence

| Crate | Trigger |
|---|---|
| `nifi-openapi-gen` | New NiFi spec, emitter bug fix, codegen-override API extension |
| `nifi-rust-client` | Picks up newer `nifi-openapi-gen`, or hand-written client change |
| `nifictl` | CLI features, porcelain commands, output formatting |

### Tags

| Crate | Tag |
|---|---|
| `nifi-openapi-gen` | `gen-vX.Y.Z` |
| `nifi-rust-client` | `client-vX.Y.Z` |
| `nifictl` | `ctl-vX.Y.Z` |

`.github/workflows/release.yml` listens for all three and runs `{prefix}-test → -publish →
-release` per crate. The test job asserts the Git tag matches the crate's `Cargo.toml`
version so mismatches fail fast.

### release.py

```bash
# Generator (must publish first when gen has new commits)
python3 release/release.py gen patch --tag-message "release message"

# Then — only after gen is on crates.io — bump the build-dep on the client:
#   1. Edit crates/nifi-rust-client/Cargo.toml [build-dependencies] nifi-openapi-gen version.
#   2. Commit: chore(client): bump nifi-openapi-gen build-dep to X.Y.Z
#   3. Run:
python3 release/release.py client patch --tag-message "release message"

# nifictl is independent
python3 release/release.py ctl patch --tag-message "release message"
```

`release.py client` first hits `https://crates.io/api/v1/crates/nifi-openapi-gen/<version>`
to confirm the build-dep is published; if not, it aborts. This is the only guard against
releasing a client that depends on an unpublished generator.

### Changelogs

`crates/{nifi-openapi-gen,nifi-rust-client,nifictl}/CHANGELOG.md` — generated from
conventional commits via `git log {old-tag}..HEAD -- <crate-paths>`. A commit touching
multiple crates appears in each relevant changelog. Internal scopes (`ci`, `release`,
`rustfmt`, `rust-analyzer`) and the `chore` type are filtered out.

### First-release-under-new-scheme fallback

When `release.py` runs the first time after the prefixed-tag split there is no
`gen-v<old>` / `client-v<old>` tag yet. The script falls back to the legacy `v<old>` tag
for both git-log range and "Unreleased" compare link. Subsequent releases use the prefixed form.

### Skipping packaging checks

`release.py` runs `cargo package -p <crate> --allow-dirty` twice (pre-bump sanity check +
post-lockfile regen). Both gated on `SKIP_PACKAGE_CHECK=1` for bootstrap situations where
nothing is on crates.io yet.

## References

| Resource | URL |
|----------|-----|
| NiFi 2.8.0 REST API docs | <https://nifi.apache.org/nifi-docs/rest-api.html> |
| NiFi OpenAPI specs (local) | `crates/nifi-openapi-gen/specs/{version}/nifi-api.json` — 237 paths each. Fetch with `./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh`. Use `grep` / `python3 -c "import json…"` rather than fetching the HTML docs. |
| nipyapi (Python client — API design reference) | <https://github.com/Chaffelson/nipyapi> |
| octocrab (Rust API client — ergonomics reference) | <https://github.com/XAMPPRocky/octocrab> |
| kube-rs (Rust K8s client — domain-adjacent reference) | <https://github.com/kube-rs/kube> |
| snafu docs | <https://docs.rs/snafu> |
