use crate::parser::{ApiSpec, Endpoint, HttpMethod, QueryParam, SubGroup, TagGroup};

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
    for ep in &tag.root_endpoints {
        out.push_str(&emit_method(ep));
    }
    for sg in &tag.sub_groups {
        out.push_str(&emit_sub_group_accessor(sg));
    }
    out.push_str("}\n");
    for sg in &tag.sub_groups {
        out.push_str(&emit_sub_struct(sg));
    }
    crate::util::format_source(&out)
}

fn emit_sub_group_accessor(sg: &SubGroup) -> String {
    let mut out = String::new();
    // Doc comment: describe the sub-resource scope and the id parameter.
    out.push_str(&format!(
        "    /// Scope operations to the `{}` sub-resource of a specific process group.\n",
        sg.name
    ));
    out.push_str("    ///\n");
    if let Some(doc) = &sg.primary_param_doc {
        out.push_str(&format!("    /// - `{}`: {}\n", sg.primary_param, doc));
    } else {
        out.push_str(&format!(
            "    /// - `{}`: The resource id.\n",
            sg.primary_param
        ));
    }
    out.push_str(&format!(
        "    pub fn {accessor}<'b>(&'b self, {param}: &'b str) -> {struct_name}<'b> {{\n        {struct_name} {{ client: self.client, {param} }}\n    }}\n\n",
        accessor = sg.accessor_fn,
        param = sg.primary_param,
        struct_name = sg.struct_name,
    ));
    out
}

/// Appends `# Parameters` rustdoc section for path params (excluding `skip_param`),
/// query params (with their descriptions), and body, if any have descriptions.
fn emit_param_docs(out: &mut String, ep: &Endpoint, skip_param: Option<&str>) {
    let path_docs: Vec<(&str, &str)> = ep
        .path_params
        .iter()
        .filter(|p| skip_param.is_none_or(|s| p.name != s))
        .filter_map(|p| p.doc.as_deref().map(|d| (p.name.as_str(), d)))
        .collect();
    let query_docs: Vec<(&str, &str)> = ep
        .query_params
        .iter()
        .filter_map(|p| p.doc.as_deref().map(|d| (p.rust_name.as_str(), d)))
        .collect();
    let has_body_doc = ep.body_doc.is_some() && ep.request_type.is_some();

    if path_docs.is_empty() && query_docs.is_empty() && !has_body_doc {
        return;
    }

    out.push_str("    ///\n    /// # Parameters\n");
    for (name, doc) in &path_docs {
        out.push_str(&format!("    /// - `{name}`: {doc}\n"));
    }
    for (name, doc) in &query_docs {
        out.push_str(&format!("    /// - `{name}`: {doc}\n"));
    }
    if let Some(doc) = &ep.body_doc
        && ep.request_type.is_some()
    {
        out.push_str(&format!("    /// - `body`: {doc}\n"));
    }
}

/// Appends `# Returns`, `# Errors`, and `# Permissions` rustdoc sections.
fn emit_error_and_permission_docs(out: &mut String, ep: &Endpoint) {
    if !ep.success_responses.is_empty() {
        out.push_str("    ///\n    /// # Returns\n");
        for (code, desc) in &ep.success_responses {
            out.push_str(&format!("    /// - `{code}`: {desc}\n"));
        }
    }

    if !ep.error_responses.is_empty() {
        out.push_str("    ///\n    /// # Errors\n");
        for (code, desc) in &ep.error_responses {
            out.push_str(&format!("    /// - `{code}`: {desc}\n"));
        }
    }

    if let Some(policies) = &ep.security {
        out.push_str("    ///\n    /// # Permissions\n");
        match policies.len() {
            0 => out.push_str("    /// No authentication required.\n"),
            1 => out.push_str(&format!("    /// Requires `{}`.\n", policies[0])),
            _ => {
                for p in policies {
                    out.push_str(&format!("    /// - `{p}`\n"));
                }
            }
        }
    }
}

