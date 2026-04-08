use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::parser::ApiSpec;
use crate::util::version_to_mod_name;

use super::lib_rs::discover_versions;

/// One-time migration: remove the old flat `src/api/`, `src/types/`, and
/// `tests/generated_tests.rs` layout if still present.
pub fn migrate_flat_layout(client: &Path) {
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

/// Remove stale `.rs` files in generated directories that are not in the written set.
///
/// Cleans up:
/// - Legacy `src/dynamic/types.rs` and `src/dynamic/conversions.rs`
/// - Stale files in `src/dynamic/{traits,dispatch}/`
/// - Stale files in `src/dynamic/impls/` (including per-version subdirectories)
/// - Stale files in per-version `src/v*/types/` and `src/v*/api/` directories
pub fn remove_stale_generated_files(
    client: &Path,
    written_paths: &HashSet<PathBuf>,
    parsed_versions: &[(String, ApiSpec)],
) {
    // Legacy src/dynamic/types.rs now that types live in src/dynamic/types/
    {
        let legacy = client.join("src/dynamic/types.rs");
        if legacy.exists() && !written_paths.contains(&legacy) {
            std::fs::remove_file(&legacy).expect("remove legacy types.rs");
            println!("  removed legacy {}", legacy.display());
        }
    }

    // Legacy src/dynamic/conversions.rs now that conversions live in src/dynamic/conversions/
    {
        let legacy = client.join("src/dynamic/conversions.rs");
        if legacy.exists() && !written_paths.contains(&legacy) {
            std::fs::remove_file(&legacy).expect("remove legacy conversions.rs");
            println!("  removed legacy {}", legacy.display());
        }
    }

    // Delete stale .rs files in dynamic subdirectories
    for subdir in ["traits", "dispatch"] {
        let dir = client.join("src/dynamic").join(subdir);
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("rs")
                    && !written_paths.contains(&path)
                {
                    std::fs::remove_file(&path).expect("remove stale dynamic file");
                    println!("  removed stale {}", path.display());
                }
            }
        }
    }

    // impls/ has per-version subdirectories
    {
        let impls_dir = client.join("src/dynamic/impls");
        if let Ok(entries) = std::fs::read_dir(&impls_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // per-version subdir: clean stale .rs files inside
                    if let Ok(sub_entries) = std::fs::read_dir(&path) {
                        for sub_entry in sub_entries.flatten() {
                            let sub_path = sub_entry.path();
                            if sub_path.extension().and_then(|e| e.to_str()) == Some("rs")
                                && !written_paths.contains(&sub_path)
                            {
                                std::fs::remove_file(&sub_path)
                                    .expect("remove stale dynamic impls file");
                                println!("  removed stale {}", sub_path.display());
                            }
                        }
                    }
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs")
                    && !written_paths.contains(&path)
                {
                    std::fs::remove_file(&path).expect("remove stale dynamic impls file");
                    println!("  removed stale {}", path.display());
                }
            }
        }
    }

    // Delete stale .rs files in per-version dirs
    for (version_str, _) in parsed_versions {
        let mod_name = version_to_mod_name(version_str);
        let versioned_src = client.join("src").join(&mod_name);
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
    }
}

/// Delete entire version dirs (`src/v*/` and `tests/v*_generated_tests.rs`) for versions
/// that exist on disk but are no longer in the spec list.
///
/// Only meaningful in full-discovery mode (no `NIFI_VERSION` override).
pub fn remove_stale_version_dirs(client: &Path, spec_versions: &[String]) {
    let on_disk_versions = discover_versions(&client.join("src"));
    let active: HashSet<&str> = spec_versions.iter().map(String::as_str).collect();
    for version_str in &on_disk_versions {
        if !active.contains(version_str.as_str()) {
            let mod_name = version_to_mod_name(version_str);
            let versioned_src = client.join("src").join(&mod_name);
            if versioned_src.exists() {
                std::fs::remove_dir_all(&versioned_src)
                    .unwrap_or_else(|_| panic!("remove {}", versioned_src.display()));
                println!("  removed stale version dir {}", versioned_src.display());
            }
            let test_file = client.join(format!(
                "tests/v{}_generated_tests.rs",
                version_str.replace('.', "_")
            ));
            if test_file.exists() {
                std::fs::remove_file(&test_file)
                    .unwrap_or_else(|_| panic!("remove {}", test_file.display()));
                println!("  removed stale test file {}", test_file.display());
            }
        }
    }
}
