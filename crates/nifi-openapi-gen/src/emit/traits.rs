use crate::parser::{ApiSpec, Endpoint, HttpMethod, QueryParam, RequestBodyKind};
use crate::util::{escape_keyword, format_source};

/// Returns `Vec<(filename, content)>` for the static traits directory.
/// Each API tag produces one file with a root trait and sub-resource traits.
pub fn emit_static_traits(spec: &ApiSpec, types_prefix: &str) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = Vec::new();

    // Generate mod.rs
    let mut mod_out = String::new();
    for tag_group in &spec.tags {
        mod_out.push_str(&format!("pub mod {};\n", tag_group.module_name));
        mod_out.push_str(&format!(
            "pub use {}::{};\n",
            tag_group.module_name, tag_group.struct_name
        ));
        for sg in &tag_group.sub_groups {
            mod_out.push_str(&format!(
                "pub use {}::{};\n",
                tag_group.module_name, sg.struct_name
            ));
        }
    }
    files.push(("mod.rs".to_string(), format_source(&mod_out)));

    // Generate per-tag trait files
    for tag_group in &spec.tags {
        let content = emit_trait_file(tag_group, types_prefix);
        files.push((
            format!("{}.rs", tag_group.module_name),
            format_source(&content),
        ));
    }

    files
}

fn emit_trait_file(tag_group: &crate::parser::TagGroup, types_prefix: &str) -> String {
    let mut out = String::new();

    out.push_str("use crate::NifiError;\n\n");

    // Emit sub-resource traits first so the root trait can reference them
    for sg in &tag_group.sub_groups {
        emit_sub_resource_trait(&mut out, sg, types_prefix);
    }

    // Emit root trait
    out.push_str(&format!("/// The {} API.\n", tag_group.tag));
    out.push_str("#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]\n");
    out.push_str(&format!("pub trait {} {{\n", tag_group.struct_name));

    // Accessor methods for sub-groups (RPITIT — no GAT declarations needed)
    for sg in &tag_group.sub_groups {
        out.push_str(&format!(
            "    fn {}<'b>(&'b self, {}: &'b str) -> impl {} + 'b;\n\n",
            sg.accessor_fn,
            escape_keyword(&sg.primary_param),
            sg.struct_name
        ));
    }

    // Root-level endpoint methods
    for ep in &tag_group.root_endpoints {
        emit_endpoint_method(&mut out, ep, types_prefix, None);
    }

    out.push_str("}\n");
    out
}

fn emit_sub_resource_trait(out: &mut String, sg: &crate::parser::SubGroup, types_prefix: &str) {
    out.push_str(&format!(
        "/// Sub-resource trait for the `{}` sub-group.\n",
        sg.name
    ));
    out.push_str("#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]\n");
    out.push_str(&format!("pub trait {} {{\n", sg.struct_name));

    for ep in &sg.endpoints {
        emit_endpoint_method(out, ep, types_prefix, Some(&sg.primary_param));
    }

    out.push_str("}\n\n");
}

fn emit_endpoint_method(
    out: &mut String,
    ep: &Endpoint,
    types_prefix: &str,
    exclude_param: Option<&str>,
) {
    // Skip form-encoded endpoints
    if ep.body_kind == Some(RequestBodyKind::FormEncoded) {
        return;
    }

    // Doc comment
    if let Some(doc) = &ep.doc {
        out.push_str(&format!("    /// {doc}\n"));
    }

    // Determine return type
    let return_ty = crate::emit::common::response_return_type(ep, types_prefix);

    // Build path param args (excluding the primary param for sub-resource traits)
    let path_param_args: String = ep
        .path_params
        .iter()
        .filter(|p| exclude_param != Some(p.name.as_str()))
        .map(|p| format!(", {}: &str", escape_keyword(&p.name)))
        .collect();

    // Query param args
    let query_param_args: String = ep
        .query_params
        .iter()
        .map(|qp| {
            let rust_type = static_query_param_type(qp, types_prefix);
            if qp.required {
                format!(", {}: {rust_type}", escape_keyword(&qp.rust_name))
            } else {
                format!(", {}: Option<{rust_type}>", escape_keyword(&qp.rust_name))
            }
        })
        .collect();

    // Body param
    let body_arg = if ep.method == HttpMethod::Delete {
        String::new()
    } else if let Some(RequestBodyKind::Json) = &ep.body_kind {
        let req_type = ep.request_type.as_deref().unwrap_or("serde_json::Value");
        format!(", body: &{types_prefix}::types::{req_type}")
    } else {
        crate::emit::common::body_kind_signature(ep.body_kind.as_ref()).to_string()
    };

    // Abstract method signature (no default impl)
    out.push_str(&format!(
        "    async fn {}(&self{path_param_args}{query_param_args}{body_arg}) -> Result<{return_ty}, NifiError>;\n\n",
        ep.fn_name
    ));
}

