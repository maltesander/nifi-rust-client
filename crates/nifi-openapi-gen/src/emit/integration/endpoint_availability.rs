use crate::diff::{EndpointSummary, VersionDiff};
use crate::parser::{ApiSpec, Endpoint, HttpMethod, SubGroup, TagGroup};
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

    for diff in diffs {
        if diff.endpoints.added.is_empty() {
            continue;
        }

        let to_feature = version_to_feature(&diff.to);

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
            let use_trait = trait_use_stmt(&tag_group.struct_name);
            let accessor = build_accessor(&tag_group.accessor_fn, sub_group);
            let call_args = build_call_args(endpoint, sub_group);

            // Positive test — only for safe endpoints (no risky required params).
            if is_safe_endpoint(added, endpoint, sub_group) {
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

            // Negative test — always emitted.
            let negative_test = format!(
                r#"#[cfg(not(feature = "{to_feature}"))]
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
                to_feature = to_feature,
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

/// Determine whether it's safe to call an endpoint positively in a test.
///
/// Safe means: GET with no required resource-specific path params,
/// or POST on a process-groups path where we can use "root" as the ID.
fn is_safe_endpoint(
    summary: &EndpointSummary,
    endpoint: &Endpoint,
    sub_group: Option<&SubGroup>,
) -> bool {
    // GET with no extra (non-baked) path params is always safe.
    if summary.method == HttpMethod::Get {
        let baked_param = sub_group.map(|sg| sg.primary_param.as_str());
        let extra_path_params: Vec<_> = endpoint
            .path_params
            .iter()
            .filter(|pp| baked_param != Some(pp.name.as_str()))
            .collect();
        return extra_path_params.is_empty();
    }

    // POST on a process-groups path (we can use "root").
    if summary.method == HttpMethod::Post && summary.path.contains("process-groups") {
        // Must have no extra path params beyond what we can fill with "root".
        let baked_param = sub_group.map(|sg| sg.primary_param.as_str());
        let extra_path_params: Vec<_> = endpoint
            .path_params
            .iter()
            .filter(|pp| baked_param != Some(pp.name.as_str()))
            .filter(|pp| !matches!(pp.name.as_str(), "id" | "process_group_id" | "parent_group_id"))
            .collect();
        return extra_path_params.is_empty();
    }

    false
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
fn build_call_args(endpoint: &Endpoint, sub_group: Option<&SubGroup>) -> String {
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

    // Add query params — all None (optional) or a default for required ones.
    for qp in &endpoint.query_params {
        if qp.required {
            args.push(default_query_param_value(&qp.ty));
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

/// Produce a use statement for the trait needed to call the given struct's methods.
fn trait_use_stmt(struct_name: &str) -> String {
    format!("use nifi_rust_client::dynamic::traits::{struct_name};")
}

fn default_query_param_value(ty: &crate::parser::QueryParamType) -> String {
    match ty {
        crate::parser::QueryParamType::Bool => "false".to_string(),
        crate::parser::QueryParamType::I32 | crate::parser::QueryParamType::I64 => "0".to_string(),
        crate::parser::QueryParamType::F64 => "0.0".to_string(),
        crate::parser::QueryParamType::Str => "\"\"".to_string(),
        crate::parser::QueryParamType::Enum(_) => "None".to_string(),
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
        assert_eq!(test_base_name("flow_api", "get_listen_ports"), "endpoint_flow_get_listen_ports");
        assert_eq!(test_base_name("processors_api", "get_processor"), "endpoint_processors_get_processor");
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
            name: "config".to_string(),
            struct_name: "FlowConfigApi".to_string(),
            accessor_fn: "config".to_string(),
            primary_param: "producer".to_string(),
            primary_param_doc: None,
            endpoints: vec![],
        };
        assert_eq!(
            build_accessor("flow_api", Some(&sg)),
            "flow_api().config(\"prometheus\")"
        );
    }

    #[test]
    fn default_path_param_value_known_params() {
        assert_eq!(default_path_param_value("producer"), "prometheus");
        assert_eq!(default_path_param_value("id"), "root");
        assert_eq!(default_path_param_value("uuid"), "test-uuid");
    }
}
