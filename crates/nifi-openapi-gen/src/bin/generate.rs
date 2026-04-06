//! Code generator binary — writes generated files in nifi-rust-client.
//!
//! Usage:
//!   cargo run -p nifi-openapi-gen                     # auto-detect spec version
//!   NIFI_VERSION=2.8.0 cargo run -p nifi-openapi-gen  # pin a specific version
//!   NIFI_API_SPEC=/path/to/spec.json cargo run -p nifi-openapi-gen  # full path override

use semver::Version;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

fn version_to_mod_name(version: &str) -> String {
    format!("v{}", version.replace('.', "_"))
}

fn version_to_feature(version: &str) -> String {
    format!("nifi-{}", version.replace('.', "-"))
}

fn sort_versions_semver(versions: &mut [String]) {
    versions.sort_by(|a, b| {
        Version::parse(a)
            .unwrap_or_else(|_| Version::new(0, 0, 0))
            .cmp(&Version::parse(b).unwrap_or_else(|_| Version::new(0, 0, 0)))
    });
}

fn generate_lib_rs_content(versions: &[&str]) -> String {
    let features: Vec<String> = versions.iter().map(|v| version_to_feature(v)).collect();
    let mod_names: Vec<String> = versions.iter().map(|v| version_to_mod_name(v)).collect();
    let latest_feature = features
        .last()
        .cloned()
        .unwrap_or_else(|| "nifi-2-8-0".to_string());

    let mut out = String::new();
    out.push_str("// @generated — do not edit; run `cargo run -p nifi-openapi-gen`\n\n");
    out.push_str("pub mod builder;\n");
    out.push_str("pub mod client;\n");
    out.push_str("pub mod error;\n");
    out.push_str("pub use builder::NifiClientBuilder;\n");
    out.push_str("pub use client::NifiClient;\n");
    out.push_str("pub use error::NifiError;\n\n");

    // At-least-one guard
    let any_list = features
        .iter()
        .map(|f| format!("feature = \"{f}\""))
        .collect::<Vec<_>>()
        .join(", ");
    out.push_str(&format!("#[cfg(not(any({any_list})))]\n"));
    out.push_str(&format!(
        "compile_error!(\"Enable at least one NiFi version feature, e.g. features = [\\\"{}\\\"]\");\n\n",
        latest_feature
    ));

    // Multi-version guards: only allowed with dynamic feature
    for i in 0..features.len() {
        let others: Vec<String> = features
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, f)| format!("feature = \"{f}\""))
            .collect();
        if !others.is_empty() {
            let others_any = others.join(", ");
            out.push_str(&format!(
                "#[cfg(all(feature = \"{}\", any({}), not(feature = \"dynamic\")))]\n",
                features[i], others_any
            ));
            out.push_str(
                "compile_error!(\"Multiple NiFi versions require the \\\"dynamic\\\" feature\");\n\n",
            );
        }
    }

    // Version modules — pub mod so dynamic users can reach version-specific types
    for (feature, mod_name) in features.iter().zip(mod_names.iter()) {
        out.push_str(&format!(
            "#[cfg(feature = \"{feature}\")]\npub mod {mod_name};\n"
        ));
    }
    out.push('\n');

    // Static re-exports (single-version mode — not dynamic)
    for (feature, mod_name) in features.iter().zip(mod_names.iter()) {
        let other_not: String = features
            .iter()
            .filter(|f| *f != feature)
            .map(|f| format!(", not(feature = \"{f}\")"))
            .collect();
        let use_cfg = if other_not.is_empty() {
            format!("all(feature = \"{feature}\", not(feature = \"dynamic\"))")
        } else {
            format!("all(feature = \"{feature}\", not(feature = \"dynamic\"){other_not})")
        };
        out.push_str(&format!(
            "#[cfg({use_cfg})]\npub use {mod_name}::{{api, types}};\n\n"
        ));
    }

    // Dynamic module
    out.push_str("#[cfg(feature = \"dynamic\")]\npub mod dynamic;\n");

    out
}

