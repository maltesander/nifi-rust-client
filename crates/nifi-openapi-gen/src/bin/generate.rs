//! Code generator binary — writes generated files in nifi-rust-client.
//!
//! Usage:
//!   cargo run -p nifi-openapi-gen                     # auto-detect spec version
//!   NIFI_VERSION=2.8.0 cargo run -p nifi-openapi-gen  # pin a specific version
//!   NIFI_API_SPEC=/path/to/spec.json cargo run -p nifi-openapi-gen  # full path override

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use nifi_openapi_gen::docs::{
    generate_api_changes_content, generate_resource_accessors_content,
    generate_versions_table_content, update_client_readme_examples,
};
use nifi_openapi_gen::repo::{
    migrate_flat_layout, remove_stale_generated_files, remove_stale_version_dirs,
    update_cargo_features_client, update_cargo_features_tests, update_docker_compose_default,
    update_lib_rs,
};
use nifi_openapi_gen::util::{
    discover_spec_versions, update_file_between_markers, version_to_feature, version_to_mod_name,
};
use nifi_openapi_gen::{
    collect_merged_field_names, collect_universal_fields, emit_api_with_prefix, emit_dynamic,
    emit_dynamic_conversions, emit_dynamic_dispatch, emit_dynamic_impls, emit_dynamic_tests,
    emit_dynamic_traits, emit_dynamic_types, emit_endpoint_availability_tests,
    emit_enum_coverage_tests, emit_field_presence_tests, emit_query_param_coverage_tests,
    emit_static_traits, emit_types, load,
};

fn write_if_changed(path: &std::path::Path, content: &str, written: &mut usize) {
    // Ensure exactly one trailing newline (avoids end-of-file-fixer churn).
    let normalized = format!("{}\n", content.trim_end());
    let on_disk = std::fs::read_to_string(path).unwrap_or_default();
    if on_disk != normalized {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create parent dirs");
        }
        std::fs::write(path, &normalized).expect("write generated file");
        println!("  wrote {}", path.display());
        *written += 1;
    }
}

