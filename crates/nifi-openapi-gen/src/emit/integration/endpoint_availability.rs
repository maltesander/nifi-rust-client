use super::common::{
    build_accessor, default_path_param_value, default_required_query_param_value, find_endpoint,
    trait_use_stmt,
};
use crate::diff::{EndpointSummary, VersionDiff};
use crate::parser::{ApiSpec, Endpoint, HttpMethod, SubGroup};
use crate::util::version_to_feature;

/// Generates integration tests verifying endpoints added in a version work on that
/// version and return `UnsupportedEndpoint` on older versions.
pub fn emit_endpoint_availability_tests(
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
) -> String {
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
        if diff.endpoints.added.is_empty() {
            continue;
        }

        let to_feature = version_to_feature(&diff.to);

        // Features for the "to" version and all later versions — the endpoint
        // exists in all of these, so negative tests must exclude them all.
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

        for added in &diff.endpoints.added {
            let ep_info = find_endpoint(to_spec, &added.method, &added.path);
            let (endpoint, tag_group, sub_group) = match ep_info {
                Some(info) => info,
                None => continue,
            };

            let base_name = test_base_name(&tag_group.accessor_fn, &endpoint.fn_name);
            let use_trait = trait_use_stmt(&tag_group.struct_name, sub_group);
            let accessor = build_accessor(&tag_group.accessor_fn, sub_group);
            let call_args = build_call_args(endpoint, sub_group);

            // Positive test — only for safe endpoints (no risky required params).
            if is_safe_endpoint(added, endpoint) {
                let positive_test = format!(
                    r#"#[cfg(feature = "{to_feature}")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {base_name}_available() {{
    {use_trait}

    let client = helpers::dynamic_logged_in_client().await;
    let result = client.{accessor}.{fn_name}({call_args}).await;
    assert!(result.is_ok(), "expected endpoint to be available, got: {{:?}}", result.unwrap_err());
}}"#,
                    to_feature = to_feature,
                    use_trait = use_trait,
                    base_name = base_name,
                    accessor = accessor,
                    fn_name = endpoint.fn_name,
                    call_args = call_args,
                );
                tests.push(positive_test);
            }

            // Negative test — only on versions that lack this endpoint.
            let negative_test = format!(
                r#"#[cfg({negative_cfg})]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {base_name}_unsupported() {{
    {use_trait}

    let client = helpers::dynamic_logged_in_client().await;
    let result = client.{accessor}.{fn_name}({call_args}).await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint {{ .. }}),
        "expected UnsupportedEndpoint, got: {{err:?}}"
    );
}}"#,
                negative_cfg = negative_cfg,
                use_trait = use_trait,
                base_name = base_name,
                accessor = accessor,
                fn_name = endpoint.fn_name,
                call_args = call_args,
            );
            tests.push(negative_test);
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

/// Determine whether it's safe to call an endpoint positively in a test.
///
/// Safe means: GET with no required resource-specific path params,
/// or POST on a process-groups path where we can use "root" as the ID.
fn is_safe_endpoint(summary: &EndpointSummary, endpoint: &Endpoint) -> bool {
    // Only GET endpoints without resource-specific path params are safe.
    // POST/PUT/DELETE may have required body fields that Default::default()
    // cannot satisfy, causing MissingRequiredField errors.
    if summary.method != HttpMethod::Get {
        return false;
    }

    endpoint.path_params.iter().all(|pp| {
        matches!(
            pp.name.as_str(),
            "id" | "process_group_id" | "parent_group_id" | "producer"
        )
    })
}

/// Build the argument list for the function call.
/// For sub-group endpoints, the primary path param is excluded (it's on the accessor).
fn build_call_args(endpoint: &Endpoint, sub_group: Option<&SubGroup>) -> String {
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

    // Add query params — all None (optional) or a default for required ones.
    for qp in &endpoint.query_params {
        if qp.required {
            args.push(default_required_query_param_value(&qp.ty));
        } else {
            args.push("None".to_string());
        }
    }

    args.join(",\n        ")
}

/// Produce the test function base name, e.g. `endpoint_flow_get_listen_ports`.
fn test_base_name(accessor_fn: &str, fn_name: &str) -> String {
    // Strip the trailing "_api" from accessor_fn if present.
    let tag = accessor_fn.strip_suffix("_api").unwrap_or(accessor_fn);
    format!("endpoint_{tag}_{fn_name}")
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
        let result = emit_endpoint_availability_tests(&[], &[]);
        assert_eq!(result, "");
    }

    #[test]
    fn no_added_endpoints_returns_empty_string() {
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
        let result = emit_endpoint_availability_tests(&specs, &diffs);
        assert_eq!(result, "");
    }

    #[test]
    fn test_base_name_strips_api_suffix() {
        assert_eq!(
            test_base_name("flow_api", "get_listen_ports"),
            "endpoint_flow_get_listen_ports"
        );
        assert_eq!(
            test_base_name("processors_api", "get_processor"),
            "endpoint_processors_get_processor"
        );
    }
}