/// Returns version strings (e.g. ["2.7.2", "2.8.0"]) for all src/v*/ dirs,
/// sorted semver-ascending.
fn discover_versions(client_src: &Path) -> Vec<String> {
    let mut versions: Vec<String> = std::fs::read_dir(client_src)
        .unwrap_or_else(|_| panic!("Cannot read {}", client_src.display()))
        .flatten()
        .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .filter_map(|e| {
            let name = e.file_name();
            let s = name.to_str()?;
            let ver = s.strip_prefix('v')?.replace('_', ".");
            Version::parse(&ver).ok()?;
            Some(ver)
        })
        .collect();
    sort_versions_semver(&mut versions);
    versions
}

fn update_lib_rs(client_src: &Path) {
    let versions = discover_versions(client_src);
    let version_strs: Vec<&str> = versions.iter().map(String::as_str).collect();
    let content = generate_lib_rs_content(&version_strs);
    let lib_path = client_src.join("lib.rs");
    let on_disk = std::fs::read_to_string(&lib_path).unwrap_or_default();
    if on_disk != content {
        std::fs::write(&lib_path, &content).expect("write lib.rs");
        println!("  wrote {}", lib_path.display());
    }
}

fn update_cargo_features_client(client_crate: &Path) {
    let versions = discover_versions(&client_crate.join("src"));
    let version_strs: Vec<&str> = versions.iter().map(String::as_str).collect();
    let cargo_path = client_crate.join("Cargo.toml");
    let on_disk = std::fs::read_to_string(&cargo_path).expect("read client Cargo.toml");
    let patched = patch_client_cargo_features(&on_disk, &version_strs);
    if on_disk != patched {
        std::fs::write(&cargo_path, &patched).expect("write client Cargo.toml");
        println!("  wrote {}", cargo_path.display());
    }
}

fn update_cargo_features_tests(client_src: &Path, tests_crate: &Path) {
    let versions = discover_versions(client_src);
    let version_strs: Vec<&str> = versions.iter().map(String::as_str).collect();
    let cargo_path = tests_crate.join("Cargo.toml");
    let on_disk = std::fs::read_to_string(&cargo_path).expect("read tests Cargo.toml");
    let patched = patch_tests_cargo_features(&on_disk, &version_strs);
    if on_disk != patched {
        std::fs::write(&cargo_path, &patched).expect("write tests Cargo.toml");
        println!("  wrote {}", cargo_path.display());
    }
}

fn migrate_flat_layout(client: &Path) {
    let old_api = client.join("src/api");
    let old_types = client.join("src/types");
    let old_tests = client.join("tests/generated_tests.rs");

    for dir in [&old_api, &old_types] {
        if dir.exists() {
            std::fs::remove_dir_all(dir).unwrap_or_else(|_| panic!("remove {}", dir.display()));
            println!("  migrated (removed old) {}", dir.display());
        }
    }
    if old_tests.exists() {
        std::fs::remove_file(&old_tests)
            .unwrap_or_else(|_| panic!("remove {}", old_tests.display()));
        println!("  migrated (removed old) {}", old_tests.display());
    }
}

fn patch_client_cargo_features(toml_str: &str, versions: &[&str]) -> String {
    let mut doc = toml_str
        .parse::<toml_edit::DocumentMut>()
        .expect("valid Cargo.toml");

    if !doc.contains_key("features") {
        doc["features"] = toml_edit::Item::Table(toml_edit::Table::new());
    }
    let features = doc["features"]
        .as_table_mut()
        .expect("[features] is a table");

    for version in versions {
        let key = version_to_feature(version);
        if !features.contains_key(&key) {
            features.insert(&key, toml_edit::value(toml_edit::Array::new()));
        }
    }

    if let Some(latest) = versions.last() {
        let mut arr = toml_edit::Array::new();
        arr.push(version_to_feature(latest).as_str());
        features.insert("default", toml_edit::value(arr));
    }

    // dynamic feature — depends on all version features
    let mut dynamic_arr = toml_edit::Array::new();
    for version in versions {
        dynamic_arr.push(version_to_feature(version).as_str());
    }
    features.insert("dynamic", toml_edit::value(dynamic_arr));

    doc.to_string()
}

