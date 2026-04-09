use std::collections::BTreeMap;

use crate::parser::{ApiSpec, HttpMethod, RequestBodyKind};
use crate::util::{
    CollectedSubGroup, EndpointInfo, collect_all_tags, collect_tag_sub_groups,
    dynamic_query_param_type, escape_keyword, format_source, merge_query_params,
    version_to_variant,
};

/// Returns `Vec<(filename, content)>` for `dynamic/dispatch/`.
/// Input: same tuple as `emit_dynamic`: `(version_str, mod_name, feature_flag, &ApiSpec)`.
pub fn emit_dynamic_dispatch(versions: &[(&str, &str, &str, &ApiSpec)]) -> Vec<(String, String)> {
    let all_tags = collect_all_tags(versions);
    let mut files: Vec<(String, String)> = Vec::new();

    // Generate mod.rs — re-export dispatch enum + sub-resource dispatch structs
    let mut mod_out = String::new();
    for (tag, struct_name, module_name, _accessor_fn) in &all_tags {
        let dispatch_name = format!("{struct_name}Dispatch");
        mod_out.push_str(&format!("mod {module_name};\n"));
        mod_out.push_str(&format!("pub use {module_name}::{dispatch_name};\n"));

        // Also re-export sub-resource dispatch structs
        let sub_groups = collect_tag_sub_groups(versions, tag);
        for sg in &sub_groups.sub_groups {
            let sub_dispatch_name = format!("{}Dispatch", sg.struct_name);
            mod_out.push_str(&format!("pub use {module_name}::{sub_dispatch_name};\n"));
        }
    }
    files.push(("mod.rs".to_string(), format_source(&mod_out)));

    // Generate per-tag dispatch files
    for (tag, struct_name, module_name, _accessor_fn) in &all_tags {
        let content = emit_dispatch_file(versions, tag, struct_name);
        files.push((format!("{module_name}.rs"), format_source(&content)));
    }

    files
}

fn emit_dispatch_file(
    versions: &[(&str, &str, &str, &ApiSpec)],
    tag: &str,
    struct_name: &str,
) -> String {
    let mut out = String::new();
    let dispatch_name = format!("{struct_name}Dispatch");

    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("use crate::NifiError;\n");
    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("use crate::dynamic::types;\n");
    out.push_str(&format!("use crate::dynamic::traits::{struct_name};\n"));

    let sub_groups = collect_tag_sub_groups(versions, tag);

    // Import sub-resource traits
    for sg in &sub_groups.sub_groups {
        out.push_str(&format!(
            "use crate::dynamic::traits::{};\n",
            sg.struct_name
        ));
    }
    out.push('\n');

    // Doc comment
    out.push_str(&format!(
        "/// Dynamic dispatch enum for the {tag} API. Use via the [`{struct_name}`] trait.\n"
    ));

    // Enum definition (unchanged)
    out.push_str("#[allow(private_interfaces)]\n");
    out.push_str("#[non_exhaustive]\n");
    out.push_str(&format!("pub enum {dispatch_name}<'a> {{\n"));
    for (ver, mod_name, _feature, _spec) in versions {
        let variant = version_to_variant(ver);
        let wrapper_struct = format!("{}{}", version_struct_prefix(mod_name), struct_name);
        out.push_str(&format!(
            "    {variant}(super::super::impls::{mod_name}::{wrapper_struct}<'a>),\n"
        ));
    }
    out.push_str("}\n\n");

    // Helper methods on the dispatch enum: client() and version()
    // Only needed when there are sub-groups (to construct sub-resource dispatch structs)
    if !sub_groups.sub_groups.is_empty() {
        emit_dispatch_helpers(&mut out, versions, &dispatch_name);
    }

    // Trait impl for the root trait
    out.push_str(&format!("impl {struct_name} for {dispatch_name}<'_> {{\n"));

    // GAT declarations and accessor methods for sub-resources
    for sg in &sub_groups.sub_groups {
        let sub_trait_name = &sg.struct_name;
        let sub_dispatch_name = format!("{sub_trait_name}Dispatch");
        let accessor = &sg.accessor_fn;
        let primary = &sg.primary_param;

        // GAT declaration
        out.push_str(&format!(
            "    type {sub_trait_name}<'b> = {sub_dispatch_name}<'b> where Self: 'b;\n"
        ));
        // Accessor method
        out.push_str(&format!(
            "    fn {accessor}<'b>(&'b self, {primary}: &'b str) -> Self::{sub_trait_name}<'b> {{\n"
        ));
        out.push_str(&format!("        {sub_dispatch_name} {{\n"));
        out.push_str("            client: self.client(),\n");
        out.push_str(&format!("            {primary}: {primary}.to_string(),\n"));
        out.push_str("            version: self.version(),\n");
        out.push_str("        }\n");
        out.push_str("    }\n\n");
    }

    // Root endpoint methods (match dispatch)
    for (fn_name, ep_by_version) in &sub_groups.root_endpoints {
        emit_dispatch_method(&mut out, versions, fn_name, ep_by_version, None);
    }

    out.push_str("}\n\n");

    // Sub-resource dispatch structs and their trait impls
    for sg in &sub_groups.sub_groups {
        emit_sub_resource_dispatch_struct(&mut out, versions, sg);
    }

    out
}

