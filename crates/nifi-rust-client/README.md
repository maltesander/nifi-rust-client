# nifi-rust-client

[![CI](https://github.com/maltesander/nifi-rust-client/actions/workflows/ci.yml/badge.svg)](https://github.com/maltesander/nifi-rust-client/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/nifi-rust-client.svg)](https://crates.io/crates/nifi-rust-client)
[![Docs.rs](https://docs.rs/nifi-rust-client/badge.svg)](https://docs.rs/nifi-rust-client)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://github.com/maltesander/nifi-rust-client/blob/main/LICENSE)
![MSRV: 1.88](https://img.shields.io/badge/MSRV-1.88-blue.svg)

Idiomatic Rust client for the Apache NiFi 2.x REST API.

All NiFi REST API endpoints are generated from the official OpenAPI spec and exposed via typed resource accessor methods. Per-version API modules and the dynamic dispatch layer are generated at build time from OpenAPI specs via `build.rs` — no manual code generation step is needed; just select a feature flag and build.

## Supported NiFi Versions

<!-- SUPPORTED_VERSIONS_START -->
| NiFi Version | Feature flag | Endpoints | Types | Changes |
|---|---|---|---|---|
| 2.9.0 | `nifi-2-9-0` | 352 | 437 | +35 endpoints, +32 types, +10 fields vs 2.8.0 |
| 2.8.0 | `nifi-2-8-0` | 317 | 405 | +2 fields, +2 enum values vs 2.7.2 |
| 2.7.2 | `nifi-2-7-2` | 317 | 405 | +17 endpoints, +10 types, +8 fields vs 2.6.0 |
| 2.6.0 | `nifi-2-6-0` | 300 | 395 | — |
<!-- SUPPORTED_VERSIONS_END -->

Full changelog: [NIFI_API_CHANGES.md](../../NIFI_API_CHANGES.md)

## Modes

### Static mode (default)

For applications built against a specific NiFi version. You get full type safety, complete IDE autocompletion, and compile-time guarantees that your code matches the NiFi API exactly.

Use this when you're building a deployment pipeline, custom processor manager, or automation tool that targets a known NiFi cluster. You move in lock-step with your NiFi version — when you upgrade NiFi, you update the feature flag.

<!-- STATIC_FEATURE_EXAMPLE_START -->
```toml
[dependencies]
nifi-rust-client = { version = "0.12", features = ["nifi-2-9-0"] }
```
<!-- STATIC_FEATURE_EXAMPLE_END -->

<!-- STATIC_RUST_EXAMPLE_START -->
```rust
let client = NifiClientBuilder::new("https://nifi:8443")?.build()?;
client.login("admin", "password").await?;

// Full type safety — ProcessorEntity is v2_9_0::types::ProcessorEntity
let proc = client.processors().get_processor("id").await?;
```
<!-- STATIC_RUST_EXAMPLE_END -->

### Dynamic mode

For tools that talk to multiple NiFi versions. The client auto-detects the NiFi version at connect time and dispatches to the correct generated API module.

Use this when you're building a monitoring dashboard, CLI tool, or fleet management system that needs to work across NiFi clusters running different versions.

<!-- DYNAMIC_FEATURE_EXAMPLE_START -->
```toml
[dependencies]
nifi-rust-client = { version = "0.12", features = ["dynamic"] }
```
<!-- DYNAMIC_FEATURE_EXAMPLE_END -->

```rust
use nifi_rust_client::dynamic::types::DiagnosticLevel;

let client = NifiClientBuilder::new("https://nifi:8443")?
    .build_dynamic()?;

// login() authenticates AND auto-detects the NiFi version.
client.login("admin", "password").await?;
if let Some(version) = client.detected_version() {
    println!("Connected to NiFi {version}");
}

// Enum query params are typed — nodewise reporting with verbose diagnostics
let diag = client.systemdiagnostics()
    .get_system_diagnostics(Some(true), Some(DiagnosticLevel::Verbose), None)
    .await?;
```

**IDE autocompletion:** For full autocompletion in dynamic mode, configure your IDE to activate the `dynamic` feature. In VS Code, add to `.vscode/settings.json`:

```json
{ "rust-analyzer.cargo.features": ["dynamic"] }
```

This switches rust-analyzer to analyze dynamic-mode code paths. To switch back to static mode, replace with `["nifi-2-9-0"]` (or your target version).

API groups are reached via per-tag accessors on `DynamicClient` that return concrete resource structs (no traits — method dispatch is inherent):

```rust
use nifi_rust_client::NifiError;

// Flat API: path parameters are passed as method arguments
let analysis = client
    .controller_services()
    .analyze_configuration("service-id", &body)
    .await?;
```

Dynamic types use `#[non_exhaustive]` for forward compatibility — match arms should include a `..` fallback when destructuring fields.

Every canonical endpoint method begins with a runtime `require_endpoint(Endpoint::FOO).await?` availability check, so calls against a server that doesn't support the endpoint return `NifiError::UnsupportedEndpoint`. Query parameters that exist only in some versions are guarded the same way: setting a value against a server that doesn't support it returns `NifiError::UnsupportedQueryParam`. Fields present in all supported versions use their natural type (e.g., `String`); fields that only exist in some versions are `Option<T>`. Enum query params and request bodies are fully typed.

### Version resolution strategies

When the dynamic client detects a NiFi version that doesn't exactly match a supported version, the configured strategy controls what happens:

| Strategy | Behavior | Use case |
|----------|----------|----------|
| `Strict` | Exact major.minor match or error. Default. | Production: ensure API compatibility |
| `Closest` | Nearest minor within same major. Ties prefer lower. | Dev/testing: tolerate minor mismatches |
| `Latest` | Highest minor within same major. | Prototyping: always use newest API surface |

All strategies refuse to cross major version boundaries (e.g. NiFi 1.x → 2.x).
Non-strict resolutions emit a `tracing::warn!` with the detected and resolved versions.

Configure via `NifiClientBuilder::version_strategy()`:

```rust
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::dynamic::VersionResolutionStrategy;

let client = NifiClientBuilder::new("https://nifi:8443")?
    .version_strategy(VersionResolutionStrategy::Closest)
    .build_dynamic()?;
```

### When to use which

| Scenario | Mode |
|---|---|
| CI/CD pipeline targeting one NiFi cluster | Static |
| Custom processor deployment tool | Static |
| Library wrapping specific NiFi features | Static |
| Monitoring dashboard across NiFi clusters | Dynamic |
| CLI tool for ad-hoc NiFi administration | Dynamic |
| Migration tool between NiFi versions | Dynamic |

## Creating a client

```rust
use std::time::Duration;
use nifi_rust_client::NifiClientBuilder;
use url::Url;

let client = NifiClientBuilder::new("https://nifi.example.com:8443")?
    .timeout(Duration::from_secs(60))
    .connect_timeout(Duration::from_secs(10))
    .proxy(Url::parse("http://proxy.internal:3128")?)
    .build()?;

client.login("admin", "password").await?;
```

### Builder options

| Method | Argument | Description |
|--------|----------|-------------|
| `NifiClientBuilder::new(url)` | `&str` | Parse base URL; returns `Err` if invalid |
| `.timeout(d)` | `Duration` | Total request timeout (connect + transfer) |
| `.connect_timeout(d)` | `Duration` | TCP connect-only timeout |
| `.proxy(url)` | `Url` | Proxy for all traffic (HTTP and HTTPS) |
| `.http_proxy(url)` | `Url` | Proxy for HTTP traffic only |
| `.https_proxy(url)` | `Url` | Proxy for HTTPS traffic only |
| `.danger_accept_invalid_certs(b)` | `bool` | Skip TLS verification — dev only |
| `.add_root_certificate(pem)` | `&[u8]` | Trust an additional PEM-encoded CA cert; call multiple times |
| `.auth_provider(p)` | `impl AuthProvider` | Enable auto token refresh on 401 |
| `.client_identity_pem(pem)` | `&[u8]` | mTLS client certificate (PEM-encoded cert+key) |
| `.proxied_entities_chain(s)` | `impl Into<String>` | Set `X-ProxiedEntitiesChain` header for proxy auth |
| `.retry_policy(p)` | `RetryPolicy` | Enable transient error retry with backoff |
| `.version_strategy(s)` | `VersionResolutionStrategy` | Version fallback for dynamic client: `Strict` (default), `Closest`, `Latest` |
| `.build()` | — | Returns `Result<NifiClient, NifiError>` |
| `.build_dynamic()` | — | Returns `Result<DynamicClient, NifiError>` (requires `dynamic` feature) |

## Authentication

`login()` posts credentials to `/nifi-api/access/token` and stores the returned JWT on the client. The token is sent as `Authorization: Bearer <token>` on every subsequent request.

### Auth providers

Instead of calling `login()` with hardcoded credentials, you can configure an auth provider.
The client will automatically re-authenticate and retry when a token expires (401 response):

```rust,no_run
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::config::auth::PasswordAuth;

let client = NifiClientBuilder::new("https://nifi:8443")?
    .auth_provider(PasswordAuth::new("admin", "password"))
    .build()?;

// Initial authentication
client.authenticate().await?;

// If a token expires mid-session, the client automatically
// re-authenticates and retries the request.
let about = client.flow().get_about_info().await?;
```

Read credentials from environment variables (`NIFI_USERNAME`, `NIFI_PASSWORD`):

```rust,no_run
use nifi_rust_client::config::auth::EnvPasswordAuth;

let client = NifiClientBuilder::new("https://nifi:8443")?
    .auth_provider(EnvPasswordAuth::new()) // reads NIFI_USERNAME, NIFI_PASSWORD
    .build()?;
```

Use a pre-obtained JWT token (e.g. from an OIDC flow or vault):

```rust,no_run
use nifi_rust_client::config::auth::StaticTokenAuth;

let client = NifiClientBuilder::new("https://nifi:8443")?
    .auth_provider(StaticTokenAuth::new("my-jwt-token"))
    .build()?;

client.authenticate().await?; // installs the token without contacting the server
```

### Token management

After a successful `login()`, the JWT can be persisted and reused across process restarts:

```rust,no_run
// Save token after login
if let Some(token) = client.token().await {
    fs::write("nifi-token.txt", token)?;
}

// Restore on the next run instead of re-authenticating
let token = fs::read_to_string("nifi-token.txt")?;
client.set_token(token).await;
```

NiFi JWTs expire after **12 hours** by default (configurable server-side via
`nifi.security.user.login.identity.provider.expiration`). An expired token causes any API call to
return `NifiError::Unauthorized { .. }`. If an auth provider is configured, the client handles
this automatically. Otherwise, re-call `login()` to obtain a fresh token:

```rust,no_run
use nifi_rust_client::NifiError;

match client.flow().get_about_info().await {
    Err(NifiError::Unauthorized { .. }) => {
        client.login("admin", "password").await?;
        // retry the original call
    }
    other => { other?; }
}
```

### Logging out

`logout()` sends `DELETE /access/logout` to invalidate the token server-side and clears it locally. The local token is always cleared, even if the server returns an error (e.g. because the token had already expired):

```rust
client.logout().await?;
// client.token() is now None
```

### Cluster / load balancer deployments

NiFi 2.x clusters share signing key material across all nodes, so a JWT issued by any node is
valid on every other node. This means **no sticky sessions are required** — point the client at
a plain round-robin load balancer and tokens work cluster-wide:

```rust,no_run
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::config::auth::PasswordAuth;

let client = NifiClientBuilder::new("https://nifi-loadbalancer:8443")?
    .auth_provider(PasswordAuth::new("admin", "password"))
    .build()?;

// The LB may route login to node1 and subsequent calls to node2 — the token is valid on both.
client.authenticate().await?;
let about = client.flow().get_about_info().await?;
```

No per-node token cache or special routing headers are needed. The existing retry logic
(re-authenticate on 401) handles edge cases like mid-rotation key propagation delays.

### Client methods

| Method | Description |
|--------|-------------|
| `client.login(user, pass)` | `POST /access/token` — authenticate and store the JWT |
| `client.authenticate()` | Authenticate using the configured auth provider |
| `client.logout()` | `DELETE /access/logout` — invalidate server-side and clear local token |
| `client.token()` | Return the current bearer token as `Option<String>` (async) |
| `client.set_token(token)` | Restore a previously obtained token (async) |

### Request correlation

Optionally emit a fresh UUIDv4 per request, both on the outgoing HTTP header and
the per-request tracing span:

```rust
use nifi_rust_client::NifiClientBuilder;

# async fn example() -> Result<(), nifi_rust_client::NifiError> {
let client = NifiClientBuilder::new("https://nifi.example.com:8443")?
    .request_id_header(Some("X-Request-Id"))
    .build()?;
# Ok(())
# }
```

When not configured (the default), no header is sent and no span field is recorded —
byte-identical behavior for users who don't opt in. Useful names are
`"X-Request-Id"`, `"X-Correlation-Id"`, or any other name your observability
stack expects.

## Streaming binary downloads

Endpoints returning `application/octet-stream` or `*/*` are emitted in
two flavors: a buffered `Vec<u8>` method and a streaming `BytesStream`
method named with a `_stream` suffix. See the top-level README for a
worked example.

## Retry policy

By default, failed requests are not retried (except for automatic token refresh when an auth
provider is configured). Enable transient error retry with exponential backoff:

```rust,no_run
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::retry::RetryPolicy;
use std::time::Duration;

let client = NifiClientBuilder::new("https://nifi:8443")?
    .retry_policy(RetryPolicy {
        max_retries: 3,
        initial_backoff: Duration::from_millis(500),
        max_backoff: Duration::from_secs(10),
    })
    .build()?;
```

This retries on HTTP 408, 429, 500, 502, 503, 504, and network errors. Non-retryable errors
(400, 404, 409, etc.) are returned immediately.

Retry composes with token refresh: if a retried request gets a 401, the client refreshes the
token (if a credential provider is configured) before continuing.

## Pagination

NiFi's `GET /flow/history` endpoint supports offset/count paging. The
crate ships a `HistoryPaginator` that walks the history one page (or
one action) at a time, hiding the offset arithmetic and the
server-reported `total` termination check. Zero new dependencies —
the paginator is a plain async state machine.

```rust
use nifi_rust_client::{NifiClientBuilder, NifiError};
use nifi_rust_client::pagination::{flow_history, HistoryFilter};

async fn example() -> Result<(), NifiError> {
    let client = NifiClientBuilder::new("https://nifi.example.com:8443")?.build()?;
    client.login("admin", "adminpassword123").await?;

    // Filter by user and ask for pages of 100.
    let filter = HistoryFilter {
        user_identity: Some("admin".to_string()),
        ..HistoryFilter::default()
    };
    let mut pag = flow_history(&client, filter, 100);

    // Page-at-a-time:
    while let Some(page) = pag.next_page().await? {
        for action in page {
            println!("{action:?}");
        }
    }

    // Or item-at-a-time (buffers one page internally):
    let mut pag = flow_history(&client, HistoryFilter::default(), 100);
    while let Some(action) = pag.next().await? {
        println!("{action:?}");
    }
    Ok(())
}
```

In dynamic mode, use `flow_history_dynamic(&dyn_client, filter, page_size)` instead — same return type and iteration semantics, gated on `#[cfg(feature = "dynamic")]`.

On a missing `actions` or `total` field in the response, the paginator surfaces `NifiError::MissingField { path }` via the same `require!` machinery used elsewhere in the crate. Transient HTTP errors are retried per-page by `NifiClient`'s existing retry policy; the paginator adds no retry logic of its own.

## Waiting and bulk actions

For state transitions that aren't synchronous, `nifi_rust_client::wait`
provides polling helpers with configurable timeout and backoff:

```rust,no_run
use std::time::Duration;
use nifi_rust_client::wait::{self, ProcessorTargetState, WaitConfig};

async fn example(client: nifi_rust_client::NifiClient) -> Result<(), nifi_rust_client::NifiError> {
    let processor = wait::processor_state(
        &client,
        "processor-id",
        ProcessorTargetState::Running,
        WaitConfig {
            timeout: Duration::from_secs(60),
            poll_interval: Duration::from_millis(250),
            ..Default::default()
        },
    ).await?;
    Ok(())
}
```

Four helpers are available: `processor_state`, `controller_service_state`,
`parameter_context_update`, and `provenance_query`. On timeout, all return
`NifiError::Timeout { operation }`.

For bulk control of a process group, `nifi_rust_client::bulk` wraps the
native NiFi bulk endpoints:

```rust,no_run
use nifi_rust_client::bulk;

async fn example(client: nifi_rust_client::NifiClient) -> Result<(), nifi_rust_client::NifiError> {
    bulk::start_process_group(&client, "group-id").await?;
    bulk::enable_all_controller_services(&client, "group-id").await?;
    Ok(())
}
```

Both modules expose `*_dynamic` siblings under the `dynamic` feature flag
for use with `DynamicClient`.

## Resource accessors

All API methods are grouped into resource structs mirroring NiFi's own API grouping. Access them via the client:

```rust
client.flow().get_about_info().await?;
client.processors().get_processor("some-id").await?;
client.processgroups().get_process_group("root").await?;
```

<!-- RESOURCE_ACCESSORS_START -->
| Accessor | Resource | 2.6.0 | 2.7.2 | 2.8.0 | 2.9.0 |
|----------|----------|-------|-------|-------|-------|
| `client.access()` | Access | 3 | 3 | 3 | 3 |
| `client.authentication()` | Authentication | 1 | 1 | 1 | 1 |
| `client.connections()` | Connections | 3 | 3 | 3 | 3 |
| `client.connectors()` | Connectors | — | — | — | 31 |
| `client.controller()` | Controller | 39 | 46 | 46 | 46 |
| `client.controller_services()` | Controller services | 13 | 14 | 14 | 14 |
| `client.counters()` | Counters | 3 | 3 | 3 | 3 |
| `client.datatransfer()` | Data transfer | 7 | 7 | 7 | 7 |
| `client.flow()` | Flow | 60 | 63 | 63 | 67 |
| `client.flowfilequeues()` | Flow file queues | 8 | 8 | 8 | 8 |
| `client.funnels()` | Funnels | 3 | 3 | 3 | 3 |
| `client.inputports()` | Input ports | 4 | 5 | 5 | 5 |
| `client.labels()` | Labels | 3 | 3 | 3 | 3 |
| `client.outputports()` | Output ports | 4 | 5 | 5 | 5 |
| `client.parametercontexts()` | Parameter contexts | 14 | 14 | 14 | 14 |
| `client.parameterproviders()` | Parameter providers | 15 | 16 | 16 | 16 |
| `client.policies()` | Policies | 5 | 5 | 5 | 5 |
| `client.processgroups()` | Process groups | 34 | 34 | 34 | 34 |
| `client.processors()` | Processors | 14 | 15 | 15 | 15 |
| `client.provenance()` | Provenance | 7 | 7 | 7 | 7 |
| `client.provenanceevents()` | Provenance events | 6 | 6 | 6 | 6 |
| `client.remoteprocessgroups()` | Remote process groups | 10 | 11 | 11 | 11 |
| `client.reportingtasks()` | Reporting tasks | 11 | 12 | 12 | 12 |
| `client.resources()` | Resources | 1 | 1 | 1 | 1 |
| `client.sitetosite()` | Site to site | 2 | 2 | 2 | 2 |
| `client.snippets()` | Snippets | 3 | 3 | 3 | 3 |
| `client.systemdiagnostics()` | System diagnostics | 2 | 2 | 2 | 2 |
| `client.tenants()` | Tenants | 11 | 11 | 11 | 11 |
| `client.versions()` | Versions | 14 | 14 | 14 | 14 |

> Numbers indicate the endpoint count available for each accessor in that NiFi version. — means the accessor is not available in that version.
<!-- RESOURCE_ACCESSORS_END -->

## Error handling

All methods return `Result<T, NifiError>`. Variants:

- `NifiError::Unauthorized { message }` — 401 response (expired token, bad credentials)
- `NifiError::Forbidden { message }` — 403 response (insufficient permissions)
- `NifiError::NotFound { message }` — 404 response (resource does not exist)
- `NifiError::Conflict { message }` — 409 response (e.g. component is running)
- `NifiError::Api { status, message }` — other non-2xx responses
- `NifiError::Http { source }` — network or transport error
- `NifiError::Auth { message }` — authentication failed
- `NifiError::InvalidBaseUrl { source }` — bad base URL
- `NifiError::InvalidCertificate { source }` — invalid CA certificate
- `NifiError::UnsupportedVersion { detected }` — dynamic mode: unsupported NiFi version
- `NifiError::UnsupportedEndpoint { endpoint, version }` — dynamic mode: endpoint not available on the detected server version
- `NifiError::UnsupportedQueryParam { endpoint, param, detected_version, supported_in }` — dynamic mode: query param set to a non-`None` value on a server that does not support it
- `NifiError::UnsupportedEnumVariant { variant, type_name, version }` — response enum variant not recognized by this client build
- `NifiError::MissingField { path }` — end-user code asked for an `Option<T>` that turned out to be `None` (via `RequireField::require` / `require!` macro)

Helper methods:

- `err.status_code()` — returns `Option<u16>` for API error variants
- `err.is_retryable()` — returns `true` for transient errors (408, 429, 5xx, network errors)

### Extracting required fields from `Option<T>`

Dynamic-mode DTOs carry every field as `Option<T>` because the union of
fields across supported NiFi versions includes values that not every
server populates. The crate provides a `RequireField` trait and a
`require!` macro to collapse chains of `.ok_or_else()` calls into a
single expression:

```rust
use nifi_rust_client::{require, NifiClientBuilder, NifiError, RequireField};

async fn example() -> Result<(), NifiError> {
    let client = NifiClientBuilder::new("https://nifi.example.com:8443")?
        .build_dynamic()?;
    client.login("admin", "adminpassword123").await?;

    let pg = client.processgroups().get_process_group("root").await?;

    // Macro: walks the chain and stamps a dotted path into the error.
    // On failure you get NifiError::MissingField { path: "component.name" }.
    let name: &String = require!(pg.component.name);
    println!("root process group: {name}");

    // Trait form: the same thing written out. When calling `.require()`
    // directly, pass the full dotted path yourself — the macro does it
    // for you.
    let component = pg.component.require("component")?;
    let _name_again: &String = component.name.require("component.name")?;

    Ok(())
}
```

The macro returns a borrow (`&T`) and expands to code that uses `?`, so
call it from a function returning `Result<_, NifiError>`. Clone the
result if you need an owned value.

On a missing field, both helpers return
`NifiError::MissingField { path }` with `path` set to the dotted
identifier chain. That variant is not retryable (`err.is_retryable()`
returns `false`).

The same helpers work on static-mode `Option<T>` fields — the module
is not mode-gated.

### Dynamic mode version coverage

<!-- markdownlint-disable MD033 -->

Auto-generated by `cargo run -p nifi-openapi-gen`. Do not edit manually.

<!-- INTEGRATION_COVERAGE_START -->
**4** NiFi versions tested · **52** added-endpoint checks (52 tested) · **1** enum param checks (1 tested) · **20** field presence checks (3 tested) · **1** query param checks (1 tested)

<details><summary>2.7.2 (vs 2.6.0)</summary>

| Category | What | Tested |
|----------|------|--------|
| Added endpoint | `POST /controller-services/{id}/bulletins/clear-requests` | ✓ |
| Added endpoint | `POST /controller/flow-analysis-rules/{id}/bulletins/clear-requests` | ✓ |
| Added endpoint | `POST /controller/parameter-providers/{id}/bulletins/clear-requests` | ✓ |
| Added endpoint | `POST /controller/registry-clients/{id}/bulletins/clear-requests` | ✓ |
| Added endpoint | `POST /controller/registry-clients/{id}/config/analysis` | ✓ |
| Added endpoint | `POST /controller/registry-clients/{id}/config/verification-requests` | ✓ |
| Added endpoint | `DELETE /controller/registry-clients/{id}/config/verification-requests/{requestId}` | ✓ |
| Added endpoint | `GET /controller/registry-clients/{id}/config/verification-requests/{requestId}` | ✓ |
| Added endpoint | `GET /flow/flow-registry-client-definition/{group}/{artifact}/{version}/{type}` | ✓ |
| Added endpoint | `GET /flow/listen-ports` | ✓ |
| Added endpoint | `POST /flow/process-groups/{id}/bulletins/clear-requests` | ✓ |
| Added endpoint | `POST /input-ports/{id}/bulletins/clear-requests` | ✓ |
| Added endpoint | `POST /output-ports/{id}/bulletins/clear-requests` | ✓ |
| Added endpoint | `POST /parameter-providers/{id}/bulletins/clear-requests` | ✓ |
| Added endpoint | `POST /processors/{id}/bulletins/clear-requests` | ✓ |
| Added endpoint | `POST /remote-process-groups/{id}/bulletins/clear-requests` | ✓ |
| Added endpoint | `POST /reporting-tasks/{id}/bulletins/clear-requests` | ✓ |
| Query param | `GET /flow/metrics/{producer}` +`flowMetricsReportingStrategy` | ✓ |
| Field presence | `BulletinDto.stack_trace` |  |
| Field presence | `BulletinDto.timestamp_iso` |  |
| Field presence | `BulletinEntity.timestamp_iso` |  |
| Field presence | `ComponentManifest.flow_registry_clients` |  |
| Field presence | `ProcessorDto.physical_state` |  |
| Field presence | `ProcessorEntity.physical_state` | ✓ |
| Field presence | `PropertyDescriptor.listen_port_definition` |  |
| Field presence | `VersionedPropertyDescriptor.listen_port_definition` |  |

</details>

<details><summary>2.8.0 (vs 2.7.2)</summary>

| Category | What | Tested |
|----------|------|--------|
| Enum value | `IncludedRegistries::VersionInfo` accepted | ✓ |
| Field presence | `ProvenanceEventDto.event_timestamp` | ✓ |
| Field presence | `ProvenanceNodeDto.component_type` |  |

</details>

<details><summary>2.9.0 (vs 2.8.0)</summary>

| Category | What | Tested |
|----------|------|--------|
| Added endpoint | `POST /connectors` | ✓ |
| Added endpoint | `GET /connectors/{connectorId}/flow/process-groups/{processGroupId}` | ✓ |
| Added endpoint | `GET /connectors/{connectorId}/flow/process-groups/{processGroupId}/controller-services` | ✓ |
| Added endpoint | `DELETE /connectors/{id}` | ✓ |
| Added endpoint | `GET /connectors/{id}` | ✓ |
| Added endpoint | `PUT /connectors/{id}` | ✓ |
| Added endpoint | `POST /connectors/{id}/apply-update` | ✓ |
| Added endpoint | `GET /connectors/{id}/assets` | ✓ |
| Added endpoint | `POST /connectors/{id}/assets` | ✓ |
| Added endpoint | `GET /connectors/{id}/assets/{assetId}` | ✓ |
| Added endpoint | `GET /connectors/{id}/configuration-steps` | ✓ |
| Added endpoint | `GET /connectors/{id}/configuration-steps/{configurationStepName}` | ✓ |
| Added endpoint | `PUT /connectors/{id}/configuration-steps/{configurationStepName}` | ✓ |
| Added endpoint | `GET /connectors/{id}/configuration-steps/{configurationStepName}/property-groups/{propertyGroupName}/properties/{propertyName}/allowable-values` | ✓ |
| Added endpoint | `POST /connectors/{id}/configuration-steps/{configurationStepName}/verify-config` | ✓ |
| Added endpoint | `DELETE /connectors/{id}/configuration-steps/{configurationStepName}/verify-config/{requestId}` | ✓ |
| Added endpoint | `GET /connectors/{id}/configuration-steps/{configurationStepName}/verify-config/{requestId}` | ✓ |
| Added endpoint | `GET /connectors/{id}/controller-services/{controllerServiceId}/state` | ✓ |
| Added endpoint | `POST /connectors/{id}/controller-services/{controllerServiceId}/state/clear-requests` | ✓ |
| Added endpoint | `DELETE /connectors/{id}/drain` | ✓ |
| Added endpoint | `POST /connectors/{id}/drain` | ✓ |
| Added endpoint | `GET /connectors/{id}/processors/{processorId}/state` | ✓ |
| Added endpoint | `POST /connectors/{id}/processors/{processorId}/state/clear-requests` | ✓ |
| Added endpoint | `POST /connectors/{id}/purge-requests` | ✓ |
| Added endpoint | `DELETE /connectors/{id}/purge-requests/{purge-request-id}` | ✓ |
| Added endpoint | `GET /connectors/{id}/purge-requests/{purge-request-id}` | ✓ |
| Added endpoint | `PUT /connectors/{id}/run-status` | ✓ |
| Added endpoint | `GET /connectors/{id}/search-results` | ✓ |
| Added endpoint | `GET /connectors/{id}/secrets` | ✓ |
| Added endpoint | `GET /connectors/{id}/status` | ✓ |
| Added endpoint | `DELETE /connectors/{id}/working-configuration` | ✓ |
| Added endpoint | `GET /flow/connector-definition/{group}/{artifact}/{version}/{type}` | ✓ |
| Added endpoint | `GET /flow/connector-types` | ✓ |
| Added endpoint | `GET /flow/connectors` | ✓ |
| Added endpoint | `GET /flow/steps/{group}/{artifact}/{version}/{connectorType}/{stepName}` | ✓ |
| Field presence | `AssetReferenceDto.missing_content` |  |
| Field presence | `ComponentManifest.connectors` |  |
| Field presence | `ConfigVerificationResultDto.subject` |  |
| Field presence | `CurrentUserEntity.connectors_permissions` |  |
| Field presence | `DifferenceDto.environmental` |  |
| Field presence | `NarDetailsEntity.connector_types` |  |
| Field presence | `ProvenanceEventDto.connector_id` | ✓ |
| Field presence | `VersionedControllerService.component_state` |  |
| Field presence | `VersionedProcessor.component_state` |  |
| Field presence | `VersionedReportingTask.component_state` |  |

</details>

<!-- INTEGRATION_COVERAGE_END -->
