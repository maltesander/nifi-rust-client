//! Emitter shim for `dynamic_v2/types/*.rs`.
//!
//! In Phase 4a this delegates to the legacy `emit_dynamic_types`, fed from
//! `CanonicalSpec.per_version_specs`. Phase 4b will rewrite this to consume
//! `CanonicalSpec.types` directly with version-set awareness.

use crate::canonical::CanonicalSpec;

pub fn emit_types(canonical: &CanonicalSpec) -> Vec<(String, String)> {
    let specs: Vec<(&str, &crate::parser::ApiSpec)> = canonical
        .per_version_specs
        .iter()
        .map(|(v, s)| (v.as_str(), s))
        .collect();
    crate::emit_dynamic_types(&specs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::canonicalize;
    use crate::parser::{
        ApiSpec, Endpoint, Field, FieldType, HttpMethod, TagGroup, TypeDef, TypeKind,
    };

    fn spec_with_about() -> ApiSpec {
        ApiSpec {
            tags: vec![TagGroup {
                tag: "Flow".to_string(),
                struct_name: "FlowApi".to_string(),
                module_name: "flow".to_string(),
                accessor_fn: "flow_api".to_string(),
                types: vec![],
                root_endpoints: vec![Endpoint {
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
                    success_responses: vec![],
                    error_responses: vec![],
                    security: None,
                }],
                sub_groups: vec![],
            }],
            all_types: vec![
                TypeDef {
                    name: "AboutEntity".to_string(),
                    kind: TypeKind::Entity {
                        field: "about".to_string(),
                        inner: "AboutDto".to_string(),
                    },
                    fields: vec![],
                    doc: None,
                },
                TypeDef {
                    name: "AboutDto".to_string(),
                    kind: TypeKind::Dto,
                    fields: vec![Field {
                        rust_name: "version".to_string(),
                        serde_name: "version".to_string(),
                        ty: FieldType::Opt(Box::new(FieldType::Str)),
                        doc: None,
                        read_only: false,
                        deprecated: false,
                    }],
                    doc: None,
                },
            ],
        }
    }

    #[test]
    fn emit_types_produces_per_tag_files() {
        let canonical = canonicalize(&[("2.8.0".to_string(), spec_with_about())]);
        let files = emit_types(&canonical);
        let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"mod.rs"));
        assert!(names.contains(&"common.rs") || names.iter().any(|n| n.starts_with("flow")));
        let all_content: String = files.iter().map(|(_, c)| c.as_str()).collect();
        assert!(all_content.contains("AboutDto"));
        assert!(all_content.contains("version"));
    }
}
