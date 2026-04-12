use std::path::Path;

use crate::layout::RepoLayout;
use crate::plan::FileEdit;
use crate::util::{update_file_between_markers, version_to_feature, version_to_mod_name};

fn read_crate_version_shorthand(workspace_root: &Path) -> String {
    let cargo_toml = workspace_root.join("crates/nifi-rust-client/Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml)
        .unwrap_or_else(|_| panic!("read {}", cargo_toml.display()));
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("version = \"") {
            if let Some(ver) = rest.strip_suffix('"') {
                let parts: Vec<&str> = ver.splitn(3, '.').collect();
                if parts.len() >= 2 {
                    return format!("{}.{}", parts[0], parts[1]);
                }
            }
        }
    }
    panic!("could not find version in {}", cargo_toml.display());
}

pub fn emit_client_readme_examples(
    layout: &RepoLayout,
    latest: &str,
    crate_version: &str,
) -> Vec<FileEdit> {
    let feature = version_to_feature(latest);
    let mod_name = version_to_mod_name(latest);

    vec![
        FileEdit::ReplaceBlock {
            path: layout.client_readme.clone(),
            start_marker: "<!-- STATIC_FEATURE_EXAMPLE_START -->".into(),
            end_marker: "<!-- STATIC_FEATURE_EXAMPLE_END -->".into(),
            content: format!(
                "```toml\n[dependencies]\nnifi-rust-client = {{ version = \"{crate_version}\", features = [\"{feature}\"] }}\n```"
            ),
        },
        FileEdit::ReplaceBlock {
            path: layout.client_readme.clone(),
            start_marker: "<!-- STATIC_RUST_EXAMPLE_START -->".into(),
            end_marker: "<!-- STATIC_RUST_EXAMPLE_END -->".into(),
            content: format!(
                "```rust\nlet client = NifiClientBuilder::new(\"https://nifi:8443\")?.build()?;\nclient.login(\"admin\", \"password\").await?;\n\n// Full type safety \u{2014} ProcessorEntity is {mod_name}::types::ProcessorEntity\nlet proc = client.processors_api().get_processor(\"id\").await?;\n```"
            ),
        },
        FileEdit::ReplaceBlock {
            path: layout.client_readme.clone(),
            start_marker: "<!-- DYNAMIC_FEATURE_EXAMPLE_START -->".into(),
            end_marker: "<!-- DYNAMIC_FEATURE_EXAMPLE_END -->".into(),
            content: format!(
                "```toml\n[dependencies]\nnifi-rust-client = {{ version = \"{crate_version}\", features = [\"dynamic\"] }}\n```"
            ),
        },
    ]
}

pub fn update_client_readme_examples(workspace_root: &Path, latest: &str) {
    let crate_readme = workspace_root.join("crates/nifi-rust-client/README.md");
    let feature = version_to_feature(latest);
    let mod_name = version_to_mod_name(latest);
    let crate_version = read_crate_version_shorthand(workspace_root);

    update_file_between_markers(
        &crate_readme,
        "<!-- STATIC_FEATURE_EXAMPLE_START -->",
        "<!-- STATIC_FEATURE_EXAMPLE_END -->",
        &format!(
            "```toml\n[dependencies]\nnifi-rust-client = {{ version = \"{crate_version}\", features = [\"{feature}\"] }}\n```"
        ),
    );
    update_file_between_markers(
        &crate_readme,
        "<!-- STATIC_RUST_EXAMPLE_START -->",
        "<!-- STATIC_RUST_EXAMPLE_END -->",
        &format!(
            "```rust\nlet client = NifiClientBuilder::new(\"https://nifi:8443\")?.build()?;\nclient.login(\"admin\", \"password\").await?;\n\n// Full type safety \u{2014} ProcessorEntity is {mod_name}::types::ProcessorEntity\nlet proc = client.processors_api().get_processor(\"id\").await?;\n```"
        ),
    );
    update_file_between_markers(
        &crate_readme,
        "<!-- DYNAMIC_FEATURE_EXAMPLE_START -->",
        "<!-- DYNAMIC_FEATURE_EXAMPLE_END -->",
        &format!(
            "```toml\n[dependencies]\nnifi-rust-client = {{ version = \"{crate_version}\", features = [\"dynamic\"] }}\n```"
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::RepoLayout;
    use crate::plan::FileEdit;
    use std::path::Path;

    #[test]
    fn emit_client_readme_examples_returns_three_replace_blocks() {
        let layout = RepoLayout::from_workspace_root(Path::new("/fake"));
        let edits = emit_client_readme_examples(&layout, "2.8.0", "0.5");
        assert_eq!(edits.len(), 3);

        for edit in &edits {
            assert!(matches!(edit, FileEdit::ReplaceBlock { path, .. }
                if *path == Path::new("/fake/crates/nifi-rust-client/README.md")
            ));
        }

        let markers: Vec<&str> = edits
            .iter()
            .map(|e| match e {
                FileEdit::ReplaceBlock { start_marker, .. } => start_marker.as_str(),
                _ => panic!("expected ReplaceBlock"),
            })
            .collect();
        assert!(markers.contains(&"<!-- STATIC_FEATURE_EXAMPLE_START -->"));
        assert!(markers.contains(&"<!-- STATIC_RUST_EXAMPLE_START -->"));
        assert!(markers.contains(&"<!-- DYNAMIC_FEATURE_EXAMPLE_START -->"));
    }
}
