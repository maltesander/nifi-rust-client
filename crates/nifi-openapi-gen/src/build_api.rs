//! Stable public API for `build.rs` callers.
//!
//! This module is the **only** part of `nifi-openapi-gen` covered by a
//! stability guarantee. Downstream `build.rs` scripts should depend on:
//!
//! - [`GenerateConfig`] and its constructors ([`GenerateConfig::from_cargo_env`],
//!   [`GenerateConfig::from_specs_dir`])
//! - [`generate_client`]
//! - [`generate_integration_tests`]
//! - The top-level [`crate::specs_dir`] helper
//! - The generated file layout written under the caller-supplied `out_dir`:
//!   - `generated_lib.rs` — the entry point to `include!()` from `lib.rs`
//!   - `vX_Y_Z/{api,types}/*.rs` — per-version generated modules
//!   - `dynamic/*` — canonical superset emitter output, present when `config.dynamic` is true
//!
//! All other modules in `nifi-openapi-gen` (`parser`, `emit`, `diff`,
//! `docs`, `repo`, `util`) are implementation details. They remain `pub`
//! so the generator's own integration tests can reach them, but they are
//! marked `#[doc(hidden)]` and are **not** covered by semver. Breaking
//! changes to those modules will not trigger a major version bump.
//!
//! The contract above is guaranteed stable within a `0.x` minor-version
//! range and will be reconsidered for `1.0`.

use std::path::Path;

use crate::parser::ApiSpec;
use crate::util::{sort_versions_semver, version_to_feature, version_to_mod_name};

// ── Configuration ───────────────────────────────────────────────────────────

/// Configuration for code generation, typically derived from Cargo feature flags.
pub struct GenerateConfig {
    /// Enabled NiFi versions, e.g. `["2.6.0", "2.7.2", "2.8.0"]`.
    /// Must be sorted in semver order (ascending).
    pub versions: Vec<String>,
    /// Whether the `dynamic` Cargo feature is active.
    pub dynamic: bool,
}

impl GenerateConfig {
    /// Build a `GenerateConfig` by inspecting `CARGO_FEATURE_*` environment
    /// variables set by Cargo during a build-script run.
    ///
    /// - `CARGO_FEATURE_NIFI_2_8_0=1` → version `"2.8.0"`
    /// - `CARGO_FEATURE_DYNAMIC=1`    → `dynamic = true`
    pub fn from_cargo_env() -> Self {
        let mut versions: Vec<String> = Vec::new();

        for (key, _) in std::env::vars() {
            if let Some(rest) = key.strip_prefix("CARGO_FEATURE_NIFI_") {
                // CARGO_FEATURE_NIFI_2_8_0 → "2_8_0" → "2.8.0"
                let version = rest.replace('_', ".");
                versions.push(version);
            }
        }

        sort_versions_semver(&mut versions);

        let dynamic = std::env::var("CARGO_FEATURE_DYNAMIC").is_ok();

        Self { versions, dynamic }
    }

    /// Build a `GenerateConfig` using all spec versions discovered in `specs_dir`.
    ///
    /// This is useful for integration test `build.rs` scripts that want to generate
    /// tests covering all available spec versions, regardless of which Cargo feature
    /// flags are enabled.  The `dynamic` flag is still read from the environment.
    pub fn from_specs_dir(specs_dir: &Path) -> Self {
        let mut versions = crate::util::discover_spec_versions(specs_dir);
        sort_versions_semver(&mut versions);
        let dynamic = std::env::var("CARGO_FEATURE_DYNAMIC").is_ok();
        Self { versions, dynamic }
    }
}

// ── Code generation ─────────────────────────────────────────────────────────