fn patch_tests_cargo_features(toml_str: &str, versions: &[&str]) -> String {
    let mut doc = toml_str
        .parse::<toml_edit::DocumentMut>()
        .expect("valid Cargo.toml");

    if !doc.contains_key("features") {
        doc["features"] = toml_edit::Item::Table(toml_edit::Table::new());
    }
    let features = doc["features"]
        .as_table_mut()
        .expect("[features] is a table");

    for version in versions {
        let key = version_to_feature(version);
        if !features.contains_key(&key) {
            let mut arr = toml_edit::Array::new();
            arr.push(format!("nifi-rust-client/{key}").as_str());
            features.insert(&key, toml_edit::value(arr));
        }
    }

    if let Some(latest) = versions.last() {
        let mut arr = toml_edit::Array::new();
        arr.push(version_to_feature(latest).as_str());
        features.insert("default", toml_edit::value(arr));
    }

    // dynamic feature — forwards to client crate
    let mut dynamic_arr = toml_edit::Array::new();
    dynamic_arr.push("nifi-rust-client/dynamic");
    features.insert("dynamic", toml_edit::value(dynamic_arr));

    doc.to_string()
}

fn main() {
    let codegen_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = codegen_dir
        .parent()
        .expect("crates/")
        .parent()
        .expect("workspace root");

    let spec_path = find_spec_path(&codegen_dir.join("specs"));
    let client = workspace_root.join("crates/nifi-rust-client");

    let spec = nifi_openapi_gen::load(spec_path.to_str().expect("UTF-8 path"));

    let version_str = spec_path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .expect("version directory name from spec path");
    let mod_name = version_to_mod_name(version_str);
    let feature_name = version_to_feature(version_str);
    let versioned_src = client.join("src").join(&mod_name);

    let mut targets: Vec<(PathBuf, String)> = vec![];

    // Types: one file per tag + common.rs + mod.rs
    for (filename, content) in nifi_openapi_gen::emit_types(&spec) {
        targets.push((versioned_src.join("types").join(&filename), content));
    }

    // API: mod.rs + one file per tag
    for (filename, content) in nifi_openapi_gen::emit_api(&spec) {
        targets.push((versioned_src.join("api").join(&filename), content));
    }

    // Version module entry point
    targets.push((
        versioned_src.join("mod.rs"),
        "pub mod api;\npub mod types;\n".to_string(),
    ));

    // Wiremock test stubs
    let test_content = format!(
        "#![cfg(feature = \"{feature_name}\")]\n\n{}",
        nifi_openapi_gen::emit_tests(&spec)
    );
    targets.push((
        client.join(format!(
            "tests/v{}_generated_tests.rs",
            version_str.replace('.', "_")
        )),
        test_content,
    ));

    // Write files that have changed.
    let mut written = 0usize;
    for (path, content) in &targets {
        let on_disk = std::fs::read_to_string(path).unwrap_or_default();
        if on_disk != *content {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).expect("create parent dirs");
            }
            std::fs::write(path, content).expect("write generated file");
            println!("  wrote {}", path.display());
            written += 1;
        }
    }

    // Delete stale .rs files in versioned types/ and api/ not produced by this run.
    let written_paths: HashSet<PathBuf> = targets.iter().map(|(p, _)| p.clone()).collect();
    for dir in [versioned_src.join("types"), versioned_src.join("api")] {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("rs")
                    && !written_paths.contains(&path)
                {
                    std::fs::remove_file(&path).expect("remove stale file");
                    println!("  removed stale {}", path.display());
                }
            }
        }
    }

    // One-time migration: remove old flat layout if present
    migrate_flat_layout(&client);

    // Update lib.rs and both Cargo.toml [features] sections
    let tests_crate = workspace_root.join("tests");
    update_lib_rs(&client.join("src"));
    update_cargo_features_client(&client);
    update_cargo_features_tests(&client.join("src"), &tests_crate);

    // Run cargo fmt so files are in canonical form.
    let status = std::process::Command::new("cargo")
        .args(["fmt", "-p", "nifi-rust-client"])
        .current_dir(workspace_root)
        .status()
        .expect("cargo fmt");

    if !status.success() {
        eprintln!("cargo fmt failed");
        std::process::exit(1);
    }

    println!("Done. {} file(s) updated.", written);
}

