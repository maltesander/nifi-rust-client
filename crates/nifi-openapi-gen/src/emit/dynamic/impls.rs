use std::collections::BTreeMap;

use crate::parser::{ApiSpec, HttpMethod, RequestBodyKind};
use crate::util::{
    EndpointInfo, collect_all_tags, collect_tag_sub_groups, dynamic_query_param_type,
    escape_keyword, format_source, merge_query_params, version_to_variant,
};

/// Returns `Vec<(path, content)>` for `dynamic/impls/`.
/// Paths are relative: `mod.rs`, `v2_8_0/mod.rs`, `v2_8_0/flow.rs`, etc.
pub fn emit_dynamic_impls(versions: &[(&str, &str, &str, &ApiSpec)]) -> Vec<(String, String)> {
    let all_tags = collect_all_tags(versions);
    let mut files: Vec<(String, String)> = Vec::new();

    // Top-level mod.rs: one `pub(crate) mod` per version module
    let mut mod_out = String::new();
    for (_ver, mod_name, _feature, _spec) in versions {
        mod_out.push_str(&format!("pub(crate) mod {mod_name};\n"));
    }
    files.push(("mod.rs".to_string(), format_source(&mod_out)));

    // Per-version: mod.rs + per-tag files
    for (ver, mod_name, _feature, spec) in versions {
        let version_files = emit_version_impls(versions, ver, mod_name, spec, &all_tags);
        for (path, content) in version_files {
            files.push((format!("{mod_name}/{path}"), content));
        }
    }

    files
}

/// Emit all files for a single version: `mod.rs` + one file per tag.
fn emit_version_impls(
    versions: &[(&str, &str, &str, &ApiSpec)],
    ver: &str,
    mod_name: &str,
    _spec: &ApiSpec,
    all_tags: &[(String, String, String, String)],
) -> Vec<(String, String)> {
    let mut files = Vec::new();

    // Build version mod.rs
    let mut mod_out = String::new();
    for (_tag, struct_name, module_name, _accessor_fn) in all_tags {
        let wrapper_struct = format!("{}{struct_name}", version_struct_prefix(mod_name));
        mod_out.push_str(&format!("pub(crate) mod {module_name};\n"));
        mod_out.push_str(&format!(
            "pub(crate) use {module_name}::{wrapper_struct};\n"
        ));
    }
    files.push(("mod.rs".to_string(), format_source(&mod_out)));

    // Per-tag impl files
    for (tag, struct_name, module_name, _accessor_fn) in all_tags {
        let content = emit_tag_impl_file(versions, ver, mod_name, tag, struct_name, module_name);
        files.push((format!("{module_name}.rs"), format_source(&content)));
    }

    files
}