fn emit_sub_struct(sg: &SubGroup) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "pub struct {}<'a> {{\n    pub(crate) client: &'a NifiClient,\n    pub(crate) {}: &'a str,\n}}\n\n",
        sg.struct_name, sg.primary_param,
    ));
    out.push_str(
        "#[allow(private_interfaces, clippy::too_many_arguments, clippy::vec_init_then_push)]\n",
    );
    out.push_str(&format!("impl<'a> {}<'a> {{\n", sg.struct_name));
    for ep in &sg.endpoints {
        out.push_str(&emit_method_for_sub_group(ep, &sg.primary_param));
    }
    out.push_str("}\n\n");
    out
}

/// Like emit_method, but excludes `primary_param` from the function signature
/// and injects `let {primary_param} = self.{primary_param};` into the body.
fn emit_method_for_sub_group(ep: &Endpoint, primary_param: &str) -> String {
    // Skip form-encoded endpoints — they require manual implementations (e.g. NifiClient::login).
    if ep.body_kind == Some(crate::parser::RequestBodyKind::FormEncoded) {
        return String::new();
    }
    let mut out = String::new();
    if let Some(doc) = &ep.doc {
        out.push_str(&format!("    /// {doc}\n"));
        if let Some(desc) = &ep.description {
            out.push_str("    ///\n");
            for line in desc.lines() {
                out.push_str(&format!("    /// {line}\n"));
            }
        }
        out.push_str(&format!(
            "    ///\n    /// Calls `{} /nifi-api{}`.\n",
            method_str(&ep.method),
            ep.path
        ));
        emit_param_docs(&mut out, ep, Some(primary_param));
        emit_error_and_permission_docs(&mut out, ep);
    }

    let return_ty = match &ep.response_inner {
        Some(inner) => format!("crate::types::{inner}"),
        None => match &ep.response_type {
            Some(ty) => format!("crate::types::{ty}"),
            None => "()".into(),
        },
    };
    let return_result = format!("Result<{return_ty}, NifiError>");

    // Only bind and exclude the primary param if it appears in this endpoint's path params.
    // Some NiFi endpoints in the same sub-group use inconsistent param names (e.g. {id} vs
    // {registry-id}), so we guard against injecting an unused binding.
    let primary_in_path = ep.path_params.iter().any(|p| p.name == primary_param);

    let path_param_args: String = ep
        .path_params
        .iter()
        .filter(|p| !primary_in_path || p.name != primary_param)
        .map(|p| format!(", {}: &str", escape_keyword(&p.name)))
        .collect();

    use crate::parser::RequestBodyKind;
    // DELETE endpoints never send a body — ignore body_kind for signature generation.
    let body_arg = if ep.method == HttpMethod::Delete {
        String::new()
    } else {
        match &ep.body_kind {
            Some(RequestBodyKind::Json) => {
                let ty = ep.request_type.as_deref().unwrap_or("serde_json::Value");
                format!(", body: &crate::types::{ty}")
            }
            Some(RequestBodyKind::OctetStream) => {
                ", filename: Option<&str>, data: Vec<u8>".to_string()
            }
            Some(RequestBodyKind::FormEncoded) | None => String::new(),
        }
    };

    let query_param_args: String = ep
        .query_params
        .iter()
        .map(|qp| {
            let rust_type = query_param_rust_type(qp);
            if qp.required {
                format!(", {}: {rust_type}", escape_keyword(&qp.rust_name))
            } else {
                format!(", {}: Option<{rust_type}>", escape_keyword(&qp.rust_name))
            }
        })
        .collect();

    out.push_str(&format!(
        "    pub async fn {fn_name}(&self{path_param_args}{query_param_args}{body_arg}) -> {return_result} {{\n",
        fn_name = ep.fn_name,
    ));
    // Inject binding so emit_method_body's format string can reference the primary param.
    if primary_in_path {
        out.push_str(&format!(
            "        let {primary_param} = self.{primary_param};\n"
        ));
    }
    out.push_str(&emit_method_body(ep));
    out.push_str("    }\n\n");
    out
}

