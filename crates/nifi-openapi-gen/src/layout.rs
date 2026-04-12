use std::path::{Path, PathBuf};

/// Centralises every repo path that the generator's repo-maintenance
/// pipeline touches. Constructed once, passed by reference to emitters.
#[derive(Debug, Clone)]
pub struct RepoLayout {
    pub workspace_root: PathBuf,
    pub client_cargo_toml: PathBuf,
    pub tests_cargo_toml: PathBuf,
    pub workspace_readme: PathBuf,
    pub client_readme: PathBuf,
    pub client_lib_rs: PathBuf,
    pub docker_compose: PathBuf,
    pub api_changes_md: PathBuf,
}

impl RepoLayout {
    /// Build a layout from an explicit workspace root.
    pub fn from_workspace_root(root: &Path) -> Self {
        let client = root.join("crates/nifi-rust-client");
        Self {
            workspace_root: root.to_path_buf(),
            client_cargo_toml: client.join("Cargo.toml"),
            tests_cargo_toml: root.join("tests/Cargo.toml"),
            workspace_readme: root.join("README.md"),
            client_readme: client.join("README.md"),
            client_lib_rs: client.join("src/lib.rs"),
            docker_compose: root.join("tests/docker-compose.yml"),
            api_changes_md: root.join("NIFI_API_CHANGES.md"),
        }
    }

    /// Discover the layout from `CARGO_MANIFEST_DIR` (the nifi-openapi-gen crate).
    pub fn discover() -> Self {
        let codegen_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = codegen_dir
            .parent()
            .expect("crates/")
            .parent()
            .expect("workspace root");
        Self::from_workspace_root(workspace_root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn discover_resolves_all_paths_from_workspace_root() {
        let layout = RepoLayout::from_workspace_root(Path::new("/fake/workspace"));
        assert_eq!(layout.workspace_root, Path::new("/fake/workspace"));
        assert_eq!(layout.client_cargo_toml, Path::new("/fake/workspace/crates/nifi-rust-client/Cargo.toml"));
        assert_eq!(layout.tests_cargo_toml, Path::new("/fake/workspace/tests/Cargo.toml"));
        assert_eq!(layout.workspace_readme, Path::new("/fake/workspace/README.md"));
        assert_eq!(layout.client_readme, Path::new("/fake/workspace/crates/nifi-rust-client/README.md"));
        assert_eq!(layout.client_lib_rs, Path::new("/fake/workspace/crates/nifi-rust-client/src/lib.rs"));
        assert_eq!(layout.docker_compose, Path::new("/fake/workspace/tests/docker-compose.yml"));
        assert_eq!(layout.api_changes_md, Path::new("/fake/workspace/NIFI_API_CHANGES.md"));
    }
}
