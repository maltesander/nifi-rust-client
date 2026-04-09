use std::path::Path;

use crate::util::version_to_feature;

use super::lib_rs::discover_versions;

/// Patch the client crate's `Cargo.toml` `[features]` section to include all version features,
/// a `default` pointing to the latest, and a `dynamic` that depends on all versions.
pub fn patch_client_cargo_features(toml_str: &str, versions: &[&str]) -> String {
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

    features.sort_values();
    doc.to_string()
}

/// Patch the integration tests crate's `Cargo.toml` `[features]` section to forward version
/// features to the client crate dependency.
pub fn patch_tests_cargo_features(toml_str: &str, versions: &[&str]) -> String {
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

    // dynamic feature — forwards to client crate and enables all version features
    let mut dynamic_arr = toml_edit::Array::new();
    dynamic_arr.push("nifi-rust-client/dynamic");
    for version in versions {
        dynamic_arr.push(version_to_feature(version).as_str());
    }
    features.insert("dynamic", toml_edit::value(dynamic_arr));

    features.sort_values();
    doc.to_string()
}

/// Discover versions from `client_crate/src` and patch the client crate's `Cargo.toml` features.
pub fn update_cargo_features_client(client_crate: &Path) {
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

/// Discover versions from `client_src` and patch the tests crate's `Cargo.toml` features.
pub fn update_cargo_features_tests(client_src: &Path, tests_crate: &Path) {
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_patch_tests_cargo_features_adds_forwarding() {
        let toml = "[package]\nname = \"nifi-integration-tests\"\n";
        let result = patch_tests_cargo_features(toml, &["2.8.0"]);
        assert!(result.contains("[features]"));
        assert!(result.contains("default = [\"nifi-2-8-0\"]"));
        assert!(result.contains("nifi-2-8-0 = [\"nifi-rust-client/nifi-2-8-0\"]"));
    }

    #[test]
    fn test_patch_tests_cargo_features_includes_dynamic() {
        let toml = "[package]\nname = \"nifi-integration-tests\"\n";
        let result = patch_tests_cargo_features(toml, &["2.7.2", "2.8.0"]);
        assert!(
            result.contains(
                "dynamic = [\"nifi-rust-client/dynamic\", \"nifi-2-7-2\", \"nifi-2-8-0\"]"
            ),
            "dynamic feature should include all version features. Got:\n{result}"
        );
    }
}
