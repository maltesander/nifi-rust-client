# nifi-rust-client

> A Rust client and CLI for the Apache NiFi 2.x REST API.

[![CI](https://github.com/maltesander/nifi-rust-client/actions/workflows/ci.yml/badge.svg)](https://github.com/maltesander/nifi-rust-client/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/nifi-rust-client.svg)](https://crates.io/crates/nifi-rust-client)
[![Docs.rs](https://docs.rs/nifi-rust-client/badge.svg)](https://docs.rs/nifi-rust-client)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://github.com/maltesander/nifi-rust-client/blob/main/LICENSE)
![MSRV: 1.88](https://img.shields.io/badge/MSRV-1.88-blue.svg)

## Crates

| Crate | Purpose |
|---|---|
| [`nifi-rust-client`](crates/nifi-rust-client/) | Library — use this in your application. |
| [`nifictl`](crates/nifictl/) | CLI — use this for ops and scripting. |
| [`nifi-openapi-gen`](crates/nifi-openapi-gen/) | Internal code generator — used as a build-dependency. |

## Supported NiFi Versions

<!-- SUPPORTED_VERSIONS_START -->
| NiFi Version | Feature flag | Endpoints | Types | Changes |
|---|---|---|---|---|
| 2.9.0 | `nifi-2-9-0` | 352 | 437 | +35 endpoints, +32 types, +10 fields vs 2.8.0 |
| 2.8.0 | `nifi-2-8-0` | 317 | 405 | +2 fields, +2 enum values vs 2.7.2 |
| 2.7.2 | `nifi-2-7-2` | 317 | 405 | +17 endpoints, +10 types, +8 fields vs 2.6.0 |
| 2.6.0 | `nifi-2-6-0` | 300 | 395 | — |
<!-- SUPPORTED_VERSIONS_END -->

Full changelog: [NIFI_API_CHANGES.md](NIFI_API_CHANGES.md)

## Quick start (library)

See [`crates/nifi-rust-client/README.md`](crates/nifi-rust-client/README.md).

## Quick start (CLI)

See [`crates/nifictl/README.md`](crates/nifictl/README.md).

## Contributing

- Architecture and conventions: [AGENTS.md](AGENTS.md)
- Adding a new NiFi version: see "Adding or bumping a NiFi version" in [AGENTS.md](AGENTS.md)
- NiFi API changes log: [NIFI_API_CHANGES.md](NIFI_API_CHANGES.md)

## Development

This repo uses [pre-commit](https://pre-commit.com) with both pre-commit
and pre-push stages. After cloning or pulling changes that modify hook
stages, re-install hooks:

```bash
pre-commit install --hook-type pre-commit --hook-type pre-push --hook-type pre-merge-commit
```

## License

Apache-2.0. See [`LICENSE`](LICENSE).
