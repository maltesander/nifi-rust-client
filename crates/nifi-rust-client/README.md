# nifi-rust-client

Idiomatic Rust client for the Apache NiFi 2.x REST API.

All NiFi REST API endpoints are generated from the official OpenAPI spec and exposed via typed resource accessor methods.

## Supported NiFi Versions

<!-- SUPPORTED_VERSIONS_START -->
| NiFi Version | Feature flag | Endpoints | Types | Changes | Default |
|---|---|---|---|---|---|
| 2.8.0 | `nifi-2-8-0` | 317 | 405 | no API changes vs 2.7.2 | ✓ |
| 2.7.2 | `nifi-2-7-2` | 317 | 405 | +17 endpoints, +10 types vs 2.6.0 |  |
| 2.6.0 | `nifi-2-6-0` | 300 | 395 | — |  |
<!-- SUPPORTED_VERSIONS_END -->

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
let mut client = NifiClientBuilder::new("https://nifi:8443")?.build()?;
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
let mut client = NifiClientBuilder::new("https://nifi:8443")?
    .build_dynamic()
    .await?;
client.login("admin", "password").await?;

// Returns common union types — fields are Option<T>
let about = client.flow_api().get_about_info().await?;
println!("NiFi version: {:?}", about.version);

// Check what was detected
println!("Connected to NiFi {}", client.detected_version());
```

Trade-offs: all fields are `Option<T>` since not every NiFi version populates every field. Endpoints that don't exist in the connected version return `NifiError::UnsupportedEndpoint`.

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

let mut client = NifiClientBuilder::new("https://nifi.example.com:8443")?
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
| `.build()` | — | Returns `Result<NifiClient, NifiError>` |

## Authentication

`login()` posts credentials to `/nifi-api/access/token` and stores the returned JWT on the client. The token is sent as `Authorization: Bearer <token>` on every subsequent request.

### Token management

After a successful `login()`, the JWT can be persisted and reused across process restarts:

```rust
// Save token after login
if let Some(token) = client.token() {
    fs::write("nifi-token.txt", token)?;
}

// Restore on the next run instead of re-authenticating
let token = fs::read_to_string("nifi-token.txt")?;
client.set_token(token);
```

NiFi JWTs expire after **12 hours** by default (configurable server-side via
`nifi.security.user.login.identity.provider.expiration`). An expired token causes any API call to
return `NifiError::Unauthorized { .. }`. Re-call `login()` to obtain a fresh token:

```rust
use nifi_rust_client::NifiError;

match client.flow_api().get_about_info().await {
    Err(NifiError::Unauthorized { .. }) => {
        client.login("admin", "password").await?;
        // retry the original call
    }
    other => { other?; }
}
```

Automatic re-login is not built in because storing credentials on the client would keep plaintext passwords in memory for the lifetime of the process.

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
| `client.logout()` | `DELETE /access/logout` — invalidate server-side and clear local token |
| `client.token()` | Return the current bearer token as `Option<&str>` |
| `client.set_token(token)` | Restore a previously obtained token |

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

- `NifiError::Api { status, message }` — NiFi returned a non-2xx response
- `NifiError::Http { source }` — network or transport error
- `NifiError::Auth { message }` — authentication failed (e.g. bad credentials)
- `NifiError::InvalidBaseUrl { source }` — bad base URL passed to the builder
- `NifiError::InvalidCertificate { source }` — invalid CA certificate passed to the builder
- `NifiError::UnsupportedVersion { detected }` — dynamic mode detected a NiFi version not compiled in
- `NifiError::UnsupportedEndpoint { endpoint, version }` — dynamic mode: endpoint not available in the connected NiFi version

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
