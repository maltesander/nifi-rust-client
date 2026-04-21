//! Shared method-body emitter used by both static (`emit::api`) and
//! dynamic (`emit::dynamic::api`) emitters. Consolidates doc-comment
//! emission, path-building, query-param serialization, header-arg
//! threading, HTTP helper dispatch, and response-type conversion.
//!
//! Divergence between static and dynamic lives in `EmitMode` and is
//! gated on `match mode` inside the helpers.

use std::collections::BTreeMap;

use crate::canonical::VersionSet;
use crate::parser::{Endpoint, HttpMethod, MultipartField, MultipartFieldType, QueryParam};
use crate::util::escape_keyword;

/// Rust type for a multipart field in a generated method signature.
fn multipart_field_rust_type(ty: MultipartFieldType) -> &'static str {
    match ty {
        MultipartFieldType::String => "&str",
        MultipartFieldType::Bool => "bool",
        MultipartFieldType::F64 => "f64",
    }
}

/// Emit the `", name: type"` fragments for a Multipart endpoint's
/// non-file schema properties, preserving the alphabetic order set by
/// the parser. Includes the trailing `filename` / `data` pair.
fn multipart_body_signature(fields: &[MultipartField]) -> String {
    let mut s = String::new();
    for f in fields {
        let rust_ty = multipart_field_rust_type(f.ty);
        let name = escape_keyword(&f.rust_name);
        if f.required {
            s.push_str(&format!(", {name}: {rust_ty}"));
        } else {
            s.push_str(&format!(", {name}: Option<{rust_ty}>"));
        }
    }
    s.push_str(", filename: &str, data: Vec<u8>");
    s
}

/// Emit the `let mut fields: Vec<(&str, String)> ...` preamble that
/// collects multipart form fields prior to the dispatch call. Returns
/// an empty string when `fields` is empty.
fn multipart_fields_preamble(fields: &[MultipartField], indent: &str) -> String {
    if fields.is_empty() {
        return String::new();
    }
    let mut s = String::new();
    s.push_str(&format!(
        "{indent}let mut fields: Vec<(&str, String)> = Vec::new();\n"
    ));
    for f in fields {
        let rust_name = escape_keyword(&f.rust_name);
        let wire = &f.name;
        if f.required {
            s.push_str(&format!(
                "{indent}fields.push((\"{wire}\", {rust_name}.to_string()));\n"
            ));
        } else {
            s.push_str(&format!(
                "{indent}if let Some(v) = {rust_name} {{ fields.push((\"{wire}\", v.to_string())); }}\n"
            ));
        }
    }
    s
}

/// Extra per-endpoint context the dynamic emitter needs that isn't
/// available from the plain `Endpoint` struct: the set of NiFi
/// versions that declare this endpoint, the per-query-param version
/// set (so optional params only available in a subset of versions
/// can be guarded), and the canonical `Endpoint::FOO` variant name
/// used by the runtime availability check.
pub struct DynamicMethodCtx<'a> {
    pub endpoint_versions: &'a VersionSet,
    pub query_param_versions: &'a BTreeMap<String, VersionSet>,
    pub endpoint_variant: String,
    pub method_lit: &'static str,
}

/// Selects whether the emitted method body targets a static
/// per-version resource struct or the dynamic-mode resource struct.
pub enum EmitMode<'a> {
    /// Static per-version emission. Type-path rewriting is handled
    /// after the fact by `emit_api_with_prefix` via a string replace,
    /// so no version or prefix fields are needed here.
    Static,
    /// Dynamic-mode emission.
    Dynamic(DynamicMethodCtx<'a>),
}

/// Whether this endpoint should also be emitted in a streaming variant.
/// True iff the response body is a raw byte payload.
pub fn endpoint_has_stream_variant(ep: &Endpoint) -> bool {
    use crate::content_type::ResponseBodyKind;
    matches!(
        ep.response_kind,
        ResponseBodyKind::OctetStream | ResponseBodyKind::Wildcard
    )
}