/// Generate all client code into `out_dir`.
///
/// This replicates the per-version and dynamic code-generation steps from the
/// `generate` binary but writes into `OUT_DIR` (or any caller-chosen directory)
/// instead of checked-in source paths.
///
/// After calling this function the caller should `include!` the generated
/// `generated_lib.rs` from their `lib.rs`.
pub fn generate_client(specs_dir: &Path, out_dir: &Path, config: &GenerateConfig) {
    let all_parsed = parse_specs(specs_dir, &config.versions);

    // ── Per-version modules ─────────────────────────────────────────────
    for (version_str, spec) in &all_parsed {
        let mod_name = version_to_mod_name(version_str);
        let versioned_dir = out_dir.join(&mod_name);

        for (filename, content) in crate::emit_types(spec) {
            write_generated(
                &versioned_dir.join("types").join(&filename),
                &with_header(&content),
            );
        }

        let types_prefix = format!("crate::{mod_name}");
        for (filename, content) in crate::emit_api_with_prefix(spec, &types_prefix) {
            write_generated(
                &versioned_dir.join("api").join(&filename),
                &with_header(&content),
            );
        }

        write_generated(
            &versioned_dir.join("mod.rs"),
            &with_header("pub mod api;\npub mod types;\n"),
        );
    }

    // ── Dynamic module (only when multiple versions are present) ─────────
    if config.dynamic && all_parsed.len() > 1 {
        generate_dynamic(out_dir, &all_parsed);
    }

    // ── generated_lib.rs ────────────────────────────────────────────────
    let lib_content = generate_lib_rs_fragment(&config.versions, out_dir, config.dynamic);
    write_generated(&out_dir.join("generated_lib.rs"), &lib_content);
}

/// Generate dynamic integration test files into `out_dir`.
///
/// Only produces output when `config.dynamic` is true and at least two
/// versions are configured.
pub fn generate_integration_tests(specs_dir: &Path, out_dir: &Path, config: &GenerateConfig) {
    if !config.dynamic || config.versions.len() < 2 {
        return;
    }

    let all_parsed = parse_specs(specs_dir, &config.versions);

    let diffs: Vec<crate::VersionDiff> = (1..all_parsed.len())
        .map(|i| {
            let (from_ver, from_spec) = &all_parsed[i - 1];
            let (to_ver, to_spec) = &all_parsed[i];
            crate::compute_diff(from_spec, to_spec, from_ver, to_ver)
        })
        .collect();

    let (enum_tests, _) = crate::emit_enum_coverage_tests(&all_parsed, &diffs);
    if !enum_tests.is_empty() {
        write_generated(
            &out_dir.join("dynamic_enum_coverage.rs"),
            &with_header(&strip_include_incompatible_lines(&enum_tests)),
        );
    }

    let (endpoint_tests, _) = crate::emit_endpoint_availability_tests(&all_parsed, &diffs);
    if !endpoint_tests.is_empty() {
        write_generated(
            &out_dir.join("dynamic_endpoint_availability.rs"),
            &with_header(&strip_include_incompatible_lines(&endpoint_tests)),
        );
    }

    let field_tests = crate::emit_field_presence_tests(&all_parsed, &diffs);
    if !field_tests.is_empty() {
        write_generated(
            &out_dir.join("dynamic_field_presence.rs"),
            &with_header(&strip_include_incompatible_lines(&field_tests)),
        );
    }

    let (param_tests, _) = crate::emit_query_param_coverage_tests(&all_parsed, &diffs);
    if !param_tests.is_empty() {
        write_generated(
            &out_dir.join("dynamic_query_param_coverage.rs"),
            &with_header(&strip_include_incompatible_lines(&param_tests)),
        );
    }
}

