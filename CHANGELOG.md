# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-04-06

### Fixed

- **docs.rs build** — use explicit feature instead of `all-features` to avoid mutually exclusive version conflict.
- **CI workflows** — replace duplicated cargo steps with `pre-commit/action` for consistent checks.
- **rustfmt warnings** — remove nightly-only options from `rustfmt.toml`, keep `style_edition = "2024"`.
- **pre-commit openapi-gen-check** — fix hook naming and `git diff` path separator.
- **CI cargo-deny** — install `cargo-deny` in CI before running pre-commit.
- **EOF newlines** — fix missing trailing newlines in `CLAUDE.md` and `nifi-api.json`.

## [0.1.0] - 2026-04-06

### Added

- **NiFi 2.8.0 support** — full generated API client covering all 237 REST endpoints across 30+ resource groups (flow, processors, process groups, connections, controller services, parameter contexts, tenants, access, reporting tasks, and more).
- **NiFi 2.7.2 support** — same full API surface via the `nifi-2-7-2` Cargo feature flag.
- **Multi-version feature flags** — select NiFi version at compile time; default is the latest (`nifi-2-8-0`).
- **`NifiClient`** — async HTTP client with JWT authentication (`login`, `logout`, `token`, `set_token`), configurable base URL, and `rustls`-backed TLS.
- **`NifiClientBuilder`** — fluent builder for constructing a `NifiClient`.
- **Resource API structs** — `FlowApi`, `ProcessorsApi`, `ProcessGroupsApi`, `ConnectionsApi`, `ControllerServicesApi`, `AccessApi`, `TenantsApi`, `ParameterContextsApi`, `ReportingTasksApi`, and 20+ others, each mirroring NiFi's own API grouping.
- **Typed enum query parameters** — OpenAPI `enum` query params are emitted as Rust enums with `Display` and `serde` impls instead of raw strings.
- **`nifi-openapi-gen`** — internal code generator that parses NiFi OpenAPI specs and emits idiomatic Rust types, API structs, and wiremock test stubs. Run `cargo run -p nifi-openapi-gen` to regenerate.
- **Wiremock test suite** — generated and hand-written wiremock tests covering every API group; run with `cargo test -p nifi-rust-client` (no Docker required).
- **Integration test suite** — Docker-based integration tests against a live NiFi instance; run with `./tests/run.sh`.
- **Structured error handling** — `NifiError` via `snafu` with distinct variants for HTTP, auth, serialization, and network errors.
- **Tracing** — all HTTP requests emit a `tracing::debug!` event with method and path before sending.

[Unreleased]: https://github.com/maltesander/nifi-rust-client/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/maltesander/nifi-rust-client/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/maltesander/nifi-rust-client/releases/tag/v0.1.0
