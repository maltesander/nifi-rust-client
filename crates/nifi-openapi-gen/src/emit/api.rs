use crate::emit::method::{self, EmitMode};
use crate::parser::{ApiSpec, TagGroup};

/// Returns a list of `(filename, content)` pairs to write into `src/api/`.
///
/// - `"mod.rs"` — module declarations + `impl NifiClient` accessors
/// - `"<tag>.rs"` — one file per API tag with its resource struct and methods
pub fn emit_api(spec: &ApiSpec) -> Vec<(String, String)> {
    emit_api_with_prefix(spec, "crate")
}

/// Like `emit_api`, but uses `types_prefix::types::X` instead of `crate::types::X`.
/// This allows per-version API code to reference its own types module directly
/// (needed when multiple version modules are active simultaneously in dynamic mode).
pub fn emit_api_with_prefix(spec: &ApiSpec, types_prefix: &str) -> Vec<(String, String)> {
    let mut files = vec![];
    files.push(("mod.rs".to_string(), emit_mod(spec)));
    for tag in &spec.tags {
        let mut content = emit_tag(tag);
        if types_prefix != "crate" {
            content = content.replace("crate::types::", &format!("{types_prefix}::types::"));
        }
        files.push((format!("{}.rs", tag.module_name), content));
    }
    files
}

fn emit_mod(spec: &ApiSpec) -> String {
    let mut out = String::new();
    for tag in &spec.tags {
        out.push_str(&format!("pub mod {};\n", tag.module_name));
    }
    out.push_str("\n#[cfg(not(feature = \"dynamic\"))]\nimpl crate::NifiClient {\n");
    for tag in &spec.tags {
        out.push_str(&format!(
            "    /// Access the [{tag} API](https://nifi.apache.org/nifi-docs/rest-api.html).\n",
            tag = tag.tag,
        ));
        out.push_str(&format!(
            "    pub fn {}(&self) -> {}::{}<'_> {{ {}::{} {{ client: self }} }}\n",
            tag.accessor_fn, tag.module_name, tag.struct_name, tag.module_name, tag.struct_name,
        ));
    }
    out.push_str("}\n");
    crate::util::format_source(&out)
}

fn emit_tag(tag: &TagGroup) -> String {
    let mut out = String::new();
    out.push_str("use crate::NifiClient;\nuse crate::NifiError;\n\n");
    out.push_str(&format!(
        "pub struct {}<'a> {{\n    pub(crate) client: &'a NifiClient,\n}}\n\n",
        tag.struct_name
    ));
    out.push_str(
        "#[allow(private_interfaces, clippy::too_many_arguments, clippy::vec_init_then_push)]\n",
    );
    out.push_str(&format!("impl<'a> {}<'a> {{\n", tag.struct_name));
    let mode = EmitMode::Static;
    for ep in tag.endpoints.iter() {
        method::emit_method(ep, &mode, &mut out);
    }
    out.push_str("}\n");
    crate::util::format_source(&out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;

    fn make_controller_services_spec() -> ApiSpec {
        let ep_root = Endpoint {
            method: HttpMethod::Get,
            path: "/controller-services/{id}".to_string(),
            fn_name: "get_controller_service".to_string(),
            raw_operation_id: String::new(),
            doc: Some("Gets a controller service".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            request_type: None,
            body_kind: None,
            body_doc: None,
            multipart_fields: Vec::new(),
            response_type: Some("ControllerServiceEntity".to_string()),
            response_inner: Some("ControllerServiceDto".to_string()),
            response_field: Some("component".to_string()),
            response_kind: crate::content_type::ResponseBodyKind::Json {
                schema_ref: "ControllerServiceEntity".to_string(),
            },
            query_params: vec![],
            header_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };

        let ep_sub = Endpoint {
            method: HttpMethod::Post,
            path: "/controller-services/{id}/config/analysis".to_string(),
            fn_name: "analyze_configuration".to_string(),
            raw_operation_id: String::new(),
            doc: Some("Performs analysis".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            request_type: Some("ConfigurationAnalysisEntity".to_string()),
            body_kind: Some(RequestBodyKind::Json),
            body_doc: None,
            multipart_fields: Vec::new(),
            response_type: Some("ConfigurationAnalysisEntity".to_string()),
            response_inner: Some("ConfigurationAnalysisDto".to_string()),
            response_field: Some("configuration_analysis".to_string()),
            response_kind: crate::content_type::ResponseBodyKind::Json {
                schema_ref: "ConfigurationAnalysisEntity".to_string(),
            },
            query_params: vec![],
            header_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };

        ApiSpec {
            tags: vec![TagGroup {
                tag: "ControllerServices".to_string(),
                struct_name: "ControllerServices".to_string(),
                module_name: "controller_services".to_string(),
                accessor_fn: "controller_services".to_string(),
                types: vec![],
                endpoints: vec![ep_root, ep_sub],
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn emit_api_emits_flat_inherent_methods() {
        let spec = make_controller_services_spec();
        let files = emit_api_with_prefix(&spec, "crate");
        let (_, content) = files
            .iter()
            .find(|(f, _)| f == "controller_services.rs")
            .unwrap();

        // Flat struct name (no "Api" suffix) and inherent impl.
        assert!(
            content.contains("pub struct ControllerServices<'a>"),
            "Missing flat struct. Content:\n{content}"
        );
        assert!(
            content.contains("impl<'a> ControllerServices<'a>"),
            "Missing inherent impl. Content:\n{content}"
        );

        // Both endpoints emitted as flat inherent methods taking `id: &str`.
        // rustfmt wraps multi-arg signatures across lines, so check for the
        // fn name and the `id: &str` param independently.
        assert!(
            content.contains("pub async fn get_controller_service"),
            "Missing get_controller_service as flat method. Content:\n{content}"
        );
        assert!(
            content.contains("pub async fn analyze_configuration"),
            "Missing analyze_configuration as flat method. Content:\n{content}"
        );
        assert!(
            content.contains("id: &str"),
            "Missing `id: &str` path param. Content:\n{content}"
        );

        // No sub-resource builder or sub-struct should be emitted.
        assert!(
            !content.contains("ControllerServicesConfigApi"),
            "Sub-resource struct should not be emitted. Content:\n{content}"
        );
        assert!(
            !content.contains("pub fn config"),
            "Sub-resource accessor should not be emitted. Content:\n{content}"
        );
        // No trait impls either.
        assert!(
            !content.contains("::traits::"),
            "Trait impls should not be emitted. Content:\n{content}"
        );
    }
}
