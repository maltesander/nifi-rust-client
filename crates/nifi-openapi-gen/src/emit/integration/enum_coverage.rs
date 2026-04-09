use crate::diff::VersionDiff;
use crate::parser::{ApiSpec, Endpoint, HttpMethod, QueryParamType, SubGroup, TagGroup};
use crate::util::{version_to_feature, wire_to_pascal};

/// Generates integration tests verifying enum query-param values are accepted
/// on their origin version and rejected (`UnsupportedEnumVariant`) on older versions.
pub fn emit_enum_coverage_tests(
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
            for param_change in &ep_change.changed_params {
                if param_change.added_enum_values.is_empty() {
                    continue;
                }

                // Look up the endpoint in the "to" spec.
                let ep_info =
                    find_endpoint(to_spec, &ep_change.method, &ep_change.path);
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
                    let base_name =
                        format!("enum_{type_lower}_{variant_lower}");

                    let use_trait = trait_use_stmt(&tag_group.struct_name);
                    let use_type =
                        format!("use nifi_rust_client::dynamic::types::{enum_type_name};");
                    let enum_arg = format!("Some({enum_type_name}::{variant})");
                    let call_args_filled =
                        call_args.replace("ENUM_VALUE", &enum_arg);

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
                        r#"#[cfg(not(feature = "{to_feature}"))]
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
                        to_feature = to_feature,
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
            // The sub-group accessor needs the primary param value.
            // Use "prometheus" for the known "producer" param, "root" for "id",
            // otherwise "test-{name}".
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

    // Add path params that aren't already baked into the sub-group accessor.
    let baked_param = sub_group.map(|sg| sg.primary_param.as_str());
    for pp in &endpoint.path_params {
        if baked_param == Some(pp.name.as_str()) {
            continue;
        }
        let val = default_path_param_value(&pp.name);
        args.push(format!("\"{val}\""));
    }

    // Add request body (if any) — use a simple `todo!()` stand-in; but since
    // these are enum-param tests the real interesting param is the query param,
    // and the body is not what we're testing. Use `&Default::default()`.
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
                args.push(default_query_param_value(qp.ty.as_str_type()));
            } else {
                args.push("None".to_string());
            }
        }
    }

    args.join(",\n        ")
}

/// Produce a use statement for the trait needed to call the given struct's methods.
fn trait_use_stmt(struct_name: &str) -> String {
    // struct_name is e.g. "FlowApi" — the dynamic trait has the same name.
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

trait QueryParamTypeExt {
    fn as_str_type(&self) -> &'static str;
}

impl QueryParamTypeExt for QueryParamType {
    fn as_str_type(&self) -> &'static str {
        match self {
            QueryParamType::Str => "str",
            QueryParamType::Bool => "bool",
            QueryParamType::I32 => "i32",
            QueryParamType::I64 => "i64",
            QueryParamType::F64 => "f64",
            QueryParamType::Enum(_) => "enum",
        }
    }
}

fn default_query_param_value(ty: &str) -> String {
    match ty {
        "bool" => "false".to_string(),
        "i32" | "i64" => "0".to_string(),
        "f64" => "0.0".to_string(),
        "str" => "\"\"".to_string(),
        _ => "None".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{ApiSpec, TagGroup};

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

    #[test]
    fn capitalize_first_works() {
        assert_eq!(capitalize_first("includedRegistries"), "IncludedRegistries");
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
}
