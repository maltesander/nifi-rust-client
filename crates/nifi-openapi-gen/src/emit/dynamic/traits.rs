use std::collections::BTreeMap;

use crate::parser::{ApiSpec, HttpMethod, RequestBodyKind};
use crate::util::{
    CollectedSubGroup, EndpointInfo, collect_all_tags, collect_tag_sub_groups,
    dynamic_query_param_type, escape_keyword, format_source, merge_query_params,
};

/// Returns `Vec<(filename, content)>` for `dynamic/traits/`.
/// Input: same tuple as `emit_dynamic`: `(version_str, mod_name, feature_flag, &ApiSpec)`.
pub fn emit_dynamic_traits(versions: &[(&str, &str, &str, &ApiSpec)]) -> Vec<(String, String)> {
    let all_tags = collect_all_tags(versions);
    let mut files: Vec<(String, String)> = Vec::new();

    // Collect all sub-resource trait names for mod.rs re-exports
    let mut mod_out = String::new();
    for (tag, struct_name, module_name, _accessor_fn) in &all_tags {
        mod_out.push_str(&format!("mod {module_name};\n"));
        mod_out.push_str(&format!("pub use {module_name}::{struct_name};\n"));

        // Also re-export sub-resource traits
        let sub_groups = collect_tag_sub_groups(versions, tag);
        for sg in &sub_groups.sub_groups {
            mod_out.push_str(&format!("pub use {module_name}::{};\n", sg.struct_name));
        }
    }
    files.push(("mod.rs".to_string(), format_source(&mod_out)));

    // Generate per-tag trait files
    for (tag, struct_name, module_name, _accessor_fn) in &all_tags {
        let content = emit_trait_file(versions, tag, struct_name);
        files.push((format!("{module_name}.rs"), format_source(&content)));
    }

    files
}

fn emit_trait_file(
    versions: &[(&str, &str, &str, &ApiSpec)],
    tag: &str,
    struct_name: &str,
) -> String {
    let mut out = String::new();

    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("use crate::NifiError;\n");
    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("use crate::dynamic::types;\n\n");

    let sub_groups = collect_tag_sub_groups(versions, tag);
    let total_versions = versions.len();

    // --- Emit sub-resource traits first (so root trait can reference them) ---
    for sg in &sub_groups.sub_groups {
        emit_sub_resource_trait(&mut out, versions, sg, total_versions);
    }

    // --- Emit root trait ---
    out.push_str(&format!("/// The {tag} API.\n"));
    out.push_str("#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]\n");
    out.push_str(&format!("pub trait {struct_name} {{\n"));

    // GAT declarations and accessor methods for sub-resources
    for sg in &sub_groups.sub_groups {
        let sub_trait_name = &sg.struct_name;
        let accessor = &sg.accessor_fn;
        let primary = &sg.primary_param;

        // Doc comment for the primary param
        if let Some(doc) = &sg.primary_param_doc {
            out.push_str(
                "    /// Returns a sub-resource accessor for config operations.\n",
            );
            out.push_str("    ///\n");
            out.push_str("    /// # Parameters\n");
            out.push_str(&format!("    /// - `{primary}`: {doc}\n"));
        }

        // GAT declaration
        out.push_str(&format!(
            "    type {sub_trait_name}<'b>: {sub_trait_name} where Self: 'b;\n"
        ));
        // Accessor method (abstract — no default impl)
        out.push_str(&format!(
            "    fn {accessor}<'b>(&'b self, {primary}: &'b str) -> Self::{sub_trait_name}<'b>;\n\n"
        ));
    }

    // Root endpoint methods
    for (fn_name, ep_by_version) in &sub_groups.root_endpoints {
        emit_trait_method(&mut out, versions, fn_name, ep_by_version, total_versions, None);
    }

    out.push_str("}\n");
    out
}

fn emit_sub_resource_trait(
    out: &mut String,
    versions: &[(&str, &str, &str, &ApiSpec)],
    sg: &CollectedSubGroup<'_>,
    total_versions: usize,
) {
    let trait_name = &sg.struct_name;
    let primary_param = &sg.primary_param;

    out.push_str(&format!("/// Sub-resource trait for {trait_name}.\n"));
    out.push_str("#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]\n");
    out.push_str(&format!("pub trait {trait_name} {{\n"));

    for (fn_name, ep_by_version) in &sg.endpoints {
        emit_trait_method(
            out,
            versions,
            fn_name,
            ep_by_version,
            total_versions,
            Some(primary_param),
        );
    }

    out.push_str("}\n\n");
}