/// Emit the body of a method for the given endpoint into `out`.
///
/// The caller is responsible for emitting the enclosing `impl`
/// block and the struct declaration; this function only appends
/// the method itself (doc comment + signature + body).
pub fn emit_method(ep: &Endpoint, mode: &EmitMode<'_>, out: &mut String) {
    emit_method_variant(ep, mode, false, out);
    if endpoint_has_stream_variant(ep) {
        emit_method_variant(ep, mode, true, out);
    }
}

fn emit_method_variant(ep: &Endpoint, mode: &EmitMode<'_>, stream: bool, out: &mut String) {
    // Skip form-encoded endpoints for static — they require manual
    // implementations (e.g. NifiClient::login). Dynamic preserves its
    // existing (somewhat quirky) behavior of emitting a stub that
    // routes through the normal dispatch fall-through.
    if matches!(mode, EmitMode::Static)
        && ep.body_kind == Some(crate::parser::RequestBodyKind::FormEncoded)
    {
        return;
    }

    if let Some(doc) = &ep.doc {
        if stream {
            out.push_str("    /// Streaming variant: yields body chunks as they arrive instead of buffering the whole response.\n    ///\n");
        }
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

    let return_ty = if stream {
        crate::emit::common::response_stream_return_type(match mode {
            EmitMode::Static => "crate",
            EmitMode::Dynamic(_) => "crate",
        })
    } else {
        match mode {
            EmitMode::Static => crate::emit::common::response_return_type(ep, "crate"),
            EmitMode::Dynamic(_) => dynamic_response_type_for(ep, stream),
        }
    };
    let return_result = format!("Result<{return_ty}, NifiError>");

    let path_param_args: String = ep
        .path_params
        .iter()
        .map(|p| match mode {
            EmitMode::Static => format!(", {}: &str", escape_keyword(&p.name)),
            EmitMode::Dynamic(_) => format!(", {}: &str", escape_keyword(&p.name)),
        })
        .collect();

    use crate::parser::RequestBodyKind;
    // For static: DELETE endpoints never send a body — ignore body_kind for
    // signature generation. For dynamic: body args appear on every method
    // (including DELETE) to preserve existing generator output; `data`
    // becomes an unused argument for DELETE methods, silenced by the
    // struct-level `unused_variables` lint.
    let body_arg = match mode {
        EmitMode::Static => {
            if ep.method == HttpMethod::Delete {
                String::new()
            } else if let Some(RequestBodyKind::Json) = &ep.body_kind {
                let ty = ep.request_type.as_deref().unwrap_or("serde_json::Value");
                format!(", body: &crate::types::{ty}")
            } else if let Some(RequestBodyKind::Multipart) = &ep.body_kind {
                multipart_body_signature(&ep.multipart_fields)
            } else {
                crate::emit::common::body_kind_signature(ep.body_kind.as_ref()).to_string()
            }
        }
        EmitMode::Dynamic(_) => match &ep.body_kind {
            Some(RequestBodyKind::Json) => {
                if let Some(rt) = &ep.request_type {
                    format!(", body: &crate::dynamic::types::{rt}")
                } else {
                    String::new()
                }
            }
            Some(RequestBodyKind::OctetStream) => ", data: Vec<u8>".to_string(),
            Some(RequestBodyKind::Multipart) => multipart_body_signature(&ep.multipart_fields),
            _ => String::new(),
        },
    };

    let query_param_args: String = ep
        .query_params
        .iter()
        .map(|qp| {
            let rust_type = match mode {
                EmitMode::Static => query_param_rust_type_static(qp),
                EmitMode::Dynamic(_) => query_param_rust_type_dynamic(qp),
            };
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

    // Dynamic reorders args: path, query, header, body — but so does static.
    // Dynamic's existing output puts query before header which matches static.
    out.push_str(&format!(
        "    pub async fn {fn_name}(&self{path_param_args}{query_param_args}{header_param_args}{body_arg}) -> {return_result} {{\n",
        fn_name = if stream {
            format!("{}_stream", ep.fn_name)
        } else {
            ep.fn_name.clone()
        },
    ));

    match mode {
        EmitMode::Static => {
            out.push_str(&emit_method_body_static(ep, stream));
        }
        EmitMode::Dynamic(ctx) => {
            emit_method_body_dynamic(ep, ctx, stream, out);
        }
    }

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

fn query_param_rust_type_static(qp: &QueryParam) -> String {
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

fn query_param_rust_type_dynamic(qp: &QueryParam) -> String {
    use crate::parser::QueryParamType::*;
    match &qp.ty {
        Str => "&str".to_string(),
        Bool => "bool".to_string(),
        I32 => "i32".to_string(),
        I64 => "i64".to_string(),
        F64 => "f64".to_string(),
        Enum(_) => qp
            .enum_type_name
            .as_ref()
            .map(|n| format!("crate::dynamic::types::{n}"))
            .unwrap_or_else(|| "&str".to_string()),
    }
}

/// Returns (prelude_lines, header_arg_expr) for emitting extra_headers.
///
/// - If no header params: prelude is empty, header_arg_expr is `"&[]"`.
/// - If all required: prelude is empty, header_arg_expr is an array literal.
/// - If any optional: prelude builds a `Vec`, header_arg_expr is `"__headers.as_slice()"`.
///
/// `indent` is the string prepended to every emitted line (e.g. `"        "` for 8 spaces).
fn emit_headers_setup(ep: &Endpoint, indent: &str) -> (String, String) {
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
    prelude.push_str(&format!(
        "{indent}let mut __headers: Vec<(&str, &str)> = Vec::new();\n"
    ));
    for hp in &ep.header_params {
        if hp.required {
            prelude.push_str(&format!(
                "{indent}__headers.push((\"{}\", {}));\n",
                hp.name, hp.rust_name
            ));
        } else {
            prelude.push_str(&format!(
                "{indent}if let Some(v) = {} {{\n{indent}    __headers.push((\"{}\", v));\n{indent}}}\n",
                hp.rust_name, hp.name
            ));
        }
    }
    (prelude, "__headers.as_slice()".to_string())
}

// ───────────────────────────── Static body ─────────────────────────────

fn emit_method_body_static(ep: &Endpoint, stream: bool) -> String {
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
        return emit_simple_method_body_static(ep, &path_expr, has_inner, inner_field, stream);
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

    let (headers_prelude, header_arg) = emit_headers_setup(ep, "        ");
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
                let helper = if stream {
                    "get_bytes_stream_with_query"
                } else {
                    "get_bytes_with_query"
                };
                format!(
                    "{setup}\n        self.client.{helper}({path_expr}, {header_arg}, &query).await\n"
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
            emit_simple_method_body_static(ep, &path_expr, has_inner, inner_field, stream)
        }
    }
}

/// The original emit_method_body logic, used when there are no query params.
fn emit_simple_method_body_static(
    ep: &Endpoint,
    path_expr: &str,
    has_inner: bool,
    inner_field: &str,
    stream: bool,
) -> String {
    use crate::content_type::ResponseBodyKind;
    let entity_ty = ep.response_type.as_deref().unwrap_or("_");
    let (headers_prelude, header_arg) = emit_headers_setup(ep, "        ");
    match ep.method {
        HttpMethod::Get => match &ep.response_kind {
            ResponseBodyKind::Json { .. } => {
                if has_inner {
                    format!(
                        "{headers_prelude}        let e: crate::types::{entity_ty} = self.client.get({path_expr}, {header_arg}).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                    )
                } else {
                    // Both schema-typed and schemaless JSON use the same helper.
                    format!(
                        "{headers_prelude}        self.client.get({path_expr}, {header_arg}).await\n"
                    )
                }
            }
            ResponseBodyKind::Text | ResponseBodyKind::Xml => {
                format!(
                    "{headers_prelude}        self.client.get_text({path_expr}, {header_arg}).await\n"
                )
            }
            ResponseBodyKind::OctetStream | ResponseBodyKind::Wildcard => {
                let helper = if stream {
                    "get_bytes_stream"
                } else {
                    "get_bytes"
                };
                format!(
                    "{headers_prelude}        self.client.{helper}({path_expr}, {header_arg}).await\n"
                )
            }
            ResponseBodyKind::Empty => {
                format!(
                    "{headers_prelude}        self.client.get_void({path_expr}, {header_arg}).await\n"
                )
            }
        },
        HttpMethod::Delete => {
            if has_inner {
                format!(
                    "{headers_prelude}        let e: crate::types::{entity_ty} = self.client.delete_returning({path_expr}, {header_arg}).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                )
            } else if ep.response_type.is_some() {
                format!(
                    "{headers_prelude}        self.client.delete_returning({path_expr}, {header_arg}).await\n"
                )
            } else {
                format!(
                    "{headers_prelude}        self.client.delete({path_expr}, {header_arg}).await\n"
                )
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
                        format!(
                            "{headers_prelude}        self.client.post({path_expr}, {header_arg}, body).await\n"
                        )
                    } else {
                        format!(
                            "{headers_prelude}        self.client.post_void({path_expr}, {header_arg}, body).await\n"
                        )
                    }
                }
                Some(RequestBodyKind::OctetStream) => {
                    if is_text_response {
                        format!(
                            "{headers_prelude}        self.client.post_octet_stream_returning_text({path_expr}, {header_arg}, data.into()).await\n"
                        )
                    } else if has_inner {
                        format!(
                            "{headers_prelude}        let e: crate::types::{entity_ty} = self.client.post_octet_stream({path_expr}, {header_arg}, data.into()).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!(
                            "{headers_prelude}        self.client.post_octet_stream({path_expr}, {header_arg}, data.into()).await\n"
                        )
                    } else {
                        format!(
                            "{headers_prelude}        self.client.post_void_octet_stream({path_expr}, {header_arg}, data.into()).await\n"
                        )
                    }
                }
                Some(RequestBodyKind::Multipart) => {
                    let fields_preamble =
                        multipart_fields_preamble(&ep.multipart_fields, "        ");
                    let (helper, extra_args) = if ep.multipart_fields.is_empty() {
                        ("post_multipart", "filename, data.into()")
                    } else {
                        ("post_multipart_with_fields", "&fields, filename, data.into()")
                    };
                    if has_inner {
                        format!(
                            "{headers_prelude}{fields_preamble}        let e: crate::types::{entity_ty} = self.client.{helper}({path_expr}, {header_arg}, {extra_args}).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!(
                            "{headers_prelude}{fields_preamble}        self.client.{helper}({path_expr}, {header_arg}, {extra_args}).await\n"
                        )
                    } else {
                        // No Multipart endpoint currently has an empty
                        // response. If one appears, the `post_void_multipart`
                        // helper needs a `_with_fields` sibling. Fail loudly
                        // at generation time when that day arrives — silently
                        // dropping `multipart_fields` would send a form that
                        // NiFi rejects.
                        if !ep.multipart_fields.is_empty() {
                            panic!(
                                "nifi-openapi-gen: Multipart endpoint with empty response and schema fields is not yet supported (operationId={:?}). Add a `post_void_multipart_with_fields` helper to NifiClient and update the static emitter to pick it when multipart_fields is non-empty.",
                                ep.raw_operation_id,
                            );
                        }
                        format!(
                            "{headers_prelude}{fields_preamble}        self.client.post_void_multipart({path_expr}, {header_arg}, filename, data.into()).await\n"
                        )
                    }
                }
                Some(RequestBodyKind::Wildcard) | Some(RequestBodyKind::FormEncoded) | None => {
                    if has_inner {
                        format!(
                            "{headers_prelude}        let e: crate::types::{entity_ty} = self.client.post_no_body({path_expr}, {header_arg}).await?;\n        Ok(e.{inner_field}.unwrap_or_default())\n"
                        )
                    } else if ep.response_type.is_some() {
                        format!(
                            "{headers_prelude}        self.client.post_no_body({path_expr}, {header_arg}).await\n"
                        )
                    } else {
                        format!(
                            "{headers_prelude}        self.client.post_void_no_body({path_expr}, {header_arg}).await\n"
                        )
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
                        format!(
                            "{headers_prelude}        self.client.put({path_expr}, {header_arg}, body).await\n"
                        )
                    } else {
                        format!(
                            "{headers_prelude}        self.client.put_void({path_expr}, {header_arg}, body).await\n"
                        )
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
                        format!(
                            "{headers_prelude}        self.client.put_no_body({path_expr}, {header_arg}).await\n"
                        )
                    } else {
                        format!(
                            "{headers_prelude}        self.client.put_void_no_body({path_expr}, {header_arg}).await\n"
                        )
                    }
                }
            }
        }
    }
}

// ──────────────────────────── Dynamic body ────────────────────────────

fn dynamic_response_type_for(ep: &Endpoint, stream: bool) -> String {
    use crate::content_type::ResponseBodyKind;
    match (&ep.response_kind, &ep.response_inner, &ep.response_type) {
        (ResponseBodyKind::Empty, _, _) => "()".to_string(),
        (ResponseBodyKind::Text | ResponseBodyKind::Xml, _, _) => "String".to_string(),
        (ResponseBodyKind::OctetStream | ResponseBodyKind::Wildcard, _, _) if stream => {
            "crate::BytesStream".to_string()
        }
        (ResponseBodyKind::OctetStream | ResponseBodyKind::Wildcard, _, _) => "Vec<u8>".to_string(),
        (_, Some(inner), _) => format!("crate::dynamic::types::{inner}"),
        (_, _, Some(rt)) => format!("crate::dynamic::types::{rt}"),
        // Inline-schema JSON response (e.g. an OpenAPI response with
        // `application/json` + an inline schema like `{"type": "string"}`).
        // The parser leaves response_inner/response_type unset because there
        // is no named schema to resolve; the response is still deserializable
        // JSON, so route it through the opaque serde_json::Value.
        (ResponseBodyKind::Json { .. }, None, None) => "serde_json::Value".to_string(),
    }
}

fn emit_method_body_dynamic(
    ep: &Endpoint,
    ctx: &DynamicMethodCtx<'_>,
    stream: bool,
    out: &mut String,
) {
    // 1. require_endpoint prelude
    out.push_str(&format!(
        "        self.client.require_endpoint(Endpoint::{variant}).await?;\n",
        variant = ctx.endpoint_variant,
    ));

    // 2. Path binding — always a local `let path` variable.
    //
    // The spec path template uses the *raw* OpenAPI param name
    // (e.g. `{componentId}`, `{drop-request-id}`), but `PathParam::name`
    // is snake_cased by the parser. Normalize the template so its
    // placeholders match the snake_case keys we emit below — otherwise
    // `.replace(...)` silently fails to substitute and the URL is sent
    // to NiFi with a literal `{componentId}` segment.
    let normalized_path = normalize_path_for_format(&ep.path);
    let mut path_expr = format!("\"{normalized_path}\".to_string()");
    for pp in &ep.path_params {
        let placeholder = format!("{{{}}}", pp.name);
        let value = escape_keyword(&pp.name);
        path_expr = format!("{path_expr}.replace(\"{placeholder}\", {value})");
    }
    out.push_str(&format!("        let path = {path_expr};\n"));

    // 3. Query vec
    let has_query = !ep.query_params.is_empty();
    if has_query {
        out.push_str("        let mut query: Vec<(&str, String)> = Vec::new();\n");
        for qp in &ep.query_params {
            emit_query_push_dynamic(out, ep, ctx, qp);
        }
    }

    // 4. Header setup
    let header_arg = emit_headers_setup_dynamic(out, ep);

    // 4.5. Multipart form fields (only for Multipart endpoints with
    // non-file schema properties). Built before the tracing call so
    // `fields` is in scope for the dispatch step.
    if matches!(
        ep.body_kind,
        Some(crate::parser::RequestBodyKind::Multipart)
    ) {
        out.push_str(&multipart_fields_preamble(&ep.multipart_fields, "        "));
    }

    // 5. Tracing
    out.push_str(&format!(
        "        tracing::debug!(method = \"{method_lit}\", path = path.as_str(), \"NiFi API request\");\n",
        method_lit = ctx.method_lit,
    ));

    // 6. Dispatch
    emit_dispatch_dynamic(out, ep, has_query, &header_arg, stream);
}

fn emit_headers_setup_dynamic(out: &mut String, ep: &Endpoint) -> String {
    let (prelude, header_arg) = emit_headers_setup(ep, "        ");
    out.push_str(&prelude);
    header_arg
}

fn emit_query_push_dynamic(
    out: &mut String,
    _ep: &Endpoint,
    ctx: &DynamicMethodCtx<'_>,
    qp: &QueryParam,
) {
    let qp_versions = ctx
        .query_param_versions
        .get(&qp.name)
        .cloned()
        .unwrap_or_else(|| ctx.endpoint_versions.clone());
    let needs_guard = qp_versions != *ctx.endpoint_versions;

    let escaped_rust_name = escape_keyword(&qp.rust_name);
    let to_string_expr = format!("{escaped_rust_name}.to_string()");

    if qp.required {
        if needs_guard {
            emit_guard(out, ctx, &qp.name, &qp_versions, "        ");
        }
        out.push_str(&format!(
            "        query.push((\"{name}\", {value}));\n",
            name = qp.name,
            value = to_string_expr,
        ));
    } else {
        out.push_str(&format!(
            "        if let Some({n}) = {n} {{\n",
            n = escaped_rust_name
        ));
        if needs_guard {
            emit_guard(out, ctx, &qp.name, &qp_versions, "            ");
        }
        out.push_str(&format!(
            "            query.push((\"{name}\", {value}));\n",
            name = qp.name,
            value = to_string_expr,
        ));
        out.push_str("        }\n");
    }
}

/// Emit a runtime `UnsupportedQueryParam` guard for `param`.
///
/// `indent` is the prefix for the outermost guard lines (e.g. `"        "` for 8
/// spaces when at statement level, `"            "` for 12 spaces when nested
/// inside an `if let Some(…)` block).
fn emit_guard(
    out: &mut String,
    ctx: &DynamicMethodCtx<'_>,
    param: &str,
    versions: &VersionSet,
    indent: &str,
) {
    let variant = &ctx.endpoint_variant;
    let supported: Vec<String> = versions
        .to_vec()
        .into_iter()
        .map(|v| format!("\"{v}\".to_string()"))
        .collect();
    let inner = format!("{indent}    ");
    out.push_str(&format!(
        "{indent}let __detected = self.client.detected_version_str();\n\
         {indent}if !crate::dynamic::availability::query_param_supported(Endpoint::{variant}, \"{param}\", &__detected) {{\n"
    ));
    out.push_str(&format!(
        "{inner}return Err(NifiError::UnsupportedQueryParam {{ endpoint: Endpoint::{variant}.as_str(), param: \"{param}\", detected_version: __detected, supported_in: vec![{}] }});\n",
        supported.join(", "),
    ));
    out.push_str(&format!("{indent}}}\n"));
}

fn emit_dispatch_dynamic(
    out: &mut String,
    ep: &Endpoint,
    has_query: bool,
    header_arg: &str,
    stream: bool,
) {
    use crate::content_type::{RequestBodyKind, ResponseBodyKind};

    let return_kind = &ep.response_kind;
    let request_kind = &ep.body_kind;
    let returns_unit = matches!(return_kind, ResponseBodyKind::Empty);
    let returns_text = matches!(return_kind, ResponseBodyKind::Text | ResponseBodyKind::Xml);
    let returns_bytes = matches!(
        return_kind,
        ResponseBodyKind::OctetStream | ResponseBodyKind::Wildcard
    );

    let path_arg = "&path";
    let use_query = has_query;

    // ── Unit (Empty) returns ─────────────────────────────────────────────
    if returns_unit {
        match (&ep.method, request_kind, use_query) {
            (HttpMethod::Delete, _, true) => {
                out.push_str(&format!(
                    "        self.client.inner().delete_with_query({path_arg}, {header_arg}, &query).await\n"
                ));
            }
            (HttpMethod::Delete, _, false) => {
                out.push_str(&format!(
                    "        self.client.inner().delete({path_arg}, {header_arg}).await\n"
                ));
            }
            (HttpMethod::Post, Some(RequestBodyKind::Json), _) => {
                out.push_str(&format!(
                    "        self.client.inner().post_void({path_arg}, {header_arg}, body).await\n"
                ));
            }
            (HttpMethod::Post, Some(RequestBodyKind::OctetStream), _) => {
                out.push_str(&format!(
                    "        self.client.inner().post_void_octet_stream({path_arg}, {header_arg}, data.into()).await\n"
                ));
            }
            (HttpMethod::Post, _, _) => {
                out.push_str(&format!(
                    "        self.client.inner().post_void_no_body({path_arg}, {header_arg}).await\n"
                ));
            }
            (HttpMethod::Put, Some(RequestBodyKind::Json), _) => {
                out.push_str(&format!(
                    "        self.client.inner().put_void({path_arg}, {header_arg}, body).await\n"
                ));
            }
            (HttpMethod::Put, _, _) => {
                out.push_str(&format!(
                    "        self.client.inner().put_void_no_body({path_arg}, {header_arg}).await\n"
                ));
            }
            _ => {
                out.push_str(&format!(
                    "        self.client.inner().get_void_with_query({path_arg}, {header_arg}, &[]).await\n"
                ));
            }
        }
        return;
    }

    // ── Text / bytes returns ─────────────────────────────────────────────
    if returns_text {
        if use_query {
            out.push_str(
                "        todo!(\"text GET with query params not implemented in dynamic\")\n",
            );
        } else {
            out.push_str(&format!(
                "        self.client.inner().get_text({path_arg}, {header_arg}).await\n"
            ));
        }
        return;
    }

    if returns_bytes {
        let (no_q_helper, q_helper) = if stream {
            ("get_bytes_stream", "get_bytes_stream_with_query")
        } else {
            ("get_bytes", "get_bytes_with_query")
        };
        if use_query {
            out.push_str(&format!(
                "        self.client.inner().{q_helper}({path_arg}, {header_arg}, &query).await\n"
            ));
        } else {
            out.push_str(&format!(
                "        self.client.inner().{no_q_helper}({path_arg}, {header_arg}).await\n"
            ));
        }
        return;
    }

    // ── Non-unit JSON returns ────────────────────────────────────────────
    let call_for_entity = |helper: &str, header: &str, body: &str, q: &str| -> String {
        format!("self.client.inner().{helper}({path_arg}, {header}{body}{q}).await?")
    };

    let (base_helper, body_args, query_args): (&str, &str, String) =
        match (&ep.method, request_kind, use_query) {
            (HttpMethod::Get, _, true) => ("get_with_query", "", ", &query".to_string()),
            (HttpMethod::Get, _, false) => ("get", "", String::new()),
            (HttpMethod::Delete, _, true) => {
                ("delete_returning_with_query", "", ", &query".to_string())
            }
            (HttpMethod::Delete, _, false) => ("delete_returning", "", String::new()),
            (HttpMethod::Post, Some(RequestBodyKind::Json), true) => {
                ("post_with_query", ", body", ", &query".to_string())
            }
            (HttpMethod::Post, Some(RequestBodyKind::Json), false) => {
                ("post", ", body", String::new())
            }
            (HttpMethod::Post, Some(RequestBodyKind::OctetStream), _) => {
                ("post_octet_stream", ", data.into()", String::new())
            }
            (HttpMethod::Post, Some(RequestBodyKind::Multipart), _) => {
                if ep.multipart_fields.is_empty() {
                    ("post_multipart", ", filename, data.into()", String::new())
                } else {
                    (
                        "post_multipart_with_fields",
                        ", &fields, filename, data.into()",
                        String::new(),
                    )
                }
            }
            (HttpMethod::Post, _, _) => ("post_no_body", "", String::new()),
            (HttpMethod::Put, Some(RequestBodyKind::Json), true) => {
                ("put_with_query", ", body", ", &query".to_string())
            }
            (HttpMethod::Put, Some(RequestBodyKind::Json), false) => {
                ("put", ", body", String::new())
            }
            (HttpMethod::Put, _, _) => ("put_no_body", "", String::new()),
        };

    match (&ep.response_inner, &ep.response_field) {
        (Some(inner), Some(field)) => {
            let entity = ep.response_type.as_deref().unwrap_or(inner);
            let call_expr = call_for_entity(base_helper, header_arg, body_args, &query_args);
            out.push_str(&format!(
                "        let wrapper: crate::dynamic::types::{entity} = {call_expr};\n"
            ));
            out.push_str(&format!(
                "        Ok(wrapper.{field}.unwrap_or_default())\n"
            ));
        }
        _ => {
            // By the time we reach here the outer Empty/Text/Xml/OctetStream
            // /Wildcard early returns have already dispatched, so the response
            // is definitionally Json. Always emit the resolved base_helper
            // call; the fn-signature builder (`dynamic_response_type_for`)
            // routes the return type through serde_json::Value when the
            // response schema is inline (no $ref).
            out.push_str(&format!(
                "        self.client.inner().{base_helper}({path_arg}, {header_arg}{body_args}{query_args}).await\n"
            ));
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

#[cfg(test)]
mod emit_fix_inline_json_tests {
    use super::*;
    use crate::content_type::ResponseBodyKind;
    use crate::parser::{Endpoint, HttpMethod, PathParam};

    fn inline_json_endpoint() -> Endpoint {
        Endpoint {
            method: HttpMethod::Get,
            path: "/process-groups/{id}/download".to_string(),
            fn_name: "export_process_group".to_string(),
            raw_operation_id: "exportProcessGroup".to_string(),
            doc: Some("Exports a process group snapshot.".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            request_type: None,
            body_kind: None,
            body_doc: None,
            multipart_fields: Vec::new(),
            response_type: None,
            response_inner: None,
            response_field: None,
            response_kind: ResponseBodyKind::Json {
                schema_ref: "serde_json::Value".to_string(),
            },
            query_params: vec![],
            header_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        }
    }

    #[test]
    fn dynamic_response_type_for_inline_json_is_value() {
        let ep = inline_json_endpoint();
        let rt = dynamic_response_type_for(&ep, false);
        assert_eq!(rt, "serde_json::Value");
    }

    #[test]
    fn dynamic_response_type_for_stream_only_applies_to_bytes() {
        let mut ep = inline_json_endpoint();
        ep.response_kind = ResponseBodyKind::OctetStream;
        assert_eq!(dynamic_response_type_for(&ep, true), "crate::BytesStream");
        assert_eq!(dynamic_response_type_for(&ep, false), "Vec<u8>");
    }

    #[test]
    fn emit_dispatch_dynamic_for_inline_json_get_uses_get_helper() {
        let ep = inline_json_endpoint();
        let mut out = String::new();
        emit_dispatch_dynamic(&mut out, &ep, false, "&[]", false);
        assert!(
            out.contains("self.client.inner().get("),
            "expected a `get` dispatch for inline-JSON GET, got: {out}"
        );
        assert!(
            !out.contains("post_void_no_body"),
            "must not fall through to post_void_no_body: {out}"
        );
    }

    #[test]
    fn emit_dispatch_dynamic_for_inline_json_get_with_query_uses_get_with_query() {
        use crate::parser::{QueryParam, QueryParamType};
        let mut ep = inline_json_endpoint();
        ep.query_params = vec![QueryParam {
            name: "includeReferencedServices".to_string(),
            rust_name: "include_referenced_services".to_string(),
            required: false,
            ty: QueryParamType::Bool,
            doc: None,
            enum_type_name: None,
        }];
        let mut out = String::new();
        emit_dispatch_dynamic(&mut out, &ep, true, "&[]", false);
        assert!(
            out.contains("self.client.inner().get_with_query("),
            "expected `get_with_query` dispatch for inline-JSON GET with query, got: {out}"
        );
    }
}
