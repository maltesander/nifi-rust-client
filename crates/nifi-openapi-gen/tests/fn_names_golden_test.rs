//! Regression test: for each committed `specs/<version>/nifi-api.json`,
//! parsing + naming post-pass must produce a `fn_names.txt` table that
//! matches the committed golden byte-for-byte.
//!
//! When this fails locally, rerun `cargo run -p nifi-openapi-gen` to
//! regenerate the goldens, then review the diff.

use std::path::PathBuf;

use nifi_openapi_gen::parser::load;
use nifi_openapi_gen::repo::format_fn_names_table;

fn specs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("specs")
}

fn discover_versions() -> Vec<String> {
    let mut versions: Vec<String> = std::fs::read_dir(specs_dir())
        .expect("read specs dir")
        .filter_map(|e| {
            let e = e.ok()?;
            if e.file_type().ok()?.is_dir() {
                Some(e.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();
    versions.sort();
    versions
}

#[test]
fn every_committed_golden_matches_regenerated_table() {
    for version in discover_versions() {
        let spec_path = specs_dir().join(&version).join("nifi-api.json");
        let golden_path = specs_dir().join(&version).join("fn_names.txt");

        assert!(
            golden_path.exists(),
            "missing golden file: {}. Run `cargo run -p nifi-openapi-gen` to generate.",
            golden_path.display()
        );

        let mut spec = load(spec_path.to_str().unwrap());
        // Apply real overrides + collision check for this version.
        nifi_openapi_gen::naming::apply_overrides_and_check_single(&mut spec, &version);

        let regenerated = format_fn_names_table(&spec);
        let committed = std::fs::read_to_string(&golden_path)
            .unwrap_or_else(|e| panic!("read {}: {e}", golden_path.display()));

        assert_eq!(
            regenerated, committed,
            "fn_names.txt for {version} is out of date. Rerun \
             `cargo run -p nifi-openapi-gen` and commit the updated golden.\n\n\
             Expected (committed):\n{committed}\n\n\
             Got (regenerated):\n{regenerated}"
        );
    }
}
