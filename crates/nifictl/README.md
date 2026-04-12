# nifictl

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

# List processors in root process group
nifictl processors list

# Get a specific processor
nifictl processors get <processor-id>

# Use JSON output
nifictl processors get <id> -o json

# Pipe to jq
nifictl processors get <id> | jq '.name'
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

## Porcelain commands

| Command | Description |
|---------|-------------|
| `nifictl login` | Authenticate and cache token |
| `nifictl logout` | Clear cached token |
| `nifictl status` | NiFi version and cluster info |

## Generated commands

Every NiFi API tag is a top-level subcommand with CRUD verbs:

```bash
nifictl processors get <id>
nifictl processors delete <id>
nifictl processors update <id> --body-file processor.json
nifictl process-groups list
nifictl flow search --q "my processor"
nifictl controller-services get <id>
```

Use `nifictl <resource> --help` to see available commands.

## Multi-version support

By default, nifictl compiles in dynamic mode (all supported NiFi versions).
Version detection happens automatically on login. For a leaner binary targeting
a single version:

```bash
cargo install --path crates/nifictl --no-default-features --features nifi-2-9-0
```