fn main() {
    let codegen_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = codegen_dir
        .parent()
        .expect("crates/")
        .parent()
        .expect("workspace root");
    let client = workspace_root.join("crates/nifi-rust-client");
    let specs_dir = codegen_dir.join("specs");

    // Always discover the full set of spec versions — used for repo maintenance (README table,
    // docker-compose default) regardless of which version(s) are being generated this run.
    let all_spec_versions = discover_spec_versions(&specs_dir);

    // 1. Determine which specs to generate code for (may be a subset of all_spec_versions)
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

    // 2. Parse all specs
    let mut parsed_specs: Vec<(String, nifi_openapi_gen::ApiSpec)> = vec![];
    for version in &spec_versions {
        let spec_path = specs_dir.join(version).join("nifi-api.json");
        if !spec_path.exists() {
            panic!("Spec not found: {}", spec_path.display());
        }
        let spec = load(spec_path.to_str().expect("UTF-8 path"));
        parsed_specs.push((version.clone(), spec));
    }

    let mut targets: Vec<(PathBuf, String)> = vec![];

    // 3. Per-version code generation
    for (version_str, spec) in &parsed_specs {
        let mod_name = version_to_mod_name(version_str);
        let versioned_src = client.join("src").join(&mod_name);

        for (filename, content) in emit_types(spec) {
            targets.push((versioned_src.join("types").join(&filename), content));
        }
        let types_prefix = format!("crate::{mod_name}");
        for (filename, content) in emit_api_with_prefix(spec, &types_prefix) {
            targets.push((versioned_src.join("api").join(&filename), content));
        }
        for (filename, content) in emit_static_traits(spec, &types_prefix) {
            targets.push((versioned_src.join("traits").join(&filename), content));
        }
        targets.push((
            versioned_src.join("mod.rs"),
            "pub mod api;\npub mod traits;\npub mod types;\n".to_string(),
        ));
    }

    // 4. Dynamic module generation (only when processing multiple specs)
    if parsed_specs.len() > 1 {
        let type_specs: Vec<(&str, &nifi_openapi_gen::ApiSpec)> =
            parsed_specs.iter().map(|(v, s)| (v.as_str(), s)).collect();

        let type_files = emit_dynamic_types(&type_specs);
        for (filename, content) in &type_files {
            targets.push((
                client.join(format!("src/dynamic/types/{filename}")),
                content.clone(),
            ));
        }

        let merged_field_names = collect_merged_field_names(&type_specs);
        let universal_fields = collect_universal_fields(&type_specs);

        let mod_names: Vec<String> = parsed_specs
            .iter()
            .map(|(v, _)| version_to_mod_name(v))
            .collect();
        let conv_specs: Vec<(&str, &str, &nifi_openapi_gen::ApiSpec)> = parsed_specs
            .iter()
            .zip(mod_names.iter())
            .map(|((v, s), m)| (v.as_str(), m.as_str(), s))
            .collect();

        let conv_files =
            emit_dynamic_conversions(&conv_specs, &merged_field_names, &universal_fields);
        for (filename, content) in &conv_files {
            targets.push((
                client.join(format!("src/dynamic/conversions/{filename}")),
                content.clone(),
            ));
        }

        let feature_names: Vec<String> = parsed_specs
            .iter()
            .map(|(v, _)| version_to_feature(v))
            .collect();
        let dispatch_specs: Vec<(&str, &str, &str, &nifi_openapi_gen::ApiSpec)> = parsed_specs
            .iter()
            .zip(mod_names.iter().zip(feature_names.iter()))
            .map(|((v, s), (m, f))| (v.as_str(), m.as_str(), f.as_str(), s))
            .collect();

        let trait_files = emit_dynamic_traits(&dispatch_specs);
        for (filename, content) in &trait_files {
            targets.push((
                client.join(format!("src/dynamic/traits/{filename}")),
                content.clone(),
            ));
        }

        let dispatch_files = emit_dynamic_dispatch(&dispatch_specs);
        for (filename, content) in &dispatch_files {
            targets.push((
                client.join(format!("src/dynamic/dispatch/{filename}")),
                content.clone(),
            ));
        }

        let impl_files = emit_dynamic_impls(&dispatch_specs);
        for (filename, content) in &impl_files {
            targets.push((
                client.join(format!("src/dynamic/impls/{filename}")),
                content.clone(),
            ));
        }

        targets.push((
            client.join("src/dynamic/mod.rs"),
            emit_dynamic(&dispatch_specs),
        ));

        targets.push((
            client.join("tests/dynamic_tests.rs"),
            emit_dynamic_tests(&conv_specs),
        ));
    }

    // 5. Ensure every .rs file has the @generated header
    const GENERATED_HEADER: &str =
        "// @generated — do not edit; run `cargo run -p nifi-openapi-gen`\n\n";
    let targets: Vec<(PathBuf, String)> = targets
        .into_iter()
        .map(|(p, content)| {
            if p.extension().and_then(|e| e.to_str()) != Some("rs")
                || content.contains("@generated")
            {
                (p, content)
            } else if content.starts_with("#![cfg") {
                if let Some(pos) = content.find('\n') {
                    let (cfg_line, rest) = content.split_at(pos + 1);
                    (p, format!("{cfg_line}\n{GENERATED_HEADER}{rest}"))
                } else {
                    (p, format!("{content}\n\n{GENERATED_HEADER}"))
                }
            } else {
                (p, format!("{GENERATED_HEADER}{content}"))
            }
        })
        .collect();

    // 6. Write changed files
    let written_paths: HashSet<PathBuf> = targets.iter().map(|(p, _)| p.clone()).collect();
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

    // 7. Cleanup stale files
    remove_stale_generated_files(&client, &written_paths, &parsed_specs);

    let is_full_discovery =
        std::env::var("NIFI_VERSION").is_err() && std::env::var("NIFI_API_SPEC").is_err();
    if is_full_discovery {
        remove_stale_version_dirs(&client, &spec_versions);
    }

    // One-time migration: remove old flat layout if present
    migrate_flat_layout(&client);

    // 8. Update lib.rs and both Cargo.toml [features] sections
    let tests_crate = workspace_root.join("tests");
    update_lib_rs(&client.join("src"));
    update_cargo_features_client(&client);
    update_cargo_features_tests(&client.join("src"), &tests_crate);

    // 9. Run cargo fmt so files are in canonical form.
    for pkg in ["nifi-rust-client", "nifi-integration-tests"] {
        let status = std::process::Command::new("cargo")
            .args(["fmt", "-p", pkg])
            .current_dir(workspace_root)
            .status()
            .expect("cargo fmt");

        if !status.success() {
            eprintln!("cargo fmt -p {pkg} failed");
            std::process::exit(1);
        }
    }

    // 10. Repo maintenance: update the supported-versions table in README.md and the default
    // docker-compose image tag.  Always uses all discovered spec versions so the table is
    // complete even when only one version was generated in this run.
    {
        let latest_version = all_spec_versions.last().cloned().unwrap_or_default();
        let all_parsed: Vec<(String, nifi_openapi_gen::ApiSpec)> = all_spec_versions
            .iter()
            .map(|v| {
                let path = specs_dir.join(v).join("nifi-api.json");
                let spec = load(path.to_str().expect("UTF-8 path"));
                (v.clone(), spec)
            })
            .collect();

        // README versions table
        {
            const START: &str = "<!-- SUPPORTED_VERSIONS_START -->";
            const END: &str = "<!-- SUPPORTED_VERSIONS_END -->";
            let content = generate_versions_table_content(&all_parsed);
            update_file_between_markers(&workspace_root.join("README.md"), START, END, &content);
            update_file_between_markers(
                &workspace_root.join("crates/nifi-rust-client/README.md"),
                START,
                END,
                &content,
            );
        }

        // Resource accessors table
        {
            const START: &str = "<!-- RESOURCE_ACCESSORS_START -->";
            const END: &str = "<!-- RESOURCE_ACCESSORS_END -->";
            let content = generate_resource_accessors_content(&all_parsed);
            update_file_between_markers(
                &workspace_root.join("crates/nifi-rust-client/README.md"),
                START,
                END,
                &content,
            );
        }

        update_client_readme_examples(workspace_root, &latest_version);
        update_docker_compose_default(
            &workspace_root.join("tests/docker-compose.yml"),
            &latest_version,
        );

        // API changes file
        {
            const START: &str = "<!-- NIFI_API_CHANGES_START -->";
            const END: &str = "<!-- NIFI_API_CHANGES_END -->";
            let path = workspace_root.join("NIFI_API_CHANGES.md");

            if !path.exists() {
                std::fs::write(
                    &path,
                    format!(
                        "# NiFi API Changes\n\n<!-- markdownlint-disable MD024 MD033 -->\n\nAuto-generated by `cargo run -p nifi-openapi-gen`. Do not edit manually.\n\n{START}\n{END}\n"
                    ),
                )
                .unwrap_or_else(|_| panic!("create {}", path.display()));
                println!("  created {}", path.display());
            }

            let content = generate_api_changes_content(&all_parsed);
            update_file_between_markers(&path, START, END, &content);
        }

        // Consecutive diffs (shared by integration tests and coverage section)
        let diffs: Vec<nifi_openapi_gen::VersionDiff> = (1..all_parsed.len())
            .map(|i| {
                let (from_ver, from_spec) = &all_parsed[i - 1];
                let (to_ver, to_spec) = &all_parsed[i];
                nifi_openapi_gen::compute_diff(from_spec, to_spec, from_ver, to_ver)
            })
            .collect();

        // Integration coverage tests (written to tests/tests/)
        // Each emitter returns (code, tested_keys); we collect the keys for
        // the coverage doc generator below.
        let tested_endpoints;
        let tested_enum_values;
        let tested_query_params;
        {
            let tests_dir = workspace_root.join("tests/tests");

            let (enum_tests, enum_keys) = emit_enum_coverage_tests(&all_parsed, &diffs);
            tested_enum_values = enum_keys;
            if !enum_tests.is_empty() {
                write_if_changed(
                    &tests_dir.join("dynamic_enum_coverage.rs"),
                    &enum_tests,
                    &mut written,
                );
            }

            let (endpoint_tests, ep_keys) = emit_endpoint_availability_tests(&all_parsed, &diffs);
            tested_endpoints = ep_keys;
            if !endpoint_tests.is_empty() {
                write_if_changed(
                    &tests_dir.join("dynamic_endpoint_availability.rs"),
                    &endpoint_tests,
                    &mut written,
                );
            }

            let field_tests = emit_field_presence_tests(&all_parsed, &diffs);
            if !field_tests.is_empty() {
                write_if_changed(
                    &tests_dir.join("dynamic_field_presence.rs"),
                    &field_tests,
                    &mut written,
                );
            }

            let (param_tests, param_keys) = emit_query_param_coverage_tests(&all_parsed, &diffs);
            tested_query_params = param_keys;
            if !param_tests.is_empty() {
                write_if_changed(
                    &tests_dir.join("dynamic_query_param_coverage.rs"),
                    &param_tests,
                    &mut written,
                );
            }
        }

        // Run rustfmt on generated integration test files (they live outside
        // nifi-rust-client so `cargo fmt -p` doesn't cover them).
        {
            let tests_dir = workspace_root.join("tests/tests");
            let generated: Vec<std::path::PathBuf> = [
                "dynamic_enum_coverage.rs",
                "dynamic_endpoint_availability.rs",
                "dynamic_field_presence.rs",
                "dynamic_query_param_coverage.rs",
            ]
            .iter()
            .map(|f| tests_dir.join(f))
            .filter(|p| p.exists())
            .collect();
            if !generated.is_empty() {
                let args: Vec<&str> = generated.iter().filter_map(|p| p.to_str()).collect();
                let _ = std::process::Command::new("rustfmt")
                    .arg("--edition=2024")
                    .args(&args)
                    .status();
            }
        }

        // Integration coverage in nifi-rust-client README
        {
            const START: &str = "<!-- INTEGRATION_COVERAGE_START -->";
            const END: &str = "<!-- INTEGRATION_COVERAGE_END -->";
            let tested_types = nifi_openapi_gen::tested_type_names();
            let content = nifi_openapi_gen::docs::generate_integration_coverage_content(
                &all_parsed,
                &diffs,
                &tested_types,
                &tested_endpoints,
                &tested_enum_values,
                &tested_query_params,
            );
            update_file_between_markers(&client.join("README.md"), START, END, &content);
        }
    }

    println!("Done. {} file(s) updated.", written);
}
