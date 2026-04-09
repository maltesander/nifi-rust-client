use super::common::{
    build_accessor, capitalize_first, default_path_param_value, default_required_query_param_value,
    find_endpoint, trait_use_stmt,
};
use crate::diff::VersionDiff;
use crate::parser::{ApiSpec, Endpoint, SubGroup};
use crate::util::{version_to_feature, wire_to_pascal};

/// Generates integration tests verifying enum query-param values are accepted
/// on their origin version and rejected (`UnsupportedEnumVariant`) on older versions.
pub fn emit_enum_coverage_tests(all_specs: &[(String, ApiSpec)], diffs: &[VersionDiff]) -> String {
    if all_specs.is_empty() || diffs.is_empty() {
        return String::new();
    }

    let mut tests: Vec<String> = Vec::new();

    // Collect all version features for cumulative gating.
    let all_features: Vec<String> = all_specs
        .iter()
        .map(|(v, _)| version_to_feature(v))
        .collect();

    for diff in diffs {
        let to_feature = version_to_feature(&diff.to);

        // Features for versions that have the new value (to + later).
        let supporting_features: Vec<&str> = all_features
            .iter()
            .skip_while(|f| f.as_str() != to_feature)
            .map(|f| f.as_str())
            .collect();
        let negative_cfg = if supporting_features.len() == 1 {
            format!("not(feature = \"{}\")", supporting_features[0])
        } else {
            let any_list: Vec<String> = supporting_features
                .iter()
                .map(|f| format!("feature = \"{f}\""))
                .collect();
            format!("not(any({}))", any_list.join(", "))
        };

        // Find the "to" spec so we can look up endpoint metadata.
        let to_spec = match all_specs.iter().find(|(v, _)| v == &diff.to) {
            Some((_, spec)) => spec,
            None => continue,
        };

        for ep_change in &diff.endpoints.changed {
            for param_change in &ep_change.changed_params {
                if param_change.added_enum_values.is_empty() {
                    continue;
                }

                // Look up the endpoint in the "to" spec.
                let ep_info = find_endpoint(to_spec, &ep_change.method, &ep_change.path);
                let (endpoint, tag_group, sub_group) = match ep_info {
                    Some(info) => info,
                    None => continue,
                };

                // Find the query param that has the enum type.
                let qp = match endpoint
                    .query_params
                    .iter()
                    .find(|qp| qp.name == param_change.name)
                {
                    Some(qp) => qp,
                    None => continue,
                };

                let enum_type_name = match &qp.enum_type_name {
                    Some(name) => name.clone(),
                    None => {
                        // Derive it from the param name (capitalize first char).
                        capitalize_first(&qp.name)
                    }
                };

                // Build accessor chain: e.g. "client.flow_api()" or
                // "client.processgroups_api().process_groups(\"root\")"
                let accessor = build_accessor(&tag_group.accessor_fn, sub_group);

                // Build the function call arguments.
                let call_args = build_call_args(endpoint, sub_group, &qp.rust_name, "ENUM_VALUE");

                for wire_value in &param_change.added_enum_values {
                    let variant = wire_to_pascal(wire_value);
                    let type_lower = enum_type_name.to_lowercase();
                    let variant_lower = variant.to_lowercase();
                    let base_name = format!("enum_{type_lower}_{variant_lower}");

                    let use_trait = trait_use_stmt(&tag_group.struct_name, sub_group);
                    let use_type =
                        format!("use nifi_rust_client::dynamic::types::{enum_type_name};");
                    let enum_arg = format!("Some({enum_type_name}::{variant})");
                    let call_args_filled = call_args.replace("ENUM_VALUE", &enum_arg);

                    // Test: accepted on the version that introduced the variant.
                    let accepted_test = format!(
                        r#"#[cfg(feature = "{to_feature}")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {base_name}_accepted() {{
    {use_trait}
    {use_type}

    let client = helpers::dynamic_logged_in_client().await;
    let result = client.{accessor}.{fn_name}({call_args_filled}).await;
    assert!(result.is_ok(), "expected {enum_type_name}::{variant} to be accepted, got: {{:?}}", result.unwrap_err());
}}"#,
                        to_feature = to_feature,
                        use_trait = use_trait,
                        use_type = use_type,
                        base_name = base_name,
                        accessor = accessor,
                        fn_name = endpoint.fn_name,
                        call_args_filled = call_args_filled,
                        enum_type_name = enum_type_name,
                        variant = variant,
                    );

                    // Test: UnsupportedEnumVariant on versions that don't have this value.
                    let unsupported_test = format!(
                        r#"#[cfg({negative_cfg})]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {base_name}_unsupported() {{
    {use_trait}
    {use_type}

    let client = helpers::dynamic_logged_in_client().await;
    let result = client.{accessor}.{fn_name}({call_args_filled}).await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEnumVariant {{ .. }}),
        "expected UnsupportedEnumVariant, got: {{err:?}}"
    );
}}"#,
                        negative_cfg = negative_cfg,
                        use_trait = use_trait,
                        use_type = use_type,
                        base_name = base_name,
                        accessor = accessor,
                        fn_name = endpoint.fn_name,
                        call_args_filled = call_args_filled,
                    );

                    tests.push(accepted_test);
                    tests.push(unsupported_test);
                }
            }
        }
    }

    if tests.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    out.push_str("// @generated — do not edit; run `cargo run -p nifi-openapi-gen`\n\n");
    out.push_str("#![cfg(feature = \"dynamic\")]\n\n");
    out.push_str("mod helpers;\n\n");
    for test in tests {
        out.push_str(&test);
        out.push_str("\n\n");
    }
    out
}

