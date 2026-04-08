# nifi-rust-client

Idiomatic Rust client for the Apache NiFi 2.x REST API.

All NiFi REST API endpoints are generated from the official OpenAPI spec and exposed via typed resource accessor methods.

## Supported NiFi Versions

<!-- SUPPORTED_VERSIONS_START -->
| NiFi Version | Feature flag | Endpoints | Types | Changes |
|---|---|---|---|---|
| 2.8.0 | `nifi-2-8-0` | 317 | 405 | +2 fields, +1 enum values vs 2.7.2 |
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
nifi-rust-client = { version = "0.3", features = ["nifi-2-8-0"] }
```
<!-- STATIC_FEATURE_EXAMPLE_END -->

<!-- STATIC_RUST_EXAMPLE_START -->
```rust
let client = NifiClientBuilder::new("https://nifi:8443")?.build()?;
client.login("admin", "password").await?;

// Full type safety — ProcessorEntity is v2_8_0::types::ProcessorEntity
let proc = client.processors_api().get_processor("id").await?;
```
<!-- STATIC_RUST_EXAMPLE_END -->

### Dynamic mode

For tools that talk to multiple NiFi versions. The client auto-detects the NiFi version at connect time and dispatches to the correct generated API module.

Use this when you're building a monitoring dashboard, CLI tool, or fleet management system that needs to work across NiFi clusters running different versions.

<!-- DYNAMIC_FEATURE_EXAMPLE_START -->
```toml
[dependencies]
nifi-rust-client = { version = "0.3", features = ["dynamic"] }
```
<!-- DYNAMIC_FEATURE_EXAMPLE_END -->

```rust
use nifi_rust_client::dynamic::traits::FlowApi;
use nifi_rust_client::dynamic::types::DiagnosticLevel;

let client = NifiClientBuilder::new("https://nifi:8443")?
    .build_dynamic()
    .await?;
client.login("admin", "password").await?;

// Fields present in all versions are non-optional; version-specific fields are Option<T>
let about = client.flow_api().get_about_info().await?;
println!("NiFi version: {:?}", about.version);

// Enum query params are typed
let diag = client.system_diagnostics_api()
    .get_system_diagnostics(Some(DiagnosticLevel::Verbose))
    .await?;

// Request bodies use typed dynamic union structs (not serde_json::Value)
println!("Connected to NiFi {}", client.detected_version());
```

All 28 API groups have corresponding traits in `dynamic::traits` (e.g., `FlowApi`, `ProcessorsApi`). Import the trait to call methods on a dispatch enum, or use traits for generic code:

```rust
use nifi_rust_client::dynamic::traits::FlowApi;
use nifi_rust_client::NifiError;

async fn check_version(api: &impl FlowApi) -> Result<(), NifiError> {
    let about = api.get_about_info().await?;
    println!("NiFi {}", about.nifi_version.unwrap_or_default());
    Ok(())
}
```

Dynamic types use `#[non_exhaustive]` for forward compatibility — match arms should include a `..` fallback when destructuring fields.

Fields present in all supported versions use their natural type (e.g., `String`); fields that only exist in some versions are `Option<T>`. Endpoints that don't exist in the connected version return `NifiError::UnsupportedEndpoint`. Enum query params and request bodies are fully typed.

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
| `.credential_provider(p)` | `impl CredentialProvider` | Enable auto token refresh on 401 |
| `.retry_policy(p)` | `RetryPolicy` | Enable transient error retry with backoff |
| `.build()` | — | Returns `Result<NifiClient, NifiError>` |

## Authentication

`login()` posts credentials to `/nifi-api/access/token` and stores the returned JWT on the client. The token is sent as `Authorization: Bearer <token>` on every subsequent request.

### Credential providers

Instead of calling `login()` with hardcoded credentials, you can configure a credential provider.
The client will automatically re-authenticate and retry when a token expires (401 response):

```rust,no_run
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::credentials::StaticCredentials;

let client = NifiClientBuilder::new("https://nifi:8443")?
    .credential_provider(StaticCredentials::new("admin", "password"))
    .build()?;

// Initial login
client.login_with_provider().await?;

// If a token expires mid-session, the client automatically
// re-authenticates and retries the request.
let about = client.flow_api().get_about_info().await?;
```

Read credentials from environment variables (`NIFI_USERNAME`, `NIFI_PASSWORD`):

```rust,no_run
use nifi_rust_client::credentials::EnvCredentials;

let client = NifiClientBuilder::new("https://nifi:8443")?
    .credential_provider(EnvCredentials::new()) // reads NIFI_USERNAME, NIFI_PASSWORD
    .build()?;
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
return `NifiError::Unauthorized { .. }`. If a credential provider is configured, the client handles
this automatically. Otherwise, re-call `login()` to obtain a fresh token:

```rust,no_run
use nifi_rust_client::NifiError;

