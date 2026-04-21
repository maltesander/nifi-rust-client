use crate::parser::{ApiSpec, Endpoint, HttpMethod, QueryParamType, TagGroup};

/// Find an endpoint in the spec by HTTP method and path.
/// Returns `(Endpoint, TagGroup)`.
pub(super) fn find_endpoint<'a>(
    spec: &'a ApiSpec,
    method: &HttpMethod,
    path: &str,
) -> Option<(&'a Endpoint, &'a TagGroup)> {
    for tag in &spec.tags {
        for ep in &tag.endpoints {
            if &ep.method == method && ep.path == path {
                return Some((ep, tag));
            }
        }
    }
    None
}

/// Build the accessor for calling the API in dynamic mode.
///
/// With the Phase 5 flat API, every endpoint lives as an inherent method
/// on the single resource struct, so the accessor is just
/// `{tag_accessor_fn}()`.
pub(super) fn build_accessor(tag_accessor_fn: &str) -> String {
    format!("{tag_accessor_fn}()")
}

/// Return a sensible placeholder value for a path parameter.
pub(super) fn default_path_param_value(param_name: &str) -> String {
    match param_name {
        "producer" => "prometheus".to_string(),
        "id" | "process_group_id" | "parent_group_id" => "root".to_string(),
        other => format!("test-{other}"),
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
    fn build_accessor_flat() {
        assert_eq!(build_accessor("flow"), "flow()");
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