fn emit_method(ep: &Endpoint) -> String {
    // Skip form-encoded endpoints — they require manual implementations (e.g. NifiClient::login).
    if ep.body_kind == Some(crate::parser::RequestBodyKind::FormEncoded) {
        return String::new();
    }
    let mut out = String::new();
    if let Some(doc) = &ep.doc {
        out.push_str(&format!("    /// {doc}\n"));
        if let Some(desc) = &ep.description {
            out.push_str("    ///\n");
            for line in desc.lines() {
                out.push_str(&format!("    /// {line}\n"));
            }
        }
        out.push_str(&format!(
            "    ///\n    /// Calls `{} /nifi-api{}`.\n",
            method_str(&ep.method),
            ep.path
        ));
        emit_param_docs(&mut out, ep, None);
        emit_error_and_permission_docs(&mut out, ep);
    }

    let return_ty = match &ep.response_inner {
        Some(inner) => format!("crate::types::{inner}"),
        None => match &ep.response_type {
            Some(ty) => format!("crate::types::{ty}"),
            None => "()".into(),
        },
    };
    let return_result = format!("Result<{return_ty}, NifiError>");

    let path_param_args: String = ep
        .path_params
        .iter()
        .map(|p| format!(", {}: &str", escape_keyword(&p.name)))
        .collect();

    use crate::parser::RequestBodyKind;
    // DELETE endpoints never send a body — ignore body_kind for signature generation.
    let body_arg = if ep.method == HttpMethod::Delete {
        String::new()
    } else {
        match &ep.body_kind {
            Some(RequestBodyKind::Json) => {
                let ty = ep.request_type.as_deref().unwrap_or("serde_json::Value");
                format!(", body: &crate::types::{ty}")
            }
            Some(RequestBodyKind::OctetStream) => {
                ", filename: Option<&str>, data: Vec<u8>".to_string()
            }
            Some(RequestBodyKind::FormEncoded) | None => String::new(),
        }
    };

    let query_param_args: String = ep
        .query_params
        .iter()
        .map(|qp| {
            let rust_type = query_param_rust_type(qp);
            if qp.required {
                format!(", {}: {rust_type}", escape_keyword(&qp.rust_name))
            } else {
                format!(", {}: Option<{rust_type}>", escape_keyword(&qp.rust_name))
            }
        })
        .collect();

    out.push_str(&format!(
        "    pub async fn {fn_name}(&self{path_param_args}{query_param_args}{body_arg}) -> {return_result} {{\n",
        fn_name = ep.fn_name,
    ));
    out.push_str(&emit_method_body(ep));
    out.push_str("    }\n\n");
    out
}

fn normalize_path_for_format(path: &str) -> String {
    // Replace {camelCase-param-name} with {snake_case_param_name}
    let mut result = String::new();
    let mut brace_content = String::new();
    let mut in_brace = false;
    for ch in path.chars() {
        match ch {
            '{' => {
                in_brace = true;
                brace_content.clear();
            }
            '}' if in_brace => {
                in_brace = false;
                let normalized = camel_hyphen_to_snake(&brace_content);
                result.push('{');
                result.push_str(&normalized);
                result.push('}');
            }
            _ if in_brace => brace_content.push(ch),
            _ => result.push(ch),
        }
    }
    result
}

fn camel_hyphen_to_snake(s: &str) -> String {
    let hyphen_to_underscore = s.replace('-', "_");
    let mut out = String::new();
    for (i, ch) in hyphen_to_underscore.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}

