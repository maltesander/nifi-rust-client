//! Code generator binary — writes generated files in nifi-rust-client.
//!
//! Usage:
//!   cargo run -p nifi-openapi-gen                     # auto-detect spec version
//!   NIFI_VERSION=2.8.0 cargo run -p nifi-openapi-gen  # pin a specific version
//!   NIFI_API_SPEC=/path/to/spec.json cargo run -p nifi-openapi-gen  # full path override

use semver::Version;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

// ─── Version utilities ────────────────────────────────────────────────────────

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

// ─── Code generation (pure: spec data → Rust code string) ────────────────────

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

// ─── Repo sync: generated Rust files ─────────────────────────────────────────

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

    features.sort_values();
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

    features.sort_values();
    doc.to_string()
}

/// Discover all version directories under specs/, sorted by semver.
fn discover_spec_versions(specs_dir: &Path) -> Vec<String> {
    let mut versions: Vec<String> = std::fs::read_dir(specs_dir)
        .unwrap_or_else(|_| {
            panic!(
                "Cannot read specs dir {}.\nRun: ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh",
                specs_dir.display()
            )
        })
        .flatten()
        .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .filter_map(|e| {
            let name = e.file_name();
            let s = name.to_str()?.to_string();
            semver::Version::parse(&s).ok()?;
            Some(s)
        })
        .collect();
    sort_versions_semver(&mut versions);
    versions
}

// ─── Repo sync: documentation and infrastructure ──────────────────────────────

fn count_spec_endpoints(spec: &nifi_openapi_gen::ApiSpec) -> usize {
    spec.tags
        .iter()
        .map(|t| {
            t.root_endpoints.len()
                + t.sub_groups
                    .iter()
                    .map(|sg| sg.endpoints.len())
                    .sum::<usize>()
        })
        .sum()
}

fn count_spec_types(spec: &nifi_openapi_gen::ApiSpec) -> usize {
    spec.all_types.len()
}

/// Generates the Markdown rows for the supported-versions table.
/// `all_specs` must be sorted semver-ascending (oldest first).
/// The table is rendered newest-first so users see the current default at a glance.
fn generate_versions_table_content(
    all_specs: &[(String, nifi_openapi_gen::ApiSpec)],
    latest: &str,
) -> String {
    let mut rows: Vec<String> = Vec::new();
    rows.push(
        "| NiFi Version | Feature flag | Endpoints | Types | Changes | Default |".to_string(),
    );
    rows.push("|---|---|---|---|---|---|".to_string());

    for (i, (version, spec)) in all_specs.iter().enumerate().rev() {
        let endpoints = count_spec_endpoints(spec);
        let types = count_spec_types(spec);
        let feature = version_to_feature(version);
        let default_mark = if version == latest { "✓" } else { "" };

        let changes = if i == 0 {
            "—".to_string()
        } else {
            let (prev_version, prev_spec) = &all_specs[i - 1];
            let prev_ep = count_spec_endpoints(prev_spec);
            let prev_ty = count_spec_types(prev_spec);
            let ep_delta = endpoints as i64 - prev_ep as i64;
            let ty_delta = types as i64 - prev_ty as i64;
            if ep_delta == 0 && ty_delta == 0 {
                format!("no API changes vs {prev_version}")
            } else {
                let mut parts: Vec<String> = Vec::new();
                if ep_delta != 0 {
                    parts.push(format!("{ep_delta:+} endpoints"));
                }
                if ty_delta != 0 {
                    parts.push(format!("{ty_delta:+} types"));
                }
                format!("{} vs {prev_version}", parts.join(", "))
            }
        };

        rows.push(format!(
            "| {version} | `{feature}` | {endpoints} | {types} | {changes} | {default_mark} |"
        ));
    }

    rows.join("\n")
}

/// Replaces the content between `start_marker` and `end_marker` (exclusive) with
/// `\n{new_content}\n`.  Panics if either marker is absent.
fn replace_between_markers(
    content: &str,
    start_marker: &str,
    end_marker: &str,
    new_content: &str,
) -> String {
    let start_pos = content.find(start_marker).unwrap_or_else(|| {
        panic!("start marker '{start_marker}' not found");
    });
    let after_start = start_pos + start_marker.len();
    let end_pos = content[after_start..]
        .find(end_marker)
        .map(|p| p + after_start)
        .unwrap_or_else(|| {
            panic!("end marker '{end_marker}' not found");
        });
    format!(
        "{}{}\n{}\n{}{}",
        &content[..start_pos],
        start_marker,
        new_content,
        end_marker,
        &content[end_pos + end_marker.len()..],
    )
}

