<!-- markdownlint-disable MD033 -->
# Changelog

All notable changes to `nifi-rust-client` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> Prior to v0.5.0, this file also tracked `nifi-openapi-gen` entries.
> Starting with the next release, `nifi-openapi-gen` has its own changelog at
> [`crates/nifi-openapi-gen/CHANGELOG.md`](../nifi-openapi-gen/CHANGELOG.md)
> and an independent release cadence.

## [Unreleased]

## [0.9.0] - 2026-04-12

## [0.8.0] - 2026-04-12

### Added

- Auto-update lib.rs doc comment version examples ([a133550](https://github.com/maltesander/nifi-rust-client/commit/a133550))
- Add AuthProvider trait replacing CredentialProvider ([3cedb00](https://github.com/maltesander/nifi-rust-client/commit/3cedb00))
- Add tracing::instrument spans to all HTTP helpers ([755a599](https://github.com/maltesander/nifi-rust-client/commit/755a599))
- Add flow_history_dynamic paginator constructor ([100b867](https://github.com/maltesander/nifi-rust-client/commit/100b867))
- Add flow_history paginator constructor for static mode ([0d2930d](https://github.com/maltesander/nifi-rust-client/commit/0d2930d))
- Add HistoryPaginator::next item-level iterator ([5964411](https://github.com/maltesander/nifi-rust-client/commit/5964411))
- Implement HistoryPaginator::next_page ([f9db99f](https://github.com/maltesander/nifi-rust-client/commit/f9db99f))
- Add pagination module skeleton ([66a1ee3](https://github.com/maltesander/nifi-rust-client/commit/66a1ee3))
- Add require! macro for nested Option field extraction ([276fdf3](https://github.com/maltesander/nifi-rust-client/commit/276fdf3))
- Wire RequireField trait into crate root ([8edb8d5](https://github.com/maltesander/nifi-rust-client/commit/8edb8d5))
- Add RequireField trait for Option<T> ([c9dd1fa](https://github.com/maltesander/nifi-rust-client/commit/c9dd1fa))
- Add NifiError::MissingField variant for require helper ([372f912](https://github.com/maltesander/nifi-rust-client/commit/372f912))

### Changed

- Delete credentials.rs, replaced by auth.rs ([6d21697](https://github.com/maltesander/nifi-rust-client/commit/6d21697))

### Fixed

- Add Send bound to BoxedFetchFuture and speed up overflow test ([47c73bb](https://github.com/maltesander/nifi-rust-client/commit/47c73bb))

### Documentation

- Document cluster/load balancer token handling ([0655e61](https://github.com/maltesander/nifi-rust-client/commit/0655e61))
- Fix stale version number in lib.rs doc examples ([efcfe12](https://github.com/maltesander/nifi-rust-client/commit/efcfe12))
- Update README and AGENTS.md for AuthProvider migration ([ac51418](https://github.com/maltesander/nifi-rust-client/commit/ac51418))
- Update credentials_and_retry example to use AuthProvider ([78eb18a](https://github.com/maltesander/nifi-rust-client/commit/78eb18a))
- Document flow_history paginator ([9a6b528](https://github.com/maltesander/nifi-rust-client/commit/9a6b528))
- Fix RequireField example to use real field chain ([00461bd](https://github.com/maltesander/nifi-rust-client/commit/00461bd))
- Document RequireField and require! helpers ([04a1b13](https://github.com/maltesander/nifi-rust-client/commit/04a1b13))

### Tests

- Add wiremock tests for X-ProxiedEntitiesChain header ([53b18d6](https://github.com/maltesander/nifi-rust-client/commit/53b18d6))
- Migrate tests from CredentialProvider to AuthProvider ([7be464e](https://github.com/maltesander/nifi-rust-client/commit/7be464e))
- Wiremock coverage for flow_history paginators ([63f9144](https://github.com/maltesander/nifi-rust-client/commit/63f9144))
- Wiremock coverage for flow_history paginators ([fc78dbd](https://github.com/maltesander/nifi-rust-client/commit/fc78dbd))

## [0.7.0] - 2026-04-12

### Added

- Emit text/bytes returns for non-JSON response endpoints ([d98f8f7](https://github.com/maltesander/nifi-rust-client/commit/d98f8f7))
- Add post_multipart helper and wiremock test ([fc6871f](https://github.com/maltesander/nifi-rust-client/commit/fc6871f))
- Recognize multipart/form-data and */* request bodies ([39ee9cd](https://github.com/maltesander/nifi-rust-client/commit/39ee9cd))
- Compact enum-variant diffs in NIFI_API_CHANGES ([3604ce0](https://github.com/maltesander/nifi-rust-client/commit/3604ce0))
- Surface breaking-change marker and tested column in README ([3105c97](https://github.com/maltesander/nifi-rust-client/commit/3105c97))

### Changed

- Lazy version detection on dynamic dispatch ([422ff49](https://github.com/maltesander/nifi-rust-client/commit/422ff49))

### Fixed

- Unwrap detected_version Option in integration test ([248ac94](https://github.com/maltesander/nifi-rust-client/commit/248ac94))

## [0.6.0] - 2026-04-11

### Added

- Add NiFi 2.9.0 support (265 paths, up from 237) ([c9edd27](https://github.com/maltesander/nifi-rust-client/commit/c9edd27))

### Changed

- Eliminate hardcoded latest version from tests and scripts ([26493d3](https://github.com/maltesander/nifi-rust-client/commit/26493d3))

### Fixed

- Stable method names across NiFi versions via path-based grouping ([a058972](https://github.com/maltesander/nifi-rust-client/commit/a058972))

## [0.5.0] - 2026-04-10

### Breaking Changes

- Mark VersionResolutionStrategy as non_exhaustive ([ae272f0](https://github.com/maltesander/nifi-rust-client/commit/ae272f0))
- Mark NifiError as non_exhaustive ([98af77b](https://github.com/maltesander/nifi-rust-client/commit/98af77b))

### Added

- Update feature flags table in crate doc on version bump ([90f069b](https://github.com/maltesander/nifi-rust-client/commit/90f069b))
- Add instrument spans on retry wrappers ([210566e](https://github.com/maltesander/nifi-rust-client/commit/210566e))
- Add compile_error guard for zero-version-feature builds ([69354c5](https://github.com/maltesander/nifi-rust-client/commit/69354c5))

### Changed

- Return Result from integration-test build.rs ([c58c3f6](https://github.com/maltesander/nifi-rust-client/commit/c58c3f6))
- Return Result from build.rs, guard zero-features config ([9844213](https://github.com/maltesander/nifi-rust-client/commit/9844213))

### Fixed

- Remove #![deny(missing_docs)] from strategy.rs and document variants ([e138746](https://github.com/maltesander/nifi-rust-client/commit/e138746))
- Move #![deny(missing_docs)] to per-file to fix cargo package verification ([0f541cd](https://github.com/maltesander/nifi-rust-client/commit/0f541cd))
- Rewrite let chains to support MSRV 1.85 ([2338f7b](https://github.com/maltesander/nifi-rust-client/commit/2338f7b))
- Rewrite let chain to support MSRV 1.85 ([9d79689](https://github.com/maltesander/nifi-rust-client/commit/9d79689))
- Guard static examples from dynamic feature compilation ([cd79f0f](https://github.com/maltesander/nifi-rust-client/commit/cd79f0f))
- Source version list from specs dir, not stale src/v*/ layout ([f516060](https://github.com/maltesander/nifi-rust-client/commit/f516060))

### Documentation

- Enforce #![deny(missing_docs)] and backfill public items ([5328866](https://github.com/maltesander/nifi-rust-client/commit/5328866))
- Add dynamic_autodetect example ([b853563](https://github.com/maltesander/nifi-rust-client/commit/b853563))
- Add basic_static quickstart example ([5059a57](https://github.com/maltesander/nifi-rust-client/commit/5059a57))
- Add custom_tls example ([3682cd8](https://github.com/maltesander/nifi-rust-client/commit/3682cd8))
- Add credentials_and_retry example ([babe0a9](https://github.com/maltesander/nifi-rust-client/commit/babe0a9))
- Add crate-level //! doc with overview and entry points ([b2a6b50](https://github.com/maltesander/nifi-rust-client/commit/b2a6b50))
- Document the build-dep stability contract ([0e5ca0f](https://github.com/maltesander/nifi-rust-client/commit/0e5ca0f))
- Hide nifi-openapi-gen internals from docs.rs via doc(hidden) ([9a37548](https://github.com/maltesander/nifi-rust-client/commit/9a37548))
- Document the stability contract ([b93497c](https://github.com/maltesander/nifi-rust-client/commit/b93497c))
- Avoid unwrap() in NifiClientBuilder example ([6b69995](https://github.com/maltesander/nifi-rust-client/commit/6b69995))

### Tests

- Add version-feature-gated test pattern ([164475d](https://github.com/maltesander/nifi-rust-client/commit/164475d))
- Cover listen-ports and flow-registry-client-definition ([ec8617e](https://github.com/maltesander/nifi-rust-client/commit/ec8617e))
- Fix create_port_transaction mock status to 201 ([9817799](https://github.com/maltesander/nifi-rust-client/commit/9817799))
- Add wiremock coverage for DataTransfer API ([54d3714](https://github.com/maltesander/nifi-rust-client/commit/54d3714))
- Add wiremock coverage for ParameterProviders API ([156196f](https://github.com/maltesander/nifi-rust-client/commit/156196f))

## [0.4.2] - 2026-04-10

### Fixed

- Bundle OpenAPI specs in nifi-openapi-gen so downstream builds work ([9dba3ec](https://github.com/maltesander/nifi-rust-client/commit/9dba3ec))

## [0.4.1] - 2026-04-10

### Added

- Add build.rs to integration test crate ([6ccfd40](https://github.com/maltesander/nifi-rust-client/commit/6ccfd40))
- Add build.rs to nifi-rust-client for build-time code generation ([4f03247](https://github.com/maltesander/nifi-rust-client/commit/4f03247))
- Add build_api module for build.rs callers ([3dfcad1](https://github.com/maltesander/nifi-rust-client/commit/3dfcad1))
- Add field presence integration tests and coverage tracking ([0394b5c](https://github.com/maltesander/nifi-rust-client/commit/0394b5c))

### Changed

- Move credentials and retry into config/ module ([4bfed11](https://github.com/maltesander/nifi-rust-client/commit/4bfed11))
- Remove code generation from binary, now handled by build.rs ([115a742](https://github.com/maltesander/nifi-rust-client/commit/115a742))
- Compact conversion emitter using impl_from! macro ([1473ad2](https://github.com/maltesander/nifi-rust-client/commit/1473ad2))

### Fixed

- Use correct docs.rs URL for nifi-openapi-gen ([2907943](https://github.com/maltesander/nifi-rust-client/commit/2907943))
- Use separate NiFi instances for static and dynamic tests ([6a4e354](https://github.com/maltesander/nifi-rust-client/commit/6a4e354))

### Documentation

- Update for build-time code generation architecture ([5bf36dd](https://github.com/maltesander/nifi-rust-client/commit/5bf36dd))

## [0.4.0] - 2026-04-09

### Breaking Changes

- Trait-based dynamic client with enum dispatch ([8bfcd9b](https://github.com/maltesander/nifi-rust-client/commit/8bfcd9b))
- Improve dynamic client UX with typed enums, reduced Options, and typed bodies ([f672c82](https://github.com/maltesander/nifi-rust-client/commit/f672c82))
- Add interior mutability for token + auto-retry on 401 ([cb8ca6a](https://github.com/maltesander/nifi-rust-client/commit/cb8ca6a))
- Add typed error variants for 401/403/404/409 ([5b4a47a](https://github.com/maltesander/nifi-rust-client/commit/5b4a47a))

### Added

- Add version_strategy() to NifiClientBuilder ([a022f50](https://github.com/maltesander/nifi-rust-client/commit/a022f50))
- Update generator to emit strategy-aware DynamicClient ([babd318](https://github.com/maltesander/nifi-rust-client/commit/babd318))
- Add VersionResolutionStrategy enum and resolve logic ([7c70cb1](https://github.com/maltesander/nifi-rust-client/commit/7c70cb1))
- Integrate resource accessors table into generator pipeline ([a2870ad](https://github.com/maltesander/nifi-rust-client/commit/a2870ad))
- Add resource_accessors doc emitter with tests ([1aba368](https://github.com/maltesander/nifi-rust-client/commit/1aba368))
- Unified trait hierarchy for static and dynamic clients ([ca36539](https://github.com/maltesander/nifi-rust-client/commit/ca36539))
- Emit hierarchical dispatch structs for sub-resource traits ([3034761](https://github.com/maltesander/nifi-rust-client/commit/3034761))
- Wire static traits into generate.rs ([60a64b8](https://github.com/maltesander/nifi-rust-client/commit/60a64b8))
- Emit static trait impls in api.rs ([7bb41ab](https://github.com/maltesander/nifi-rust-client/commit/7bb41ab))
- Add static trait emitter (emit/traits.rs) ([2854b0c](https://github.com/maltesander/nifi-rust-client/commit/2854b0c))
- Add collect_tag_sub_groups utility for hierarchical endpoint grouping ([23d364d](https://github.com/maltesander/nifi-rust-client/commit/23d364d))
- Lazy version detection in DynamicClient ([720bc6a](https://github.com/maltesander/nifi-rust-client/commit/720bc6a))
- Generate dynamic integration tests and README coverage section ([5c0a355](https://github.com/maltesander/nifi-rust-client/commit/5c0a355))
- Add integration coverage section to README ([38d41e9](https://github.com/maltesander/nifi-rust-client/commit/38d41e9))
- Implement query param coverage integration test emitter ([600d6c3](https://github.com/maltesander/nifi-rust-client/commit/600d6c3))
- Implement endpoint availability integration test emitter ([b536e28](https://github.com/maltesander/nifi-rust-client/commit/b536e28))
- Implement field presence integration test emitter (empty mapping) ([2d35fd5](https://github.com/maltesander/nifi-rust-client/commit/2d35fd5))
- Implement enum coverage integration test emitter ([85f9086](https://github.com/maltesander/nifi-rust-client/commit/85f9086))
- Wire integration test generation into generate.rs ([a676cf8](https://github.com/maltesander/nifi-rust-client/commit/a676cf8))
- Add run-matrix.sh for multi-version integration testing ([02243df](https://github.com/maltesander/nifi-rust-client/commit/02243df))
- Scaffold emit/integration module with stubs ([0be46c5](https://github.com/maltesander/nifi-rust-client/commit/0be46c5))
- Add openapi-diff CLI tool for spec comparison ([737e55a](https://github.com/maltesander/nifi-rust-client/commit/737e55a))
- Generate NIFI_API_CHANGES.md changelog with per-version diffs ([5bdee6d](https://github.com/maltesander/nifi-rust-client/commit/5bdee6d))
- Add generate_api_changes_content() and format_diff_body() ([c49bc93](https://github.com/maltesander/nifi-rust-client/commit/c49bc93))
- Use VersionDiff::summary() in README versions table ([8dbdb0c](https://github.com/maltesander/nifi-rust-client/commit/8dbdb0c))
- Export diff module from nifi-openapi-gen lib ([66e9177](https://github.com/maltesander/nifi-rust-client/commit/66e9177))
- Add VersionDiff::summary() for table column ([104b7a3](https://github.com/maltesander/nifi-rust-client/commit/104b7a3))
- Add VersionDiff structs and compute_diff() with unit tests ([eb8c7d2](https://github.com/maltesander/nifi-rust-client/commit/eb8c7d2))
- Add Redacted<T> type to mask passwords in debug output ([d91fcd8](https://github.com/maltesander/nifi-rust-client/commit/d91fcd8))
- Wire up emit_dynamic_traits/dispatch/impls in generate.rs ([610ec56](https://github.com/maltesander/nifi-rust-client/commit/610ec56))
- Add trait emitter for dynamic per-tag API traits ([f15f7f7](https://github.com/maltesander/nifi-rust-client/commit/f15f7f7))
- Add non_exhaustive, doc comments, and skip_serializing_if to dynamic types ([f6787d3](https://github.com/maltesander/nifi-rust-client/commit/f6787d3))
- Add version docs, enum helper, body-by-value, and narrowing tests ([d605eaf](https://github.com/maltesander/nifi-rust-client/commit/d605eaf))
- Add opt-in RetryPolicy for transient error retry with exponential backoff ([1d6ad84](https://github.com/maltesander/nifi-rust-client/commit/1d6ad84))
- Add CredentialProvider trait with Static and Env implementations ([ac70cb2](https://github.com/maltesander/nifi-rust-client/commit/ac70cb2))

### Changed

- Replace GATs with RPITIT in trait hierarchy ([0ae5554](https://github.com/maltesander/nifi-rust-client/commit/0ae5554))
- Update dynamic impls for hierarchical sub-resource traits ([17d51a3](https://github.com/maltesander/nifi-rust-client/commit/17d51a3))
- Emit hierarchical dynamic traits with sub-resource grouping ([d6c7390](https://github.com/maltesander/nifi-rust-client/commit/d6c7390))
- Move integration coverage section to nifi-rust-client README ([9a22717](https://github.com/maltesander/nifi-rust-client/commit/9a22717))
- Disable per-version wiremock stub test generation ([d3249a8](https://github.com/maltesander/nifi-rust-client/commit/d3249a8))
- Extract shared helpers in emit/integration/common ([f6d191f](https://github.com/maltesander/nifi-rust-client/commit/f6d191f))
- Extract dynamic_logged_in_client to shared helpers ([0fab92e](https://github.com/maltesander/nifi-rust-client/commit/0fab92e))
- Remove Default column from versions table, link to API changes ([ea7aed8](https://github.com/maltesander/nifi-rust-client/commit/ea7aed8))
- Namespaced re-exports in lib.rs ([2c00e8e](https://github.com/maltesander/nifi-rust-client/commit/2c00e8e))
- Slim generate.rs to thin orchestrator ([8f938bf](https://github.com/maltesander/nifi-rust-client/commit/8f938bf))
- Extract repo/ module from generate.rs ([0383b7a](https://github.com/maltesander/nifi-rust-client/commit/0383b7a))
- Extract docs/ module from generate.rs ([d1f6007](https://github.com/maltesander/nifi-rust-client/commit/d1f6007))
- Move dynamic emitters into emit/dynamic/ ([29dd9a1](https://github.com/maltesander/nifi-rust-client/commit/29dd9a1))
- Move static emitters into emit/ module ([d178500](https://github.com/maltesander/nifi-rust-client/commit/d178500))
- Add emit/common.rs with shared helpers ([1dfadbd](https://github.com/maltesander/nifi-rust-client/commit/1dfadbd))
- Expand util.rs with shared helpers ([603500f](https://github.com/maltesander/nifi-rust-client/commit/603500f))
- Extract shared utilities into util.rs ([98185e1](https://github.com/maltesander/nifi-rust-client/commit/98185e1))

### Fixed

- Disable MD033 lint in generated CHANGELOG ([34decee](https://github.com/maltesander/nifi-rust-client/commit/34decee))
- Add @generated label to dynamic test emitters ([ee576f2](https://github.com/maltesander/nifi-rust-client/commit/ee576f2))
- Revert tests crate dynamic feature enabling all versions ([941ae67](https://github.com/maltesander/nifi-rust-client/commit/941ae67))
- Update integration test emitters for builder pattern ([d132c55](https://github.com/maltesander/nifi-rust-client/commit/d132c55))
- Update dynamic dispatch tests for builder pattern ([0286f9c](https://github.com/maltesander/nifi-rust-client/commit/0286f9c))
- Add type conversions to sub-resource dispatch methods ([6a50b82](https://github.com/maltesander/nifi-rust-client/commit/6a50b82))
- Only emit positive tests for GET endpoints ([2adeb44](https://github.com/maltesander/nifi-rust-client/commit/2adeb44))
- Use cumulative feature gates for negative integration tests ([5c622b8](https://github.com/maltesander/nifi-rust-client/commit/5c622b8))
- Use by-value Default::default() for dynamic trait body params ([7540482](https://github.com/maltesander/nifi-rust-client/commit/7540482))
- Generate dynamic-mode calls in integration tests ([26f5862](https://github.com/maltesander/nifi-rust-client/commit/26f5862))
- Gate static integration tests with cfg(not(feature = "dynamic")) ([f6e1600](https://github.com/maltesander/nifi-rust-client/commit/f6e1600))
- Wait for DISABLED state before clearing controller service ([f8ccf34](https://github.com/maltesander/nifi-rust-client/commit/f8ccf34))
- Run static and dynamic integration tests separately ([00ab083](https://github.com/maltesander/nifi-rust-client/commit/00ab083))
- Resolve all pre-commit hook failures ([c442f94](https://github.com/maltesander/nifi-rust-client/commit/c442f94))
- Include dynamic feature in integration test runs ([4110a2b](https://github.com/maltesander/nifi-rust-client/commit/4110a2b))
- Add markdownlint-disable for generated NIFI_API_CHANGES.md ([ca75340](https://github.com/maltesander/nifi-rust-client/commit/ca75340))
- Count removed enum values in summary(); update AGENTS.md bump procedure ([fafa0a3](https://github.com/maltesander/nifi-rust-client/commit/fafa0a3))
- Clarify code and documentation generation ([6032c8a](https://github.com/maltesander/nifi-rust-client/commit/6032c8a))
- Add trait imports and fix non_exhaustive struct construction in dynamic tests ([7d54d59](https://github.com/maltesander/nifi-rust-client/commit/7d54d59))
- Remove redundant explicit link target in doc comment ([6fe034a](https://github.com/maltesander/nifi-rust-client/commit/6fe034a))
- Use nifi_rust_client::retry::RetryPolicy import path consistently ([fe3fdaf](https://github.com/maltesander/nifi-rust-client/commit/fe3fdaf))
- Classify 3xx responses as success in generated docs ([14e33f1](https://github.com/maltesander/nifi-rust-client/commit/14e33f1))

### Documentation

- Add IDE autocompletion note for dynamic mode ([816e8fa](https://github.com/maltesander/nifi-rust-client/commit/816e8fa))
- Update README wording for static version feature ([5e4b2cb](https://github.com/maltesander/nifi-rust-client/commit/5e4b2cb))
- Note rust-analyzer.toml needs manual update on version bump ([cec81c1](https://github.com/maltesander/nifi-rust-client/commit/cec81c1))
- Document VersionResolutionStrategy in READMEs and AGENTS.md ([efc8118](https://github.com/maltesander/nifi-rust-client/commit/efc8118))
- Minor README tweaks ([5e2f4c3](https://github.com/maltesander/nifi-rust-client/commit/5e2f4c3))
- Update generator intro and AGENTS.md for current structure ([a63492e](https://github.com/maltesander/nifi-rust-client/commit/a63492e))
- Document resource_accessors emitter in AGENTS.md and gen README ([d5ed4e1](https://github.com/maltesander/nifi-rust-client/commit/d5ed4e1))
- Add RESOURCE_ACCESSORS markers to client README ([16230c5](https://github.com/maltesander/nifi-rust-client/commit/16230c5))
- Add implementation plan for resource accessors table ([9d84cc4](https://github.com/maltesander/nifi-rust-client/commit/9d84cc4))
- Add design spec for auto-generated resource accessors table ([0245f58](https://github.com/maltesander/nifi-rust-client/commit/0245f58))
- Update READMEs and AGENTS.md for unified trait hierarchy ([21bc8c6](https://github.com/maltesander/nifi-rust-client/commit/21bc8c6))
- Add unified trait hierarchy implementation plan ([7f154b3](https://github.com/maltesander/nifi-rust-client/commit/7f154b3))
- Add unified trait hierarchy design spec ([e81bd2d](https://github.com/maltesander/nifi-rust-client/commit/e81bd2d))
- Update dynamic client examples for lazy version detection ([324ed2b](https://github.com/maltesander/nifi-rust-client/commit/324ed2b))
- Document run-matrix.sh and integration test emitters ([c4f14fa](https://github.com/maltesander/nifi-rust-client/commit/c4f14fa))
- Update README for new module structure ([bec06fc](https://github.com/maltesander/nifi-rust-client/commit/bec06fc))
- Update READMEs for credential providers, retry policy, and typed errors ([4175e23](https://github.com/maltesander/nifi-rust-client/commit/4175e23))

### Tests

- Add wiremock tests for version resolution strategies ([6eb9f54](https://github.com/maltesander/nifi-rust-client/commit/6eb9f54))

## [0.3.1] - 2026-04-08

### Added

- Keep crate README in sync with generator ([a3904e6](https://github.com/maltesander/nifi-rust-client/commit/a3904e6))

### Changed

- Tag version examples in READMEs for automated updates ([8d7f54f](https://github.com/maltesander/nifi-rust-client/commit/8d7f54f))

### Fixed

- Set real version in static feature example ([7d47eb3](https://github.com/maltesander/nifi-rust-client/commit/7d47eb3))
- Move 2xx responses to # Returns doc section, fix 206 misclassified as error ([39beab2](https://github.com/maltesander/nifi-rust-client/commit/39beab2))

### Documentation

- Add missing integration test files to coverage table; fix clippy ([f222289](https://github.com/maltesander/nifi-rust-client/commit/f222289))

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

[Unreleased]: https://github.com/maltesander/nifi-rust-client/compare/client-v0.9.0...HEAD
[0.9.0]: https://github.com/maltesander/nifi-rust-client/compare/client-v0.8.0...client-v0.9.0
[0.8.0]: https://github.com/maltesander/nifi-rust-client/compare/client-v0.7.0...client-v0.8.0
[0.7.0]: https://github.com/maltesander/nifi-rust-client/compare/client-v0.6.0...client-v0.7.0
[0.6.0]: https://github.com/maltesander/nifi-rust-client/compare/v0.5.0...client-v0.6.0
[0.5.0]: https://github.com/maltesander/nifi-rust-client/compare/v0.4.2...v0.5.0
[0.4.2]: https://github.com/maltesander/nifi-rust-client/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/maltesander/nifi-rust-client/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/maltesander/nifi-rust-client/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/maltesander/nifi-rust-client/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/maltesander/nifi-rust-client/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/maltesander/nifi-rust-client/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/maltesander/nifi-rust-client/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/maltesander/nifi-rust-client/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/maltesander/nifi-rust-client/releases/tag/v0.1.0