fn escape_keyword(name: &str) -> String {
    match name {
        "type" | "ref" | "use" | "mod" | "fn" | "let" | "match" | "for" | "if" | "else"
        | "return" | "struct" | "enum" | "impl" | "trait" | "pub" | "super" | "self" | "crate"
        | "where" | "true" | "false" | "in" | "loop" | "while" | "break" | "continue" | "mut"
        | "move" | "async" | "await" | "dyn" | "box" | "const" | "static" | "extern" | "unsafe"
        | "as" => format!("r#{name}"),
        _ => name.to_string(),
    }
}

fn query_param_rust_type(qp: &QueryParam) -> String {
    use crate::parser::QueryParamType;
    match &qp.ty {
        QueryParamType::Str => "&str".to_string(),
        QueryParamType::Bool => "bool".to_string(),
        QueryParamType::I32 => "i32".to_string(),
        QueryParamType::I64 => "i64".to_string(),
        QueryParamType::F64 => "f64".to_string(),
        QueryParamType::Enum(_) => {
            // Use the named enum type (will be defined in the types module)
            let type_name = qp
                .enum_type_name
                .as_deref()
                .expect("enum param must have type name");
            format!("crate::types::{type_name}")
        }
    }
}

fn emit_method_body(ep: &Endpoint) -> String {
    let path_expr = if ep.path_params.is_empty() {
        format!("\"{}\"", ep.path)
    } else {
        let fmt_path = normalize_path_for_format(&ep.path);
        format!("&format!(\"{fmt_path}\")")
    };

    let has_inner = ep.response_inner.is_some();
    let inner_field = ep.response_field.as_deref().unwrap_or("");

    // If no query params, use the existing simple helpers.
    if ep.query_params.is_empty() {
        return emit_simple_method_body(ep, &path_expr, has_inner, inner_field);
    }

    // Build query vec construction code.
    let mut query_lines: Vec<String> =
        vec!["        let mut query: Vec<(&str, String)> = vec![];".to_string()];
    for qp in &ep.query_params {
        let rust_name = escape_keyword(&qp.rust_name);
        if qp.required {
            query_lines.push(format!(
                "        query.push((\"{name}\", {rust_name}.to_string()));",
                name = qp.name
            ));
        } else {
            query_lines.push(format!(
                "        if let Some(v) = {rust_name} {{ query.push((\"{name}\", v.to_string())); }}",
                name = qp.name
            ));
        }
    }
    let query_setup = query_lines.join("\n");

    let entity_ty = ep.response_type.as_deref().unwrap_or("_");

    match ep.method {
        HttpMethod::Get => {
            if has_inner {
                format!(
                    "{query_setup}\n        let e: crate::types::{entity_ty} = self.client.get_with_query({path_expr}, &query).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                )
            } else if ep.response_type.is_some() {
                format!(
                    "{query_setup}\n        self.client.get_with_query({path_expr}, &query).await\n"
                )
            } else {
                format!(
                    "{query_setup}\n        self.client.get_void_with_query({path_expr}, &query).await\n"
                )
            }
        }
        HttpMethod::Delete => {
            if has_inner {
                format!(
                    "{query_setup}\n        let e: crate::types::{entity_ty} = self.client.delete_returning_with_query({path_expr}, &query).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                )
            } else if ep.response_type.is_some() {
                format!(
                    "{query_setup}\n        self.client.delete_returning_with_query({path_expr}, &query).await\n"
                )
            } else {
                format!(
                    "{query_setup}\n        self.client.delete_with_query({path_expr}, &query).await\n"
                )
            }
        }
        HttpMethod::Post => {
            use crate::parser::RequestBodyKind;
            let body_expr = match &ep.body_kind {
                Some(RequestBodyKind::Json) => "body",
                _ => "&serde_json::json!({})", // no-body w/ query — post_with_query still works with empty JSON
            };
            if has_inner {
                format!(
                    "{query_setup}\n        let e: crate::types::{entity_ty} = self.client.post_with_query({path_expr}, {body_expr}, &query).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                )
            } else if ep.response_type.is_some() {
                format!(
                    "{query_setup}\n        self.client.post_with_query({path_expr}, {body_expr}, &query).await\n"
                )
            } else {
                format!(
                    "{query_setup}\n        self.client.post_void_with_query({path_expr}, {body_expr}, &query).await\n"
                )
            }
        }
        HttpMethod::Put => {
            // No PUT endpoints with query params in the current spec.
            // Fall back to the simple helpers.
            emit_simple_method_body(ep, &path_expr, has_inner, inner_field)
        }
    }
}

/// The original emit_method_body logic, used when there are no query params.
fn emit_simple_method_body(
    ep: &Endpoint,
    path_expr: &str,
    has_inner: bool,
    inner_field: &str,
) -> String {
    let entity_ty = ep.response_type.as_deref().unwrap_or("_");
    match ep.method {
        HttpMethod::Get => {
            if has_inner {
                format!(
                    "        let e: crate::types::{entity_ty} = self.client.get({path_expr}).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                )
            } else if ep.response_type.is_some() {
                format!("        self.client.get({path_expr}).await\n")
            } else {
                format!("        self.client.get_void({path_expr}).await\n")
            }
        }
        HttpMethod::Delete => {
            if has_inner {
                format!(
                    "        let e: crate::types::{entity_ty} = self.client.delete_returning({path_expr}).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                )
            } else if ep.response_type.is_some() {
                format!("        self.client.delete_returning({path_expr}).await\n")
            } else {
                format!("        self.client.delete({path_expr}).await\n")
            }
        }
        HttpMethod::Post => {
            use crate::parser::RequestBodyKind;
            match &ep.body_kind {
                Some(RequestBodyKind::Json) => {
                    if has_inner {
                        format!(
                            "        let e: crate::types::{entity_ty} = self.client.post({path_expr}, body).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!("        self.client.post({path_expr}, body).await\n")
                    } else {
                        format!("        self.client.post_void({path_expr}, body).await\n")
                    }
                }
                Some(RequestBodyKind::OctetStream) => {
                    if has_inner {
                        format!(
                            "        let e: crate::types::{entity_ty} = self.client.post_octet_stream({path_expr}, filename, data).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!(
                            "        self.client.post_octet_stream({path_expr}, filename, data).await\n"
                        )
                    } else {
                        format!(
                            "        self.client.post_void_octet_stream({path_expr}, filename, data).await\n"
                        )
                    }
                }
                Some(RequestBodyKind::FormEncoded) | None => {
                    if has_inner {
                        format!(
                            "        let e: crate::types::{entity_ty} = self.client.post_no_body({path_expr}).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!("        self.client.post_no_body({path_expr}).await\n")
                    } else {
                        format!("        self.client.post_void_no_body({path_expr}).await\n")
                    }
                }
            }
        }
        HttpMethod::Put => {
            use crate::parser::RequestBodyKind;
            match &ep.body_kind {
                Some(RequestBodyKind::Json) => {
                    if has_inner {
                        format!(
                            "        let e: crate::types::{entity_ty} = self.client.put({path_expr}, body).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!("        self.client.put({path_expr}, body).await\n")
                    } else {
                        format!("        self.client.put_void({path_expr}, body).await\n")
                    }
                }
                Some(RequestBodyKind::OctetStream) | Some(RequestBodyKind::FormEncoded) | None => {
                    if has_inner {
                        format!(
                            "        let e: crate::types::{entity_ty} = self.client.put_no_body({path_expr}).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!("        self.client.put_no_body({path_expr}).await\n")
                    } else {
                        format!("        self.client.put_void_no_body({path_expr}).await\n")
                    }
                }
            }
        }
    }
}

fn method_str(m: &HttpMethod) -> &'static str {
    match m {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Put => "PUT",
        HttpMethod::Delete => "DELETE",
    }
}
