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
use nifi_rust_client::dynamic::traits::SystemDiagnosticsApi;
use nifi_rust_client::dynamic::types::DiagnosticLevel;

let client = NifiClientBuilder::new("https://nifi:8443")?
    .build_dynamic()?;

// login() authenticates AND auto-detects the NiFi version.
client.login("admin", "password").await?;
println!("Connected to NiFi {}", client.detected_version());

// Enum query params are typed — nodewise reporting with verbose diagnostics
let diag = client.systemdiagnostics_api()
    .get_system_diagnostics(Some(true), Some(DiagnosticLevel::Verbose), None)
    .await?;
```

**IDE autocompletion:** For full autocompletion in dynamic mode, configure your IDE to activate the `dynamic` feature. In VS Code, add to `.vscode/settings.json`:

```json
{ "rust-analyzer.cargo.features": ["dynamic"] }
```

This switches rust-analyzer to analyze dynamic-mode code paths. To switch back to static mode, replace with `["nifi-2-8-0"]` (or your target version).

All API groups have corresponding traits in `dynamic::traits` (e.g., `FlowApi`, `ProcessorsApi`), plus sub-resource traits for endpoints with path parameters (e.g., `ControllerServicesConfigApi`, `ProcessorsRunStatusApi`). Import the trait to call methods on a dispatch enum, or use traits for generic code:

```rust
use nifi_rust_client::dynamic::traits::{FlowApi, ControllerServicesApi, ControllerServicesConfigApi};
use nifi_rust_client::NifiError;

async fn check_version(api: &impl FlowApi) -> Result<(), NifiError> {
    let about = api.get_about_info().await?;
    println!("NiFi {}", about.nifi_version.unwrap_or_default());
    Ok(())
}

// Sub-resource builder chain — same pattern as static mode
let analysis = client.controller_services_api()
    .config("service-id")
    .analyze_configuration(&body)
    .await?;
```

Dynamic types use `#[non_exhaustive]` for forward compatibility — match arms should include a `..` fallback when destructuring fields.

Fields present in all supported versions use their natural type (e.g., `String`); fields that only exist in some versions are `Option<T>`. Endpoints that don't exist in the connected version return `NifiError::UnsupportedEndpoint`. Enum query params and request bodies are fully typed.

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
| `.credential_provider(p)` | `impl CredentialProvider` | Enable auto token refresh on 401 |
| `.retry_policy(p)` | `RetryPolicy` | Enable transient error retry with backoff |
| `.version_strategy(s)` | `VersionResolutionStrategy` | Version fallback for dynamic client: `Strict` (default), `Closest`, `Latest` |
| `.build()` | — | Returns `Result<NifiClient, NifiError>` |
| `.build_dynamic()` | — | Returns `Result<DynamicClient, NifiError>` (requires `dynamic` feature) |

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

<!-- RESOURCE_ACCESSORS_START -->
| Accessor | Resource | 2.6.0 | 2.7.2 | 2.8.0 |
|----------|----------|-------|-------|-------|
| `client.access_api()` | Access | 3 | 3 | 3 |
| `client.authentication_api()` | Authentication | 1 | 1 | 1 |
| `client.connections_api()` | Connections | 3 | 3 | 3 |
| `client.controller_api()` | Controller | 39 | 46 | 46 |
| `client.controller_services_api()` | Controller services | 13 | 14 | 14 |
| `client.counters_api()` | Counters | 3 | 3 | 3 |
| `client.datatransfer_api()` | Data transfer | 7 | 7 | 7 |
| `client.flow_api()` | Flow | 60 | 63 | 63 |
| `client.flowfilequeues_api()` | Flow file queues | 8 | 8 | 8 |
| `client.funnels_api()` | Funnels | 3 | 3 | 3 |
| `client.inputports_api()` | Input ports | 4 | 5 | 5 |
| `client.labels_api()` | Labels | 3 | 3 | 3 |
| `client.outputports_api()` | Output ports | 4 | 5 | 5 |
| `client.parametercontexts_api()` | Parameter contexts | 14 | 14 | 14 |
| `client.parameterproviders_api()` | Parameter providers | 15 | 16 | 16 |
| `client.policies_api()` | Policies | 5 | 5 | 5 |
| `client.processgroups_api()` | Process groups | 34 | 34 | 34 |
| `client.processors_api()` | Processors | 14 | 15 | 15 |
| `client.provenance_api()` | Provenance | 7 | 7 | 7 |
| `client.provenanceevents_api()` | Provenance events | 6 | 6 | 6 |
| `client.remoteprocessgroups_api()` | Remote process groups | 10 | 11 | 11 |
| `client.reportingtasks_api()` | Reporting tasks | 11 | 12 | 12 |
| `client.resources_api()` | Resources | 1 | 1 | 1 |
| `client.sitetosite_api()` | Site to site | 2 | 2 | 2 |
| `client.snippets_api()` | Snippets | 3 | 3 | 3 |
| `client.systemdiagnostics_api()` | System diagnostics | 2 | 2 | 2 |
| `client.tenants_api()` | Tenants | 11 | 11 | 11 |
| `client.versions_api()` | Versions | 14 | 14 | 14 |

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
- `NifiError::UnsupportedEndpoint { endpoint, version }` — dynamic mode: endpoint not available
- `NifiError::UnsupportedEnumVariant { variant, type_name, version }` — dynamic mode: enum variant not in target version
- `NifiError::MissingRequiredField { field, type_name, version }` — dynamic mode: required field is `None` in request body

Helper methods:

- `err.status_code()` — returns `Option<u16>` for API error variants
- `err.is_retryable()` — returns `true` for transient errors (408, 429, 5xx, network errors)

### Dynamic mode version coverage

<!-- markdownlint-disable MD033 -->

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