fn update_file_between_markers(
    path: &Path,
    start_marker: &str,
    end_marker: &str,
    new_content: &str,
) {
    let on_disk =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("read {}", path.display()));
    let patched = replace_between_markers(&on_disk, start_marker, end_marker, new_content);
    if on_disk != patched {
        std::fs::write(path, &patched).unwrap_or_else(|_| panic!("write {}", path.display()));
        println!("  wrote {}", path.display());
    }
}

fn update_readme_versions_table(
    workspace_root: &Path,
    all_specs: &[(String, nifi_openapi_gen::ApiSpec)],
    latest: &str,
) {
    const START: &str = "<!-- SUPPORTED_VERSIONS_START -->";
    const END: &str = "<!-- SUPPORTED_VERSIONS_END -->";
    let content = generate_versions_table_content(all_specs, latest);
    update_file_between_markers(&workspace_root.join("README.md"), START, END, &content);
    update_file_between_markers(
        &workspace_root.join("crates/nifi-rust-client/README.md"),
        START,
        END,
        &content,
    );
}

fn read_crate_version_shorthand(workspace_root: &Path) -> String {
    let cargo_toml = workspace_root.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml)
        .unwrap_or_else(|_| panic!("read {}", cargo_toml.display()));
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("version = \"")
            && let Some(ver) = rest.strip_suffix('"')
        {
            let parts: Vec<&str> = ver.splitn(3, '.').collect();
            if parts.len() >= 2 {
                return format!("{}.{}", parts[0], parts[1]);
            }
        }
    }
    panic!("could not find version in {}", cargo_toml.display());
}

