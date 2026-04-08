use std::collections::BTreeMap;

use crate::parser::{ApiSpec, Endpoint, QueryParam, QueryParamType};

pub fn format_source(src: &str) -> String {
    match syn::parse_file(src) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => src.to_string(),
    }
}

pub(crate) fn version_to_variant(version: &str) -> String {
    format!("V{}", version.replace('.', "_"))
}

/// Info about an endpoint in the context of a specific version.
pub(crate) struct EndpointInfo<'a> {
    pub endpoint: &'a Endpoint,
    /// None for root endpoints, Some(struct_name) for sub-group endpoints.
    pub sub_struct_name: Option<&'a str>,
    /// The field name for the primary param in the sub-struct (e.g. "id", "port_id").
    pub primary_param: Option<&'a str>,
}

/// Collect all unique (tag, struct_name, module_name, accessor_fn) across versions.
pub(crate) fn collect_all_tags(
    versions: &[(&str, &str, &str, &ApiSpec)],
) -> Vec<(String, String, String, String)> {
    let mut seen: BTreeMap<String, (String, String, String)> = BTreeMap::new();
    for (_, _, _, spec) in versions {
        for tag in &spec.tags {
            seen.entry(tag.tag.clone()).or_insert_with(|| {
                (
                    tag.struct_name.clone(),
                    tag.module_name.clone(),
                    tag.accessor_fn.clone(),
                )
            });
        }
    }
    seen.into_iter()
        .map(|(tag, (sn, mn, af))| (tag, sn, mn, af))
        .collect()
}

/// For a given tag, collect all endpoints (including sub-group endpoints flattened) across versions.
/// Returns BTreeMap<fn_name, BTreeMap<version_str, EndpointInfo>>
pub(crate) fn collect_tag_endpoints<'a>(
    versions: &[(&'a str, &'a str, &'a str, &'a ApiSpec)],
    tag: &str,
) -> BTreeMap<String, BTreeMap<&'a str, EndpointInfo<'a>>> {
    let mut result: BTreeMap<String, BTreeMap<&'a str, EndpointInfo<'a>>> = BTreeMap::new();

    for (ver, _mod_name, _feature, spec) in versions {
        for tg in &spec.tags {
            if tg.tag != tag {
                continue;
            }
            for ep in &tg.root_endpoints {
                result.entry(ep.fn_name.clone()).or_default().insert(
                    ver,
                    EndpointInfo {
                        endpoint: ep,
                        sub_struct_name: None,
                        primary_param: None,
                    },
                );
            }
            // Flatten sub-group endpoints
            for sg in &tg.sub_groups {
                for ep in &sg.endpoints {
                    result.entry(ep.fn_name.clone()).or_default().insert(
                        ver,
                        EndpointInfo {
                            endpoint: ep,
                            sub_struct_name: Some(&sg.struct_name),
                            primary_param: Some(&sg.primary_param),
                        },
                    );
                }
            }
        }
    }

    result
}

/// Merge query params from all versions of an endpoint into a superset.
/// Returns each unique param (by rust_name) along with the count of versions it appears in.
/// Preserves insertion order: params appear in the order first seen across versions.
pub(crate) fn merge_query_params(
    ep_by_version: &BTreeMap<&str, EndpointInfo<'_>>,
) -> Vec<(QueryParam, usize)> {
    let mut result: Vec<(QueryParam, usize)> = Vec::new();
    for info in ep_by_version.values() {
        for qp in &info.endpoint.query_params {
            if let Some((existing, count)) =
                result.iter_mut().find(|(q, _)| q.rust_name == qp.rust_name)
            {
                *count += 1;
                // If any version marks it optional, keep it optional
                if !qp.required {
                    existing.required = false;
                }
            } else {
                result.push((qp.clone(), 1));
            }
        }
    }
    result
}

/// For dynamic mode, enum query params use the dynamic union enum type.
pub(crate) fn dynamic_query_param_type(qp: &QueryParam) -> String {
    match &qp.ty {
        QueryParamType::Str => "&str".to_string(),
        QueryParamType::Bool => "bool".to_string(),
        QueryParamType::I32 => "i32".to_string(),
        QueryParamType::I64 => "i64".to_string(),
        QueryParamType::F64 => "f64".to_string(),
        QueryParamType::Enum(_) => {
            let type_name = qp.enum_type_name.as_deref().unwrap_or("String");
            format!("types::{type_name}")
        }
    }
}

/// Escape Rust keywords by prefixing with `r#`.
pub(crate) fn escape_keyword(name: &str) -> String {
    match name {
        "type" | "ref" | "use" | "mod" | "fn" | "let" | "match" | "for" | "if" | "else"
        | "return" | "struct" | "enum" | "impl" | "trait" | "pub" | "super" | "self" | "crate"
        | "where" | "true" | "false" | "in" | "loop" | "while" | "break" | "continue" | "mut"
        | "move" | "async" | "await" | "dyn" | "box" | "const" | "static" | "extern" | "unsafe"
        | "as" => format!("r#{name}"),
        _ => name.to_string(),
    }
}
