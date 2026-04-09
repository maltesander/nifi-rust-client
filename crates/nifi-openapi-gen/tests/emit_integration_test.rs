use std::path::Path;

fn load_all_specs() -> Vec<(String, nifi_openapi_gen::ApiSpec)> {
    let codegen_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let specs_dir = codegen_dir.join("specs");
    let versions = nifi_openapi_gen::util::discover_spec_versions(&specs_dir);
    versions
        .into_iter()
        .map(|v| {
            let path = specs_dir.join(&v).join("nifi-api.json");
            let spec = nifi_openapi_gen::load(path.to_str().unwrap());
            (v, spec)
        })
        .collect()
}

fn compute_diffs(
    all_specs: &[(String, nifi_openapi_gen::ApiSpec)],
) -> Vec<nifi_openapi_gen::VersionDiff> {
    (1..all_specs.len())
        .map(|i| {
            let (from_ver, from_spec) = &all_specs[i - 1];
            let (to_ver, to_spec) = &all_specs[i];
            nifi_openapi_gen::compute_diff(from_spec, to_spec, from_ver, to_ver)
        })
        .collect()
}

#[test]
fn enum_coverage_contains_version_info_test() {
    let all_specs = load_all_specs();
    if all_specs.len() < 2 {
        return;
    }
    let diffs = compute_diffs(&all_specs);
    let output = nifi_openapi_gen::emit_enum_coverage_tests(&all_specs, &diffs);

    assert!(
        output.contains("IncludedRegistries"),
        "should reference IncludedRegistries enum"
    );
    assert!(
        output.contains("VersionInfo"),
        "should reference VersionInfo variant"
    );
    assert!(
        output.contains("cfg(feature"),
        "should have feature-gated tests"
    );
    assert!(
        output.contains("UnsupportedEnumVariant"),
        "should test UnsupportedEnumVariant on older versions"
    );
}

#[test]
fn field_presence_output_is_valid() {
    let all_specs = load_all_specs();
    if all_specs.len() < 2 {
        return;
    }
    let diffs = compute_diffs(&all_specs);
    let output = nifi_openapi_gen::emit_field_presence_tests(&all_specs, &diffs);

    // TESTABLE_TYPES is empty, so output should be empty — that's OK
    if output.is_empty() {
        return;
    }

    assert!(
        output.contains("#![cfg(feature = \"dynamic\")]"),
        "should have dynamic feature gate"
    );
    assert!(
        output.contains("cfg(feature"),
        "should have version feature gates"
    );
}

#[test]
fn endpoint_availability_contains_added_endpoints() {
    let all_specs = load_all_specs();
    if all_specs.len() < 2 {
        return;
    }
    let diffs = compute_diffs(&all_specs);
    let output = nifi_openapi_gen::emit_endpoint_availability_tests(&all_specs, &diffs);

    assert!(output.contains("UnsupportedEndpoint"), "should test UnsupportedEndpoint on older versions");
    assert!(output.contains("cfg(feature"), "should have feature-gated tests");
    assert!(
        output.contains("listen_ports") || output.contains("clear_bulletins"),
        "should reference known added endpoints"
    );
}

#[test]
fn query_param_coverage_contains_flow_metrics_strategy() {
    let all_specs = load_all_specs();
    if all_specs.len() < 2 {
        return;
    }
    let diffs = compute_diffs(&all_specs);
    let output = nifi_openapi_gen::emit_query_param_coverage_tests(&all_specs, &diffs);

    assert!(
        output.contains("flow_metrics") || output.contains("flowMetrics"),
        "should reference the flow metrics endpoint"
    );
    assert!(output.contains("cfg(feature"), "should have feature-gated tests");
}
