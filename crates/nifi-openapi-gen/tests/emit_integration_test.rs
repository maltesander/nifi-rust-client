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
    let (output, tested_enums) = nifi_openapi_gen::emit_enum_coverage_tests(&all_specs, &diffs);

    assert!(
        !tested_enums.is_empty(),
        "should have tracked tested enum values"
    );
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

    assert!(
        !output.is_empty(),
        "TESTABLE_TYPES has entries, so output should not be empty"
    );
    assert!(
        output.contains("#![cfg(feature = \"dynamic\")]"),
        "should have dynamic feature gate"
    );
    assert!(
        output.contains("cfg(feature"),
        "should have version feature gates"
    );
    assert!(
        output.contains("get_test_processor_entity"),
        "should reference test flow helper for ProcessorEntity"
    );
    assert!(
        output.contains("get_test_provenance_event"),
        "should reference test flow helper for ProvenanceEventDto"
    );
}

#[test]
fn tested_type_names_matches_testable_types() {
    let names = nifi_openapi_gen::tested_type_names();
    assert!(
        names.contains(&"ProcessorEntity"),
        "should include ProcessorEntity"
    );
    assert!(
        names.contains(&"ProvenanceEventDto"),
        "should include ProvenanceEventDto"
    );
}

#[test]
fn endpoint_availability_contains_added_endpoints() {
    let all_specs = load_all_specs();
    if all_specs.len() < 2 {
        return;
    }
    let diffs = compute_diffs(&all_specs);
    let (output, tested_endpoints) =
        nifi_openapi_gen::emit_endpoint_availability_tests(&all_specs, &diffs);

    assert!(
        !tested_endpoints.is_empty(),
        "should have tracked tested endpoints"
    );
    assert!(
        output.contains("UnsupportedEndpoint"),
        "should test UnsupportedEndpoint on older versions"
    );
    assert!(
        output.contains("cfg(feature"),
        "should have feature-gated tests"
    );
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
    let (output, tested_params) =
        nifi_openapi_gen::emit_query_param_coverage_tests(&all_specs, &diffs);

    assert!(
        !tested_params.is_empty(),
        "should have tracked tested query params"
    );
    assert!(
        output.contains("flow_metrics") || output.contains("flowMetrics"),
        "should reference the flow metrics endpoint"
    );
    assert!(
        output.contains("cfg(feature"),
        "should have feature-gated tests"
    );
}

#[test]
fn integration_coverage_content_not_empty() {
    let all_specs = load_all_specs();
    if all_specs.len() < 2 {
        return;
    }
    let diffs = compute_diffs(&all_specs);
    let tested_types = nifi_openapi_gen::tested_type_names();
    let (_, tested_endpoints) =
        nifi_openapi_gen::emit_endpoint_availability_tests(&all_specs, &diffs);
    let (_, tested_enum_values) = nifi_openapi_gen::emit_enum_coverage_tests(&all_specs, &diffs);
    let (_, tested_query_params) =
        nifi_openapi_gen::emit_query_param_coverage_tests(&all_specs, &diffs);
    let output =
        nifi_openapi_gen::docs::integration_coverage::generate_integration_coverage_content(
            &all_specs,
            &diffs,
            &tested_types,
            &tested_endpoints,
            &tested_enum_values,
            &tested_query_params,
        );

    assert!(!output.is_empty(), "coverage content should not be empty");
    assert!(
        output.contains("NiFi versions"),
        "should mention NiFi versions in summary"
    );
    assert!(
        output.contains("tested"),
        "should indicate tested checks in summary"
    );
    assert!(output.contains('✓'), "should mark tested rows with ✓");
}
