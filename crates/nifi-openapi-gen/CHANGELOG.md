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

[0.9.0]: https://github.com/maltesander/nifi-rust-client/compare/gen-v0.8.0...gen-v0.9.0
[0.8.0]: https://github.com/maltesander/nifi-rust-client/compare/gen-v0.7.0...gen-v0.8.0
[0.7.0]: https://github.com/maltesander/nifi-rust-client/compare/gen-v0.6.0...gen-v0.7.0
[0.6.0]: https://github.com/maltesander/nifi-rust-client/compare/v0.5.0...gen-v0.6.0