fn emit_trait_method(
    out: &mut String,
    versions: &[(&str, &str, &str, &ApiSpec)],
    fn_name: &str,
    ep_by_version: &BTreeMap<&str, EndpointInfo<'_>>,
    total_versions: usize,
    skip_primary: Option<&str>,
) {
    // Use the first version's endpoint as the representative for docs/signatures
    let representative = ep_by_version.values().next().unwrap();
    let ep = representative.endpoint;

    // Skip form-encoded endpoints (same as emit_dynamic)
    if ep.body_kind == Some(RequestBodyKind::FormEncoded) {
        return;
    }

    // --- Doc comments ---
    emit_doc_comments(out, versions, ep_by_version, total_versions, skip_primary);

    // --- Determine return type ---
    let return_ty = match &ep.response_inner {
        Some(inner) => format!("types::{inner}"),
        None => match &ep.response_type {
            Some(ty) => format!("types::{ty}"),
            None => "()".into(),
        },
    };
    let return_result = format!("Result<{return_ty}, NifiError>");

    // --- Build path param args (skipping primary if in sub-resource) ---
    let mut path_param_names: Vec<String> = Vec::new();
    if let Some(primary) = representative.primary_param
        && (skip_primary.is_none() || skip_primary != Some(primary))
    {
        path_param_names.push(primary.to_string());
    }
    for p in &ep.path_params {
        if skip_primary == Some(p.name.as_str()) {
            continue;
        }
        if !path_param_names.contains(&p.name) {
            path_param_names.push(p.name.clone());
        }
    }
    let path_param_args: String = path_param_names
        .iter()
        .map(|name| format!(", {}: &str", escape_keyword(name)))
        .collect();

    // --- Query params ---
    let merged_query_params = merge_query_params(ep_by_version);
    let all_version_count = ep_by_version.len();
    let query_param_args: String = merged_query_params
        .iter()
        .map(|(qp, presence_count)| {
            let rust_type = dynamic_query_param_type(qp);
            let force_option = *presence_count < all_version_count;
            if qp.required && !force_option {
                format!(", {}: {rust_type}", escape_keyword(&qp.rust_name))
            } else {
                format!(", {}: Option<{rust_type}>", escape_keyword(&qp.rust_name))
            }
        })
        .collect();

    // --- Body param (borrowed for dynamic traits) ---
    let body_arg = if ep.method == HttpMethod::Delete {
        String::new()
    } else {
        match &ep.body_kind {
            Some(RequestBodyKind::Json) => {
                let req_type = ep.request_type.as_deref().unwrap_or("serde_json::Value");
                format!(", body: &types::{req_type}")
            }
            Some(RequestBodyKind::OctetStream) => {
                ", filename: Option<&str>, data: Vec<u8>".to_string()
            }
            Some(RequestBodyKind::FormEncoded) | None => String::new(),
        }
    };

    // --- Method signature with default impl ---
    out.push_str(&format!(
        "    async fn {fn_name}(&self{path_param_args}{query_param_args}{body_arg}) -> {return_result} {{\n"
    ));
    out.push_str(&format!(
        "        Err(NifiError::UnsupportedEndpoint {{ endpoint: \"{fn_name}\".to_string(), version: \"unknown\".to_string() }})\n"
    ));
    out.push_str("    }\n\n");
}