/// Emit `client()` and `version()` helper methods on the dispatch enum.
fn emit_dispatch_helpers(
    out: &mut String,
    versions: &[(&str, &str, &str, &ApiSpec)],
    dispatch_name: &str,
) {
    out.push_str(&format!("impl<'a> {dispatch_name}<'a> {{\n"));

    // client()
    out.push_str("    fn client(&self) -> &'a crate::NifiClient {\n");
    out.push_str("        match self {\n");
    for (ver, _mod_name, _feature, _spec) in versions {
        let variant = version_to_variant(ver);
        out.push_str(&format!(
            "            Self::{variant}(api) => api.client,\n"
        ));
    }
    out.push_str("        }\n");
    out.push_str("    }\n");

    // version()
    out.push_str("    fn version(&self) -> crate::dynamic::DetectedVersion {\n");
    out.push_str("        match self {\n");
    for (ver, _mod_name, _feature, _spec) in versions {
        let variant = version_to_variant(ver);
        out.push_str(&format!(
            "            Self::{variant}(_) => crate::dynamic::DetectedVersion::{variant},\n"
        ));
    }
    out.push_str("        }\n");
    out.push_str("    }\n");

    out.push_str("}\n\n");
}

/// Emit a sub-resource dispatch struct and its trait impl.
fn emit_sub_resource_dispatch_struct(
    out: &mut String,
    versions: &[(&str, &str, &str, &ApiSpec)],
    sg: &CollectedSubGroup<'_>,
) {
    let sub_trait_name = &sg.struct_name;
    let sub_dispatch_name = format!("{sub_trait_name}Dispatch");
    let primary = &sg.primary_param;

    // Struct definition
    out.push_str(&format!(
        "/// Sub-resource dispatch struct for [{sub_trait_name}].\n"
    ));
    out.push_str(&format!("pub struct {sub_dispatch_name}<'a> {{\n"));
    out.push_str("    pub(crate) client: &'a crate::NifiClient,\n");
    out.push_str(&format!("    pub(crate) {primary}: String,\n"));
    out.push_str("    pub(crate) version: crate::dynamic::DetectedVersion,\n");
    out.push_str("}\n\n");

    // Trait impl
    out.push_str(&format!(
        "impl {sub_trait_name} for {sub_dispatch_name}<'_> {{\n"
    ));

    for (fn_name, ep_by_version) in &sg.endpoints {
        emit_sub_dispatch_method(out, versions, fn_name, ep_by_version, sg);
    }

    out.push_str("}\n\n");
}

