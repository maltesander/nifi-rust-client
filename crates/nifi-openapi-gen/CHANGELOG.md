<!-- markdownlint-disable MD033 -->
# Changelog

All notable changes to `nifi-openapi-gen` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> Historic entries up to and including v0.5.0 were tracked together with
> `nifi-rust-client` in the root `CHANGELOG.md`, which is now
> [`crates/nifi-rust-client/CHANGELOG.md`](../nifi-rust-client/CHANGELOG.md).
> Starting with the next release, `nifi-openapi-gen` has an independent
> release cadence.

## [Unreleased]

## [0.12.0] - 2026-04-28

### Breaking Changes

- Deserialize date-time DTO fields from string or integer ([d27a643](https://github.com/maltesander/nifi-rust-client/commit/d27a643))

## [0.11.1] - 2026-04-27

### Fixed

- Normalize OUT_DIR backslashes in generated #[path] attributes ([996dd7c](https://github.com/maltesander/nifi-rust-client/commit/996dd7c))

## [0.11.0] - 2026-04-21

### Added

- Validate tag identifier chars at parse time ([a7be413](https://github.com/maltesander/nifi-rust-client/commit/a7be413))
- Emit confirm branch for every generated DELETE handler ([b4a48f9](https://github.com/maltesander/nifi-rust-client/commit/b4a48f9))
- Emit dry-run branch for every mutating generated handler ([e7763e1](https://github.com/maltesander/nifi-rust-client/commit/e7763e1))
- Emit dynamic-mode inline-enum helpers per owning tag file ([7ac7856](https://github.com/maltesander/nifi-rust-client/commit/7ac7856))
- Add emit_inline_enum_helper for dynamic-mode field enums ([513beea](https://github.com/maltesander/nifi-rust-client/commit/513beea))
- Panic on inline-enum helper name collision ([e30c3e6](https://github.com/maltesander/nifi-rust-client/commit/e30c3e6))
- Collect dynamic-mode inline-enum helpers in merge_all_types ([4420e31](https://github.com/maltesander/nifi-rust-client/commit/4420e31))
- Emit as_str() on static-mode field-level string enums ([918284e](https://github.com/maltesander/nifi-rust-client/commit/918284e))
- Emit streaming variant for binary GET endpoints (dynamic) ([e28a640](https://github.com/maltesander/nifi-rust-client/commit/e28a640))
- Emit streaming variant for binary GET endpoints (static) ([c06026e](https://github.com/maltesander/nifi-rust-client/commit/c06026e))
- Emit doc comments for dynamic API methods ([0395ec8](https://github.com/maltesander/nifi-rust-client/commit/0395ec8))

### Changed

- Move impl VersionDiff into diff/report.rs ([5be454e](https://github.com/maltesander/nifi-rust-client/commit/5be454e))
- Split diff.rs into endpoint/types/index submodules ([3d173ef](https://github.com/maltesander/nifi-rust-client/commit/3d173ef))
- Extract shared helpers between static and dynamic type emitters ([fa1e06e](https://github.com/maltesander/nifi-rust-client/commit/fa1e06e))
- Promote extract_enum to emit/common as extract_inline_enum_variants ([12a3eb3](https://github.com/maltesander/nifi-rust-client/commit/12a3eb3))

### Fixed

- Multipart emitter carries required schema properties ([17c89d6](https://github.com/maltesander/nifi-rust-client/commit/17c89d6))
- Inline-JSON responses emit get helper + serde_json::Value ([1e443ec](https://github.com/maltesander/nifi-rust-client/commit/1e443ec))
- Resolve rustdoc broken intra-doc links + markdown lint ([888d80d](https://github.com/maltesander/nifi-rust-client/commit/888d80d))
- Tighten merge_inline_enum panic message ([74cda35](https://github.com/maltesander/nifi-rust-client/commit/74cda35))

### Documentation

- Fix rustdoc warnings on doc build ([ed62e18](https://github.com/maltesander/nifi-rust-client/commit/ed62e18))
- Front-load stability scope banner ([3ee9b54](https://github.com/maltesander/nifi-rust-client/commit/3ee9b54))
- Sweep terminology for consistency ([7c213b9](https://github.com/maltesander/nifi-rust-client/commit/7c213b9))
- Banner separates helpers from DTOs in dynamic type files ([132bc36](https://github.com/maltesander/nifi-rust-client/commit/132bc36))
- Add module-level //! header to dynamic per-tag type files ([41295da](https://github.com/maltesander/nifi-rust-client/commit/41295da))

### Tests

- Add exhaustive emit_method dispatch unit tests ([e4ed981](https://github.com/maltesander/nifi-rust-client/commit/e4ed981))
- Add unit tests for MutationPlan apply/check ([dba141a](https://github.com/maltesander/nifi-rust-client/commit/dba141a))
- Tighten stream emission assertions and cover with-query branch ([78793bd](https://github.com/maltesander/nifi-rust-client/commit/78793bd))

## [0.10.1] - 2026-04-14

### Fixed

- Normalize dynamic path template placeholders ([160d46d](https://github.com/maltesander/nifi-rust-client/commit/160d46d))

## [0.10.0] - 2026-04-14

### Breaking Changes

- Flatten public API — client.flow() replaces client.flow_api() ([b254b8b](https://github.com/maltesander/nifi-rust-client/commit/b254b8b))

### Added

- Scaffold shared emit::method module ([64a394f](https://github.com/maltesander/nifi-rust-client/commit/64a394f))
- Accept dynamic feature without version feature ([f0c60e2](https://github.com/maltesander/nifi-rust-client/commit/f0c60e2))
- Dynamic emission reads all specs from disk ([4bd7aa6](https://github.com/maltesander/nifi-rust-client/commit/4bd7aa6))
- Emit dynamic = [] instead of per-version union ([dc993e0](https://github.com/maltesander/nifi-rust-client/commit/dc993e0))
- Emit header params as method arguments ([bce7cd1](https://github.com/maltesander/nifi-rust-client/commit/bce7cd1))
- Collect header params into Endpoint ([d88a700](https://github.com/maltesander/nifi-rust-client/commit/d88a700))
- Rewire build_api.rs to canonical dynamic emitter only ([4b0d960](https://github.com/maltesander/nifi-rust-client/commit/4b0d960))
- Delete legacy dispatch-based dynamic emitter ([83ec27a](https://github.com/maltesander/nifi-rust-client/commit/83ec27a))
- Inline dynamic types emitter into canonical module ([cdb43b7](https://github.com/maltesander/nifi-rust-client/commit/cdb43b7))
- Emit DetectedVersion + SUPPORTED_VERSIONS from canonical availability emitter ([06b1eb8](https://github.com/maltesander/nifi-rust-client/commit/06b1eb8))
- Orchestrate dynamic_v2 emit (availability + types + api + mod) ([661b891](https://github.com/maltesander/nifi-rust-client/commit/661b891))
- Emit full canonical api with sub-groups, params, body kinds ([3cd51a9](https://github.com/maltesander/nifi-rust-client/commit/3cd51a9))
- Emit canonical api stubs for no-arg GETs in dynamic_v2 ([52d4618](https://github.com/maltesander/nifi-rust-client/commit/52d4618))
- Emit canonical dynamic_v2 types via legacy shim ([2cdb06b](https://github.com/maltesander/nifi-rust-client/commit/2cdb06b))
- Emit Endpoint enum and availability tables for dynamic_v2 ([b7db02b](https://github.com/maltesander/nifi-rust-client/commit/b7db02b))
- Scaffold dynamic_v2 emit module + endpoint index ([abae7ed](https://github.com/maltesander/nifi-rust-client/commit/abae7ed))
- Add project function returning per-version ApiSpec ([9de301e](https://github.com/maltesander/nifi-rust-client/commit/9de301e))
- Populate per_version_specs during canonicalization ([1dfa8c9](https://github.com/maltesander/nifi-rust-client/commit/1dfa8c9))
- Add per_version_specs field to CanonicalSpec ([2785de8](https://github.com/maltesander/nifi-rust-client/commit/2785de8))
- Wire canonicalize + non-additive detector into generate pipeline ([c0e2f65](https://github.com/maltesander/nifi-rust-client/commit/c0e2f65))
- Panic with actionable message on non-additive changes ([6f0d3d7](https://github.com/maltesander/nifi-rust-client/commit/6f0d3d7))
- Add non-additive override table (empty) ([9d8a515](https://github.com/maltesander/nifi-rust-client/commit/9d8a515))
- Detect enum variant removal as non-additive ([09c1f69](https://github.com/maltesander/nifi-rust-client/commit/09c1f69))
- Detect field type changes as non-additive ([64dc793](https://github.com/maltesander/nifi-rust-client/commit/64dc793))
- Detect field removal as non-additive change ([b583c04](https://github.com/maltesander/nifi-rust-client/commit/b583c04))
- Detect type removal as non-additive change ([435908b](https://github.com/maltesander/nifi-rust-client/commit/435908b))
- Detect endpoint removal as non-additive change ([99ecf9e](https://github.com/maltesander/nifi-rust-client/commit/99ecf9e))
- Add non-additive detector module skeleton ([3001da9](https://github.com/maltesander/nifi-rust-client/commit/3001da9))
- Canonicalize enum variants with version sets ([94718ea](https://github.com/maltesander/nifi-rust-client/commit/94718ea))
- Canonicalize types and fields with version sets ([6e63b39](https://github.com/maltesander/nifi-rust-client/commit/6e63b39))
- Canonicalize endpoints with version sets ([950e99e](https://github.com/maltesander/nifi-rust-client/commit/950e99e))
- Add canonical spec type skeletons ([f4a7616](https://github.com/maltesander/nifi-rust-client/commit/f4a7616))
- Add VersionSet for canonical spec metadata ([3d811b1](https://github.com/maltesander/nifi-rust-client/commit/3d811b1))
- Emit fn_names.txt goldens per spec version ([be16106](https://github.com/maltesander/nifi-rust-client/commit/be16106))
- Add fn_names golden emitter writing per-version tables ([c3b7aa5](https://github.com/maltesander/nifi-rust-client/commit/c3b7aa5))
- Add specs_dir to RepoLayout for per-version artifact emitters ([092a1ee](https://github.com/maltesander/nifi-rust-client/commit/092a1ee))
- Apply naming post-pass in generate binary ([e92723a](https://github.com/maltesander/nifi-rust-client/commit/e92723a))
- Apply naming overrides and run collision/drift checks in parse_specs ([f98b3b8](https://github.com/maltesander/nifi-rust-client/commit/f98b3b8))
- Panic on cross-version fn_name drift with actionable message ([4ef7713](https://github.com/maltesander/nifi-rust-client/commit/4ef7713))
- Panic on same-tag fn_name collision with actionable message ([668cd53](https://github.com/maltesander/nifi-rust-client/commit/668cd53))
- Add naming module with override application ([33052a4](https://github.com/maltesander/nifi-rust-client/commit/33052a4))
- Add naming_overrides scaffold for method-name pinning ([d11c4af](https://github.com/maltesander/nifi-rust-client/commit/d11c4af))
- Strip _N suffix from operationId-derived fn_name ([0f87700](https://github.com/maltesander/nifi-rust-client/commit/0f87700))

### Changed

- Dedupe header-setup and guard emission helpers ([445fc49](https://github.com/maltesander/nifi-rust-client/commit/445fc49))
- Dedupe static+dynamic emit_method via EmitMode ([d826bab](https://github.com/maltesander/nifi-rust-client/commit/d826bab))
- Move static emit_method body to emit::method ([ff6f0df](https://github.com/maltesander/nifi-rust-client/commit/ff6f0df))
- Flatten parser SubGroup via compat shim ([93345ae](https://github.com/maltesander/nifi-rust-client/commit/93345ae))
- Rename dynamic_v2 to dynamic after legacy deletion ([8e7f0a2](https://github.com/maltesander/nifi-rust-client/commit/8e7f0a2))
- Derive Debug, Clone on ApiSpec and TagGroup ([fe05c08](https://github.com/maltesander/nifi-rust-client/commit/fe05c08))
- Drop redundant fields from CanonicalEndpoint and hoist imports ([e7d0f30](https://github.com/maltesander/nifi-rust-client/commit/e7d0f30))
- Derive Default for VersionSet ([3969038](https://github.com/maltesander/nifi-rust-client/commit/3969038))

### Fixed

- Strict-fail on cookie params instead of silently dropping ([a71d010](https://github.com/maltesander/nifi-rust-client/commit/a71d010))
- Panic when dynamic enabled with <2 specs on disk ([69eca27](https://github.com/maltesander/nifi-rust-client/commit/69eca27))
- Tighten parser silent drops ([2150783](https://github.com/maltesander/nifi-rust-client/commit/2150783))
- Address Phase A code-review findings ([887aa98](https://github.com/maltesander/nifi-rust-client/commit/887aa98))
- Align integration coverage emitters with post-phase-4b runtime ([d815931](https://github.com/maltesander/nifi-rust-client/commit/d815931))
- Adapt hand-written wiremock tests to canonical dynamic ([b04a871](https://github.com/maltesander/nifi-rust-client/commit/b04a871))
- Sort VersionSet iteration in semver order, not lexicographic ([fd9d086](https://github.com/maltesander/nifi-rust-client/commit/fd9d086))
- Treat inline enum variant additions as additive ([61d8700](https://github.com/maltesander/nifi-rust-client/commit/61d8700))
- Guard cluster_node_id auto-injection on nodewise flag ([a91b34c](https://github.com/maltesander/nifi-rust-client/commit/a91b34c))

### Documentation

- Rewrite README against current emitter tree ([7d85b19](https://github.com/maltesander/nifi-rust-client/commit/7d85b19))

### Tests

- Adapt imports for canonical dynamic path ([db29eb9](https://github.com/maltesander/nifi-rust-client/commit/db29eb9))
- Cover canonicalize_or_panic per_version_specs population ([d5c271f](https://github.com/maltesander/nifi-rust-client/commit/d5c271f))
- Multi-version round-trip byte-identity against real specs ([e29ac8a](https://github.com/maltesander/nifi-rust-client/commit/e29ac8a))
- Single-version round-trip byte-identity against real specs ([28d99d4](https://github.com/maltesander/nifi-rust-client/commit/28d99d4))
- Golden detector test against real spec chain ([4db4c5d](https://github.com/maltesander/nifi-rust-client/commit/4db4c5d))
- Smoke-test canonicalize on real spec chain ([aaae64a](https://github.com/maltesander/nifi-rust-client/commit/aaae64a))
- Regression test asserting fn_names.txt goldens match regeneration ([2355f76](https://github.com/maltesander/nifi-rust-client/commit/2355f76))
- Cover strip_trailing_numeric_suffix edge cases ([baeb4b5](https://github.com/maltesander/nifi-rust-client/commit/baeb4b5))

## [0.9.0] - 2026-04-12

### Added

- Add cluster node discovery and auto-inject cluster_node_id ([0ddadcc](https://github.com/maltesander/nifi-rust-client/commit/0ddadcc))
- Add nifictl CLI tool ([9b3a33e](https://github.com/maltesander/nifi-rust-client/commit/9b3a33e))

## [0.8.0] - 2026-04-12

### Added

- Reformat NIFI_API_CHANGES.md as tables with tag columns ([17f26f8](https://github.com/maltesander/nifi-rust-client/commit/17f26f8))
- Auto-update lib.rs doc comment version examples ([a133550](https://github.com/maltesander/nifi-rust-client/commit/a133550))
- Rewrite generate.rs to use RepoLayout and MutationPlan ([6882c51](https://github.com/maltesander/nifi-rust-client/commit/6882c51))
- Extract collect_*_metadata functions for coverage stats ([9412ad0](https://github.com/maltesander/nifi-rust-client/commit/9412ad0))
- Re-export all emit_* functions from docs::mod ([19ef593](https://github.com/maltesander/nifi-rust-client/commit/19ef593))
- Add emit_integration_coverage returning FileEdit ([67c7a40](https://github.com/maltesander/nifi-rust-client/commit/67c7a40))
- Add emit_client_readme_examples returning FileEdits ([de548a4](https://github.com/maltesander/nifi-rust-client/commit/de548a4))
- Add emit_api_changes returning FileEdit ([d7ebd53](https://github.com/maltesander/nifi-rust-client/commit/d7ebd53))
- Add emit_resource_accessors returning FileEdit ([e541ee7](https://github.com/maltesander/nifi-rust-client/commit/e541ee7))
- Add emit_versions_table returning FileEdits ([ccb4b22](https://github.com/maltesander/nifi-rust-client/commit/ccb4b22))
- Add emit_lib_rs_feature_flags returning FileEdit ([ca736c0](https://github.com/maltesander/nifi-rust-client/commit/ca736c0))
- Add emit_docker_compose_default returning FileEdit ([a659f6f](https://github.com/maltesander/nifi-rust-client/commit/a659f6f))
- Add emit_cargo_features_client/tests returning FileEdit ([1f93307](https://github.com/maltesander/nifi-rust-client/commit/1f93307))
- Implement MutationPlan::check with truncated diff output ([02f0a44](https://github.com/maltesander/nifi-rust-client/commit/02f0a44))
- Implement MutationPlan::apply ([33ef041](https://github.com/maltesander/nifi-rust-client/commit/33ef041))
- Add FileEdit, MutationPlan, and validation ([d65cb92](https://github.com/maltesander/nifi-rust-client/commit/d65cb92))
- Add RepoLayout struct for centralised path discovery ([a22d48e](https://github.com/maltesander/nifi-rust-client/commit/a22d48e))

### Changed

- Address review feedback ([00a8af9](https://github.com/maltesander/nifi-rust-client/commit/00a8af9))
- Remove old update_* functions replaced by emit_* + MutationPlan ([af56cbb](https://github.com/maltesander/nifi-rust-client/commit/af56cbb))

## [0.7.0] - 2026-04-12

### Added

- Diff detects deprecation changes and enum narrowing ([b3024c3](https://github.com/maltesander/nifi-rust-client/commit/b3024c3))
- Emit text/bytes returns for non-JSON response endpoints ([d98f8f7](https://github.com/maltesander/nifi-rust-client/commit/d98f8f7))
- Capture non-JSON response body kinds ([27bf17a](https://github.com/maltesander/nifi-rust-client/commit/27bf17a))
- Recognize multipart/form-data and */* request bodies ([39ee9cd](https://github.com/maltesander/nifi-rust-client/commit/39ee9cd))
- Add content_type allow-list module ([9f85474](https://github.com/maltesander/nifi-rust-client/commit/9f85474))
- Panic on unknown query parameter type ([a440fb7](https://github.com/maltesander/nifi-rust-client/commit/a440fb7))
- Panic on unknown field type in OpenAPI spec ([4bf0215](https://github.com/maltesander/nifi-rust-client/commit/4bf0215))
- Add strict-parsing panic helper ([122236d](https://github.com/maltesander/nifi-rust-client/commit/122236d))
- Compact enum-variant diffs in NIFI_API_CHANGES ([3604ce0](https://github.com/maltesander/nifi-rust-client/commit/3604ce0))
- Surface breaking-change marker and tested column in README ([3105c97](https://github.com/maltesander/nifi-rust-client/commit/3105c97))
- Add SemverBump, is_breaking(), semver_bump() to VersionDiff ([4c4a554](https://github.com/maltesander/nifi-rust-client/commit/4c4a554))
- Detect field base type changes in diff_type_fields ([7dbf1f5](https://github.com/maltesander/nifi-rust-client/commit/7dbf1f5))
- Carry required flag on AddedParam; detect query param type transitions (Str↔Enum) ([88b965d](https://github.com/maltesander/nifi-rust-client/commit/88b965d))
- Detect request_type, response_type, body_kind changes on endpoints ([7606905](https://github.com/maltesander/nifi-rust-client/commit/7606905))
- Detect path parameter add/remove in endpoint diff ([e2ac18e](https://github.com/maltesander/nifi-rust-client/commit/e2ac18e))
- Add tag field to EndpointChanges for grouping changed endpoints ([d50e140](https://github.com/maltesander/nifi-rust-client/commit/d50e140))

### Changed

- Lazy version detection on dynamic dispatch ([422ff49](https://github.com/maltesander/nifi-rust-client/commit/422ff49))
- Replace FieldChange.description string with FieldChangeKind enum ([e7113a6](https://github.com/maltesander/nifi-rust-client/commit/e7113a6))
- Consolidate HttpMethod::as_str(), remove duplicated method_key/method_str ([47b7788](https://github.com/maltesander/nifi-rust-client/commit/47b7788))

### Fixed

- Escape Vec<u8> in content_type doc comments ([5783d0c](https://github.com/maltesander/nifi-rust-client/commit/5783d0c))
- Classify added path params as breaking; add Serialize to SemverBump ([3ccba75](https://github.com/maltesander/nifi-rust-client/commit/3ccba75))
- Detect StringEnum variant changes in diff_types (was silently dropped) ([a4cd56d](https://github.com/maltesander/nifi-rust-client/commit/a4cd56d))

## [0.6.0] - 2026-04-11

### Added

- Add override registry for integration test exceptions ([7fe9531](https://github.com/maltesander/nifi-rust-client/commit/7fe9531))
- Add NiFi 2.9.0 support (265 paths, up from 237) ([c9edd27](https://github.com/maltesander/nifi-rust-client/commit/c9edd27))

### Changed

- Eliminate hardcoded latest version from tests and scripts ([26493d3](https://github.com/maltesander/nifi-rust-client/commit/26493d3))

### Fixed

- Stable method names across NiFi versions via path-based grouping ([a058972](https://github.com/maltesander/nifi-rust-client/commit/a058972))

[0.12.0]: https://github.com/maltesander/nifi-rust-client/compare/gen-v0.11.1...gen-v0.12.0
[0.11.1]: https://github.com/maltesander/nifi-rust-client/compare/gen-v0.11.0...gen-v0.11.1
[0.11.0]: https://github.com/maltesander/nifi-rust-client/compare/gen-v0.10.1...gen-v0.11.0
[0.10.1]: https://github.com/maltesander/nifi-rust-client/compare/gen-v0.10.0...gen-v0.10.1
[0.10.0]: https://github.com/maltesander/nifi-rust-client/compare/gen-v0.9.0...gen-v0.10.0
[0.9.0]: https://github.com/maltesander/nifi-rust-client/compare/gen-v0.8.0...gen-v0.9.0
[0.8.0]: https://github.com/maltesander/nifi-rust-client/compare/gen-v0.7.0...gen-v0.8.0
[0.7.0]: https://github.com/maltesander/nifi-rust-client/compare/gen-v0.6.0...gen-v0.7.0
[0.6.0]: https://github.com/maltesander/nifi-rust-client/compare/v0.5.0...gen-v0.6.0