/// Returns the Rust type string for a query param, using the given types_prefix for enums.
fn static_query_param_type(qp: &QueryParam, types_prefix: &str) -> String {
    use crate::parser::QueryParamType;
    match &qp.ty {
        QueryParamType::Str => "&str".to_string(),
        QueryParamType::Bool => "bool".to_string(),
        QueryParamType::I32 => "i32".to_string(),
        QueryParamType::I64 => "i64".to_string(),
        QueryParamType::F64 => "f64".to_string(),
        QueryParamType::Enum(_) => {
            let type_name = qp
                .enum_type_name
                .as_deref()
                .expect("enum param must have type name");
            format!("{types_prefix}::types::{type_name}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;

    fn make_spec() -> ApiSpec {
        let ep_root = Endpoint {
            method: HttpMethod::Get,
            path: "/controller-services/{id}".to_string(),
            fn_name: "get_controller_service".to_string(),
            doc: Some("Gets a controller service".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: Some("The controller service id.".to_string()),
            }],
            request_type: None,
            body_kind: None,
            body_doc: None,
            response_type: Some("ControllerServiceEntity".to_string()),
            response_inner: Some("ControllerServiceDto".to_string()),
            response_field: Some("component".to_string()),
            response_kind: crate::content_type::ResponseBodyKind::Json {
                schema_ref: "ControllerServiceEntity".to_string(),
            },
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };

        let ep_sub = Endpoint {
            method: HttpMethod::Post,
            path: "/controller-services/{id}/config/analysis".to_string(),
            fn_name: "analyze_configuration".to_string(),
            doc: Some("Performs analysis of the component's configuration".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            request_type: Some("ConfigurationAnalysisEntity".to_string()),
            body_kind: Some(RequestBodyKind::Json),
            body_doc: None,
            response_type: Some("ConfigurationAnalysisEntity".to_string()),
            response_inner: Some("ConfigurationAnalysisDto".to_string()),
            response_field: Some("configuration_analysis".to_string()),
            response_kind: crate::content_type::ResponseBodyKind::Json {
                schema_ref: "ConfigurationAnalysisEntity".to_string(),
            },
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };

        ApiSpec {
            tags: vec![TagGroup {
                tag: "ControllerServices".to_string(),
                struct_name: "ControllerServicesApi".to_string(),
                module_name: "controller_services".to_string(),
                accessor_fn: "controller_services_api".to_string(),
                types: vec![],
                root_endpoints: vec![ep_root],
                sub_groups: vec![SubGroup {
                    name: "config".to_string(),
                    struct_name: "ControllerServicesConfigApi".to_string(),
                    accessor_fn: "config".to_string(),
                    primary_param: "id".to_string(),
                    primary_param_doc: Some("The controller service id.".to_string()),
                    endpoints: vec![ep_sub],
                }],
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn emit_static_traits_generates_root_and_sub_traits() {
        let spec = make_spec();
        let files = emit_static_traits(&spec, "crate::v2_8_0");
        assert!(files.len() >= 2); // mod.rs + controller_services.rs

        let (_, mod_content) = files.iter().find(|(f, _)| f == "mod.rs").unwrap();
        assert!(mod_content.contains("pub mod controller_services;"));

        let (_, content) = files
            .iter()
            .find(|(f, _)| f == "controller_services.rs")
            .unwrap();

        // Root trait with RPITIT accessor
        assert!(content.contains("pub trait ControllerServicesApi"));
        assert!(
            !content.contains("type ControllerServicesConfigApi<'b>"),
            "GAT declaration should not be present"
        );
        assert!(content.contains(
            "fn config<'b>(&'b self, id: &'b str) -> impl ControllerServicesConfigApi + 'b"
        ));

        // Root method
        assert!(content.contains("async fn get_controller_service("));
        assert!(content.contains("id: &str"));

        // Sub-resource trait
        assert!(content.contains("pub trait ControllerServicesConfigApi"));
        assert!(content.contains("async fn analyze_configuration("));
        assert!(content.contains("body: &crate::v2_8_0::types::ConfigurationAnalysisEntity"));

        // Sub-resource trait method should NOT have id param
        let config_trait_start = content
            .find("pub trait ControllerServicesConfigApi")
            .unwrap();
        let config_trait_section = &content[config_trait_start..];
        let analyze_sig_start = config_trait_section
            .find("fn analyze_configuration")
            .unwrap();
        let analyze_sig_end = config_trait_section[analyze_sig_start..]
            .find('{')
            .or_else(|| config_trait_section[analyze_sig_start..].find(';'))
            .unwrap();
        let analyze_sig =
            &config_trait_section[analyze_sig_start..analyze_sig_start + analyze_sig_end];
        assert!(
            !analyze_sig.contains("id: &str"),
            "Sub-resource trait method should not include primary path param 'id'"
        );
    }
}
