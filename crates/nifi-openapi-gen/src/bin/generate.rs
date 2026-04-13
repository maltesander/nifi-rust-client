//! Repo-maintenance binary for nifi-rust-client.
//!
//! Updates Cargo.toml features, README tables, docker-compose defaults, API
//! change docs, and integration coverage sections.  Code generation (types,
//! API structs, dynamic layer, tests) is handled by `build.rs` at build time
//! — this binary no longer writes to `src/` or generated test files.
//!
//! Usage:
//!   cargo run -p nifi-openapi-gen                              # validate + apply
//!   cargo run -p nifi-openapi-gen -- --check                   # validate + diff, exit 1 on drift
//!   cargo run -p nifi-openapi-gen -- --check --verbose         # full diffs
//!   NIFI_VERSION=2.8.0 cargo run -p nifi-openapi-gen           # pin a specific version

use std::path::{Path, PathBuf};

use nifi_openapi_gen::docs::{
    emit_api_changes, emit_client_readme_examples, emit_integration_coverage, emit_lib_rs_examples,
    emit_resource_accessors, emit_versions_table,
};
use nifi_openapi_gen::layout::RepoLayout;
use nifi_openapi_gen::plan::{FileEdit, MutationPlan};
use nifi_openapi_gen::repo::{
    emit_cargo_features_client, emit_cargo_features_tests, emit_docker_compose_default,
    emit_fn_names_goldens, emit_lib_rs_feature_flags,
};
use nifi_openapi_gen::util::discover_spec_versions;
use nifi_openapi_gen::canonical::canonicalize_or_panic;
use nifi_openapi_gen::non_additive_overrides::NonAdditiveOverrides;
use nifi_openapi_gen::{
    ApiSpec, VersionDiff, collect_endpoint_metadata, collect_enum_metadata,
    collect_query_param_metadata, compute_diff, load, tested_type_names,
};

