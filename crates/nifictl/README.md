# nifictl

[![CI](https://github.com/maltesander/nifi-rust-client/actions/workflows/ci.yml/badge.svg)](https://github.com/maltesander/nifi-rust-client/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/nifictl.svg)](https://crates.io/crates/nifictl)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://github.com/maltesander/nifictl/blob/main/LICENSE)
![MSRV: 1.88](https://img.shields.io/badge/MSRV-1.88-blue.svg)

CLI tool for [Apache NiFi](https://nifi.apache.org) 2.x, built on
[nifi-rust-client](../nifi-rust-client/).

Every NiFi REST API endpoint is available as a generated subcommand, plus
hand-written porcelain for common operator workflows.

## Installation

```bash
cargo install --path crates/nifictl
```

## Quick start

```bash
# Configure a context
mkdir -p ~/.nifictl
cat > ~/.nifictl/config.toml <<'EOF'
current_context = "local"

[[contexts]]
name = "local"
url = "https://localhost:8443"
insecure = true

[contexts.auth]
type = "password"
username = "admin"
password_env = "NIFI_PASSWORD"
EOF

# Authenticate
export NIFI_PASSWORD=adminpassword123
nifictl login

# Check status
nifictl status

# Get a specific processor (subcommand names are derived from the generated
# function names — the same names used in the nifi-rust-client Rust API)
nifictl processors get-processor <processor-id>

# Use JSON output
nifictl processors get-processor <id> -o json

# Pipe to jq
nifictl processors get-processor <id> | jq '.name'
```

## Configuration

Config file: `~/.nifictl/config.toml` (override with `--config` or `NIFICTL_CONFIG`).

### Auth types

**Password:**

```toml
[contexts.auth]
type = "password"
username = "admin"
password_env = "NIFI_PASSWORD"     # reads env var at runtime
```

**Token (pre-obtained JWT):**

```toml
[contexts.auth]
type = "token"
token_env = "NIFI_TOKEN"
```

**mTLS:**

```toml
[contexts.auth]
type = "mtls"
client_identity_path = "/path/to/identity.pem"
```

### Context management

```bash
nifictl config view               # show config
nifictl config contexts           # list contexts
nifictl config use-context prod   # switch context
nifictl config delete-context old # remove context
```

### Precedence

CLI flags > environment variables > active context > defaults.

## Output formats

| Flag | Format | Default when |
|------|--------|-------------|
| `-o auto` | Table (TTY) or JSON (piped) | always |
| `-o json` | Pretty JSON | |
| `-o json-compact` | Compact JSON / NDJSON for lists | |
| `-o yaml` | YAML | |
| `-o table` | Table | |
| `-o raw` | Raw JSON passthrough | |

## Shell completions

```bash
nifictl completions bash > ~/.bash_completion.d/nifictl
nifictl completions zsh > ~/.zfunc/_nifictl
nifictl completions fish > ~/.config/fish/completions/nifictl.fish
```

## Waiting for state transitions (`--wait`)

Some state-changing generated commands have a corresponding poll helper
in `nifi-rust-client`'s `wait::` module. Passing `--wait` on these
commands polls the target resource after the initial request and returns
the final entity when it converges to the requested state.

| Command | Waits for |
|---------|-----------|
| `processors update-run-status <id> --wait` | processor state matches the body's `state` (RUNNING, STOPPED, or DISABLED) |
| `controller-services update-run-status <id> --wait` | service state matches the body's `state` (ENABLED or DISABLED) |
| `parameter-contexts submit-parameter-context-update <context-id> --wait` | update request reports `complete: true` (polls the request id returned by submit; trailing DELETE is best-effort) |
| `provenance submit-provenance-request --wait` | query reports `finished: true` (polls the query id returned by submit; trailing DELETE is best-effort) |

Default timeout is 30s; override with `--wait-timeout=<duration>`
(e.g. `--wait-timeout=2m`, `--wait-timeout=500ms`).

Transient target states (`RUN_ONCE` for processors, `ENABLING`/`DISABLING`
for controller services) are rejected with a clear error — there is no
steady state for `--wait` to converge on.

Example:

````bash
nifictl processors update-run-status proc-1 \
    --body '{"state":"RUNNING"}' \
    --wait --wait-timeout=60s
````

## Porcelain commands

| Command | Description |
|---------|-------------|
| `nifictl login` | Authenticate and cache token |
| `nifictl logout` | Clear cached token |
| `nifictl status` | NiFi version and cluster info |

### Operator bulk commands (`ops`)

Thin wrappers over `bulk::*_dynamic` in `nifi-rust-client`. Each command
applies an all-or-nothing state transition across a process group.

| Command | NiFi endpoint |
|---------|---------------|
| `nifictl ops start-pg <pg-id>` | `PUT /flow/process-groups/{id}` (state: RUNNING) |
| `nifictl ops stop-pg <pg-id>` | `PUT /flow/process-groups/{id}` (state: STOPPED) |
| `nifictl ops enable-services <pg-id>` | `PUT /flow/process-groups/{id}/controller-services` (state: ENABLED) |
| `nifictl ops disable-services <pg-id>` | `PUT /flow/process-groups/{id}/controller-services` (state: DISABLED) |

The `--wait` flag is **not yet supported** on `ops` subcommands (a
client-side process-group wait helper is needed first). These commands
return immediately after the server acknowledges the state-change
request.

## Generated commands

Every NiFi API tag is a top-level subcommand. Command names are derived
directly from the generated Rust function names (kebab-cased), so the CLI
surface is 1:1 with `nifi-rust-client`'s public API:

```bash
nifictl processors get-processor <id>
nifictl processors delete-processor <id>
nifictl processors update-processor <id> --body-file processor.json
nifictl processgroups get-process-groups <parent-id>
nifictl flow search-cluster --q "my processor"
nifictl controller_services get-controller-service <id>
```

> **Phase 5 change:** subcommand names were previously CRUD verbs (`get`,
> `list`, `config-get`, `run-status-put`, etc.). They are now the exact
> flat-API function names with dashes, which lines up with the Rust API and
> makes the NiFi REST docs trivially discoverable. Use `nifictl <resource>
> --help` to list the available commands for any resource.

## Multi-version support

By default, nifictl compiles in dynamic mode (all supported NiFi versions).
Version detection happens automatically on login. For a leaner binary targeting
a single version:

```bash
cargo install --path crates/nifictl --no-default-features --features nifi-2-9-0
```
