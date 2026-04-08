use crate::parser::{ApiSpec, Endpoint, HttpMethod, QueryParam, SubGroup, TagGroup};

pub fn emit_tests(spec: &ApiSpec) -> String {
    let mut out = String::new();
    out.push_str("use nifi_rust_client::NifiClientBuilder;\n");
    out.push_str("use wiremock::{MockServer, Mock, ResponseTemplate};\n");
    out.push_str("use wiremock::matchers::{header, method, path, query_param};\n\n");

    for tag in &spec.tags {
        out.push_str(&emit_tag_tests(tag));
    }

    crate::util::format_source(&out)
}

fn emit_tag_tests(tag: &TagGroup) -> String {
    let mut out = String::new();
    let mod_name = tag.module_name.replace('-', "_");
    out.push_str(&format!(
        "#[cfg(test)]\nmod {mod_name}_generated_tests {{\n"
    ));
    out.push_str("    use super::*;\n\n");
    // Root endpoints
    for ep in &tag.root_endpoints {
        out.push_str(&emit_endpoint_test(ep, &tag.accessor_fn, None));
    }
    // Sub-group endpoints
    for sg in &tag.sub_groups {
        for ep in &sg.endpoints {
            out.push_str(&emit_endpoint_test(ep, &tag.accessor_fn, Some(sg)));
        }
    }
    out.push_str("}\n\n");
    out
}

fn emit_endpoint_test(ep: &Endpoint, accessor: &str, sub_group: Option<&SubGroup>) -> String {
    // Skip form-encoded endpoints — they have no generated API method.
    if ep.body_kind == Some(crate::parser::RequestBodyKind::FormEncoded) {
        return String::new();
    }
    let http_method = method_str(&ep.method);
    let param_names: Vec<String> = ep.path_params.iter().map(|p| p.name.clone()).collect();
    let nifi_path = if param_names.is_empty() {
        format!("/nifi-api{}", ep.path)
    } else {
        format!("/nifi-api{}", make_concrete_path(&ep.path, &param_names))
    };
    let has_response = ep.response_type.is_some();

    use crate::parser::{HttpMethod, RequestBodyKind};
    // DELETE endpoints never send a body — ignore body_kind for test generation.
    let (body_setup, body_arg) = if ep.method == HttpMethod::Delete {
        (String::new(), None)
    } else {
        match &ep.body_kind {
            Some(RequestBodyKind::Json) => {
                let ty = ep.request_type.as_deref().unwrap_or("serde_json::Value");
                (
                    format!(
                        "    let body = nifi_rust_client::types::{ty} {{ ..Default::default() }};\n"
                    ),
                    Some("&body".to_string()),
                )
            }
            Some(RequestBodyKind::OctetStream) => (
                String::new(),
                Some("Some(\"test.nar\"), vec![1u8, 2, 3]".to_string()),
            ),
            Some(RequestBodyKind::FormEncoded) | None => (String::new(), None),
        }
    };

    let response_setup = if has_response {
        let ty = ep.response_type.as_deref().unwrap_or("serde_json::Value");
        format!(
            "ResponseTemplate::new(200).set_body_json(\
             serde_json::to_value(nifi_rust_client::types::{ty}::default()).unwrap())"
        )
    } else {
        "ResponseTemplate::new(200)".to_string()
    };

    let assert_line = "    assert!(result.is_ok(), \"{:?}\", result);\n";

    // Query param matchers for required params
    let query_matchers: String = ep
        .query_params
        .iter()
        .filter(|qp| qp.required)
        .map(|qp| {
            let test_val = query_test_value(qp);
            format!(
                "\n            .and(query_param(\"{name}\", \"{test_val}\"))",
                name = qp.name
            )
        })
        .collect();
    let octet_stream_matcher =
        if ep.method != HttpMethod::Delete && ep.body_kind == Some(RequestBodyKind::OctetStream) {
            "\n            .and(header(\"content-type\", \"application/octet-stream\"))".to_string()
        } else {
            String::new()
        };

    let call_expr = match sub_group {
        None => {
            // client.accessor().fn_name(path_args..., query_args..., body?)
            let mut all_args: Vec<String> = ep
                .path_params
                .iter()
                .map(|p| format!("\"test-{}\"", p.name))
                .collect();
            for qp in &ep.query_params {
                let arg = if qp.required {
                    query_test_arg(qp)
                } else {
                    "None".to_string()
                };
                all_args.push(arg);
            }
            if let Some(b) = &body_arg {
                all_args.push(b.clone());
            }
            format!(
                "client.{accessor}().{fn_name}({args})",
                fn_name = ep.fn_name,
                args = all_args.join(", ")
            )
        }
        Some(sg) => {
            // client.accessor().sub_accessor("test-{primary}").fn_name(remaining_args..., query_args..., body?)
            let primary = &sg.primary_param;
            let mut all_args: Vec<String> = ep
                .path_params
                .iter()
                .filter(|p| p.name != *primary)
                .map(|p| format!("\"test-{}\"", p.name))
                .collect();
            for qp in &ep.query_params {
                let arg = if qp.required {
                    query_test_arg(qp)
                } else {
                    "None".to_string()
                };
                all_args.push(arg);
            }
            if let Some(b) = &body_arg {
                all_args.push(b.clone());
            }
            format!(
                "client.{accessor}().{sub_accessor}(\"test-{primary}\").{fn_name}({args})",
                sub_accessor = sg.accessor_fn,
                fn_name = ep.fn_name,
                args = all_args.join(", ")
            )
        }
    };

    format!(
        "    #[tokio::test]\n    async fn test_{fn_name}() {{\n        let mock_server = MockServer::start().await;\n        Mock::given(method(\"{http_method}\"))\n            .and(path(\"{nifi_path}\")){query_matchers}{octet_stream_matcher}\n            .respond_with({response_setup})\n            .mount(&mock_server)\n            .await;\n{body_setup}        let client = NifiClientBuilder::new(&mock_server.uri()).unwrap().build().unwrap();\n        let result = {call_expr}.await;\n{assert_line}    }}\n\n",
        fn_name = ep.fn_name,
    )
}