// ─── Private helpers ──────────────────────────────────────────────────────────

/// Build the argument list for the function call.
/// `enum_param_name` is the `rust_name` of the enum query param to fill;
/// `enum_placeholder` is what to substitute for it.
/// All other params get a default value.
fn build_call_args(
    endpoint: &Endpoint,
    sub_group: Option<&SubGroup>,
    enum_param_rust_name: &str,
    enum_placeholder: &str,
) -> String {
    let mut args: Vec<String> = Vec::new();
    let primary_param = sub_group.map(|sg| sg.primary_param.as_str());

    // Path params (excluding primary for sub-group endpoints).
    for pp in &endpoint.path_params {
        if primary_param == Some(pp.name.as_str()) {
            continue;
        }
        let val = default_path_param_value(&pp.name);
        args.push(format!("\"{val}\""));
    }

    // Add request body (if any) — borrowed.
    if endpoint.request_type.is_some() {
        args.push("&Default::default()".to_string());
    }

    // Add query params.
    for qp in &endpoint.query_params {
        if qp.rust_name == enum_param_rust_name {
            args.push(enum_placeholder.to_string());
        } else {
            // Default: None for optional, sensible literal for required.
            if qp.required {
                args.push(default_required_query_param_value(&qp.ty));
            } else {
                args.push("None".to_string());
            }
        }
    }

    args.join(",\n        ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ApiSpec;

    fn empty_spec() -> ApiSpec {
        ApiSpec {
            tags: vec![],
            all_types: vec![],
        }
    }

    #[test]
    fn empty_inputs_returns_empty_string() {
        let result = emit_enum_coverage_tests(&[], &[]);
        assert_eq!(result, "");
    }

    #[test]
    fn no_changed_params_returns_empty_string() {
        use crate::diff::{EndpointDiff, TypesDiff, VersionDiff};
        let specs = vec![
            ("2.7.2".to_string(), empty_spec()),
            ("2.8.0".to_string(), empty_spec()),
        ];
        let diffs = vec![VersionDiff {
            from: "2.7.2".to_string(),
            to: "2.8.0".to_string(),
            endpoints: EndpointDiff {
                added: vec![],
                removed: vec![],
                changed: vec![],
            },
            types: TypesDiff {
                added: vec![],
                removed: vec![],
                changed: vec![],
            },
        }];
        let result = emit_enum_coverage_tests(&specs, &diffs);
        assert_eq!(result, "");
    }
}
