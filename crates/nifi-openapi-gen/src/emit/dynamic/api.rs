//! Emitter for `dynamic/api/*.rs`.
//!
//! Generates one canonical struct per tag (e.g. `Flow<'a>`) holding a
//! borrow of `DynamicClient`. Endpoint methods start with a
//! `require_endpoint(...)` availability check, then dispatch via the
//! shared `NifiClient` HTTP helpers.

use std::collections::BTreeMap;

use crate::canonical::CanonicalSpec;
use crate::emit::method::{self, DynamicMethodCtx, EmitMode};

use super::availability::endpoint_variant_name;
use super::index::{EndpointIndex, IndexedEndpoint};

pub fn emit_api(canonical: &CanonicalSpec, index: &EndpointIndex<'_>) -> Vec<(String, String)> {
    // Group indexed endpoints by tag.
    let mut by_tag: BTreeMap<&str, Vec<&IndexedEndpoint<'_>>> = BTreeMap::new();
    for ep in &index.endpoints {
        by_tag.entry(ep.tag).or_default().push(ep);
    }

    // Tag metadata (struct_name, module_name, accessor_fn) — pull from the latest
    // version's TagGroup definition. This survives 4a; 4b moves it onto Canonical.
    let tag_meta = collect_tag_meta(canonical);

    let mut files: Vec<(String, String)> = Vec::new();

    // mod.rs
    let mut mod_out = String::new();
    for tag in by_tag.keys() {
        if let Some(meta) = tag_meta.get(*tag) {
            mod_out.push_str(&format!("pub mod {};\n", meta.module_name));
        }
    }
    mod_out.push('\n');
    mod_out.push_str("impl crate::dynamic::DynamicClient {\n");
    for tag in by_tag.keys() {
        if let Some(meta) = tag_meta.get(*tag) {
            mod_out.push_str(&format!(
                "    /// Access the [{tag} API](https://nifi.apache.org/nifi-docs/rest-api.html) (canonical dynamic).\n",
            ));
            mod_out.push_str(&format!(
                "    pub fn {accessor}(&self) -> {module}::{struct_name}<'_> {{\n        {module}::{struct_name} {{ client: self }}\n    }}\n",
                accessor = meta.accessor_fn,
                module = meta.module_name,
                struct_name = meta.struct_name,
            ));
        }
    }
    mod_out.push_str("}\n");
    files.push(("mod.rs".to_string(), crate::util::format_source(&mod_out)));

    // Per-tag api file
    for (tag, eps) in &by_tag {
        if let Some(meta) = tag_meta.get(*tag) {
            let content = emit_tag_file(meta, eps);
            files.push((
                format!("{}.rs", meta.module_name),
                crate::util::format_source(&content),
            ));
        }
    }

    files
}

#[derive(Debug)]
struct TagMeta<'a> {
    struct_name: &'a str,
    module_name: &'a str,
    accessor_fn: &'a str,
}

fn collect_tag_meta(canonical: &CanonicalSpec) -> BTreeMap<&str, TagMeta<'_>> {
    let mut out: BTreeMap<&str, TagMeta<'_>> = BTreeMap::new();
    for spec in canonical.per_version_specs.values() {
        for tag in &spec.tags {
            out.entry(tag.tag.as_str()).or_insert(TagMeta {
                struct_name: tag.struct_name.as_str(),
                module_name: tag.module_name.as_str(),
                accessor_fn: tag.accessor_fn.as_str(),
            });
        }
    }
    out
}

