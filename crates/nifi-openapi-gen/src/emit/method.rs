//! Shared method-body emitter used by both static (`emit::api`) and
//! dynamic (`emit::dynamic::api`) emitters. Consolidates doc-comment
//! emission, path-building, query-param serialization, header-arg
//! threading, HTTP helper dispatch, and response-type conversion.
//!
//! Divergence between static and dynamic lives in `EmitMode` and is
//! gated on `match mode` inside the helpers.

use crate::parser::{Endpoint, HttpMethod, QueryParam};
use crate::util::escape_keyword;

/// Selects whether the emitted method body targets a static
/// per-version resource struct or the canonical-superset dynamic
/// resource struct.
pub enum EmitMode<'a> {
    /// Static per-version emission. `version` is the semver string
    /// (e.g. `"2.9.0"`); `api_type_path` is the Rust path prefix used
    /// when referencing types from this version's type module
    /// (e.g. `"crate::v2_9_0"`).
    Static {
        version: &'a str,
        api_type_path: &'a str,
    },
    /// Canonical-superset dynamic emission.
    Dynamic,
}

impl<'a> EmitMode<'a> {
    /// Expression used to reach the underlying `NifiClient` from the
    /// current `self`.
    pub fn client_expr(&self) -> &'static str {
        match self {
            EmitMode::Static { .. } => "self.client",
            EmitMode::Dynamic => "self.client.inner()",
        }
    }

    /// Rust path prefix for referencing a type (DTO or enum) from
    /// generated code.
    pub fn type_path(&self, ty_name: &str) -> String {
        match self {
            EmitMode::Static { api_type_path, .. } => {
                format!("{api_type_path}::types::{ty_name}")
            }
            EmitMode::Dynamic => format!("crate::dynamic::types::{ty_name}"),
        }
    }
}