/// Build the struct prefix from the mod_name, e.g. "v2_8_0" -> "V2_8_0".
fn version_struct_prefix(mod_name: &str) -> String {
    let mut chars = mod_name.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn emit_tag_impl_file(
    versions: &[(&str, &str, &str, &ApiSpec)],
    ver: &str,
    mod_name: &str,
    tag: &str,
    struct_name: &str,
    _module_name: &str,
) -> String {
    let mut out = String::new();
    let prefix = version_struct_prefix(mod_name);
    let wrapper_struct = format!("{prefix}{struct_name}");

    let sub_groups = collect_tag_sub_groups(versions, tag);

    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("use crate::NifiError;\n");
    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("use crate::dynamic::types;\n");
    out.push_str(&format!("use crate::dynamic::traits::{struct_name};\n"));

    // Import sub-resource trait names (needed for RPITIT bounds)
    for sg in &sub_groups.sub_groups {
        out.push_str(&format!(
            "#[allow(unused_imports)]\nuse crate::dynamic::traits::{};\n",
            sg.struct_name
        ));
    }
    out.push('\n');

    // Struct definition
    out.push_str(&format!(
        "pub(crate) struct {wrapper_struct}<'a> {{\n    pub(crate) client: &'a crate::NifiClient,\n}}\n\n"
    ));

    // Trait impl
    out.push_str("#[allow(unused_variables)]\n");
    out.push_str(&format!("impl {struct_name} for {wrapper_struct}<'_> {{\n"));

    // Accessor methods for sub-resources (RPITIT — no GAT bindings needed)
    let variant = version_to_variant(ver);
    for sg in &sub_groups.sub_groups {
        let sub_trait_name = &sg.struct_name;
        let sub_dispatch_name = format!("{sub_trait_name}Dispatch");
        let accessor = &sg.accessor_fn;
        let primary = &sg.primary_param;

        // Accessor method (RPITIT)
        out.push_str(&format!(
            "    fn {accessor}<'b>(&'b self, {primary}: &'b str) -> impl {sub_trait_name} + 'b {{\n"
        ));
        out.push_str(&format!(
            "        crate::dynamic::dispatch::{sub_dispatch_name} {{\n"
        ));
        out.push_str("            client: self.client,\n");
        out.push_str(&format!("            {primary}: {primary}.to_string(),\n"));
        out.push_str(&format!(
            "            version: crate::dynamic::DetectedVersion::{variant},\n"
        ));
        out.push_str("        }\n");
        out.push_str("    }\n\n");
    }

    // Only root endpoint methods (sub-group endpoints are handled by dispatch structs)
    for (fn_name, ep_by_version) in &sub_groups.root_endpoints {
        // Only emit methods that exist in this version; missing ones use the trait default.
        if let Some(info) = ep_by_version.get(ver) {
            emit_impl_method(
                &mut out,
                versions,
                ver,
                mod_name,
                fn_name,
                info,
                ep_by_version,
                struct_name,
            );
        }
    }

    out.push_str("}\n");
    out
}

#[allow(clippy::too_many_arguments)]
fn emit_impl_method(
    out: &mut String,
    versions: &[(&str, &str, &str, &ApiSpec)],
    ver: &str,
    mod_name: &str,
    fn_name: &str,
    info: &EndpointInfo<'_>,
    ep_by_version: &BTreeMap<&str, EndpointInfo<'_>>,
    tag_struct_name: &str,
) {
    let ep = info.endpoint;

    // Skip form-encoded endpoints (same as trait emitter)
    if ep.body_kind == Some(RequestBodyKind::FormEncoded) {
        return;
    }

    // --- Return type (must match trait) ---
    let return_ty = match &ep.response_inner {
        Some(inner) => format!("types::{inner}"),
        None => match &ep.response_type {
            Some(ty) => format!("types::{ty}"),
            None => "()".into(),
        },
    };
    let return_result = format!("Result<{return_ty}, NifiError>");
    let is_void = ep.response_type.is_none() && ep.response_inner.is_none();

    // Use the first version's endpoint as the representative (same as trait emitter)
    let representative = ep_by_version.values().next().unwrap();

    // --- Path param args (must match trait signature) ---
    let mut path_param_names: Vec<String> = Vec::new();
    if let Some(primary) = representative.primary_param {
        path_param_names.push(primary.to_string());
    }
    for p in &representative.endpoint.path_params {
        if !path_param_names.contains(&p.name) {
            path_param_names.push(p.name.clone());
        }
    }
    let path_param_args: String = path_param_names
        .iter()
        .map(|name| format!(", {}: &str", escape_keyword(name)))
        .collect();

    // --- Query params (must match trait signature — use merged union) ---
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

    // --- Body param (must match trait signature — borrowed for dynamic) ---
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

    // --- Method signature ---
    out.push_str(&format!(
        "    async fn {fn_name}(&self{path_param_args}{query_param_args}{body_arg}) -> {return_result} {{\n"
    ));

    // --- Construct the version-specific API struct ---
    // Find the tag's module_name from the versions data
    let tag_module_name = find_tag_module_name(versions, ver, tag_struct_name);

    match (info.sub_struct_name, info.primary_param) {
        (Some(sub_struct), Some(primary_param)) => {
            let arg = escape_keyword(primary_param);
            if arg == primary_param {
                out.push_str(&format!(
                    "        let api = crate::{mod_name}::api::{tag_module_name}::{sub_struct} {{ client: self.client, {primary_param} }};\n"
                ));
            } else {
                out.push_str(&format!(
                    "        let api = crate::{mod_name}::api::{tag_module_name}::{sub_struct} {{ client: self.client, {primary_param}: {arg} }};\n"
                ));
            }
        }
        _ => {
            out.push_str(&format!(
                "        let api = crate::{mod_name}::api::{tag_module_name}::{tag_struct_name} {{ client: self.client }};\n"
            ));
        }
    }

    // --- Build call arguments ---
    let mut call_args = Vec::new();

    // Path params (skip primary_param for sub-groups)
    let primary_to_skip = info.primary_param;
    for p in &ep.path_params {
        if primary_to_skip.is_some_and(|pp| pp == p.name) {
            continue;
        }
        call_args.push(escape_keyword(&p.name));
    }

    // Query params — only pass params this version actually has
    for qp in &ep.query_params {
        let forced_option = merged_query_params
            .iter()
            .find(|(mq, _)| mq.rust_name == qp.rust_name)
            .map(|(_, count)| *count < all_version_count)
            .unwrap_or(false);

        if qp.enum_type_name.is_some() {
            let type_name = qp.enum_type_name.as_deref().unwrap();
            if qp.required && !forced_option {
                call_args.push(format!(
                    "crate::{mod_name}::types::{type_name}::try_from({name})?",
                    name = escape_keyword(&qp.rust_name),
                ));
            } else if qp.required && forced_option {
                call_args.push(format!(
                    "crate::{mod_name}::types::{type_name}::try_from({name}.ok_or_else(|| NifiError::UnsupportedEndpoint {{ endpoint: \"{fn_name} (missing required param {raw_name})\".to_string(), version: \"{ver}\".to_string() }})?)?",
                    name = escape_keyword(&qp.rust_name),
                    raw_name = qp.rust_name,
                ));
            } else {
                call_args.push(format!(
                    "{name}.map(crate::{mod_name}::types::{type_name}::try_from).transpose()?",
                    name = escape_keyword(&qp.rust_name),
                ));
            }
        } else if qp.required && forced_option {
            call_args.push(format!(
                "{name}.ok_or_else(|| NifiError::UnsupportedEndpoint {{ endpoint: \"{fn_name} (missing required param {raw_name})\".to_string(), version: \"{ver}\".to_string() }})?",
                name = escape_keyword(&qp.rust_name),
                raw_name = qp.rust_name,
            ));
        } else {
            call_args.push(escape_keyword(&qp.rust_name));
        }
    }

    // Body param
    if ep.method != HttpMethod::Delete {
        match &ep.body_kind {
            Some(RequestBodyKind::Json) => {
                let req_type = ep.request_type.as_deref().unwrap_or("serde_json::Value");
                call_args.push(format!(
                    "&crate::{mod_name}::types::{req_type}::try_from(body.clone())?",
                ));
            }
            Some(RequestBodyKind::OctetStream) => {
                call_args.push("filename".to_string());
                call_args.push("data".to_string());
            }
            Some(RequestBodyKind::FormEncoded) | None => {}
        }
    }

    let args_str = call_args.join(", ");

    if is_void {
        out.push_str(&format!("        api.{fn_name}({args_str}).await\n"));
    } else {
        out.push_str(&format!(
            "        Ok(api.{fn_name}({args_str}).await?.into())\n"
        ));
    }

    out.push_str("    }\n\n");
}

/// Find the module name for a tag struct in a specific version.
/// Falls back to a snake_case conversion of the struct name if not found.
fn find_tag_module_name(
    versions: &[(&str, &str, &str, &ApiSpec)],
    ver: &str,
    tag_struct_name: &str,
) -> String {
    for (v, _, _, spec) in versions {
        if *v == ver {
            for tg in &spec.tags {
                if tg.struct_name == tag_struct_name {
                    return tg.module_name.clone();
                }
            }
        }
    }
    // Fallback: convert struct name to snake_case module name
    let mut result = String::new();
    for (i, ch) in tag_struct_name.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
    }
    // Strip trailing "_api" if present (module names don't include it)
    result.strip_suffix("_api").unwrap_or(&result).to_string()
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
    fn emit_dynamic_impls_hierarchical() {
        let spec = make_spec();
        let versions: Vec<(&str, &str, &str, &ApiSpec)> =
            vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];
        let files = emit_dynamic_impls(&versions);

        let (_, content) = files
            .iter()
            .find(|(f, _)| f == "v2_8_0/controller_services.rs")
            .unwrap();

        // Struct definition
        assert!(
            content.contains("pub(crate) struct V2_8_0ControllerServicesApi<'a>"),
            "Missing struct definition"
        );

        // RPITIT accessor (no GAT type binding)
        assert!(
            !content.contains("type ControllerServicesConfigApi<'b>"),
            "GAT type binding should not be present. Content:\n{content}"
        );

        // Accessor method with RPITIT return type constructs dispatch struct
        assert!(
            content.contains(
                "fn config<'b>(&'b self, id: &'b str) -> impl ControllerServicesConfigApi + 'b"
            ),
            "Missing RPITIT accessor method. Content:\n{content}"
        );
        assert!(
            content.contains("crate::dynamic::dispatch::ControllerServicesConfigApiDispatch {"),
            "Missing dispatch struct construction. Content:\n{content}"
        );
        assert!(
            content.contains("DetectedVersion::V2_8_0"),
            "Missing version variant. Content:\n{content}"
        );

        // Root endpoint method present
        assert!(
            content.contains("async fn get_controller_service("),
            "Missing root endpoint method. Content:\n{content}"
        );

        // Sub-group endpoint method NOT present (handled by dispatch)
        assert!(
            !content.contains("async fn analyze_configuration("),
            "Sub-group method should NOT be in per-version impl. Content:\n{content}"
        );

        // Sub-resource trait import
        assert!(
            content.contains("use crate::dynamic::traits::ControllerServicesConfigApi"),
            "Missing sub-resource trait import. Content:\n{content}"
        );
    }

    #[test]
    fn emit_dynamic_impls_borrowed_body() {
        let spec = make_spec();
        // Add a root endpoint with a body to test borrowed body
        let mut spec_with_body = spec;
        spec_with_body.tags[0].root_endpoints.push(Endpoint {
            method: HttpMethod::Put,
            path: "/controller-services/{id}".to_string(),
            fn_name: "update_controller_service".to_string(),
            doc: Some("Updates a controller service".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            request_type: Some("ControllerServiceEntity".to_string()),
            body_kind: Some(RequestBodyKind::Json),
            body_doc: None,
            response_type: Some("ControllerServiceEntity".to_string()),
            response_inner: Some("ControllerServiceDto".to_string()),
            response_field: Some("component".to_string()),
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        });

        let versions: Vec<(&str, &str, &str, &ApiSpec)> =
            vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec_with_body)];
        let files = emit_dynamic_impls(&versions);

        let (_, content) = files
            .iter()
            .find(|(f, _)| f == "v2_8_0/controller_services.rs")
            .unwrap();

        // Body param should be borrowed
        assert!(
            content.contains("body: &types::ControllerServiceEntity"),
            "Body param should be borrowed. Content:\n{content}"
        );

        // Body conversion should clone (rustfmt may split across lines)
        assert!(
            content.contains("body.clone()"),
            "Body conversion should clone. Content:\n{content}"
        );
    }
}
