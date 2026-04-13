use crate::parser::compat::{self, SubGroup};
use crate::parser::{ApiSpec, Endpoint, HttpMethod, QueryParamType, TagGroup};

/// Find an endpoint in the spec by HTTP method and path.
/// Returns `(Endpoint, TagGroup, Option<SubGroup>)`.
pub(super) fn find_endpoint<'a>(
    spec: &'a ApiSpec,
    method: &HttpMethod,
    path: &str,
) -> Option<(&'a Endpoint, &'a TagGroup, Option<SubGroup<'a>>)> {
    for tag in &spec.tags {
        let (root_eps, sub_groups) = compat::regroup(tag);
        for ep in root_eps.iter().copied() {
            if &ep.method == method && ep.path == path {
                return Some((ep, tag, None));
            }
        }
        for sg in sub_groups {
            for ep in sg.endpoints.iter().copied() {
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
pub(super) fn build_accessor(tag_accessor_fn: &str, sub_group: Option<&SubGroup<'_>>) -> String {
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

/// Produce `use` statements needed to call the given struct's methods.
///
/// The canonical dynamic path (Phase 4b+) has no traits — the concrete
/// struct is returned directly by `client.{accessor}()` and its methods
/// are inherent, not trait methods. So neither the root nor sub-group
/// struct needs to be imported. This function returns an empty string;
/// it exists so call sites stay consistent with the legacy expectation
/// that an import line precedes the call.
pub(super) fn trait_use_stmt(_struct_name: &str, _sub_group: Option<&SubGroup<'_>>) -> String {
    String::new()
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
    use crate::parser::compat::SubGroup;

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
    fn trait_use_stmt_canonical_emits_nothing() {
        assert_eq!(trait_use_stmt("FlowApi", None), "");
        let sg = SubGroup {
            name: "metrics".to_string(),
            struct_name: "FlowMetricsApi".to_string(),
            accessor_fn: "metrics".to_string(),
            primary_param: "producer".to_string(),
            primary_param_doc: None,
            endpoints: vec![],
        };
        assert_eq!(trait_use_stmt("FlowApi", Some(&sg)), "");
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