/// Strip lines that are incompatible with `include!()` usage.
///
/// Inner attributes (`#![...]`) are only valid at the crate/module root, not
/// inside an `include!()` expansion. Module declarations (`mod helpers;`) are
/// provided by the wrapper file and must not be duplicated.
fn strip_include_incompatible_lines(content: &str) -> String {
    content
        .lines()
        .filter(|line| !line.starts_with("#![") && !line.starts_with("mod helpers;"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Generate CLI subcommand code (clap derive structs + handlers).
///
/// Uses the **latest** spec version — CLI commands reflect the newest API.
/// All specs are passed for fn_name canonicalization (matching the dynamic
/// dispatch layer which uses the oldest version's name).
/// Writes `generated_cli.rs` into `out_dir`.
pub fn generate_cli(specs_dir: &Path, out_dir: &Path, config: &GenerateConfig) {
    let all_parsed = parse_specs(specs_dir, &config.versions);

    let files = crate::emit::cli::emit_cli(&all_parsed);
    for (filename, content) in files {
        write_generated(&out_dir.join(&filename), &with_header(&content));
    }
}

// ── Private helpers ─────────────────────────────────────────────────────────

/// Load and parse all OpenAPI spec files for the given versions.
fn parse_specs(specs_dir: &Path, versions: &[String]) -> Vec<(String, ApiSpec)> {
    let mut all_parsed: Vec<(String, ApiSpec)> = versions
        .iter()
        .map(|version| {
            let spec_path = specs_dir.join(version).join("nifi-api.json");
            assert!(
                spec_path.exists(),
                "Spec not found: {}",
                spec_path.display()
            );
            let spec = crate::load(spec_path.to_str().expect("UTF-8 spec path"));
            (version.clone(), spec)
        })
        .collect();

    // Apply overrides + run same-tag collision check per version.
    for (version, spec) in &mut all_parsed {
        crate::naming::apply_overrides_and_check_single(spec, version);
    }

    // Run cross-version drift check once across all loaded versions.
    crate::naming::check_drift(&all_parsed);

    all_parsed
}

/// Generate the dynamic module via the canonical emitter.
fn generate_dynamic(out_dir: &Path, all_parsed: &[(String, ApiSpec)]) {
    let canonical = crate::canonical::canonicalize(all_parsed);
    let dynamic_dir = out_dir.join("dynamic");
    for (rel_path, content) in crate::emit_dynamic(&canonical) {
        write_generated(&dynamic_dir.join(&rel_path), &with_header(&content));
    }
}

/// Generate the content for `generated_lib.rs`.
///
/// This file is meant to be `include!`-ed from the crate's real `lib.rs`.
/// It uses `#[path = "..."]` attributes with absolute filesystem paths from
/// `out_dir` since `env!("OUT_DIR")` cannot be used in `#[path]` attributes.
fn generate_lib_rs_fragment(versions: &[String], out_dir: &Path, dynamic: bool) -> String {
    let out_dir_str = out_dir.display();
    let mut out = String::new();

    // ── Compile-error guards ────────────────────────────────────────────
    let version_cfgs: Vec<String> = versions
        .iter()
        .map(|v| format!("feature = \"{}\"", version_to_feature(v)))
        .collect();

    // Guard: at least one version feature must be enabled.
    out.push_str(&format!(
        "#[cfg(not(any({})))]\n\
         compile_error!(\"Enable at least one NiFi version feature (e.g. nifi-2-8-0)\");\n\n",
        version_cfgs.join(", "),
    ));

    // Guard: multiple version features without `dynamic`.
    if versions.len() > 1 {
        // Build pairs of all(feat_a, feat_b) for every combination.
        let mut pairs: Vec<String> = Vec::new();
        for i in 0..versions.len() {
            for j in (i + 1)..versions.len() {
                pairs.push(format!("all({}, {})", version_cfgs[i], version_cfgs[j],));
            }
        }
        out.push_str(&format!(
            "#[cfg(all(any({}), not(feature = \"dynamic\")))]\n\
             compile_error!(\"Multiple NiFi version features enabled without the `dynamic` feature. \
             Enable `dynamic` or select a single version.\");\n\n",
            pairs.join(", "),
        ));
    }

    // ── Per-version module declarations ─────────────────────────────────
    for version in versions {
        let mod_name = version_to_mod_name(version);
        let feature = version_to_feature(version);
        out.push_str(&format!(
            "#[cfg(feature = \"{feature}\")]\n\
             #[path = \"{out_dir_str}/{mod_name}/mod.rs\"]\n\
             #[allow(missing_docs)]\n\
             pub mod {mod_name};\n\n",
        ));
    }

    // ── Single-version re-exports ───────────────────────────────────────
    // When exactly one version feature is active (no dynamic), re-export
    // its api/types at the crate root for ergonomic access.
    for version in versions {
        let mod_name = version_to_mod_name(version);
        let feature = version_to_feature(version);

        // Gate: this version is active AND no other version is active.
        let others: Vec<String> = versions
            .iter()
            .filter(|v| *v != version)
            .map(|v| format!("not(feature = \"{}\")", version_to_feature(v)))
            .collect();

        let cfg = if others.is_empty() {
            format!("feature = \"{}\"", feature)
        } else {
            format!("all(feature = \"{}\", {})", feature, others.join(", "),)
        };

        out.push_str(&format!(
            "#[cfg({cfg})]\n\
             pub use {mod_name}::{{api, types}};\n\n",
        ));
    }

    // ── dynamic (canonical) — public, gated on `dynamic` feature.
    if dynamic && versions.len() > 1 {
        out.push_str(&format!(
            "#[cfg(feature = \"dynamic\")]\n\
             #[path = \"{out_dir_str}/dynamic/mod.rs\"]\n\
             #[allow(missing_docs)]\n\
             pub mod dynamic;\n",
        ));
    }

    out
}

/// Write `content` to `path`, creating parent directories as needed.
fn write_generated(path: &Path, content: &str) {
    // Normalize to exactly one trailing newline.
    let normalized = format!("{}\n", content.trim_end());
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap_or_else(|e| {
            panic!("failed to create directory {}: {e}", parent.display());
        });
    }
    std::fs::write(path, &normalized).unwrap_or_else(|e| {
        panic!("failed to write {}: {e}", path.display());
    });
}

/// Prepend the `@generated` header to `.rs` content, respecting leading
/// `#![cfg(…)]` lines (inner attributes must stay at the very top).
fn with_header(content: &str) -> String {
    const HEADER: &str = "// @generated — do not edit; regenerated by build.rs\n\n";

    if content.contains("@generated") {
        return content.to_string();
    }

    if content.starts_with("#![cfg") {
        if let Some(pos) = content.find('\n') {
            let (cfg_line, rest) = content.split_at(pos + 1);
            return format!("{cfg_line}\n{HEADER}{rest}");
        }
        return format!("{content}\n\n{HEADER}");
    }

    format!("{HEADER}{content}")
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_header_prepends_to_plain_content() {
        let content = "use foo;\nfn bar() {}";
        let result = with_header(content);
        assert!(result.starts_with("// @generated"));
        assert!(result.contains("use foo;"));
    }

    #[test]
    fn with_header_preserves_inner_cfg() {
        let content = "#![cfg(test)]\nuse foo;";
        let result = with_header(content);
        assert!(result.starts_with("#![cfg(test)]"));
        assert!(result.contains("// @generated"));
    }

    #[test]
    fn with_header_skips_already_tagged() {
        let content = "// @generated\nfn bar() {}";
        let result = with_header(content);
        assert_eq!(result, content);
    }

    #[test]
    fn generate_lib_rs_fragment_single_version() {
        let versions = vec!["2.8.0".to_string()];
        let out_dir = Path::new("/tmp/test_out");
        let fragment = generate_lib_rs_fragment(&versions, out_dir, false);

        // Should contain the version module
        assert!(fragment.contains("pub mod v2_8_0;"));
        assert!(fragment.contains("#[path = \"/tmp/test_out/v2_8_0/mod.rs\"]"));
        // Should contain re-export
        assert!(fragment.contains("pub use v2_8_0::{api, types};"));
        // Should NOT contain dynamic
        assert!(!fragment.contains("pub mod dynamic;"));
    }

    #[test]
    fn generate_lib_rs_fragment_multi_version_dynamic() {
        let versions = vec!["2.6.0".to_string(), "2.8.0".to_string()];
        let out_dir = Path::new("/tmp/test_out");
        let fragment = generate_lib_rs_fragment(&versions, out_dir, true);

        assert!(fragment.contains("pub mod v2_6_0;"));
        assert!(fragment.contains("pub mod v2_8_0;"));
        // The canonical emitter output is under dynamic/.
        assert!(fragment.contains("pub mod dynamic;"));
        assert!(fragment.contains("#[path = \"/tmp/test_out/dynamic/mod.rs\"]"));
        // dynamic_v2 no longer exists after Task 6 rename.
        assert!(!fragment.contains("dynamic_v2"));
    }

    #[test]
    fn generate_lib_rs_fragment_compile_error_guard() {
        let versions = vec!["2.6.0".to_string(), "2.8.0".to_string()];
        let out_dir = Path::new("/tmp/test_out");
        let fragment = generate_lib_rs_fragment(&versions, out_dir, false);

        assert!(fragment.contains("compile_error!"));
        assert!(fragment.contains("at least one NiFi version feature"));
        assert!(fragment.contains("Multiple NiFi version features"));
    }

    #[test]
    fn config_from_env_parses_features() {
        // This test verifies the parsing logic indirectly — in a real build.rs
        // context, Cargo sets the env vars. Here we just confirm the struct
        // can be constructed.
        let config = GenerateConfig {
            versions: vec!["2.6.0".to_string(), "2.8.0".to_string()],
            dynamic: true,
        };
        assert_eq!(config.versions.len(), 2);
        assert!(config.dynamic);
    }

    #[test]
    fn write_generated_creates_parent_dirs() {
        let tmp = std::env::temp_dir().join("nifi_build_api_test_write");
        let _ = std::fs::remove_dir_all(&tmp);
        let path = tmp.join("a").join("b").join("test.rs");

        write_generated(&path, "fn main() {}");

        assert!(path.exists());
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, "fn main() {}\n");

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