fn emit_doc_comments(
    out: &mut String,
    versions: &[(&str, &str, &str, &ApiSpec)],
    ep_by_version: &BTreeMap<&str, EndpointInfo<'_>>,
    total_versions: usize,
    skip_primary: Option<&str>,
) {
    let representative = ep_by_version.values().next().unwrap();
    let ep = representative.endpoint;

    // Summary
    if let Some(doc) = &ep.doc {
        out.push_str(&format!("    /// {doc}\n"));
    }

    // Description
    if let Some(desc) = &ep.description {
        out.push_str("    ///\n");
        for line in desc.lines() {
            if line.is_empty() {
                out.push_str("    ///\n");
            } else {
                out.push_str(&format!("    /// {line}\n"));
            }
        }
    }

    // HTTP call
    let method = match ep.method {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Put => "PUT",
        HttpMethod::Delete => "DELETE",
    };
    out.push_str("    ///\n");
    out.push_str(&format!(
        "    /// Calls `{method} /nifi-api{path}`.\n",
        path = ep.path
    ));

    // Parameters section — skip the primary param if in sub-resource context
    let has_path_params = ep
        .path_params
        .iter()
        .any(|p| skip_primary != Some(p.name.as_str()))
        || (representative.primary_param.is_some()
            && skip_primary != representative.primary_param);
    let has_query_params = !ep.query_params.is_empty();
    if has_path_params || has_query_params {
        out.push_str("    ///\n");
        out.push_str("    /// # Parameters\n");
        if let Some(primary) = representative.primary_param
            && skip_primary != Some(primary)
        {
            let doc = ep
                .path_params
                .iter()
                .find(|p| p.name == primary)
                .and_then(|p| p.doc.as_deref());
            if let Some(doc) = doc {
                out.push_str(&format!("    /// - `{primary}`: {doc}\n"));
            } else {
                out.push_str(&format!("    /// - `{primary}`\n"));
            }
        }
        for p in &ep.path_params {
            if skip_primary == Some(p.name.as_str()) {
                continue;
            }
            if representative.primary_param.is_some_and(|pp| pp == p.name) {
                continue; // Already emitted above
            }
            if let Some(doc) = &p.doc {
                out.push_str(&format!("    /// - `{}`: {doc}\n", p.name));
            } else {
                out.push_str(&format!("    /// - `{}`\n", p.name));
            }
        }
        for qp in &ep.query_params {
            if let Some(doc) = &qp.doc {
                out.push_str(&format!("    /// - `{}`: {doc}\n", qp.rust_name));
            } else {
                out.push_str(&format!("    /// - `{}`\n", qp.rust_name));
            }
        }
    }

    // Error responses
    if !ep.error_responses.is_empty() {
        out.push_str("    ///\n");
        out.push_str("    /// # Errors\n");
        for (code, desc) in &ep.error_responses {
            out.push_str(&format!("    /// - `{code}`: {desc}\n"));
        }
    }

    // Permissions / security
    if let Some(security) = &ep.security
        && !security.is_empty()
    {
        out.push_str("    ///\n");
        out.push_str("    /// # Permissions\n");
        for perm in security {
            out.push_str(&format!("    /// Requires `{perm}`.\n"));
        }
    }

    // Version availability
    let available_versions: Vec<&str> = versions
        .iter()
        .filter(|(ver, _, _, _)| ep_by_version.contains_key(ver))
        .map(|(ver, _, _, _)| *ver)
        .collect();
    if available_versions.len() < total_versions {
        let ver_list = available_versions.join(", ");
        out.push_str("    ///\n");
        out.push_str(&format!("    /// *Supported in NiFi: {ver_list}*\n"));
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
                doc: None,
            }],
            request_type: None,
            body_kind: None,
            body_doc: None,
            response_type: Some("ControllerServiceEntity".to_string()),
            response_inner: Some("ControllerServiceDto".to_string()),
            response_field: Some("component".to_string()),
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };
        let ep_sub = Endpoint {
            method: HttpMethod::Post,
            path: "/controller-services/{id}/config/analysis".to_string(),
            fn_name: "analyze_configuration".to_string(),
            doc: Some("Performs analysis".to_string()),
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
    fn emit_dynamic_traits_hierarchical() {
        let spec = make_spec();
        let versions: Vec<(&str, &str, &str, &ApiSpec)> =
            vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];
        let files = emit_dynamic_traits(&versions);

        let (_, content) = files
            .iter()
            .find(|(f, _)| f == "controller_services.rs")
            .unwrap();

        // Root trait has GAT accessor
        assert!(
            content.contains("pub trait ControllerServicesApi"),
            "Missing root trait"
        );
        assert!(
            content.contains("ControllerServicesConfigApi<'b>"),
            "Missing GAT declaration"
        );
        assert!(
            content.contains("fn config<'b>("),
            "Missing accessor method"
        );

        // Root method — id as method param
        assert!(
            content.contains("async fn get_controller_service("),
            "Missing root method"
        );

        // Sub-resource trait — no id param on methods
        assert!(
            content.contains("pub trait ControllerServicesConfigApi"),
            "Missing sub-resource trait"
        );
        let config_trait_start = content.find("pub trait ControllerServicesConfigApi").unwrap();
        let config_section = &content[config_trait_start..];
        let analyze_start = config_section.find("fn analyze_configuration").unwrap();
        let analyze_end = config_section[analyze_start..].find('{').unwrap();
        let analyze_sig = &config_section[analyze_start..analyze_start + analyze_end];
        assert!(
            !analyze_sig.contains("id: &str"),
            "Sub-resource trait method should not have primary param. Sig: {analyze_sig}"
        );

        // Body type uses borrowed dynamic types:: prefix
        assert!(
            content.contains("&types::ConfigurationAnalysisEntity"),
            "Body should be borrowed"
        );

        // mod.rs re-exports both traits
        let (_, mod_content) = files.iter().find(|(f, _)| f == "mod.rs").unwrap();
        assert!(
            mod_content.contains("ControllerServicesApi"),
            "Missing root trait re-export"
        );
        assert!(
            mod_content.contains("ControllerServicesConfigApi"),
            "Missing sub-resource re-export"
        );
    }
}
