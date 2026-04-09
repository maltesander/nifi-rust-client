use crate::diff::VersionDiff;
use crate::parser::{ApiSpec, Endpoint, HttpMethod, QueryParamType, SubGroup, TagGroup};
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

    for diff in diffs {
        let to_feature = version_to_feature(&diff.to);

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
                let qp = match endpoint.query_params.iter().find(|qp| &qp.name == param_name) {
                    Some(qp) => qp,
                    None => continue,
                };

                let accessor = build_accessor(&tag_group.accessor_fn, sub_group);
                let tag = tag_group
                    .accessor_fn
                    .strip_suffix("_api")
                    .unwrap_or(&tag_group.accessor_fn);
                let base_name = format!("param_{tag}_{fn_name}_{rust_name}",
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
                        let use_stmt = format!(
                            "use nifi_rust_client::dynamic::types::{enum_type_name};"
                        );
                        (arg, Some(use_stmt))
                    }
                    _ => ("Some(\"test-value\")".to_string(), None),
                };

                let call_args =
                    build_call_args(endpoint, sub_group, &qp.rust_name, &param_arg);

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
                    r#"#[cfg(not(feature = "{to_feature}"))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {base_name}_ignored_on_older() {{
    {use_trait}{use_type_line}

    let client = helpers::dynamic_logged_in_client().await;
    let result = client.{accessor}.{fn_name}({call_args}).await;
    // Param should be silently skipped — no error expected
    assert!(result.is_ok(), "expected param to be silently skipped, got: {{:?}}", result.unwrap_err());
}}"#,
                    to_feature = to_feature,
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

/// Find an endpoint in the spec by HTTP method and path.
/// Returns `(Endpoint, TagGroup, Option<SubGroup>)`.
fn find_endpoint<'a>(
    spec: &'a ApiSpec,
    method: &HttpMethod,
    path: &str,
) -> Option<(&'a Endpoint, &'a TagGroup, Option<&'a SubGroup>)> {
    for tag in &spec.tags {
        for ep in &tag.root_endpoints {
            if &ep.method == method && ep.path == path {
                return Some((ep, tag, None));
            }
        }
        for sg in &tag.sub_groups {
            for ep in &sg.endpoints {
                if &ep.method == method && ep.path == path {
                    return Some((ep, tag, Some(sg)));
                }
            }
        }
    }
    None
}

/// Build the accessor chain for calling the API, e.g.
/// `flow_api()` or `processgroups_api().process_groups("root")`.
fn build_accessor(tag_accessor_fn: &str, sub_group: Option<&SubGroup>) -> String {
    match sub_group {
        None => format!("{tag_accessor_fn}()"),
        Some(sg) => {
            let param_val = default_path_param_value(&sg.primary_param);
            format!("{tag_accessor_fn}().{}(\"{param_val}\")", sg.accessor_fn)
        }
    }
}

/// Return a sensible placeholder value for a path parameter.
fn default_path_param_value(param_name: &str) -> String {
    match param_name {
        "producer" => "prometheus".to_string(),
        "id" | "process_group_id" | "parent_group_id" => "root".to_string(),
        other => format!("test-{other}"),
    }
}

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

    // Add path params that aren't already baked into the sub-group accessor.
    let baked_param = sub_group.map(|sg| sg.primary_param.as_str());
    for pp in &endpoint.path_params {
        if baked_param == Some(pp.name.as_str()) {
            continue;
        }
        let val = default_path_param_value(&pp.name);
        args.push(format!("\"{val}\""));
    }

    // Add request body (if any).
    if endpoint.request_type.is_some() {
        args.push("&Default::default()".to_string());
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

/// Produce a use statement for the trait needed to call the given struct's methods.
fn trait_use_stmt(struct_name: &str) -> String {
    format!("use nifi_rust_client::dynamic::traits::{struct_name};")
}

/// Capitalize the first character of a string.
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn default_required_query_param_value(ty: &QueryParamType) -> String {
    match ty {
        QueryParamType::Bool => "false".to_string(),
        QueryParamType::I32 | QueryParamType::I64 => "0".to_string(),
        QueryParamType::F64 => "0.0".to_string(),
        QueryParamType::Str => "\"\"".to_string(),
        QueryParamType::Enum(_) => "None".to_string(),
    }
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

    #[test]
    fn capitalize_first_works() {
        assert_eq!(capitalize_first("flowMetricsReportingStrategy"), "FlowMetricsReportingStrategy");
        assert_eq!(capitalize_first(""), "");
        assert_eq!(capitalize_first("a"), "A");
    }

    #[test]
    fn default_path_param_value_known_params() {
        assert_eq!(default_path_param_value("producer"), "prometheus");
        assert_eq!(default_path_param_value("id"), "root");
        assert_eq!(default_path_param_value("uuid"), "test-uuid");
    }

    #[test]
    fn trait_use_stmt_formats_correctly() {
        assert_eq!(
            trait_use_stmt("FlowApi"),
            "use nifi_rust_client::dynamic::traits::FlowApi;"
        );
    }

    #[test]
    fn build_accessor_root_endpoint() {
        assert_eq!(build_accessor("flow_api", None), "flow_api()");
    }

    #[test]
    fn build_accessor_sub_group() {
        let sg = SubGroup {
            name: "metrics".to_string(),
            struct_name: "FlowMetricsApi".to_string(),
            accessor_fn: "metrics".to_string(),
            primary_param: "producer".to_string(),
            primary_param_doc: None,
            endpoints: vec![],
        };
        assert_eq!(
            build_accessor("flow_api", Some(&sg)),
            "flow_api().metrics(\"prometheus\")"
        );
    }
}