/// Locate the OpenAPI spec file. Resolution order:
///   1. `NIFI_API_SPEC` env var — full path to a spec file
///   2. `NIFI_VERSION` env var — version string → `specs/{version}/nifi-api.json`
///   3. Auto-detect — scan `specs/` for subdirectories; use the only one, or the
///      lexicographically latest if multiple (logs a warning)
fn find_spec_path(specs_dir: &Path) -> PathBuf {
    if let Ok(path) = std::env::var("NIFI_API_SPEC") {
        return PathBuf::from(path);
    }
    if let Ok(version) = std::env::var("NIFI_VERSION") {
        return specs_dir.join(&version).join("nifi-api.json");
    }
    let mut version_dirs: Vec<PathBuf> = std::fs::read_dir(specs_dir)
        .unwrap_or_else(|_| {
            panic!(
                "Cannot read specs dir {}.\nRun: ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh",
                specs_dir.display()
            )
        })
        .flatten()
        .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .map(|e| e.path())
        .collect();
    version_dirs.sort_by(|a, b| {
        let parse = |p: &PathBuf| {
            let s = p.file_name().unwrap().to_str().unwrap();
            Version::parse(s).unwrap_or_else(|_| Version::new(0, 0, 0))
        };
        parse(a).cmp(&parse(b))
    });
    match version_dirs.len() {
        0 => panic!(
            "No version directories found in {}.\nRun: ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh",
            specs_dir.display()
        ),
        1 => version_dirs.remove(0).join("nifi-api.json"),
        _ => {
            let names: Vec<_> = version_dirs
                .iter()
                .map(|p| p.file_name().unwrap().to_str().unwrap())
                .collect();
            eprintln!(
                "Multiple NiFi specs found: {names:?} — using latest. Set NIFI_VERSION to override."
            );
            version_dirs.last().unwrap().join("nifi-api.json")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_to_mod_name() {
        assert_eq!(version_to_mod_name("2.8.0"), "v2_8_0");
        assert_eq!(version_to_mod_name("2.7.2"), "v2_7_2");
        assert_eq!(version_to_mod_name("2.10.0"), "v2_10_0");
    }

    #[test]
    fn test_version_to_feature() {
        assert_eq!(version_to_feature("2.8.0"), "nifi-2-8-0");
        assert_eq!(version_to_feature("2.7.2"), "nifi-2-7-2");
        assert_eq!(version_to_feature("2.10.0"), "nifi-2-10-0");
    }

    #[test]
    fn test_semver_sort_is_not_lexicographic() {
        // "2.10.0" sorts after "2.9.0" by semver but before by string
        let mut versions = vec![
            "2.10.0".to_string(),
            "2.9.0".to_string(),
            "2.7.2".to_string(),
        ];
        sort_versions_semver(&mut versions);
        assert_eq!(versions, vec!["2.7.2", "2.9.0", "2.10.0"]);
    }

    #[test]
    fn test_generate_lib_rs_single_version() {
        let content = generate_lib_rs_content(&["2.8.0"]);
        assert!(content.contains("pub mod builder;"));
        assert!(content.contains("pub mod client;"));
        assert!(content.contains("pub mod error;"));
        assert!(content.contains("pub use builder::NifiClientBuilder;"));
        assert!(content.contains("pub use client::NifiClient;"));
        assert!(content.contains("pub use error::NifiError;"));
        assert!(content.contains("#[cfg(not(any(feature = \"nifi-2-8-0\")))]"));
        assert!(content.contains("compile_error!"));
        assert!(content.contains("#[cfg(feature = \"nifi-2-8-0\")]"));
        assert!(content.contains("pub mod v2_8_0;"));
        assert!(content.contains("pub use v2_8_0::{api, types};"));
        // Dynamic module gate always present
        assert!(content.contains("#[cfg(feature = \"dynamic\")]"));
        assert!(content.contains("pub mod dynamic;"));
        // No multi-version conflict guard needed for a single version
        assert!(!content.contains("Multiple NiFi versions"));
    }

    #[test]
    fn test_generate_lib_rs_two_versions() {
        let content = generate_lib_rs_content(&["2.7.2", "2.8.0"]);
        // at-least-one guard covers both features
        assert!(content.contains("feature = \"nifi-2-7-2\""));
        assert!(content.contains("feature = \"nifi-2-8-0\""));
        // conflict guard references dynamic
        assert!(content.contains("not(feature = \"dynamic\")"));
        assert!(content.contains(
            "Multiple NiFi versions require the \\\"dynamic\\\" feature"
        ));
        // both version modules present as pub
        assert!(content.contains("pub mod v2_7_2;"));
        assert!(content.contains("pub mod v2_8_0;"));
        // compile_error hint references the latest version
        assert!(content.contains("nifi-2-8-0"));
        // dynamic module gate
        assert!(content.contains("pub mod dynamic;"));
    }

    #[test]
    fn test_generate_lib_rs_dynamic_feature() {
        let content = generate_lib_rs_content(&["2.7.2", "2.8.0"]);
        // Version modules are pub (for dynamic users to reach in)
        assert!(content.contains("pub mod v2_7_2;"));
        assert!(content.contains("pub mod v2_8_0;"));
        // Dynamic module gated
        assert!(content.contains("#[cfg(feature = \"dynamic\")]"));
        assert!(content.contains("pub mod dynamic;"));
        // Conflict guards allow multiple versions when dynamic is on
        assert!(content.contains("not(feature = \"dynamic\")"));
    }

    #[test]
    fn test_patch_client_cargo_features_adds_section() {
        let toml = "[package]\nname = \"nifi-rust-client\"\n";
        let result = patch_client_cargo_features(toml, &["2.8.0"]);
        assert!(result.contains("[features]"));
        assert!(result.contains("default = [\"nifi-2-8-0\"]"));
        assert!(result.contains("nifi-2-8-0 = []"));
    }

    #[test]
    fn test_patch_client_cargo_features_idempotent() {
        let toml = "[package]\nname = \"nifi-rust-client\"\n\n[features]\ndefault = [\"nifi-2-8-0\"]\nnifi-2-8-0 = []\n";
        let result = patch_client_cargo_features(toml, &["2.8.0"]);
        assert_eq!(result.matches("nifi-2-8-0 = []").count(), 1);
    }

    #[test]
    fn test_patch_client_cargo_features_bumps_default_to_latest() {
        let toml = "[package]\nname = \"nifi-rust-client\"\n\n[features]\ndefault = [\"nifi-2-8-0\"]\nnifi-2-8-0 = []\n";
        let result = patch_client_cargo_features(toml, &["2.8.0", "2.9.0"]);
        assert!(result.contains("default = [\"nifi-2-9-0\"]"));
        assert!(result.contains("nifi-2-8-0 = []"));
        assert!(result.contains("nifi-2-9-0 = []"));
    }

    #[test]
    fn test_patch_tests_cargo_features_adds_forwarding() {
        let toml = "[package]\nname = \"nifi-integration-tests\"\n";
        let result = patch_tests_cargo_features(toml, &["2.8.0"]);
        assert!(result.contains("[features]"));
        assert!(result.contains("default = [\"nifi-2-8-0\"]"));
        assert!(result.contains("nifi-2-8-0 = [\"nifi-rust-client/nifi-2-8-0\"]"));
    }

    #[test]
    fn test_patch_client_cargo_features_includes_dynamic() {
        let toml = "[package]\nname = \"nifi-rust-client\"\n";
        let result = patch_client_cargo_features(toml, &["2.7.2", "2.8.0"]);
        assert!(result.contains("dynamic = [\"nifi-2-7-2\", \"nifi-2-8-0\"]"));
    }

    #[test]
    fn test_patch_client_cargo_features_dynamic_updates_with_new_version() {
        let toml = "[package]\nname = \"nifi-rust-client\"\n\n[features]\ndefault = [\"nifi-2-8-0\"]\nnifi-2-7-2 = []\nnifi-2-8-0 = []\ndynamic = [\"nifi-2-7-2\", \"nifi-2-8-0\"]\n";
        let result = patch_client_cargo_features(toml, &["2.7.2", "2.8.0", "2.9.0"]);
        assert!(result.contains("default = [\"nifi-2-9-0\"]"));
        assert!(result.contains("nifi-2-9-0 = []"));
        // dynamic should include all three versions
        assert!(result.contains("nifi-2-7-2"));
        assert!(result.contains("nifi-2-8-0"));
        assert!(result.contains("nifi-2-9-0"));
    }

    #[test]
    fn test_patch_tests_cargo_features_includes_dynamic() {
        let toml = "[package]\nname = \"nifi-integration-tests\"\n";
        let result = patch_tests_cargo_features(toml, &["2.7.2", "2.8.0"]);
        assert!(result.contains("dynamic = [\"nifi-rust-client/dynamic\"]"));
    }
}