fn update_client_readme_examples(workspace_root: &Path, latest: &str) {
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
            "```rust\nlet client = NifiClientBuilder::new(\"https://nifi:8443\")?.build()?;\nclient.login(\"admin\", \"password\").await?;\n\n// Full type safety — ProcessorEntity is {mod_name}::types::ProcessorEntity\nlet proc = client.processors_api().get_processor(\"id\").await?;\n```"
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

/// Replaces the `${NIFI_IMAGE_TAG:-<old>}` default tag in a docker-compose file content.
fn replace_image_tag_default(content: &str, latest: &str) -> String {
    const PREFIX: &str = "${NIFI_IMAGE_TAG:-";
    if let Some(start) = content.find(PREFIX) {
        let ver_start = start + PREFIX.len();
        if let Some(end_offset) = content[ver_start..].find('}') {
            let ver_end = ver_start + end_offset;
            return format!("{}{}{}", &content[..ver_start], latest, &content[ver_end..]);
        }
    }
    content.to_string()
}

fn update_docker_compose_default(path: &Path, latest: &str) {
    let on_disk =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("read {}", path.display()));
    let patched = replace_image_tag_default(&on_disk, latest);
    if on_disk != patched {
        std::fs::write(path, &patched).unwrap_or_else(|_| panic!("write {}", path.display()));
        println!("  wrote {}", path.display());
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
        let spec = nifi_openapi_gen::load(spec_path.to_str().expect("UTF-8 path"));
        parsed_specs.push((version.clone(), spec));
    }

    let mut targets: Vec<(PathBuf, String)> = vec![];

    // 3. Per-version code generation
    for (version_str, spec) in &parsed_specs {
        let mod_name = version_to_mod_name(version_str);
        let feature_name = version_to_feature(version_str);
        let versioned_src = client.join("src").join(&mod_name);

        for (filename, content) in nifi_openapi_gen::emit_types(spec) {
            targets.push((versioned_src.join("types").join(&filename), content));
        }
        let types_prefix = format!("crate::{mod_name}");
        for (filename, content) in nifi_openapi_gen::emit_api_with_prefix(spec, &types_prefix) {
            targets.push((versioned_src.join("api").join(&filename), content));
        }
        targets.push((
            versioned_src.join("mod.rs"),
            "pub mod api;\npub mod types;\n".to_string(),
        ));

        let test_content = format!(
            "#![cfg(all(feature = \"{feature_name}\", not(feature = \"dynamic\")))]\n\n{}",
            nifi_openapi_gen::emit_tests(spec)
        );
        targets.push((
            client.join(format!(
                "tests/v{}_generated_tests.rs",
                version_str.replace('.', "_")
            )),
            test_content,
        ));
    }

    // 4. Dynamic module generation (only when processing multiple specs)
    if parsed_specs.len() > 1 {
        let type_specs: Vec<(&str, &nifi_openapi_gen::ApiSpec)> =
            parsed_specs.iter().map(|(v, s)| (v.as_str(), s)).collect();

        let type_files = nifi_openapi_gen::emit_dynamic_types(&type_specs);
        for (filename, content) in &type_files {
            targets.push((
                client.join(format!("src/dynamic/types/{filename}")),
                content.clone(),
            ));
        }

        let merged_field_names = nifi_openapi_gen::collect_merged_field_names(&type_specs);
        let universal_fields = nifi_openapi_gen::collect_universal_fields(&type_specs);

        let mod_names: Vec<String> = parsed_specs
            .iter()
            .map(|(v, _)| version_to_mod_name(v))
            .collect();
        let conv_specs: Vec<(&str, &str, &nifi_openapi_gen::ApiSpec)> = parsed_specs
            .iter()
            .zip(mod_names.iter())
            .map(|((v, s), m)| (v.as_str(), m.as_str(), s))
            .collect();

        let conv_files = nifi_openapi_gen::emit_dynamic_conversions(
            &conv_specs,
            &merged_field_names,
            &universal_fields,
        );
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

        // Dynamic traits
        let trait_files = nifi_openapi_gen::emit_dynamic_traits(&dispatch_specs);
        for (filename, content) in &trait_files {
            targets.push((
                client.join(format!("src/dynamic/traits/{filename}")),
                content.clone(),
            ));
        }

        // Dynamic dispatch enums
        let dispatch_files = nifi_openapi_gen::emit_dynamic_dispatch(&dispatch_specs);
        for (filename, content) in &dispatch_files {
            targets.push((
                client.join(format!("src/dynamic/dispatch/{filename}")),
                content.clone(),
            ));
        }

        // Dynamic per-version impls
        let impl_files = nifi_openapi_gen::emit_dynamic_impls(&dispatch_specs);
        for (filename, content) in &impl_files {
            targets.push((
                client.join(format!("src/dynamic/impls/{filename}")),
                content.clone(),
            ));
        }

        targets.push((
            client.join("src/dynamic/mod.rs"),
            nifi_openapi_gen::emit_dynamic(&dispatch_specs),
        ));

        targets.push((
            client.join("tests/dynamic_tests.rs"),
            nifi_openapi_gen::emit_dynamic_tests(&conv_specs),
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
                // Not an .rs file or already has the header
                (p, content)
            } else if content.starts_with("#![cfg") {
                // Test files: insert header after the #![cfg(...)] line
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

    // 6a. Remove legacy src/dynamic/types.rs now that types live in src/dynamic/types/
    {
        let legacy = client.join("src/dynamic/types.rs");
        if legacy.exists() && !written_paths.contains(&legacy) {
            std::fs::remove_file(&legacy).expect("remove legacy types.rs");
            println!("  removed legacy {}", legacy.display());
        }
    }

    // 6a. Remove legacy src/dynamic/conversions.rs now that conversions live in src/dynamic/conversions/
    {
        let legacy = client.join("src/dynamic/conversions.rs");
        if legacy.exists() && !written_paths.contains(&legacy) {
            std::fs::remove_file(&legacy).expect("remove legacy conversions.rs");
            println!("  removed legacy {}", legacy.display());
        }
    }

    // 6a. Delete stale .rs files in dynamic subdirectories
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

    // 6. Delete stale .rs files in per-version dirs
    for (version_str, _) in &parsed_specs {
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

    // 6b. Delete entire version dirs (src/v*/ and tests/v*_generated_tests.rs) for versions
    // that exist on disk but are no longer in the spec list. Only applies when running in
    // full-discovery mode (no NIFI_VERSION / NIFI_API_SPEC override), so a single-version
    // regeneration run doesn't accidentally wipe sibling versions.
    let is_full_discovery =
        std::env::var("NIFI_VERSION").is_err() && std::env::var("NIFI_API_SPEC").is_err();
    if is_full_discovery {
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

    // One-time migration: remove old flat layout if present
    migrate_flat_layout(&client);

    // 7. Update lib.rs and both Cargo.toml [features] sections
    let tests_crate = workspace_root.join("tests");
    update_lib_rs(&client.join("src"));
    update_cargo_features_client(&client);
    update_cargo_features_tests(&client.join("src"), &tests_crate);

    // 8. Run cargo fmt so files are in canonical form.
    let status = std::process::Command::new("cargo")
        .args(["fmt", "-p", "nifi-rust-client"])
        .current_dir(workspace_root)
        .status()
        .expect("cargo fmt");

    if !status.success() {
        eprintln!("cargo fmt failed");
        std::process::exit(1);
    }

    // 9. Repo maintenance: update the supported-versions table in README.md and the default
    // docker-compose image tag.  Always uses all discovered spec versions so the table is
    // complete even when only one version was generated in this run.
    {
        let latest_version = all_spec_versions.last().cloned().unwrap_or_default();
        let all_parsed: Vec<(String, nifi_openapi_gen::ApiSpec)> = all_spec_versions
            .iter()
            .map(|v| {
                let path = specs_dir.join(v).join("nifi-api.json");
                let spec = nifi_openapi_gen::load(path.to_str().expect("UTF-8 path"));
                (v.clone(), spec)
            })
            .collect();
        update_readme_versions_table(workspace_root, &all_parsed, &latest_version);
        update_client_readme_examples(workspace_root, &latest_version);
        update_docker_compose_default(
            &workspace_root.join("tests/docker-compose.yml"),
            &latest_version,
        );
    }

    println!("Done. {} file(s) updated.", written);
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

    // ─── Repo sync: documentation tests ──────────────────────────────────────

    #[test]
    fn test_replace_between_markers_replaces_content() {
        let input = "before\n<!-- START -->\nold content\n<!-- END -->\nafter\n";
        let result =
            replace_between_markers(input, "<!-- START -->", "<!-- END -->", "new content");
        assert_eq!(
            result,
            "before\n<!-- START -->\nnew content\n<!-- END -->\nafter\n"
        );
    }

    #[test]
    fn test_replace_between_markers_idempotent() {
        let input = "before\n<!-- START -->\nnew content\n<!-- END -->\nafter\n";
        let result =
            replace_between_markers(input, "<!-- START -->", "<!-- END -->", "new content");
        assert_eq!(result, input);
    }

    #[test]
    fn test_replace_image_tag_default_updates_version() {
        let input = "image: apache/nifi:${NIFI_IMAGE_TAG:-2.7.2}\n";
        let result = replace_image_tag_default(input, "2.8.0");
        assert_eq!(result, "image: apache/nifi:${NIFI_IMAGE_TAG:-2.8.0}\n");
    }

    #[test]
    fn test_replace_image_tag_default_idempotent() {
        let input = "image: apache/nifi:${NIFI_IMAGE_TAG:-2.8.0}\n";
        let result = replace_image_tag_default(input, "2.8.0");
        assert_eq!(result, input);
    }

    #[test]
    fn test_generate_versions_table_latest_first_and_marked() {
        let specs = vec![
            (
                "2.7.2".to_string(),
                nifi_openapi_gen::ApiSpec {
                    tags: vec![],
                    all_types: vec![],
                },
            ),
            (
                "2.8.0".to_string(),
                nifi_openapi_gen::ApiSpec {
                    tags: vec![],
                    all_types: vec![],
                },
            ),
        ];
        let table = generate_versions_table_content(&specs, "2.8.0");
        // Latest version appears first in the rendered table
        let pos_new = table.find("2.8.0").unwrap();
        let pos_old = table.find("2.7.2").unwrap();
        assert!(pos_new < pos_old, "latest version should appear first");
        // Default mark only on the latest
        assert!(table.contains("✓"), "default mark missing");
        // Both versions have equal counts → no API changes message
        assert!(
            table.contains("no API changes vs 2.7.2"),
            "expected no-changes message"
        );
        // Oldest shows em dash
        assert!(table.contains("| — |"), "oldest version should show —");
    }

    #[test]
    fn test_generate_versions_table_shows_delta() {
        use nifi_openapi_gen::{ApiSpec, TagGroup};
        fn make_spec(endpoint_count: usize) -> ApiSpec {
            ApiSpec {
                tags: vec![TagGroup {
                    tag: "flow".to_string(),
                    struct_name: "FlowApi".to_string(),
                    module_name: "flow".to_string(),
                    accessor_fn: "flow_api".to_string(),
                    types: vec![],
                    root_endpoints: (0..endpoint_count)
                        .map(|_| nifi_openapi_gen::Endpoint {
                            method: nifi_openapi_gen::HttpMethod::Get,
                            path: "/nifi-api/flow/about".to_string(),
                            fn_name: "get_about".to_string(),
                            doc: None,
                            description: None,
                            path_params: vec![],
                            request_type: None,
                            body_kind: None,
                            body_doc: None,
                            response_type: None,
                            response_inner: None,
                            response_field: None,
                            query_params: vec![],
                            success_responses: vec![],
                            error_responses: vec![],
                            security: None,
                        })
                        .collect(),
                    sub_groups: vec![],
                }],
                all_types: vec![],
            }
        }

        let specs = vec![
            ("2.6.0".to_string(), make_spec(10)),
            ("2.7.2".to_string(), make_spec(12)),
        ];
        let table = generate_versions_table_content(&specs, "2.7.2");
        assert!(
            table.contains("+2 endpoints vs 2.6.0"),
            "expected +2 endpoints delta"
        );
    }
}
