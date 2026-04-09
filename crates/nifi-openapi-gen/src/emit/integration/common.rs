use crate::parser::{ApiSpec, Endpoint, HttpMethod, QueryParamType, SubGroup, TagGroup};

/// Find an endpoint in the spec by HTTP method and path.
/// Returns `(Endpoint, TagGroup, Option<SubGroup>)`.
pub(super) fn find_endpoint<'a>(
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

/// Build the accessor for calling the API in dynamic mode.
/// For sub-group endpoints, chains the sub-group accessor with the primary param.
pub(super) fn build_accessor(tag_accessor_fn: &str, sub_group: Option<&SubGroup>) -> String {
    match sub_group {
        Some(sg) => {
            let val = default_path_param_value(&sg.primary_param);
            format!(
                "{tag_accessor_fn}().{accessor}(\"{val}\")",
                accessor = sg.accessor_fn
            )
        }
        None => format!("{tag_accessor_fn}()"),
    }
}

/// Return a sensible placeholder value for a path parameter.
pub(super) fn default_path_param_value(param_name: &str) -> String {
    match param_name {
        "producer" => "prometheus".to_string(),
        "id" | "process_group_id" | "parent_group_id" => "root".to_string(),
        other => format!("test-{other}"),
    }
}

/// Produce use statements for the traits needed to call the given struct's methods.
/// For sub-group endpoints, imports both the root trait (for the accessor) and the
/// sub-resource trait (for the leaf method).
pub(super) fn trait_use_stmt(struct_name: &str, sub_group: Option<&SubGroup>) -> String {
    match sub_group {
        Some(sg) => format!(
            "use nifi_rust_client::dynamic::traits::{{{struct_name}, {sub_trait}}};",
            sub_trait = sg.struct_name
        ),
        None => format!("use nifi_rust_client::dynamic::traits::{struct_name};"),
    }
}

/// Capitalize the first character of a string.
pub(super) fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Return a default value expression for a required query parameter.
pub(super) fn default_required_query_param_value(ty: &QueryParamType) -> String {
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
    use crate::parser::SubGroup;

    #[test]
    fn capitalize_first_works() {
        assert_eq!(capitalize_first("includedRegistries"), "IncludedRegistries");
        assert_eq!(
            capitalize_first("flowMetricsReportingStrategy"),
            "FlowMetricsReportingStrategy"
        );
        assert_eq!(capitalize_first(""), "");
        assert_eq!(capitalize_first("a"), "A");
    }

    #[test]
    fn default_path_param_value_known_params() {
        assert_eq!(default_path_param_value("producer"), "prometheus");
        assert_eq!(default_path_param_value("id"), "root");
        assert_eq!(default_path_param_value("process_group_id"), "root");
        assert_eq!(default_path_param_value("parent_group_id"), "root");
        assert_eq!(default_path_param_value("uuid"), "test-uuid");
    }

    #[test]
    fn trait_use_stmt_root() {
        assert_eq!(
            trait_use_stmt("FlowApi", None),
            "use nifi_rust_client::dynamic::traits::FlowApi;"
        );
    }

    #[test]
    fn trait_use_stmt_sub_group() {
        let sg = SubGroup {
            name: "metrics".to_string(),
            struct_name: "FlowMetricsApi".to_string(),
            accessor_fn: "metrics".to_string(),
            primary_param: "producer".to_string(),
            primary_param_doc: None,
            endpoints: vec![],
        };
        assert_eq!(
            trait_use_stmt("FlowApi", Some(&sg)),
            "use nifi_rust_client::dynamic::traits::{FlowApi, FlowMetricsApi};"
        );
    }

    #[test]
    fn build_accessor_root_endpoint() {
        assert_eq!(build_accessor("flow_api", None), "flow_api()");
    }

    #[test]
    fn build_accessor_sub_group_chains() {
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

    #[test]
    fn default_required_query_param_value_all_types() {
        assert_eq!(
            default_required_query_param_value(&QueryParamType::Bool),
            "false"
        );
        assert_eq!(
            default_required_query_param_value(&QueryParamType::I32),
            "0"
        );
        assert_eq!(
            default_required_query_param_value(&QueryParamType::I64),
            "0"
        );
        assert_eq!(
            default_required_query_param_value(&QueryParamType::F64),
            "0.0"
        );
        assert_eq!(
            default_required_query_param_value(&QueryParamType::Str),
            "\"\""
        );
        assert_eq!(
            default_required_query_param_value(&QueryParamType::Enum(vec![])),
            "None"
        );
    }
}