/// Emit the body of a method for the given endpoint into `out`.
///
/// The caller is responsible for emitting the enclosing `impl`
/// block and the struct declaration; this function only appends
/// the method itself (doc comment + signature + body).
pub fn emit_method(ep: &Endpoint, mode: &EmitMode<'_>, out: &mut String) {
    // Skip form-encoded endpoints — they require manual implementations (e.g. NifiClient::login).
    if ep.body_kind == Some(crate::parser::RequestBodyKind::FormEncoded) {
        return;
    }

    // Dynamic-only: emit_method dispatch fills this in C.8.
    if let EmitMode::Dynamic = mode {
        // Placeholder for C.8. Static emission path below is the only
        // active path in C.7.
    }

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
        emit_param_docs(out, ep, None);
        emit_error_and_permission_docs(out, ep);
    }

    let return_ty = crate::emit::common::response_return_type(ep, "crate");
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
    } else if let Some(RequestBodyKind::Json) = &ep.body_kind {
        let ty = ep.request_type.as_deref().unwrap_or("serde_json::Value");
        format!(", body: &crate::types::{ty}")
    } else {
        crate::emit::common::body_kind_signature(ep.body_kind.as_ref()).to_string()
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

    let header_param_args: String = ep
        .header_params
        .iter()
        .map(|hp| {
            if hp.required {
                format!(", {}: &str", hp.rust_name)
            } else {
                format!(", {}: Option<&str>", hp.rust_name)
            }
        })
        .collect();

    out.push_str(&format!(
        "    pub async fn {fn_name}(&self{path_param_args}{query_param_args}{header_param_args}{body_arg}) -> {return_result} {{\n",
        fn_name = ep.fn_name,
    ));
    out.push_str(&emit_method_body(ep));
    out.push_str("    }\n\n");
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

/// Returns (prelude_lines, header_arg_expr) for emitting extra_headers.
///
/// - If no header params: prelude is empty, header_arg_expr is `"&[]"`.
/// - If all required: prelude is empty, header_arg_expr is an array literal.
/// - If any optional: prelude builds a `Vec`, header_arg_expr is `"__headers.as_slice()"`.
fn emit_headers_setup(ep: &Endpoint) -> (String, String) {
    if ep.header_params.is_empty() {
        return (String::new(), "&[]".to_string());
    }
    if ep.header_params.iter().all(|h| h.required) {
        let pairs: Vec<String> = ep
            .header_params
            .iter()
            .map(|h| format!("(\"{}\", {})", h.name, h.rust_name))
            .collect();
        return (String::new(), format!("&[{}]", pairs.join(", ")));
    }
    // At least one optional: build a Vec.
    let mut prelude = String::new();
    prelude.push_str("        let mut __headers: Vec<(&str, &str)> = Vec::new();\n");
    for hp in &ep.header_params {
        if hp.required {
            prelude.push_str(&format!(
                "        __headers.push((\"{}\", {}));\n",
                hp.name, hp.rust_name
            ));
        } else {
            prelude.push_str(&format!(
                "        if let Some(v) = {} {{\n            __headers.push((\"{}\", v));\n        }}\n",
                hp.rust_name, hp.name
            ));
        }
    }
    (prelude, "__headers.as_slice()".to_string())
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

    let (headers_prelude, header_arg) = emit_headers_setup(ep);
    let setup = if headers_prelude.is_empty() {
        query_setup
    } else {
        format!("{query_setup}\n{headers_prelude}")
    };

    let entity_ty = ep.response_type.as_deref().unwrap_or("_");

    use crate::content_type::ResponseBodyKind;
    match ep.method {
        HttpMethod::Get => match &ep.response_kind {
            ResponseBodyKind::Json { .. } => {
                if has_inner {
                    format!(
                        "{setup}\n        let e: crate::types::{entity_ty} = self.client.get_with_query({path_expr}, {header_arg}, &query).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                    )
                } else {
                    // Both schema-typed and schemaless JSON use the same helper;
                    // schemaless falls back to `serde_json::Value` via the return type.
                    format!(
                        "{setup}\n        self.client.get_with_query({path_expr}, {header_arg}, &query).await\n"
                    )
                }
            }
            ResponseBodyKind::OctetStream | ResponseBodyKind::Wildcard => {
                format!(
                    "{setup}\n        self.client.get_bytes_with_query({path_expr}, {header_arg}, &query).await\n"
                )
            }
            ResponseBodyKind::Text | ResponseBodyKind::Xml => {
                // No current NiFi GET endpoint has query params AND text/xml response.
                // Emit a todo!() so a future spec addition surfaces loudly at runtime.
                format!(
                    "{setup}\n        todo!(\"text/xml GET with query params not implemented\")\n"
                )
            }
            ResponseBodyKind::Empty => {
                format!(
                    "{setup}\n        self.client.get_void_with_query({path_expr}, {header_arg}, &query).await\n"
                )
            }
        },
        HttpMethod::Delete => {
            if has_inner {
                format!(
                    "{setup}\n        let e: crate::types::{entity_ty} = self.client.delete_returning_with_query({path_expr}, {header_arg}, &query).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                )
            } else if ep.response_type.is_some() {
                format!(
                    "{setup}\n        self.client.delete_returning_with_query({path_expr}, {header_arg}, &query).await\n"
                )
            } else {
                format!(
                    "{setup}\n        self.client.delete_with_query({path_expr}, {header_arg}, &query).await\n"
                )
            }
        }
        HttpMethod::Post => {
            use crate::parser::RequestBodyKind;
            let body_expr = match &ep.body_kind {
                Some(RequestBodyKind::Json) => "body",
                Some(
                    RequestBodyKind::OctetStream
                    | RequestBodyKind::Multipart
                    | RequestBodyKind::Wildcard
                    | RequestBodyKind::FormEncoded,
                )
                | None => "&serde_json::json!({})",
            };
            if has_inner {
                format!(
                    "{setup}\n        let e: crate::types::{entity_ty} = self.client.post_with_query({path_expr}, {header_arg}, {body_expr}, &query).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                )
            } else if ep.response_type.is_some() {
                format!(
                    "{setup}\n        self.client.post_with_query({path_expr}, {header_arg}, {body_expr}, &query).await\n"
                )
            } else {
                format!(
                    "{setup}\n        self.client.post_void_with_query({path_expr}, {header_arg}, {body_expr}, &query).await\n"
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
    use crate::content_type::ResponseBodyKind;
    let entity_ty = ep.response_type.as_deref().unwrap_or("_");
    let (headers_prelude, header_arg) = emit_headers_setup(ep);
    match ep.method {
        HttpMethod::Get => match &ep.response_kind {
            ResponseBodyKind::Json { .. } => {
                if has_inner {
                    format!(
                        "{headers_prelude}        let e: crate::types::{entity_ty} = self.client.get({path_expr}, {header_arg}).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                    )
                } else {
                    // Both schema-typed and schemaless JSON use the same helper.
                    format!("{headers_prelude}        self.client.get({path_expr}, {header_arg}).await\n")
                }
            }
            ResponseBodyKind::Text | ResponseBodyKind::Xml => {
                format!("{headers_prelude}        self.client.get_text({path_expr}, {header_arg}).await\n")
            }
            ResponseBodyKind::OctetStream | ResponseBodyKind::Wildcard => {
                format!("{headers_prelude}        self.client.get_bytes({path_expr}, {header_arg}).await\n")
            }
            ResponseBodyKind::Empty => {
                format!("{headers_prelude}        self.client.get_void({path_expr}, {header_arg}).await\n")
            }
        },
        HttpMethod::Delete => {
            if has_inner {
                format!(
                    "{headers_prelude}        let e: crate::types::{entity_ty} = self.client.delete_returning({path_expr}, {header_arg}).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                )
            } else if ep.response_type.is_some() {
                format!("{headers_prelude}        self.client.delete_returning({path_expr}, {header_arg}).await\n")
            } else {
                format!("{headers_prelude}        self.client.delete({path_expr}, {header_arg}).await\n")
            }
        }
        HttpMethod::Post => {
            use crate::parser::RequestBodyKind;
            let is_text_response = matches!(
                &ep.response_kind,
                ResponseBodyKind::Text | ResponseBodyKind::Xml
            );
            match &ep.body_kind {
                Some(RequestBodyKind::Json) => {
                    if is_text_response {
                        format!(
                            "{headers_prelude}        self.client.post_returning_text({path_expr}, {header_arg}, body).await\n"
                        )
                    } else if has_inner {
                        format!(
                            "{headers_prelude}        let e: crate::types::{entity_ty} = self.client.post({path_expr}, {header_arg}, body).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!("{headers_prelude}        self.client.post({path_expr}, {header_arg}, body).await\n")
                    } else {
                        format!("{headers_prelude}        self.client.post_void({path_expr}, {header_arg}, body).await\n")
                    }
                }
                Some(RequestBodyKind::OctetStream) => {
                    if is_text_response {
                        format!(
                            "{headers_prelude}        self.client.post_octet_stream_returning_text({path_expr}, {header_arg}, data).await\n"
                        )
                    } else if has_inner {
                        format!(
                            "{headers_prelude}        let e: crate::types::{entity_ty} = self.client.post_octet_stream({path_expr}, {header_arg}, data).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!(
                            "{headers_prelude}        self.client.post_octet_stream({path_expr}, {header_arg}, data).await\n"
                        )
                    } else {
                        format!(
                            "{headers_prelude}        self.client.post_void_octet_stream({path_expr}, {header_arg}, data).await\n"
                        )
                    }
                }
                Some(RequestBodyKind::Multipart) => {
                    if has_inner {
                        format!(
                            "{headers_prelude}        let e: crate::types::{entity_ty} = self.client.post_multipart({path_expr}, {header_arg}, filename, data).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!(
                            "{headers_prelude}        self.client.post_multipart({path_expr}, {header_arg}, filename, data).await\n"
                        )
                    } else {
                        format!(
                            "{headers_prelude}        self.client.post_void_multipart({path_expr}, {header_arg}, filename, data).await\n"
                        )
                    }
                }
                Some(RequestBodyKind::Wildcard) | Some(RequestBodyKind::FormEncoded) | None => {
                    if has_inner {
                        format!(
                            "{headers_prelude}        let e: crate::types::{entity_ty} = self.client.post_no_body({path_expr}, {header_arg}).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!("{headers_prelude}        self.client.post_no_body({path_expr}, {header_arg}).await\n")
                    } else {
                        format!("{headers_prelude}        self.client.post_void_no_body({path_expr}, {header_arg}).await\n")
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
                            "{headers_prelude}        let e: crate::types::{entity_ty} = self.client.put({path_expr}, {header_arg}, body).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!("{headers_prelude}        self.client.put({path_expr}, {header_arg}, body).await\n")
                    } else {
                        format!("{headers_prelude}        self.client.put_void({path_expr}, {header_arg}, body).await\n")
                    }
                }
                Some(RequestBodyKind::OctetStream)
                | Some(RequestBodyKind::Multipart)
                | Some(RequestBodyKind::Wildcard)
                | Some(RequestBodyKind::FormEncoded)
                | None => {
                    if has_inner {
                        format!(
                            "{headers_prelude}        let e: crate::types::{entity_ty} = self.client.put_no_body({path_expr}, {header_arg}).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!("{headers_prelude}        self.client.put_no_body({path_expr}, {header_arg}).await\n")
                    } else {
                        format!("{headers_prelude}        self.client.put_void_no_body({path_expr}, {header_arg}).await\n")
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
