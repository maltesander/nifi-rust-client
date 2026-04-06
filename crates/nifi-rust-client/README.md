# nifi-rust-client

Idiomatic Rust client for the Apache NiFi 2.x REST API.

All 237 NiFi 2.8.0 REST API endpoints are generated from the OpenAPI spec and exposed via typed resource accessor methods.

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
return `NifiError::Api { status: 401, .. }`. Re-call `login()` to obtain a fresh token:

```rust
use nifi_rust_client::NifiError;

match client.flow_api().get_about_info().await {
    Err(NifiError::Api { status: 401, .. }) => {
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
- `NifiError::Json { source }` — response deserialization failed
- `NifiError::InvalidUrl { source }` — bad base URL passed to the builder

## Integration test coverage

The following API groups have CRUD lifecycle tests (create → get → update → delete → verify gone):

| File | Groups covered |
|------|---------------|
| `tests/tests/processgroups.rs` | process groups, processors (with `includeDescendantGroups`) |
| `tests/tests/labels.rs` | labels |
| `tests/tests/funnels.rs` | funnels |
| `tests/tests/inputports.rs` | input ports |
| `tests/tests/outputports.rs` | output ports |
| `tests/tests/parametercontexts.rs` | parameter contexts |
| `tests/tests/reportingtasks.rs` | reporting tasks (type discovered at runtime) |
| `tests/tests/tenants.rs` | users, user groups |
| `tests/tests/readonly.rs` | system diagnostics, resources, site-to-site, counters, authentication (smoke only) |

The following API groups are **not yet covered** due to requiring complex setup beyond a fresh NiFi instance:

| Group | Reason |
|-------|--------|
| `connections` | Requires source + destination ports to already exist and be connected |
| `flowfilequeues` | Requires active connections with queued flow files |
| `remoteprocessgroups` | Requires a reachable remote NiFi URL |
| `datatransfer` | Requires active site-to-site input/output ports |
| `provenance` / `provenanceevents` | Requires processors to have run and produced provenance events |
| `versions` | Requires a connected flow registry |
| `policies` | Depends on the specific authorization model of the test instance |
| `parameterproviders` | Requires a parameter provider extension class to be installed |
| `snippets` | Requires existing canvas components to copy |
