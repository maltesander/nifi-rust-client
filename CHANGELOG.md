# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-04-08

### Added

- Add provenance + flowfile-queue integration tests; fix generator for 202 responses and optional entity fields ([60985b3](https://github.com/maltesander/nifi-rust-client/commit/60985b3))

### Changed

- Remove redundant section about client modes ([e22fb43](https://github.com/maltesander/nifi-rust-client/commit/e22fb43))

### Fixed

- Clippy warnings and dynamic conversions for optional entity fields ([15da24e](https://github.com/maltesander/nifi-rust-client/commit/15da24e))

## [0.2.1] - 2026-04-07

### Added

- Multi-line docs, skip_serializing for readOnly DTO fields ([40ed6c8](https://github.com/maltesander/nifi-rust-client/commit/40ed6c8))
- Parse readOnly field flag from OpenAPI spec ([2e7e54b](https://github.com/maltesander/nifi-rust-client/commit/2e7e54b))
- Emit # Errors and # Permissions doc sections ([77e6146](https://github.com/maltesander/nifi-rust-client/commit/77e6146))
- Parse error_responses and security from OpenAPI spec ([be95660](https://github.com/maltesander/nifi-rust-client/commit/be95660))

### Changed

- Auto-sync versions table, client README, and docker-compose default ([29cd40c](https://github.com/maltesander/nifi-rust-client/commit/29cd40c))
- Fix format ([6b266b5](https://github.com/maltesander/nifi-rust-client/commit/6b266b5))
- Fix formatting ([9508670](https://github.com/maltesander/nifi-rust-client/commit/9508670))
- Regenerate with enriched doc comments and readOnly support ([7e8d9f5](https://github.com/maltesander/nifi-rust-client/commit/7e8d9f5))

### Fixed

- Remove skip_serializing from readOnly DTO fields ([0dbe851](https://github.com/maltesander/nifi-rust-client/commit/0dbe851))
- Remove stale version dirs when a NiFi version is dropped ([2b04aa2](https://github.com/maltesander/nifi-rust-client/commit/2b04aa2))

### Documentation

- Reorder README sections for better first-impression flow ([96e0304](https://github.com/maltesander/nifi-rust-client/commit/96e0304))
- Update READMEs for error variants, test coverage, and codegen features ([3081482](https://github.com/maltesander/nifi-rust-client/commit/3081482))

## [0.2.0] - 2026-04-07

### Added

- Make commit hashes clickable GitHub links in changelog ([b7a5cd0](https://github.com/maltesander/nifi-rust-client/commit/b7a5cd0))
- Add short commit hash to changelog entries ([31275b0](https://github.com/maltesander/nifi-rust-client/commit/31275b0))
- Add release automation script ([5c37aec](https://github.com/maltesander/nifi-rust-client/commit/5c37aec))
- Add NiFi 2.6.0 support and fix clippy/fmt warnings ([c6d5246](https://github.com/maltesander/nifi-rust-client/commit/c6d5246))
- Add build_dynamic() to NifiClientBuilder ([7fd3a9d](https://github.com/maltesander/nifi-rust-client/commit/7fd3a9d))
- Wire dynamic emitters into generator and generate dynamic module ([6a8feed](https://github.com/maltesander/nifi-rust-client/commit/6a8feed))
- Add dynamic wiremock test emitter ([e1c8630](https://github.com/maltesander/nifi-rust-client/commit/e1c8630))
- Add dynamic dispatch emitter for runtime version selection ([be16a95](https://github.com/maltesander/nifi-rust-client/commit/be16a95))
- Add From impl emitter for dynamic type conversions ([23ac996](https://github.com/maltesander/nifi-rust-client/commit/23ac996))
- Add common union type emitter for dynamic mode ([05da7c3](https://github.com/maltesander/nifi-rust-client/commit/05da7c3))
- Add dynamic feature to Cargo.toml patching ([6e6025f](https://github.com/maltesander/nifi-rust-client/commit/6e6025f))
- Update lib.rs generation for dynamic feature support ([e1a294e](https://github.com/maltesander/nifi-rust-client/commit/e1a294e))
- Add UnsupportedVersion and UnsupportedEndpoint error variants ([601d5af](https://github.com/maltesander/nifi-rust-client/commit/601d5af))

### Changed

- Sort Cargo.toml features alphabetically ([cad3890](https://github.com/maltesander/nifi-rust-client/commit/cad3890))

### Fixed

- Add --allow-dirty to cargo publish dry-run ([1300c54](https://github.com/maltesander/nifi-rust-client/commit/1300c54))
- Add blank lines around headings in changelog output ([5e65a7f](https://github.com/maltesander/nifi-rust-client/commit/5e65a7f))
- Revert rust-cache to v2 (v3 does not exist yet) ([c7f9240](https://github.com/maltesander/nifi-rust-client/commit/c7f9240))
- Bump actions to Node.js 22 compatible versions ([2c81d49](https://github.com/maltesander/nifi-rust-client/commit/2c81d49))
- Handle None stdout when capture=False in run() ([f24f42d](https://github.com/maltesander/nifi-rust-client/commit/f24f42d))
- Consolidate pre-commit hooks, fix clippy warnings in generated code ([fde3307](https://github.com/maltesander/nifi-rust-client/commit/fde3307))
- Handle differing query params across versions in dynamic dispatch ([a9b0e69](https://github.com/maltesander/nifi-rust-client/commit/a9b0e69))
- Add @generated header to generated test files ([394e067](https://github.com/maltesander/nifi-rust-client/commit/394e067))
- Ensure all generated .rs files have @generated header ([a12252b](https://github.com/maltesander/nifi-rust-client/commit/a12252b))
- Gate per-version tests with not(dynamic), fix DynamicClient Debug/from_client ([0b9acca](https://github.com/maltesander/nifi-rust-client/commit/0b9acca))
- Gate per-version tests with not(feature = "dynamic") ([46c3306](https://github.com/maltesander/nifi-rust-client/commit/46c3306))

### Documentation

- Add supported NiFi versions section to root README ([f3f18cd](https://github.com/maltesander/nifi-rust-client/commit/f3f18cd))
- Update root README for static and dynamic modes ([5bb8d01](https://github.com/maltesander/nifi-rust-client/commit/5bb8d01))
- Set docs.rs to build with dynamic feature for full API docs ([81d3447](https://github.com/maltesander/nifi-rust-client/commit/81d3447))
- Document static vs dynamic modes, update AGENTS.md ([6c06a97](https://github.com/maltesander/nifi-rust-client/commit/6c06a97))

### Tests

- Add query param dispatch tests for version-differing endpoints ([593a60a](https://github.com/maltesander/nifi-rust-client/commit/593a60a))
- Add integration tests for dynamic client ([bfbfcbc](https://github.com/maltesander/nifi-rust-client/commit/bfbfcbc))
- Add wiremock tests for dynamic client API dispatch ([9a02a83](https://github.com/maltesander/nifi-rust-client/commit/9a02a83))

### Other

- Add dynamic feature testing to CI and release workflows ([67f19fb](https://github.com/maltesander/nifi-rust-client/commit/67f19fb))

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

[Unreleased]: https://github.com/maltesander/nifi-rust-client/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/maltesander/nifi-rust-client/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/maltesander/nifi-rust-client/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/maltesander/nifi-rust-client/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/maltesander/nifi-rust-client/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/maltesander/nifi-rust-client/releases/tag/v0.1.0