fn emit_dispatch_method(
    out: &mut String,
    versions: &[(&str, &str, &str, &ApiSpec)],
    fn_name: &str,
    ep_by_version: &BTreeMap<&str, EndpointInfo<'_>>,
    skip_primary: Option<&str>,
) {
    let representative = ep_by_version.values().next().unwrap();
    let ep = representative.endpoint;

    // Skip form-encoded endpoints (same as trait and impls emitters)
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

    // --- Path param args (must match trait signature) ---
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

    // --- Body param (borrowed — matches trait) ---
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

    // --- Collect param names for forwarding ---
    let mut forward_args = Vec::new();
    for name in &path_param_names {
        forward_args.push(escape_keyword(name));
    }
    for (qp, _) in &merged_query_params {
        forward_args.push(escape_keyword(&qp.rust_name));
    }
    if ep.method != HttpMethod::Delete {
        match &ep.body_kind {
            Some(RequestBodyKind::Json) => forward_args.push("body".to_string()),
            Some(RequestBodyKind::OctetStream) => {
                forward_args.push("filename".to_string());
                forward_args.push("data".to_string());
            }
            Some(RequestBodyKind::FormEncoded) | None => {}
        }
    }
    let forward_args_str = forward_args.join(", ");

    // --- Method signature ---
    out.push_str(&format!(
        "    async fn {fn_name}(&self{path_param_args}{query_param_args}{body_arg}) -> {return_result} {{\n"
    ));

    // --- Match dispatch ---
    out.push_str("        match self {\n");
    for (ver, _mod_name, _feature, _spec) in versions {
        let variant = version_to_variant(ver);
        out.push_str(&format!(
            "            Self::{variant}(api) => api.{fn_name}({forward_args_str}).await,\n"
        ));
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");
}

/// Emit a method on a sub-resource dispatch struct.
/// The sub-resource dispatch dispatches on `self.version` and constructs
/// per-version static sub-structs, passing `&self.{primary_param}`.
fn emit_sub_dispatch_method(
    out: &mut String,
    versions: &[(&str, &str, &str, &ApiSpec)],
    fn_name: &str,
    ep_by_version: &BTreeMap<&str, EndpointInfo<'_>>,
    sg: &CollectedSubGroup<'_>,
) {
    let representative = ep_by_version.values().next().unwrap();
    let ep = representative.endpoint;

    // Skip form-encoded endpoints
    if ep.body_kind == Some(RequestBodyKind::FormEncoded) {
        return;
    }

    let primary = &sg.primary_param;

    // --- Return type ---
    let return_ty = match &ep.response_inner {
        Some(inner) => format!("types::{inner}"),
        None => match &ep.response_type {
            Some(ty) => format!("types::{ty}"),
            None => "()".into(),
        },
    };
    let return_result = format!("Result<{return_ty}, NifiError>");

    // --- Path param args (skip primary — it's on self) ---
    let mut path_param_names: Vec<String> = Vec::new();
    for p in &ep.path_params {
        if p.name == *primary {
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

    // --- Body param (borrowed) ---
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

    let is_void = ep.response_type.is_none() && ep.response_inner.is_none();

    // --- Method signature ---
    out.push_str(&format!(
        "    async fn {fn_name}(&self{path_param_args}{query_param_args}{body_arg}) -> {return_result} {{\n"
    ));

    // --- Version match dispatch ---
    // For each version, construct the static sub-struct and call the method
    // with proper type conversions (dynamic -> version-specific -> dynamic)
    out.push_str("        #[allow(unreachable_patterns)]\n");
    out.push_str("        match self.version {\n");
    for (ver, mod_name, _feature, spec) in versions {
        let variant = version_to_variant(ver);

        // Check if this endpoint exists in this version
        if !ep_by_version.contains_key(ver) {
            out.push_str(&format!(
                "            crate::dynamic::DetectedVersion::{variant} => Err(NifiError::UnsupportedEndpoint {{ endpoint: \"{fn_name}\".to_string(), version: \"{ver}\".to_string() }}),\n"
            ));
            continue;
        }

        let ver_info = &ep_by_version[ver];
        let ver_ep = ver_info.endpoint;

        // Find the static sub-struct name for this version
        let static_sub_struct_name = find_static_sub_struct_name(spec, &sg.struct_name);
        let static_module = find_static_module_name(spec, &sg.struct_name);

        // Build call arguments with type conversions
        let mut call_args = Vec::new();

        // Secondary path params (pass directly — they're &str)
        for name in &path_param_names {
            call_args.push(escape_keyword(name));
        }

        // Query params — convert enums, handle forced options
        for qp in &ver_ep.query_params {
            if qp.name == *primary {
                continue;
            }
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

        // Body param with type conversion
        if ver_ep.method != HttpMethod::Delete {
            match &ver_ep.body_kind {
                Some(RequestBodyKind::Json) => {
                    let req_type = ver_ep
                        .request_type
                        .as_deref()
                        .unwrap_or("serde_json::Value");
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

        out.push_str(&format!(
            "            crate::dynamic::DetectedVersion::{variant} => {{\n"
        ));
        out.push_str(&format!(
            "                let api = crate::{mod_name}::api::{static_module}::{static_sub_struct_name} {{\n"
        ));
        out.push_str("                    client: self.client,\n");
        out.push_str(&format!(
            "                    {primary}: &self.{primary},\n"
        ));
        out.push_str("                };\n");
        if is_void {
            out.push_str(&format!(
                "                api.{fn_name}({args_str}).await\n"
            ));
        } else {
            out.push_str(&format!(
                "                Ok(api.{fn_name}({args_str}).await?.into())\n"
            ));
        }
        out.push_str("            }\n");
    }
    out.push_str("            _ => Err(NifiError::UnsupportedEndpoint { endpoint: \"");
    out.push_str(fn_name);
    out.push_str("\".to_string(), version: \"unknown\".to_string() }),\n");
    out.push_str("        }\n");
    out.push_str("    }\n\n");
}

/// Find the static sub-struct name (e.g., `ControllerServicesConfigApi`) in a spec's tag groups.
fn find_static_sub_struct_name(spec: &ApiSpec, dynamic_struct_name: &str) -> String {
    for tg in &spec.tags {
        for sg in &tg.sub_groups {
            if sg.struct_name == dynamic_struct_name {
                return sg.struct_name.clone();
            }
        }
    }
    // Fallback — should not happen
    dynamic_struct_name.to_string()
}

/// Find the static module name for a sub-group's parent tag.
fn find_static_module_name(spec: &ApiSpec, dynamic_struct_name: &str) -> String {
    for tg in &spec.tags {
        for sg in &tg.sub_groups {
            if sg.struct_name == dynamic_struct_name {
                return tg.module_name.clone();
            }
        }
    }
    // Fallback
    String::new()
}

/// Build the struct prefix from the mod_name, e.g. "v2_8_0" -> "V2_8_0".
fn version_struct_prefix(mod_name: &str) -> String {
    let mut chars = mod_name.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
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
        let ep_sub_delete = Endpoint {
            method: HttpMethod::Delete,
            path: "/controller-services/{id}/config/verification-requests/{requestId}".to_string(),
            fn_name: "delete_verification_request".to_string(),
            doc: Some("Deletes a verification request".to_string()),
            description: None,
            path_params: vec![
                PathParam {
                    name: "id".to_string(),
                    doc: None,
                },
                PathParam {
                    name: "request_id".to_string(),
                    doc: Some("The ID of the Verification Request".to_string()),
                },
            ],
            request_type: None,
            body_kind: None,
            body_doc: None,
            response_type: Some("VerifyConfigRequestDto".to_string()),
            response_inner: None,
            response_field: None,
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
                    endpoints: vec![ep_sub, ep_sub_delete],
                }],
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn emit_dynamic_dispatch_hierarchical() {
        let spec = make_spec();
        let versions: Vec<(&str, &str, &str, &ApiSpec)> =
            vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];
        let files = emit_dynamic_dispatch(&versions);

        let (_, content) = files
            .iter()
            .find(|(f, _)| f == "controller_services.rs")
            .unwrap();

        // Dispatch enum exists
        assert!(
            content.contains("pub enum ControllerServicesApiDispatch<'a>"),
            "Missing dispatch enum"
        );

        // Helper methods exist
        assert!(
            content.contains("fn client(&self) -> &'a crate::NifiClient"),
            "Missing client() helper"
        );
        assert!(
            content.contains("fn version(&self) -> crate::dynamic::DetectedVersion"),
            "Missing version() helper"
        );

        // Root trait impl has GAT
        assert!(
            content.contains(
                "type ControllerServicesConfigApi<'b> = ControllerServicesConfigApiDispatch<'b>"
            ),
            "Missing GAT in root trait impl"
        );

        // Root trait impl has accessor method
        assert!(
            content.contains(
                "fn config<'b>(&'b self, id: &'b str) -> Self::ControllerServicesConfigApi<'b>"
            ),
            "Missing accessor method in root trait impl"
        );

        // Root endpoint stays on the dispatch enum
        assert!(
            content.contains("async fn get_controller_service("),
            "Missing root method on dispatch enum"
        );

        // Sub-resource dispatch struct exists
        assert!(
            content.contains("pub struct ControllerServicesConfigApiDispatch<'a>"),
            "Missing sub-resource dispatch struct"
        );

        // Sub-resource struct has the right fields
        assert!(
            content.contains("pub(crate) client: &'a crate::NifiClient"),
            "Missing client field on sub-resource struct"
        );
        assert!(
            content.contains("pub(crate) id: String"),
            "Missing id field on sub-resource struct"
        );
        assert!(
            content.contains("pub(crate) version: crate::dynamic::DetectedVersion"),
            "Missing version field on sub-resource struct"
        );

        // Sub-resource trait impl exists
        assert!(
            content.contains(
                "impl ControllerServicesConfigApi for ControllerServicesConfigApiDispatch<'_>"
            ),
            "Missing sub-resource trait impl"
        );

        // Sub-resource method — analyze_configuration should NOT have id param
        let sub_impl_start = content
            .find("impl ControllerServicesConfigApi for ControllerServicesConfigApiDispatch")
            .unwrap();
        let sub_section = &content[sub_impl_start..];
        let analyze_start = sub_section.find("fn analyze_configuration").unwrap();
        let analyze_end = sub_section[analyze_start..].find('{').unwrap();
        let analyze_sig = &sub_section[analyze_start..analyze_start + analyze_end];
        assert!(
            !analyze_sig.contains("id: &str"),
            "Sub-resource method should not have primary param. Sig: {analyze_sig}"
        );

        // Body param is borrowed
        assert!(
            analyze_sig.contains("body: &types::ConfigurationAnalysisEntity"),
            "Body should be borrowed in sub-resource dispatch. Sig: {analyze_sig}"
        );

        // Sub-resource delete method has secondary path param but not primary
        let delete_start = sub_section.find("fn delete_verification_request").unwrap();
        let delete_end = sub_section[delete_start..].find('{').unwrap();
        let delete_sig = &sub_section[delete_start..delete_start + delete_end];
        assert!(
            delete_sig.contains("request_id: &str"),
            "Sub-resource method should have secondary param. Sig: {delete_sig}"
        );
        // Check that `id: &str` does not appear as a standalone param (it should be on self).
        // We check for ", id: &str" because "request_id: &str" contains the substring "id: &str".
        assert!(
            !delete_sig.contains(", id: &str"),
            "Sub-resource method should not have primary param. Sig: {delete_sig}"
        );

        // Version match dispatch in sub-resource
        assert!(
            content.contains("match self.version"),
            "Sub-resource should dispatch on self.version"
        );

        // Static sub-struct construction
        assert!(
            content
                .contains("crate::v2_8_0::api::controller_services::ControllerServicesConfigApi"),
            "Should construct static sub-struct"
        );
        assert!(
            content.contains("id: &self.id"),
            "Should pass &self.id to static sub-struct"
        );

        // Type conversion: body should use try_from with clone (rustfmt may split lines)
        assert!(
            content.contains("try_from(body.clone())") || content.contains("body.clone()"),
            "Sub-resource body arg should use body.clone() for conversion. Content:\n{content}"
        );
        // Type conversion: non-void return should use Ok(... .into())
        // Check in the sub-resource impl section specifically
        assert!(
            sub_section.contains(".into()"),
            "Sub-resource non-void return should use .into(). Content:\n{content}"
        );

        // mod.rs re-exports both dispatch enum and sub-resource dispatch struct
        let (_, mod_content) = files.iter().find(|(f, _)| f == "mod.rs").unwrap();
        assert!(
            mod_content.contains("ControllerServicesApiDispatch"),
            "Missing dispatch enum re-export"
        );
        assert!(
            mod_content.contains("ControllerServicesConfigApiDispatch"),
            "Missing sub-resource dispatch struct re-export"
        );
    }

    #[test]
    fn emit_dynamic_dispatch_multi_version_unsupported() {
        // Test that versions missing an endpoint get UnsupportedEndpoint
        let spec_with = make_spec();
        let spec_without = ApiSpec {
            tags: vec![TagGroup {
                tag: "ControllerServices".to_string(),
                struct_name: "ControllerServicesApi".to_string(),
                module_name: "controller_services".to_string(),
                accessor_fn: "controller_services_api".to_string(),
                types: vec![],
                root_endpoints: vec![],
                sub_groups: vec![SubGroup {
                    name: "config".to_string(),
                    struct_name: "ControllerServicesConfigApi".to_string(),
                    accessor_fn: "config".to_string(),
                    primary_param: "id".to_string(),
                    primary_param_doc: None,
                    endpoints: vec![],
                }],
            }],
            all_types: vec![],
        };

        let versions: Vec<(&str, &str, &str, &ApiSpec)> = vec![
            ("2.6.0", "v2_6_0", "nifi-2-6-0", &spec_without),
            ("2.8.0", "v2_8_0", "nifi-2-8-0", &spec_with),
        ];
        let files = emit_dynamic_dispatch(&versions);

        let (_, content) = files
            .iter()
            .find(|(f, _)| f == "controller_services.rs")
            .unwrap();

        // V2_6_0 should have UnsupportedEndpoint for the sub-resource methods.
        // After rustfmt, the match arm and error are on separate lines, so check individually.
        assert!(
            content.contains("DetectedVersion::V2_6_0 =>"),
            "V2_6_0 should have a match arm"
        );
        assert!(
            content.contains("version: \"2.6.0\""),
            "V2_6_0 should return UnsupportedEndpoint for missing endpoints"
        );
    }
}