/// Extract major.minor version shorthand from Cargo.toml content.
fn read_crate_version_shorthand(toml_content: &str) -> String {
    for line in toml_content.lines() {
        if let Some(rest) = line.strip_prefix("version = \"")
            && let Some(ver) = rest.strip_suffix('"')
        {
            let parts: Vec<&str> = ver.splitn(3, '.').collect();
            if parts.len() >= 2 {
                return format!("{}.{}", parts[0], parts[1]);
            }
        }
    }
    panic!("could not find version in Cargo.toml content");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let check_mode = args.iter().any(|a| a == "--check");
    let verbose = args.iter().any(|a| a == "--verbose");

    let layout = RepoLayout::discover();
    let specs_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("specs");

    // Discover all spec versions for repo maintenance.
    let all_spec_versions = discover_spec_versions(&specs_dir);

    // Determine which specs are being added/updated this run.
    let spec_versions = if let Ok(version) = std::env::var("NIFI_VERSION") {
        vec![version]
    } else if let Ok(path) = std::env::var("NIFI_API_SPEC") {
        let p = PathBuf::from(&path);
        let version = p
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .expect("version directory name from NIFI_API_SPEC path")
            .to_string();
        vec![version]
    } else {
        all_spec_versions.clone()
    };

    // Verify requested specs exist.
    for version in &spec_versions {
        let spec_path = specs_dir.join(version).join("nifi-api.json");
        if !spec_path.exists() {
            panic!("Spec not found: {}", spec_path.display());
        }
    }

    // Load all specs for repo maintenance.
    let latest_version = all_spec_versions.last().cloned().unwrap_or_default();
    let version_strs: Vec<&str> = all_spec_versions.iter().map(String::as_str).collect();
    let mut all_parsed: Vec<(String, ApiSpec)> = all_spec_versions
        .iter()
        .map(|v| {
            let path = specs_dir.join(v).join("nifi-api.json");
            let spec = load(path.to_str().expect("UTF-8 path"));
            (v.clone(), spec)
        })
        .collect();

    // Apply naming overrides + per-version same-tag collision check.
    for (version, spec) in &mut all_parsed {
        nifi_openapi_gen::naming::apply_overrides_and_check_single(spec, version);
    }
    // Cross-version drift check.
    nifi_openapi_gen::naming::check_drift(&all_parsed);

    // Canonicalize all specs and run the non-additive-change detector.
    // Panics with an actionable message if a newer spec breaks the
    // monotonic-additive assumption that canonical superset codegen
    // relies on.
    let specs_dir_for_canonical = specs_dir.clone();
    let canonical = canonicalize_or_panic(
        &all_parsed,
        |v| {
            specs_dir_for_canonical
                .join(v)
                .join("nifi-api.json")
                .display()
                .to_string()
        },
        &NonAdditiveOverrides::empty(),
    );
    eprintln!(
        "canonical: {} endpoints, {} types",
        canonical.endpoints.len(),
        canonical.types.len()
    );

    // Read files that need patching (Overwrite edits need current content).
    let client_toml =
        std::fs::read_to_string(&layout.client_cargo_toml).expect("read client Cargo.toml");
    let tests_toml =
        std::fs::read_to_string(&layout.tests_cargo_toml).expect("read tests Cargo.toml");
    let dc_content =
        std::fs::read_to_string(&layout.docker_compose).expect("read docker-compose.yml");

    // Build the mutation plan from all emitters.
    let mut edits: Vec<FileEdit> = Vec::new();

    // Cargo.toml features
    edits.extend(emit_cargo_features_client(
        &layout,
        &client_toml,
        &version_strs,
    ));
    edits.extend(emit_cargo_features_tests(
        &layout,
        &tests_toml,
        &version_strs,
    ));

    // README versions table (workspace + client)
    edits.extend(emit_versions_table(&layout, &all_parsed));

    // Resource accessors table (client README)
    edits.extend(emit_resource_accessors(&layout, &all_parsed));

    // README examples (client README)
    let crate_version = read_crate_version_shorthand(&client_toml);
    edits.extend(emit_client_readme_examples(
        &layout,
        &latest_version,
        &crate_version,
    ));

    // lib.rs doc comment version examples
    edits.extend(emit_lib_rs_examples(
        &layout,
        &latest_version,
        &crate_version,
    ));

    // Docker-compose default image tag
    edits.extend(emit_docker_compose_default(
        &layout,
        &dc_content,
        &latest_version,
    ));

    // API changes doc
    edits.extend(emit_api_changes(&layout, &all_parsed));

    // fn_names goldens (one per version)
    edits.extend(emit_fn_names_goldens(&layout, &all_parsed));

    // Feature flags in lib.rs
    edits.extend(emit_lib_rs_feature_flags(&layout, &version_strs));

    // Integration coverage section
    let diffs: Vec<VersionDiff> = (1..all_parsed.len())
        .map(|i| {
            let (from_ver, from_spec) = &all_parsed[i - 1];
            let (to_ver, to_spec) = &all_parsed[i];
            compute_diff(from_spec, to_spec, from_ver, to_ver)
        })
        .collect();

    let tested_endpoints = collect_endpoint_metadata(&all_parsed, &diffs);
    let tested_enum_values = collect_enum_metadata(&all_parsed, &diffs);
    let tested_query_params = collect_query_param_metadata(&all_parsed, &diffs);
    let tested_types = tested_type_names();
    edits.extend(emit_integration_coverage(
        &layout,
        &all_parsed,
        &diffs,
        &tested_types,
        &tested_endpoints,
        &tested_enum_values,
        &tested_query_params,
    ));

    let plan = MutationPlan { edits };

    // Apply or check.
    if check_mode {
        match plan.check() {
            Ok(report) => {
                if verbose {
                    print!("{report:#}");
                } else {
                    print!("{report}");
                }
                if report.has_drift() {
                    std::process::exit(1);
                }
            }
            Err(errors) => {
                for e in &errors {
                    eprintln!("ERROR: {e}");
                }
                std::process::exit(2);
            }
        }
    } else {
        match plan.apply() {
            Ok(report) => {
                println!(
                    "Done. {} file(s) written, {} unchanged.",
                    report.written, report.unchanged
                );
            }
            Err(errors) => {
                for e in &errors {
                    eprintln!("ERROR: {e}");
                }
                std::process::exit(2);
            }
        }
    }
}
