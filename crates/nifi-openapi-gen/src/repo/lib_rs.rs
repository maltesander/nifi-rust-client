use std::path::Path;

use semver::Version;

use crate::util::{sort_versions_semver, version_to_feature, version_to_mod_name};

/// Returns version strings (e.g. ["2.7.2", "2.8.0"]) for all src/v*/ dirs,
/// sorted semver-ascending.
pub fn discover_versions(client_src: &Path) -> Vec<String> {
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

/// Generate the content for `lib.rs` in the client crate, gated by version feature flags.
pub fn generate_lib_rs_content(versions: &[&str]) -> String {
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
    out.push_str("pub mod credentials;\n");
    out.push_str("pub mod error;\n");
    out.push_str("pub mod retry;\n");
    out.push_str("pub use builder::NifiClientBuilder;\n");
    out.push_str("pub use credentials::CredentialProvider;\n");
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

/// Discover versions from `client_src` and write `lib.rs` if its content changed.
pub fn update_lib_rs(client_src: &Path) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_lib_rs_single_version() {
        let content = generate_lib_rs_content(&["2.8.0"]);
        assert!(content.contains("pub mod builder;"));
        assert!(content.contains("pub mod client;"));
        assert!(content.contains("pub mod credentials;"));
        assert!(content.contains("pub mod error;"));
        assert!(content.contains("pub mod retry;"));
        assert!(content.contains("pub use builder::NifiClientBuilder;"));
        assert!(content.contains("pub use credentials::CredentialProvider;"));
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
        assert!(content.contains("Multiple NiFi versions require the \\\"dynamic\\\" feature"));
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
}
