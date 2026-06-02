<!-- markdownlint-disable MD033 -->
# Changelog

All notable changes to `nifictl` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-06-02

### Breaking Changes

- Rename resource subcommands to kebab-case (C9) ([ba5ee96](https://github.com/maltesander/nifi-rust-client/commit/ba5ee96))
- Tighten public Zeroizing API on token/set_token (B9.2) ([fe49656](https://github.com/maltesander/nifi-rust-client/commit/fe49656))

### Added

- JSON array on auto-pipe; honour NO_COLOR (C5 + C13) ([32a7946](https://github.com/maltesander/nifi-rust-client/commit/32a7946))
- Warn on world-readable config with plaintext secrets (B4) ([1f1925d](https://github.com/maltesander/nifi-rust-client/commit/1f1925d))
- Builder-time validation for insecure/header/chain (B6 + B8 + B10) ([457e584](https://github.com/maltesander/nifi-rust-client/commit/457e584))
- Hide --token from help and warn on use (B2) ([1c89a6c](https://github.com/maltesander/nifi-rust-client/commit/1c89a6c))
- Add ResolvedParams::resolve_url_only for dry-run flows ([53ad9e9](https://github.com/maltesander/nifi-rust-client/commit/53ad9e9))
- Use cached JWT in every dispatch arm (was always re-login) ([63ce5ce](https://github.com/maltesander/nifi-rust-client/commit/63ce5ce))
- Build_client_with_cache for token reuse across commands ([633b6e4](https://github.com/maltesander/nifi-rust-client/commit/633b6e4))

### Changed

- Single confirm-gate site for destructive commands (C10) ([c780668](https://github.com/maltesander/nifi-rust-client/commit/c780668))
- Extract token-cache helpers into porcelain/token_cache ([21c675e](https://github.com/maltesander/nifi-rust-client/commit/21c675e))

### Fixed

- Replace unsound serde_yml with serde_norway, fix doc link, bump deps ([49e5efa](https://github.com/maltesander/nifi-rust-client/commit/49e5efa))
- Reject path-traversal in --context name (B3) ([f4a59c8](https://github.com/maltesander/nifi-rust-client/commit/f4a59c8))
- Route flow porcelain dry-run output to stderr ([ea0c7a7](https://github.com/maltesander/nifi-rust-client/commit/ea0c7a7))
- 404 hint no longer suggests nonexistent 'list' verb ([1416de6](https://github.com/maltesander/nifi-rust-client/commit/1416de6))
- --dry-run no longer requires authentication ([38cb002](https://github.com/maltesander/nifi-rust-client/commit/38cb002))
- Allow dead_code on token_cache helpers wired in by next task ([f05359e](https://github.com/maltesander/nifi-rust-client/commit/f05359e))

### Documentation

- Fix gen + nifictl README sync issues ([7bfa776](https://github.com/maltesander/nifi-rust-client/commit/7bfa776))

## [0.2.0] - 2026-04-21

### Added

- Print hint line on operator-facing errors ([243db2d](https://github.com/maltesander/nifi-rust-client/commit/243db2d))
- Add CliError::hint with six remediation paths ([5d82e5b](https://github.com/maltesander/nifi-rust-client/commit/5d82e5b))
- Prompt for password on TTY when none configured ([bf0c343](https://github.com/maltesander/nifi-rust-client/commit/bf0c343))
- Add interactive password prompt helper ([a281422](https://github.com/maltesander/nifi-rust-client/commit/a281422))
- Warn on login when JWT expires within 24h ([624314b](https://github.com/maltesander/nifi-rust-client/commit/624314b))
- Add JWT exp-claim decoder ([ca089e7](https://github.com/maltesander/nifi-rust-client/commit/ca089e7))
- Wire flow export/import/replace into top-level Commands ([6a1d882](https://github.com/maltesander/nifi-rust-client/commit/6a1d882))
- Add --stop-first orchestration to flow replace ([4aa2f33](https://github.com/maltesander/nifi-rust-client/commit/4aa2f33))
- Add porcelain::flow::replace (without --stop-first) ([044defb](https://github.com/maltesander/nifi-rust-client/commit/044defb))
- Add porcelain::flow::import ([b097385](https://github.com/maltesander/nifi-rust-client/commit/b097385))
- Add porcelain::flow::export ([f2b0087](https://github.com/maltesander/nifi-rust-client/commit/f2b0087))
- Describe_wait_plan for --dry-run + --wait interaction ([7b8cf78](https://github.com/maltesander/nifi-rust-client/commit/7b8cf78))
- Emit dry-run branch for every mutating generated handler ([e7763e1](https://github.com/maltesander/nifi-rust-client/commit/e7763e1))
- Thread CliCtx into ops porcelain; dry-run + confirm on stop-pg/disable-services ([13de7bb](https://github.com/maltesander/nifi-rust-client/commit/13de7bb))
- Add confirm::confirm_destructive with --yes bypass ([7c7a624](https://github.com/maltesander/nifi-rust-client/commit/7c7a624))
- Add dry_run::print + format_url helpers ([869e44a](https://github.com/maltesander/nifi-rust-client/commit/869e44a))
- Add --dry-run and --yes/-y global flags + CliCtx ([6e050e7](https://github.com/maltesander/nifi-rust-client/commit/6e050e7))
- Wire --wait on provenance submit-provenance-request ([e2c74ee](https://github.com/maltesander/nifi-rust-client/commit/e2c74ee))
- Wire --wait on parameter-contexts submit-parameter-context-update ([89664a6](https://github.com/maltesander/nifi-rust-client/commit/89664a6))
- Wire --wait on controller-services update-run-status ([156af52](https://github.com/maltesander/nifi-rust-client/commit/156af52))
- Wire --wait on processors update-run-status ([8150e3c](https://github.com/maltesander/nifi-rust-client/commit/8150e3c))
- Add nifictl ops disable-services ([51f4b50](https://github.com/maltesander/nifi-rust-client/commit/51f4b50))
- Add nifictl ops enable-services ([729cb48](https://github.com/maltesander/nifi-rust-client/commit/729cb48))
- Add nifictl ops stop-pg ([efde846](https://github.com/maltesander/nifi-rust-client/commit/efde846))
- Add nifictl ops subcommand tree + start-pg ([1cf7ac5](https://github.com/maltesander/nifi-rust-client/commit/1cf7ac5))
- Add --wait / --wait-timeout global flags and wait_wire scaffolding ([9a7c6e4](https://github.com/maltesander/nifi-rust-client/commit/9a7c6e4))

### Changed

- Move resource dispatch into dispatch module ([4d4bcaf](https://github.com/maltesander/nifi-rust-client/commit/4d4bcaf))
- Extract clap definitions into cli.rs ([8bcbaca](https://github.com/maltesander/nifi-rust-client/commit/8bcbaca))
- Extract reject_transient_state helper ([779dc30](https://github.com/maltesander/nifi-rust-client/commit/779dc30))
- Rename and scope prompt constants to avoid collision with confirm.rs ([09fc2b0](https://github.com/maltesander/nifi-rust-client/commit/09fc2b0))
- Fold in Task 5 review feedback (dyn Error sig, drop redundant keyword, drop reqwest direct dep) ([a378896](https://github.com/maltesander/nifi-rust-client/commit/a378896))
- Enumerate generated resources explicitly in Commands ([2a7e6f3](https://github.com/maltesander/nifi-rust-client/commit/2a7e6f3))

### Fixed

- Refuse plaintext config secrets unless NIFICTL_ALLOW_PLAINTEXT_SECRETS=1 ([0d769a0](https://github.com/maltesander/nifi-rust-client/commit/0d769a0))
- Hide --password from help, warn on CLI use ([fc47de8](https://github.com/maltesander/nifi-rust-client/commit/fc47de8))
- Write cached JWT with 0o600 on Unix ([2de1c94](https://github.com/maltesander/nifi-rust-client/commit/2de1c94))
- Drop redundant SystemTime import in login tests ([9f83ebd](https://github.com/maltesander/nifi-rust-client/commit/9f83ebd))
- Move mod jwt; to correct alphabetical position ([a1510ef](https://github.com/maltesander/nifi-rust-client/commit/a1510ef))
- Dispatch generated flow subcommands; disambiguate --output id ([cc021a8](https://github.com/maltesander/nifi-rust-client/commit/cc021a8))
- Reject --wait with RUN_ONCE body with a clear message ([800bba8](https://github.com/maltesander/nifi-rust-client/commit/800bba8))
- Address Tasks 2+3 code review — error on unimplemented ops verbs, clearer --wait message ([0175215](https://github.com/maltesander/nifi-rust-client/commit/0175215))
- Address Task 1 code review — allow(dead_code), tighter tests, drop dup default ([087cf90](https://github.com/maltesander/nifi-rust-client/commit/087cf90))

### Documentation

- Fix rustdoc warnings on doc build ([ed62e18](https://github.com/maltesander/nifi-rust-client/commit/ed62e18))
- Remove generated comments annotation ([1af9153](https://github.com/maltesander/nifi-rust-client/commit/1af9153))
- Document interactive login, JWT expiry, and troubleshooting hints ([b913c85](https://github.com/maltesander/nifi-rust-client/commit/b913c85))
- Document flow portability + Commands enumeration pattern ([1c296e5](https://github.com/maltesander/nifi-rust-client/commit/1c296e5))
- Explain why FlowExportArgs::output needs a distinct clap id ([342a443](https://github.com/maltesander/nifi-rust-client/commit/342a443))
- Document --dry-run and confirmation prompts ([0f72bc6](https://github.com/maltesander/nifi-rust-client/commit/0f72bc6))
- Fix subcommand names in --wait table ([7a1fb07](https://github.com/maltesander/nifi-rust-client/commit/7a1fb07))
- Document ops porcelain and --wait flag ([a0fdc8d](https://github.com/maltesander/nifi-rust-client/commit/a0fdc8d))

### Tests

- Remove in-process non-tty confirm tests ([41955fa](https://github.com/maltesander/nifi-rust-client/commit/41955fa))
- Pin --password warning fires only for CLI flag ([e0996df](https://github.com/maltesander/nifi-rust-client/commit/e0996df))
- Close coverage gaps on wait, multipart, enums, and hint paths ([e01fe29](https://github.com/maltesander/nifi-rust-client/commit/e01fe29))
- Pin non-inclusive 24h expiry-warning boundary ([976cf76](https://github.com/maltesander/nifi-rust-client/commit/976cf76))
- Cover flow porcelain end-to-end ([0ea1789](https://github.com/maltesander/nifi-rust-client/commit/0ea1789))
- Cover --dry-run and non-TTY confirm refusal end-to-end ([52cdbf2](https://github.com/maltesander/nifi-rust-client/commit/52cdbf2))
- Tighten controller_service_target_from_body DISABLING test ([a18b183](https://github.com/maltesander/nifi-rust-client/commit/a18b183))

## [0.1.1] - 2026-04-14

- Normalize dynamic path template placeholders in `openapi-gen` ([160d46d](https://github.com/maltesander/nifi-rust-client/commit/160d46d))

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
[0.3.0]: https://github.com/maltesander/nifi-rust-client/compare/ctl-v0.2.0...ctl-v0.3.0
[0.2.0]: https://github.com/maltesander/nifi-rust-client/compare/ctl-v0.1.1...ctl-v0.2.0
[0.1.1]: https://github.com/maltesander/nifi-rust-client/compare/ctl-v0.1.0...ctl-v0.1.1
[0.1.0]: https://github.com/maltesander/nifi-rust-client/compare/ctl-v0.0.0...ctl-v0.1.0
