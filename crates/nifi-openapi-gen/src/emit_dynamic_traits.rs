use std::collections::BTreeMap;

use crate::parser::{ApiSpec, HttpMethod, RequestBodyKind};
use crate::util::{
    EndpointInfo, collect_all_tags, collect_tag_endpoints, dynamic_query_param_type,
    escape_keyword, format_source, merge_query_params,
};

/// Returns `Vec<(filename, content)>` for `dynamic/traits/`.
/// Input: same tuple as `emit_dynamic`: `(version_str, mod_name, feature_flag, &ApiSpec)`.
pub fn emit_dynamic_traits(versions: &[(&str, &str, &str, &ApiSpec)]) -> Vec<(String, String)> {
    let all_tags = collect_all_tags(versions);
    let mut files: Vec<(String, String)> = Vec::new();

    // Generate mod.rs
    let mut mod_out = String::new();
    for (_tag, struct_name, module_name, _accessor_fn) in &all_tags {
        mod_out.push_str(&format!("mod {module_name};\n"));
        mod_out.push_str(&format!("pub use {module_name}::{struct_name};\n"));
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

    // Trait-level doc comment: use the tag name
    out.push_str(&format!("/// The {tag} API.\n"));

    // Use #[allow(unused_variables)] to suppress warnings for default impls
    // Use #[allow(async_fn_in_trait)] since we don't need Send bounds
    out.push_str("#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]\n");
    out.push_str(&format!("pub trait {struct_name} {{\n"));

    let endpoints = collect_tag_endpoints(versions, tag);
    let total_versions = versions.len();

    for (fn_name, ep_by_version) in &endpoints {
        emit_trait_method(&mut out, versions, fn_name, ep_by_version, total_versions);
    }

    out.push_str("}\n");
    out
}

fn emit_trait_method(
    out: &mut String,
    versions: &[(&str, &str, &str, &ApiSpec)],
    fn_name: &str,
    ep_by_version: &BTreeMap<&str, EndpointInfo<'_>>,
    total_versions: usize,
) {
    // Use the first version's endpoint as the representative for docs/signatures
    let representative = ep_by_version.values().next().unwrap();
    let ep = representative.endpoint;

    // Skip form-encoded endpoints (same as emit_dynamic)
    if ep.body_kind == Some(RequestBodyKind::FormEncoded) {
        return;
    }

    // --- Doc comments ---
    emit_doc_comments(out, versions, ep_by_version, total_versions);

    // --- Determine return type ---
    let return_ty = match &ep.response_inner {
        Some(inner) => format!("types::{inner}"),
        None => match &ep.response_type {
            Some(ty) => format!("types::{ty}"),
            None => "()".into(),
        },
    };
    let return_result = format!("Result<{return_ty}, NifiError>");

    // --- Build path param args ---
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

    // --- Body param ---
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

    // Parameters section
    let has_path_params = !ep.path_params.is_empty() || representative.primary_param.is_some();
    let has_query_params = !ep.query_params.is_empty();
    if has_path_params || has_query_params {
        out.push_str("    ///\n");
        out.push_str("    /// # Parameters\n");
        if let Some(primary) = representative.primary_param {
            // Primary param from sub-group may not be in path_params
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
