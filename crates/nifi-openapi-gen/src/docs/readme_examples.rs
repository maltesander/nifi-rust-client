use crate::layout::RepoLayout;
use crate::plan::FileEdit;
use crate::util::{version_to_feature, version_to_mod_name};

/// Returns `ReplaceBlock` edits for the version/feature examples in `lib.rs` doc comments.
pub fn emit_lib_rs_examples(
    layout: &RepoLayout,
    latest: &str,
    crate_version: &str,
) -> Vec<FileEdit> {
    let feature = version_to_feature(latest);

    vec![
        FileEdit::ReplaceBlock {
            path: layout.client_lib_rs.clone(),
            start_marker: "<!-- LIB_STATIC_FEATURE_EXAMPLE_START -->".into(),
            end_marker: "<!-- LIB_STATIC_FEATURE_EXAMPLE_END -->".into(),
            content: format!(
                "//! ```toml\n//! [dependencies]\n//! nifi-rust-client = {{ version = \"{crate_version}\", features = [\"{feature}\"] }}\n//! ```"
            ),
        },
        FileEdit::ReplaceBlock {
            path: layout.client_lib_rs.clone(),
            start_marker: "<!-- LIB_DYNAMIC_FEATURE_EXAMPLE_START -->".into(),
            end_marker: "<!-- LIB_DYNAMIC_FEATURE_EXAMPLE_END -->".into(),
            content: format!(
                "//! ```toml\n//! [dependencies]\n//! nifi-rust-client = {{ version = \"{crate_version}\", features = [\"dynamic\"] }}\n//! ```"
            ),
        },
    ]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::RepoLayout;
    use crate::plan::FileEdit;
    use std::path::Path;

    #[test]
    fn emit_lib_rs_examples_returns_two_replace_blocks() {
        let layout = RepoLayout::from_workspace_root(Path::new("/fake"));
        let edits = emit_lib_rs_examples(&layout, "2.8.0", "0.7");
        assert_eq!(edits.len(), 2);

        for edit in &edits {
            assert!(matches!(edit, FileEdit::ReplaceBlock { path, .. }
                if *path == Path::new("/fake/crates/nifi-rust-client/src/lib.rs")
            ));
        }

        let markers: Vec<&str> = edits
            .iter()
            .map(|e| match e {
                FileEdit::ReplaceBlock { start_marker, .. } => start_marker.as_str(),
                _ => panic!("expected ReplaceBlock"),
            })
            .collect();
        assert!(markers.contains(&"<!-- LIB_STATIC_FEATURE_EXAMPLE_START -->"));
        assert!(markers.contains(&"<!-- LIB_DYNAMIC_FEATURE_EXAMPLE_START -->"));
    }

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