fn emit_tag_file(meta: &TagMeta<'_>, endpoints: &[&IndexedEndpoint<'_>]) -> String {
    let mut out = String::new();
    out.push_str("use crate::NifiError;\n");
    out.push_str("use crate::dynamic::DynamicClient;\n");
    out.push_str("use crate::dynamic::availability::Endpoint;\n\n");

    out.push_str(&format!(
        "pub struct {}<'a> {{\n    pub(crate) client: &'a DynamicClient,\n}}\n\n",
        meta.struct_name
    ));

    out.push_str("#[allow(clippy::too_many_arguments, clippy::let_unit_value, clippy::vec_init_then_push, unused_variables)]\n");
    out.push_str(&format!("impl<'a> {}<'a> {{\n", meta.struct_name));
    for ep in endpoints {
        let ctx = DynamicMethodCtx {
            endpoint_versions: &ep.versions,
            query_param_versions: &ep.query_param_versions,
            endpoint_variant: endpoint_variant_name(&ep.key.method, &ep.key.path),
            method_lit: ep.key.method.as_str(),
        };
        let mode = EmitMode::Dynamic(ctx);
        method::emit_method(ep.endpoint, &mode, &mut out);
    }
    out.push_str("}\n\n");

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::canonicalize;
    use crate::parser::{ApiSpec, Endpoint, HttpMethod, TagGroup};

    fn flow_about_spec() -> ApiSpec {
        ApiSpec {
            tags: vec![TagGroup {
                tag: "Flow".to_string(),
                struct_name: "Flow".to_string(),
                module_name: "flow".to_string(),
                accessor_fn: "flow".to_string(),
                types: vec![],
                endpoints: vec![Endpoint {
                    method: HttpMethod::Get,
                    path: "/flow/about".to_string(),
                    fn_name: "get_about_info".to_string(),
                    raw_operation_id: "getAboutInfo".to_string(),
                    doc: None,
                    description: None,
                    path_params: vec![],
                    request_type: None,
                    body_kind: None,
                    body_doc: None,
                    response_type: Some("AboutEntity".to_string()),
                    response_inner: Some("AboutDto".to_string()),
                    response_field: Some("about".to_string()),
                    response_kind: crate::content_type::ResponseBodyKind::Json {
                        schema_ref: "AboutEntity".to_string(),
                    },
                    query_params: vec![],
                    header_params: vec![],
                    success_responses: vec![],
                    error_responses: vec![],
                    security: None,
                }],
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn emit_api_produces_struct_and_method() {
        let canonical = canonicalize(&[("2.8.0".to_string(), flow_about_spec())]);
        let index = EndpointIndex::build(&canonical);
        let files = emit_api(&canonical, &index);
        let flow = files.iter().find(|(n, _)| n == "flow.rs").unwrap();
        assert!(flow.1.contains("pub struct Flow"));
        assert!(flow.1.contains("pub async fn get_about_info"));
        assert!(flow.1.contains("Endpoint::GET_FLOW_ABOUT"));
        assert!(flow.1.contains("require_endpoint"));
        let mod_rs = files.iter().find(|(n, _)| n == "mod.rs").unwrap();
        assert!(mod_rs.1.contains("pub mod flow"));
        assert!(mod_rs.1.contains("pub fn flow"));
    }

    #[test]
    fn emit_api_handles_path_params_and_query_params_flat() {
        use crate::parser::{PathParam, QueryParam, QueryParamType};
        let mut spec = flow_about_spec();
        let processors_tag = TagGroup {
            tag: "Processors".to_string(),
            struct_name: "Processors".to_string(),
            module_name: "processors".to_string(),
            accessor_fn: "processors".to_string(),
            types: vec![],
            endpoints: vec![
                Endpoint {
                    method: HttpMethod::Get,
                    path: "/processors/{id}".to_string(),
                    fn_name: "get_processor".to_string(),
                    raw_operation_id: "getProcessor".to_string(),
                    doc: None,
                    description: None,
                    path_params: vec![PathParam {
                        name: "id".to_string(),
                        doc: None,
                    }],
                    request_type: None,
                    body_kind: None,
                    body_doc: None,
                    response_type: Some("ProcessorEntity".to_string()),
                    response_inner: Some("ProcessorDto".to_string()),
                    response_field: Some("component".to_string()),
                    response_kind: crate::content_type::ResponseBodyKind::Json {
                        schema_ref: "ProcessorEntity".to_string(),
                    },
                    query_params: vec![QueryParam {
                        name: "verbose".to_string(),
                        rust_name: "verbose".to_string(),
                        ty: QueryParamType::Bool,
                        required: false,
                        doc: None,
                        enum_type_name: None,
                    }],
                    header_params: vec![],
                    success_responses: vec![],
                    error_responses: vec![],
                    security: None,
                },
                Endpoint {
                    method: HttpMethod::Get,
                    path: "/processors/{id}/config".to_string(),
                    fn_name: "get_config".to_string(),
                    raw_operation_id: "getConfig".to_string(),
                    doc: None,
                    description: None,
                    path_params: vec![PathParam {
                        name: "id".to_string(),
                        doc: None,
                    }],
                    request_type: None,
                    body_kind: None,
                    body_doc: None,
                    response_type: None,
                    response_inner: None,
                    response_field: None,
                    response_kind: crate::content_type::ResponseBodyKind::Empty,
                    query_params: vec![],
                    header_params: vec![],
                    success_responses: vec![],
                    error_responses: vec![],
                    security: None,
                },
            ],
        };
        spec.tags.push(processors_tag);
        let canonical = canonicalize(&[("2.8.0".to_string(), spec)]);
        let index = EndpointIndex::build(&canonical);
        let files = emit_api(&canonical, &index);
        let proc_file = files.iter().find(|(n, _)| n == "processors.rs").unwrap();
        let body = &proc_file.1;
        // Flat struct name
        assert!(body.contains("pub struct Processors"));
        // Both endpoints as flat inherent methods — rustfmt wraps arg lists
        // across lines, so check method name and `id: &str` separately.
        assert!(body.contains("pub async fn get_processor"));
        assert!(body.contains("pub async fn get_config"));
        assert!(body.contains("id: &str"));
        assert!(body.contains("/processors/"));
        // Query param assembled
        assert!(body.contains("verbose"));
        // No sub-resource structs / accessors
        assert!(!body.contains("ProcessorsConfigApi"));
        assert!(!body.contains("pub fn config"));
    }
}
