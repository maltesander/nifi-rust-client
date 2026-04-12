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
        "/// Dynamic dispatch struct for the {tag} API. Use via the [`{struct_name}`] trait.\n"
    ));
    out.push_str("///\n");
    out.push_str("/// Holds a borrow of the [`DynamicClient`](crate::dynamic::DynamicClient);\n");
    out.push_str("/// the server version is detected lazily on the first API call.\n");

    // Struct definition: holds a borrow of DynamicClient so each method can
    // detect the version lazily.
    out.push_str("#[non_exhaustive]\n");
    out.push_str(&format!("pub struct {dispatch_name}<'a> {{\n"));
    out.push_str("    pub(crate) client: &'a crate::dynamic::DynamicClient,\n");
    out.push_str("}\n\n");

    // Trait impl for the root trait
    out.push_str(&format!("impl {struct_name} for {dispatch_name}<'_> {{\n"));

    // Accessor methods for sub-resources (RPITIT — no GAT bindings needed)
    for sg in &sub_groups.sub_groups {
        let sub_trait_name = &sg.struct_name;
        let sub_dispatch_name = format!("{sub_trait_name}Dispatch");
        let accessor = &sg.accessor_fn;
        let primary = &sg.primary_param;

        // Accessor method (RPITIT) — infallible; the sub-resource also detects
        // the version lazily when its own methods are called.
        out.push_str(&format!(
            "    fn {accessor}<'b>(&'b self, {primary}: &'b str) -> impl {sub_trait_name} + 'b {{\n"
        ));
        out.push_str(&format!("        {sub_dispatch_name} {{\n"));
        out.push_str("            client: self.client,\n");
        out.push_str(&format!("            {primary}: {primary}.to_string(),\n"));
        out.push_str("        }\n");
        out.push_str("    }\n\n");
    }

    // Root endpoint methods (lazy-detect + dispatch)
    for (fn_name, ep_by_version) in &sub_groups.root_endpoints {
        emit_dispatch_method(&mut out, versions, fn_name, ep_by_version, struct_name);
    }

    out.push_str("}\n\n");

    // Sub-resource dispatch structs and their trait impls
    for sg in &sub_groups.sub_groups {
        emit_sub_resource_dispatch_struct(&mut out, versions, sg);
    }

    out
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

    // Struct definition — holds a borrow of DynamicClient so each method
    // can detect the version lazily.
    out.push_str(&format!(
        "/// Sub-resource dispatch struct for [{sub_trait_name}].\n"
    ));
    out.push_str(&format!("pub struct {sub_dispatch_name}<'a> {{\n"));
    out.push_str("    pub(crate) client: &'a crate::dynamic::DynamicClient,\n");
    out.push_str(&format!("    pub(crate) {primary}: String,\n"));
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

/// For known body types that carry `clusterNodeId` nested inside a request field,
/// returns the injection code that sets the field from `DynamicClient::cluster_node_id()`
/// when the caller left it as `None`.
///
/// Known types:
/// - `LineageEntity` → `lineage.request.cluster_node_id`
/// - `ProvenanceEntity` → `provenance.request.cluster_node_id`
fn cluster_node_id_body_injection(body_type: &str) -> Option<(&'static str, &'static str)> {
    // Returns (outer_field, inner_field) for the injection path:
    //   body.{outer_field}.{inner_field}.cluster_node_id
    match body_type {
        "LineageEntity" => Some(("lineage", "request")),
        "ProvenanceEntity" => Some(("provenance", "request")),
        _ => None,
    }
}

