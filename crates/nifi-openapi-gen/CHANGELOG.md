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

## [0.6.0] - 2026-04-11

### Added

- Add override registry for integration test exceptions ([7fe9531](https://github.com/maltesander/nifi-rust-client/commit/7fe9531))
- Add NiFi 2.9.0 support (265 paths, up from 237) ([c9edd27](https://github.com/maltesander/nifi-rust-client/commit/c9edd27))

### Changed

- Eliminate hardcoded latest version from tests and scripts ([26493d3](https://github.com/maltesander/nifi-rust-client/commit/26493d3))

### Fixed

- Stable method names across NiFi versions via path-based grouping ([a058972](https://github.com/maltesander/nifi-rust-client/commit/a058972))

[0.6.0]: https://github.com/maltesander/nifi-rust-client/compare/v0.5.0...gen-v0.6.0