fn query_test_value(qp: &QueryParam) -> String {
    use crate::parser::QueryParamType;
    match &qp.ty {
        QueryParamType::Str => "test-value".to_string(),
        QueryParamType::Bool => "true".to_string(),
        QueryParamType::I32 | QueryParamType::I64 => "0".to_string(),
        QueryParamType::F64 => "0".to_string(),
        QueryParamType::Enum(variants) => {
            // Use the first variant as a test value; fall back to "test-value"
            variants
                .first()
                .map(|v| v.as_str())
                .unwrap_or("test-value")
                .to_string()
        }
    }
}

fn query_test_arg(qp: &QueryParam) -> String {
    use crate::parser::QueryParamType;
    match &qp.ty {
        QueryParamType::Str => "\"test-value\"".to_string(),
        QueryParamType::Bool => "true".to_string(),
        QueryParamType::I32 => "0_i32".to_string(),
        QueryParamType::I64 => "0_i64".to_string(),
        QueryParamType::F64 => "0.0_f64".to_string(),
        QueryParamType::Enum(variants) => {
            // Use the first variant; fall back to a string literal
            if let Some(v) = variants.first() {
                let type_name = qp
                    .enum_type_name
                    .as_deref()
                    .expect("enum param must have type name");
                let variant = crate::emit_types::pascal_case(v);
                format!("crate::types::{type_name}::{variant}")
            } else {
                "\"test-value\"".to_string()
            }
        }
    }
}

fn make_concrete_path(path: &str, path_params: &[String]) -> String {
    let normalized = normalize_path_params(path);
    path_params.iter().fold(normalized, |p, param| {
        p.replace(&format!("{{{param}}}"), &format!("test-{param}"))
    })
}

fn normalize_path_params(path: &str) -> String {
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

fn method_str(m: &HttpMethod) -> &'static str {
    match m {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Put => "PUT",
        HttpMethod::Delete => "DELETE",
    }
}

