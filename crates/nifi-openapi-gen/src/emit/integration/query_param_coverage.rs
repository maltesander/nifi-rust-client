use super::common::{
    build_accessor, capitalize_first, default_path_param_value, default_required_query_param_value,
    find_endpoint, trait_use_stmt,
};
use crate::diff::VersionDiff;
use crate::parser::{ApiSpec, Endpoint, QueryParamType, SubGroup};
use crate::util::{version_to_feature, wire_to_pascal};

/// Generates integration tests verifying added query params are passed on
/// supporting versions and silently skipped on older versions.
pub fn emit_query_param_coverage_tests(
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
) -> String {
    if all_specs.is_empty() || diffs.is_empty() {
        return String::new();
    }

    let mut tests: Vec<String> = Vec::new();

    // Collect all version features for cumulative gating.
    let all_features: Vec<String> = all_specs.iter().map(|(v, _)| version_to_feature(v)).collect();

    for diff in diffs {
        let to_feature = version_to_feature(&diff.to);

        // Features for versions that have the new param (to + later).
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
            if ep_change.added_params.is_empty() {
                continue;
            }

            // Look up the endpoint in the "to" spec.
            let ep_info = find_endpoint(to_spec, &ep_change.method, &ep_change.path);
            let (endpoint, tag_group, sub_group) = match ep_info {
                Some(info) => info,
                None => continue,
            };

            for param_name in &ep_change.added_params {
                // Look up the query param metadata by its camelCase name.
                let qp = match endpoint
                    .query_params
                    .iter()
                    .find(|qp| &qp.name == param_name)
                {
                    Some(qp) => qp,
                    None => continue,
                };

                let accessor = build_accessor(&tag_group.accessor_fn, sub_group);
                let tag = tag_group
                    .accessor_fn
                    .strip_suffix("_api")
                    .unwrap_or(&tag_group.accessor_fn);
                let base_name = format!(
                    "param_{tag}_{fn_name}_{rust_name}",
                    fn_name = endpoint.fn_name,
                    rust_name = qp.rust_name,
                );

                let use_trait = trait_use_stmt(&tag_group.struct_name);

                // Build the argument expression for the target param.
                let (param_arg, extra_use) = match &qp.ty {
                    QueryParamType::Enum(variants) => {
                        let enum_type_name = qp
                            .enum_type_name
                            .clone()
                            .unwrap_or_else(|| capitalize_first(&qp.name));
                        let first_variant = variants
                            .first()
                            .map(|v| wire_to_pascal(v))
                            .unwrap_or_else(|| "Unknown".to_string());
                        let arg = format!("Some({enum_type_name}::{first_variant})");
                        let use_stmt =
                            format!("use nifi_rust_client::dynamic::types::{enum_type_name};");
                        (arg, Some(use_stmt))
                    }
                    _ => ("Some(\"test-value\")".to_string(), None),
                };

                let call_args = build_call_args(endpoint, sub_group, &qp.rust_name, &param_arg);

                // Extra use statement line (if any).
                let use_type_line = extra_use
                    .as_deref()
                    .map(|s| format!("\n    {s}"))
                    .unwrap_or_default();

                // Positive test — param accepted on the version that added it.
                let accepted_test = format!(
                    r#"#[cfg(feature = "{to_feature}")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {base_name}_accepted() {{
    {use_trait}{use_type_line}

    let client = helpers::dynamic_logged_in_client().await;
    let result = client.{accessor}.{fn_name}({call_args}).await;
    assert!(result.is_ok(), "expected param {param_name} to be accepted, got: {{:?}}", result.unwrap_err());
}}"#,
                    to_feature = to_feature,
                    base_name = base_name,
                    use_trait = use_trait,
                    use_type_line = use_type_line,
                    accessor = accessor,
                    fn_name = endpoint.fn_name,
                    call_args = call_args,
                    param_name = param_name,
                );

                // Negative test — param silently ignored on older versions.
                let ignored_test = format!(
                    r#"#[cfg({negative_cfg})]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {base_name}_ignored_on_older() {{
    {use_trait}{use_type_line}

    let client = helpers::dynamic_logged_in_client().await;
    let result = client.{accessor}.{fn_name}({call_args}).await;
    // Param should be silently skipped — no error expected
    assert!(result.is_ok(), "expected param to be silently skipped, got: {{:?}}", result.unwrap_err());
}}"#,
                    negative_cfg = negative_cfg,
                    base_name = base_name,
                    use_trait = use_trait,
                    use_type_line = use_type_line,
                    accessor = accessor,
                    fn_name = endpoint.fn_name,
                    call_args = call_args,
                );

                tests.push(accepted_test);
                tests.push(ignored_test);
            }
        }
    }

    if tests.is_empty() {
        return String::new();
    }

    let mut out = String::new();
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
/// `target_param_rust_name` is the `rust_name` of the param being tested;
/// `target_param_arg` is the argument expression to use for it.
/// All other params get defaults.
fn build_call_args(
    endpoint: &Endpoint,
    sub_group: Option<&SubGroup>,
    target_param_rust_name: &str,
    target_param_arg: &str,
) -> String {
    let mut args: Vec<String> = Vec::new();

    // All path params (dynamic traits flatten sub-groups).
    for pp in &endpoint.path_params {
        let val = default_path_param_value(&pp.name);
        args.push(format!("\"{val}\""));
    }

    // Add request body (if any).
    if endpoint.request_type.is_some() {
        args.push("Default::default()".to_string());
    }

    // Add query params.
    for qp in &endpoint.query_params {
        if qp.rust_name == target_param_rust_name {
            args.push(target_param_arg.to_string());
        } else if qp.required {
            args.push(default_required_query_param_value(&qp.ty));
        } else {
            args.push("None".to_string());
        }
    }

    args.join(",\n        ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff::{EndpointDiff, TypesDiff, VersionDiff};
    use crate::parser::ApiSpec;

    fn empty_spec() -> ApiSpec {
        ApiSpec {
            tags: vec![],
            all_types: vec![],
        }
    }

    #[test]
    fn empty_inputs_returns_empty_string() {
        let result = emit_query_param_coverage_tests(&[], &[]);
        assert_eq!(result, "");
    }

    #[test]
    fn no_changed_endpoints_returns_empty_string() {
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
        let result = emit_query_param_coverage_tests(&specs, &diffs);
        assert_eq!(result, "");
    }
}
