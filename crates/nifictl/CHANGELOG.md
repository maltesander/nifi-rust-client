<!-- markdownlint-disable MD033 -->
# Changelog

All notable changes to `nifictl` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-04-14

### Breaking Changes

- Flatten public API — client.flow() replaces client.flow_api() ([b254b8b](https://github.com/maltesander/nifi-rust-client/commit/b254b8b))

### Added

- Forward token/set_token/authenticate on DynamicClient ([f46a9d2](https://github.com/maltesander/nifi-rust-client/commit/f46a9d2))
- Add nifictl CLI tool ([9b3a33e](https://github.com/maltesander/nifi-rust-client/commit/9b3a33e))

### Fixed

- Use dynamic feature for client build. ([1c5970e](https://github.com/maltesander/nifi-rust-client/commit/1c5970e))
- Use nifi-rust-client 0.10.0 instead of workspace dependency. ([291d281](https://github.com/maltesander/nifi-rust-client/commit/291d281))

[Unreleased]: https://github.com/maltesander/nifi-rust-client/commits/HEAD
[0.1.0]: https://github.com/maltesander/nifi-rust-client/compare/ctl-v0.0.0...ctl-v0.1.0
