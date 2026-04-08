use std::collections::BTreeMap;

use crate::parser::{ApiSpec, HttpMethod, RequestBodyKind};
use crate::util::{
    collect_all_tags, collect_tag_endpoints, dynamic_query_param_type, escape_keyword,
    format_source, merge_query_params, version_to_variant, EndpointInfo,
};

/// Returns `Vec<(filename, content)>` for `dynamic/dispatch/`.
/// Input: same tuple as `emit_dynamic`: `(version_str, mod_name, feature_flag, &ApiSpec)`.
pub fn emit_dynamic_dispatch(versions: &[(&str, &str, &str, &ApiSpec)]) -> Vec<(String, String)> {
    let all_tags = collect_all_tags(versions);
    let mut files: Vec<(String, String)> = Vec::new();

    // Generate mod.rs
    let mut mod_out = String::new();
    for (_tag, struct_name, module_name, _accessor_fn) in &all_tags {
        let dispatch_name = format!("{struct_name}Dispatch");
        mod_out.push_str(&format!("mod {module_name};\n"));
        mod_out.push_str(&format!("pub use {module_name}::{dispatch_name};\n"));
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
    out.push_str(&format!("use crate::dynamic::traits::{struct_name};\n\n"));

    // Doc comment
    out.push_str(&format!(
        "/// Dynamic dispatch enum for the {tag} API. Use via the [`{struct_name}`] trait.\n"
    ));

    // Enum definition
    out.push_str("#[allow(private_interfaces)]\n");
    out.push_str("#[non_exhaustive]\n");
    out.push_str(&format!("pub enum {dispatch_name}<'a> {{\n"));
    for (ver, mod_name, _feature, _spec) in versions {
        let variant = version_to_variant(ver);
        let wrapper_struct = format!("{}{}",  version_struct_prefix(mod_name), struct_name);
        out.push_str(&format!(
            "    {variant}(super::super::impls::{mod_name}::{wrapper_struct}<'a>),\n"
        ));
    }
    out.push_str("}\n\n");

    // Trait impl
    out.push_str(&format!("impl {struct_name} for {dispatch_name}<'_> {{\n"));

    let endpoints = collect_tag_endpoints(versions, tag);

    for (fn_name, ep_by_version) in &endpoints {
        emit_dispatch_method(&mut out, versions, fn_name, ep_by_version);
    }

    out.push_str("}\n");
    out
}

fn emit_dispatch_method(
    out: &mut String,
    versions: &[(&str, &str, &str, &ApiSpec)],
    fn_name: &str,
    ep_by_version: &BTreeMap<&str, EndpointInfo<'_>>,
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

    // --- Body param (must match trait signature) ---
    let body_arg = if ep.method == HttpMethod::Delete {
        String::new()
    } else {
        match &ep.body_kind {
            Some(RequestBodyKind::Json) => {
                let req_type = ep.request_type.as_deref().unwrap_or("serde_json::Value");
                format!(", body: types::{req_type}")
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

/// Build the struct prefix from the mod_name, e.g. "v2_8_0" -> "V2_8_0".
fn version_struct_prefix(mod_name: &str) -> String {
    let mut chars = mod_name.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