fn emit_dispatch_method(
    out: &mut String,
    versions: &[(&str, &str, &str, &ApiSpec)],
    fn_name: &str,
    ep_by_version: &BTreeMap<&str, EndpointInfo<'_>>,
    tag_struct_name: &str,
) {
    let representative = ep_by_version.values().next().unwrap();
    let ep = representative.endpoint;

    // Skip form-encoded endpoints (same as trait and impls emitters)
    if ep.body_kind == Some(RequestBodyKind::FormEncoded) {
        return;
    }

    // --- Return type (must match trait) ---
    let return_ty = crate::emit::common::response_return_type(ep, "");
    let return_result = format!("Result<{return_ty}, NifiError>");

    // --- Path param args (must match trait signature) ---
    let mut path_param_names: Vec<String> = Vec::new();
    if let Some(primary) = representative.primary_param {
        path_param_names.push(primary.to_string());
    }
    for p in &ep.path_params {
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
    } else if let Some(RequestBodyKind::Json) = &ep.body_kind {
        let req_type = ep.request_type.as_deref().unwrap_or("serde_json::Value");
        format!(", body: &types::{req_type}")
    } else {
        crate::emit::common::body_kind_signature(ep.body_kind.as_ref()).to_string()
    };

    // --- Collect param names for forwarding to per-version impl methods ---
    let mut forward_args = Vec::new();
    for name in &path_param_names {
        forward_args.push(escape_keyword(name));
    }
    for (qp, _) in &merged_query_params {
        forward_args.push(escape_keyword(&qp.rust_name));
    }
    if ep.method != HttpMethod::Delete {
        for name in crate::emit::common::body_kind_forward_arg_names(ep.body_kind.as_ref()) {
            forward_args.push((*name).to_string());
        }
    }
    let forward_args_str = forward_args.join(", ");

    // --- Method signature ---
    out.push_str(&format!(
        "    async fn {fn_name}(&self{path_param_args}{query_param_args}{body_arg}) -> {return_result} {{\n"
    ));

    // --- Auto-inject cluster_node_id from DynamicClient when caller passes None ---
    let has_cluster_node_id_qp = merged_query_params
        .iter()
        .any(|(qp, _)| qp.rust_name == "cluster_node_id");
    if has_cluster_node_id_qp {
        out.push_str(
            "        let cluster_node_id = cluster_node_id.or(self.client.cluster_node_id());\n",
        );
    }

    // --- Auto-inject cluster_node_id into known body types ---
    if ep.body_kind.as_ref() == Some(&RequestBodyKind::Json)
        && let Some((outer, inner)) = ep
            .request_type
            .as_deref()
            .and_then(cluster_node_id_body_injection)
    {
        out.push_str("        let body = {\n");
        out.push_str("            let mut b = body.clone();\n");
        out.push_str(&format!(
            "            if let Some(ref mut {outer}) = b.{outer}\n"
        ));
        out.push_str(&format!(
            "                && let Some(ref mut {inner}) = {outer}.{inner}\n"
        ));
        out.push_str(&format!(
            "                && {inner}.cluster_node_id.is_none()\n"
        ));
        out.push_str("            {\n");
        out.push_str(&format!("                {inner}.cluster_node_id = self.client.cluster_node_id().map(|s| s.to_string());\n"));
        out.push_str("            }\n");
        out.push_str("            b\n");
        out.push_str("        };\n");
        out.push_str("        let body = &body;\n");
    }

    // --- Lazy version detection + dispatch ---
    // Detect the server version on first call (idempotent via OnceCell), then
    // construct the per-version impl wrapper inline and delegate. Versions
    // that don't have this endpoint return UnsupportedEndpoint.
    out.push_str("        #[allow(unreachable_patterns)]\n");
    out.push_str("        match self.client.detect_version().await? {\n");
    for (ver, mod_name, _feature, _spec) in versions {
        let variant = version_to_variant(ver);
        if ep_by_version.contains_key(ver) {
            let wrapper_struct = format!("{}{}", version_struct_prefix(mod_name), tag_struct_name);
            out.push_str(&format!(
                "            crate::dynamic::DetectedVersion::{variant} => {{\n"
            ));
            out.push_str(&format!(
                "                let api = crate::dynamic::impls::{mod_name}::{wrapper_struct} {{ client: &self.client.client }};\n"
            ));
            out.push_str(&format!(
                "                api.{fn_name}({forward_args_str}).await\n"
            ));
            out.push_str("            }\n");
        } else {
            out.push_str(&format!(
                "            crate::dynamic::DetectedVersion::{variant} => Err(NifiError::UnsupportedEndpoint {{ endpoint: \"{fn_name}\".to_string(), version: \"{ver}\".to_string() }}),\n"
            ));
        }
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
    let return_ty = crate::emit::common::response_return_type(ep, "");
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
    } else if let Some(RequestBodyKind::Json) = &ep.body_kind {
        let req_type = ep.request_type.as_deref().unwrap_or("serde_json::Value");
        format!(", body: &types::{req_type}")
    } else {
        crate::emit::common::body_kind_signature(ep.body_kind.as_ref()).to_string()
    };

    // `is_void` controls whether the dispatch body forwards the version-
    // specific return value with `.into()` (JSON type conversion) or passes
    // it through verbatim. Anything that isn't a schema-backed JSON type is
    // returned as-is — non-JSON (String/Vec<u8>), schemaless JSON
    // (serde_json::Value), and empty bodies all need direct passthrough.
    let is_void = ep.response_type.is_none();

    // --- Method signature ---
    out.push_str(&format!(
        "    async fn {fn_name}(&self{path_param_args}{query_param_args}{body_arg}) -> {return_result} {{\n"
    ));

    // --- Auto-inject cluster_node_id from DynamicClient when caller passes None ---
    let has_cluster_node_id_qp = merged_query_params
        .iter()
        .any(|(qp, _)| qp.rust_name == "cluster_node_id");
    if has_cluster_node_id_qp {
        out.push_str(
            "        let cluster_node_id = cluster_node_id.or(self.client.cluster_node_id());\n",
        );
    }

    // --- Lazy version detection + dispatch ---
    // Detect the server version on first call (idempotent via OnceCell), then
    // construct the static sub-struct and call the method with proper type
    // conversions (dynamic -> version-specific -> dynamic).
    out.push_str("        #[allow(unreachable_patterns)]\n");
    out.push_str("        match self.client.detect_version().await? {\n");
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
                Some(RequestBodyKind::OctetStream) | Some(RequestBodyKind::Multipart) => {
                    for name in
                        crate::emit::common::body_kind_forward_arg_names(ver_ep.body_kind.as_ref())
                    {
                        call_args.push((*name).to_string());
                    }
                }
                Some(RequestBodyKind::Wildcard) | Some(RequestBodyKind::FormEncoded) | None => {}
            }
        }

        let args_str = call_args.join(", ");
        // Use the version-specific operation name (ver_ep.fn_name) for the
        // delegation call, not the canonical name (fn_name), because the static
        // sub-struct only exposes the name from its own spec.
        let ver_fn_name = &ver_ep.fn_name;

        out.push_str(&format!(
            "            crate::dynamic::DetectedVersion::{variant} => {{\n"
        ));
        out.push_str(&format!(
            "                let api = crate::{mod_name}::api::{static_module}::{static_sub_struct_name} {{\n"
        ));
        out.push_str("                    client: &self.client.client,\n");
        out.push_str(&format!(
            "                    {primary}: &self.{primary},\n"
        ));
        out.push_str("                };\n");
        if is_void {
            out.push_str(&format!(
                "                api.{ver_fn_name}({args_str}).await\n"
            ));
        } else {
            out.push_str(&format!(
                "                Ok(api.{ver_fn_name}({args_str}).await?.into())\n"
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

    fn json_resp(schema: &str) -> crate::content_type::ResponseBodyKind {
        crate::content_type::ResponseBodyKind::Json {
            schema_ref: schema.to_string(),
        }
    }

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
            response_kind: json_resp("ControllerServiceEntity"),
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
            response_kind: json_resp("ConfigurationAnalysisEntity"),
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
            response_kind: json_resp("VerifyConfigRequestDto"),
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

        // Root dispatch is a struct (not an enum) holding &DynamicClient.
        assert!(
            content.contains("pub struct ControllerServicesApiDispatch<'a>"),
            "Missing root dispatch struct"
        );
        assert!(
            content.contains("pub(crate) client: &'a crate::dynamic::DynamicClient"),
            "Root dispatch should borrow &DynamicClient"
        );
        assert!(
            !content.contains("pub enum ControllerServicesApiDispatch"),
            "Root dispatch should no longer be an enum"
        );

        // Root trait impl has RPITIT accessor (no GAT binding)
        assert!(
            !content.contains("type ControllerServicesConfigApi<'b>"),
            "GAT binding should not be present"
        );
        assert!(
            content.contains(
                "fn config<'b>(&'b self, id: &'b str) -> impl ControllerServicesConfigApi + 'b"
            ),
            "Missing RPITIT accessor method in root trait impl"
        );

        // Root endpoint stays on the dispatch struct
        assert!(
            content.contains("async fn get_controller_service("),
            "Missing root method on dispatch struct"
        );
        // Root endpoint method should lazy-detect via detect_version().await?
        assert!(
            content.contains("self.client.detect_version().await?"),
            "Root dispatch method should lazy-detect the version"
        );

        // Sub-resource dispatch struct exists
        assert!(
            content.contains("pub struct ControllerServicesConfigApiDispatch<'a>"),
            "Missing sub-resource dispatch struct"
        );

        // Sub-resource struct now holds a &DynamicClient; no version field.
        assert!(
            content.contains("pub(crate) client: &'a crate::dynamic::DynamicClient"),
            "Sub-resource dispatch struct should borrow &DynamicClient"
        );
        assert!(
            content.contains("pub(crate) id: String"),
            "Missing id field on sub-resource struct"
        );
        assert!(
            !content.contains("pub(crate) version: crate::dynamic::DetectedVersion"),
            "Sub-resource struct should no longer carry a version field"
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

        // Sub-resource methods lazy-detect via detect_version().await?
        assert!(
            content.contains("self.client.detect_version().await?"),
            "Sub-resource method should lazy-detect the version"
        );

        // Static sub-struct construction — client comes from &self.client.client
        // (the NifiClient inside the DynamicClient reference).
        assert!(
            content
                .contains("crate::v2_8_0::api::controller_services::ControllerServicesConfigApi"),
            "Should construct static sub-struct"
        );
        assert!(
            content.contains("client: &self.client.client"),
            "Should pass &self.client.client to static sub-struct"
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

        // mod.rs re-exports both root and sub-resource dispatch structs
        let (_, mod_content) = files.iter().find(|(f, _)| f == "mod.rs").unwrap();
        assert!(
            mod_content.contains("ControllerServicesApiDispatch"),
            "Missing root dispatch struct re-export"
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

    #[test]
    fn emit_dispatch_auto_injects_cluster_node_id_query_param() {
        let ep = Endpoint {
            method: HttpMethod::Get,
            path: "/provenance/lineage/{id}".to_string(),
            fn_name: "get_lineage".to_string(),
            doc: Some("Gets a lineage query".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            request_type: None,
            body_kind: None,
            body_doc: None,
            response_type: Some("LineageDto".to_string()),
            response_inner: None,
            response_field: None,
            response_kind: json_resp("LineageDto"),
            query_params: vec![QueryParam {
                name: "clusterNodeId".to_string(),
                rust_name: "cluster_node_id".to_string(),
                ty: QueryParamType::Str,
                required: false,
                doc: Some("The cluster node id".to_string()),
                enum_type_name: None,
            }],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };
        let spec = ApiSpec {
            tags: vec![TagGroup {
                tag: "Provenance".to_string(),
                struct_name: "ProvenanceApi".to_string(),
                module_name: "provenance".to_string(),
                accessor_fn: "provenance_api".to_string(),
                types: vec![],
                root_endpoints: vec![ep],
                sub_groups: vec![],
            }],
            all_types: vec![],
        };
        let versions: Vec<(&str, &str, &str, &ApiSpec)> =
            vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];
        let files = emit_dynamic_dispatch(&versions);

        let (_, content) = files.iter().find(|(f, _)| f == "provenance.rs").unwrap();

        // Should contain the auto-injection rebinding
        assert!(
            content.contains(
                "let cluster_node_id = cluster_node_id.or(self.client.cluster_node_id());"
            ),
            "Missing cluster_node_id auto-injection. Content:\n{content}"
        );
    }

    #[test]
    fn emit_dispatch_auto_injects_cluster_node_id_into_lineage_body() {
        let ep = Endpoint {
            method: HttpMethod::Post,
            path: "/provenance/lineage".to_string(),
            fn_name: "submit_lineage_request".to_string(),
            doc: Some("Submits a lineage query".to_string()),
            description: None,
            path_params: vec![],
            request_type: Some("LineageEntity".to_string()),
            body_kind: Some(RequestBodyKind::Json),
            body_doc: None,
            response_type: Some("LineageDto".to_string()),
            response_inner: None,
            response_field: None,
            response_kind: json_resp("LineageDto"),
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };
        let spec = ApiSpec {
            tags: vec![TagGroup {
                tag: "Provenance".to_string(),
                struct_name: "ProvenanceApi".to_string(),
                module_name: "provenance".to_string(),
                accessor_fn: "provenance_api".to_string(),
                types: vec![],
                root_endpoints: vec![ep],
                sub_groups: vec![],
            }],
            all_types: vec![],
        };
        let versions: Vec<(&str, &str, &str, &ApiSpec)> =
            vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];
        let files = emit_dynamic_dispatch(&versions);

        let (_, content) = files.iter().find(|(f, _)| f == "provenance.rs").unwrap();

        // Should clone body and inject cluster_node_id
        assert!(
            content.contains("let mut b = body.clone()"),
            "Missing body clone for cluster_node_id injection. Content:\n{content}"
        );
        assert!(
            content.contains("b.lineage"),
            "Missing lineage field access. Content:\n{content}"
        );
        assert!(
            content.contains(".cluster_node_id"),
            "Missing cluster_node_id field access. Content:\n{content}"
        );
        assert!(
            content.contains("cluster_node_id()"),
            "Missing DynamicClient cluster_node_id() call. Content:\n{content}"
        );
    }
}