match client.flow_api().get_about_info().await {
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

### Client methods

| Method | Description |
|--------|-------------|
| `client.login(user, pass)` | `POST /access/token` — authenticate and store the JWT |
| `client.login_with_provider()` | Authenticate using the configured credential provider |
| `client.logout()` | `DELETE /access/logout` — invalidate server-side and clear local token |
| `client.token()` | Return the current bearer token as `Option<String>` (async) |
| `client.set_token(token)` | Restore a previously obtained token (async) |

## Retry policy

By default, failed requests are not retried (except for automatic token refresh when a credential
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

## Resource accessors

All API methods are grouped into resource structs mirroring NiFi's own API grouping. Access them via the client:

```rust
client.flow_api().get_about_info().await?;
client.processors_api().get_processor("some-id").await?;
client.processgroups_api().get_process_group("root").await?;
```

| Accessor | Resource |
|----------|----------|
| `client.access_api()` | Token exchange, access configuration |
| `client.authentication_api()` | Authentication configuration |
| `client.connections_api()` | Connection CRUD, flow-file queue management |
| `client.controller_api()` | Controller-level config and bulletins |
| `client.controller_services_api()` | Controller service CRUD |
| `client.counters_api()` | Counter reset and retrieval |
| `client.datatransfer_api()` | Site-to-site data transfer |
| `client.flow_api()` | Flow status, about, current user, registry clients, … |
| `client.flowfilequeues_api()` | Flow-file queue inspection and purge |
| `client.funnels_api()` | Funnel CRUD |
| `client.inputports_api()` | Input port CRUD |
| `client.labels_api()` | Label CRUD |
| `client.outputports_api()` | Output port CRUD |
| `client.parametercontexts_api()` | Parameter context CRUD |
| `client.parameterproviders_api()` | Parameter provider CRUD |
| `client.policies_api()` | Access policy CRUD |
| `client.processgroups_api()` | Process group CRUD, instantiation, export |
| `client.processors_api()` | Processor CRUD |
| `client.provenance_api()` | Provenance queries |
| `client.provenanceevents_api()` | Provenance event retrieval |
| `client.remoteprocessgroups_api()` | Remote process group CRUD |
| `client.reportingtasks_api()` | Reporting task CRUD |
| `client.resources_api()` | Resource listing |
| `client.sitetosite_api()` | Site-to-site configuration |
| `client.snippets_api()` | Snippet CRUD |
| `client.systemdiagnostics_api()` | System diagnostics |
| `client.tenants_api()` | User and group CRUD |
| `client.versions_api()` | Flow version control |

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
- `NifiError::UnsupportedEndpoint { endpoint, version }` — dynamic mode: endpoint not available
- `NifiError::UnsupportedEnumVariant { variant, type_name, version }` — dynamic mode: enum variant not in target version
- `NifiError::MissingRequiredField { field, type_name, version }` — dynamic mode: required field is `None` in request body

Helper methods:

- `err.status_code()` — returns `Option<u16>` for API error variants
- `err.is_retryable()` — returns `true` for transient errors (408, 429, 5xx, network errors)

## Integration test coverage

The following API groups have CRUD lifecycle tests (create → get → update → delete → verify gone):

| File | Groups covered |
|------|---------------|
| `tests/tests/connect.rs` | client builder (insecure TLS, custom CA cert) |
| `tests/tests/flow.rs` | flow status, current user identity, logout |
| `tests/tests/processgroups.rs` | process groups, processors (with `includeDescendantGroups`) |
| `tests/tests/connections.rs` | connections (full CRUD lifecycle) |
| `tests/tests/controller_services.rs` | controller services (type discovered at runtime) |
| `tests/tests/labels.rs` | labels |
| `tests/tests/funnels.rs` | funnels |
| `tests/tests/inputports.rs` | input ports |
| `tests/tests/outputports.rs` | output ports |
| `tests/tests/parametercontexts.rs` | parameter contexts |
| `tests/tests/reportingtasks.rs` | reporting tasks (type discovered at runtime) |
| `tests/tests/tenants.rs` | users, user groups |
| `tests/tests/readonly.rs` | system diagnostics, resources, site-to-site, counters, authentication (smoke only) |
| `tests/tests/dynamic.rs` | dynamic mode: version detection, about, resources, system diagnostics, current user, process groups |
| `tests/tests/flow_status.rs` | flow status reads |
| `tests/tests/flowfilequeues.rs` | flow-file queue listing and drop |
| `tests/tests/provenance.rs` | provenance search and events |

The following API groups are **not yet covered** due to requiring complex setup beyond a fresh NiFi instance:

| Group | Reason |
|-------|--------|
| `remoteprocessgroups` | Requires a reachable remote NiFi URL |
| `datatransfer` | Requires active site-to-site input/output ports |
| `versions` | Requires a connected flow registry |
| `policies` | Depends on the specific authorization model of the test instance |
| `parameterproviders` | Requires a parameter provider extension class to be installed |
| `snippets` | Requires existing canvas components to copy |
