//! Per-permutation unit tests for `emit::method::emit_method`.
//!
//! These tests build a minimal `Endpoint` fixture for each meaningful
//! (HTTP method, request body kind, response body kind) triple and assert
//! on substrings of the emitted Rust source. The goal is to pin the
//! dispatch behavior so a future refactor of `emit/method.rs` cannot
//! silently change which HTTP helper a given endpoint is wired to.
//!
//! The existing `mod emit_fix_inline_json_tests` at the end of
//! `emit/method.rs` pins the inline-JSON regression specifically. These
//! tests complement that coverage across the rest of the matrix.

use std::collections::BTreeMap;

use nifi_openapi_gen::canonical::VersionSet;
use nifi_openapi_gen::content_type::ResponseBodyKind;
use nifi_openapi_gen::emit::method::{DynamicMethodCtx, EmitMode, emit_method};
use nifi_openapi_gen::parser::{
    Endpoint, HttpMethod, MultipartField, MultipartFieldType, PathParam, RequestBodyKind,
};

/// Build a minimal `Endpoint` with sensible defaults. Callers override
/// `method`, `body_kind`, `response_kind`, and `response_type` to
/// exercise each dispatch branch.
fn fixture(
    method: HttpMethod,
    path: &str,
    fn_name: &str,
    body: Option<RequestBodyKind>,
    response: ResponseBodyKind,
) -> Endpoint {
    let path_params: Vec<PathParam> = path
        .split('/')
        .filter_map(|seg| {
            seg.strip_prefix('{')
                .and_then(|s| s.strip_suffix('}'))
                .map(|name| PathParam {
                    name: name.to_string(),
                    doc: None,
                })
        })
        .collect();

    Endpoint {
        method,
        path: path.to_string(),
        fn_name: fn_name.to_string(),
        raw_operation_id: fn_name.to_string(),
        doc: None,
        description: None,
        path_params,
        request_type: None,
        body_kind: body,
        body_doc: None,
        multipart_fields: Vec::new(),
        response_type: None,
        response_inner: None,
        response_field: None,
        response_kind: response,
        query_params: vec![],
        header_params: vec![],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    }
}

fn emit_static(ep: &Endpoint) -> String {
    let mut out = String::new();
    emit_method(ep, &EmitMode::Static, &mut out);
    out
}

fn emit_dynamic(ep: &Endpoint, endpoint_variant: &str) -> String {
    let versions = VersionSet::with("2.8.0");
    let query_param_versions: BTreeMap<String, VersionSet> = BTreeMap::new();
    let ctx = DynamicMethodCtx {
        endpoint_versions: &versions,
        query_param_versions: &query_param_versions,
        endpoint_variant: endpoint_variant.to_string(),
        method_lit: ep.method.as_str(),
    };
    let mut out = String::new();
    emit_method(ep, &EmitMode::Dynamic(ctx), &mut out);
    out
}

// ───────────────────── Static mode: GET permutations ─────────────────────

#[test]
fn static_get_no_body_json_uses_get_helper() {
    let ep = fixture(
        HttpMethod::Get,
        "/foo",
        "get_foo",
        None,
        ResponseBodyKind::Json {
            schema_ref: "FooEntity".to_string(),
        },
    );
    let out = emit_static(&ep);
    assert!(
        out.contains("self.client.get("),
        "expected bare .get() call; got:\n{out}"
    );
    assert!(
        !out.contains("get_with_query"),
        "no query params so no _with_query variant; got:\n{out}"
    );
}

#[test]
fn static_get_no_body_text_uses_get_text() {
    let ep = fixture(
        HttpMethod::Get,
        "/foo",
        "get_foo",
        None,
        ResponseBodyKind::Text,
    );
    let out = emit_static(&ep);
    assert!(
        out.contains("self.client.get_text("),
        "expected get_text dispatch; got:\n{out}"
    );
    assert!(
        out.contains("-> Result<String, NifiError>"),
        "text response returns String; got:\n{out}"
    );
}

#[test]
fn static_get_octet_stream_emits_buffered_and_streaming_variants() {
    let ep = fixture(
        HttpMethod::Get,
        "/content",
        "download",
        None,
        ResponseBodyKind::OctetStream,
    );
    let out = emit_static(&ep);
    // Both variants must be present: buffered + streaming.
    assert!(
        out.contains("pub async fn download("),
        "buffered variant missing; got:\n{out}"
    );
    assert!(
        out.contains("pub async fn download_stream("),
        "streaming variant missing; got:\n{out}"
    );
    assert!(
        out.contains("self.client.get_bytes("),
        "buffered variant should call get_bytes; got:\n{out}"
    );
    assert!(
        out.contains("self.client.get_bytes_stream("),
        "streaming variant should call get_bytes_stream; got:\n{out}"
    );
    assert!(
        out.contains("crate::BytesStream"),
        "streaming variant should return BytesStream; got:\n{out}"
    );
}

// ──────────────────── Static mode: POST permutations ────────────────────

#[test]
fn static_post_no_body_with_response_uses_post_no_body() {
    let mut ep = fixture(
        HttpMethod::Post,
        "/foo",
        "create_foo",
        None,
        ResponseBodyKind::Json {
            schema_ref: "FooEntity".to_string(),
        },
    );
    ep.response_type = Some("FooEntity".to_string());
    let out = emit_static(&ep);
    assert!(
        out.contains("self.client.post_no_body("),
        "expected post_no_body for no-body POST with response_type; got:\n{out}"
    );
    assert!(
        !out.contains("post_void_no_body"),
        "must not fall through to post_void_no_body; got:\n{out}"
    );
}

#[test]
fn static_post_no_body_empty_response_uses_post_void_no_body() {
    let ep = fixture(
        HttpMethod::Post,
        "/foo",
        "poke_foo",
        None,
        ResponseBodyKind::Empty,
    );
    let out = emit_static(&ep);
    assert!(
        out.contains("self.client.post_void_no_body("),
        "expected post_void_no_body for no-body POST with empty response; got:\n{out}"
    );
    assert!(
        out.contains("-> Result<(), NifiError>"),
        "empty response returns unit; got:\n{out}"
    );
}

#[test]
fn static_post_json_body_json_response_uses_post() {
    let mut ep = fixture(
        HttpMethod::Post,
        "/foo",
        "create_foo",
        Some(RequestBodyKind::Json),
        ResponseBodyKind::Json {
            schema_ref: "FooEntity".to_string(),
        },
    );
    ep.request_type = Some("FooEntity".to_string());
    ep.response_type = Some("FooEntity".to_string());
    let out = emit_static(&ep);
    assert!(
        out.contains("self.client.post("),
        "expected .post() call; got:\n{out}"
    );
    assert!(
        out.contains("body: &crate::types::FooEntity"),
        "JSON body parameter expected; got:\n{out}"
    );
}

#[test]
fn static_post_octet_stream_uses_post_octet_stream_with_bytes_conversion() {
    let mut ep = fixture(
        HttpMethod::Post,
        "/upload",
        "upload_foo",
        Some(RequestBodyKind::OctetStream),
        ResponseBodyKind::Json {
            schema_ref: "FooEntity".to_string(),
        },
    );
    ep.response_type = Some("FooEntity".to_string());
    let out = emit_static(&ep);
    assert!(
        out.contains("self.client.post_octet_stream("),
        "expected post_octet_stream dispatch; got:\n{out}"
    );
    // Phase 3.3 added `data.into()` to convert Vec<u8> to bytes::Bytes.
    assert!(
        out.contains("data.into()"),
        "expected data.into() (Bytes conversion); got:\n{out}"
    );
    assert!(
        out.contains(", data: Vec<u8>"),
        "octet-stream body parameter expected; got:\n{out}"
    );
}

#[test]
fn static_post_multipart_no_schema_fields_uses_post_multipart() {
    let mut ep = fixture(
        HttpMethod::Post,
        "/upload-file",
        "upload",
        Some(RequestBodyKind::Multipart),
        ResponseBodyKind::Json {
            schema_ref: "FooEntity".to_string(),
        },
    );
    ep.response_type = Some("FooEntity".to_string());
    let out = emit_static(&ep);
    assert!(
        out.contains("self.client.post_multipart("),
        "expected post_multipart dispatch (no schema fields); got:\n{out}"
    );
    assert!(
        !out.contains("post_multipart_with_fields"),
        "must not use _with_fields when multipart_fields is empty; got:\n{out}"
    );
    assert!(
        out.contains(", filename: &str, data: Vec<u8>"),
        "multipart signature missing; got:\n{out}"
    );
}

#[test]
fn static_post_multipart_with_schema_fields_uses_post_multipart_with_fields() {
    let mut ep = fixture(
        HttpMethod::Post,
        "/upload-file",
        "upload",
        Some(RequestBodyKind::Multipart),
        ResponseBodyKind::Json {
            schema_ref: "FooEntity".to_string(),
        },
    );
    ep.response_type = Some("FooEntity".to_string());
    ep.multipart_fields = vec![MultipartField {
        name: "groupName".to_string(),
        rust_name: "group_name".to_string(),
        ty: MultipartFieldType::String,
        required: true,
        doc: None,
    }];
    let out = emit_static(&ep);
    assert!(
        out.contains("self.client.post_multipart_with_fields("),
        "expected post_multipart_with_fields; got:\n{out}"
    );
    assert!(
        out.contains("let mut fields: Vec<(&str, String)>"),
        "fields preamble missing; got:\n{out}"
    );
    assert!(
        out.contains("fields.push((\"groupName\", group_name.to_string()));"),
        "required field push missing; got:\n{out}"
    );
    assert!(
        out.contains(", group_name: &str"),
        "multipart schema field must appear in signature; got:\n{out}"
    );
}

// ──────────────────── Static mode: PUT permutations ────────────────────

#[test]
fn static_put_json_body_uses_put_helper() {
    let mut ep = fixture(
        HttpMethod::Put,
        "/foo/{id}",
        "update_foo",
        Some(RequestBodyKind::Json),
        ResponseBodyKind::Json {
            schema_ref: "FooEntity".to_string(),
        },
    );
    ep.request_type = Some("FooEntity".to_string());
    ep.response_type = Some("FooEntity".to_string());
    let out = emit_static(&ep);
    assert!(
        out.contains("self.client.put("),
        "expected .put() call; got:\n{out}"
    );
    assert!(
        out.contains("body: &crate::types::FooEntity"),
        "JSON body parameter expected; got:\n{out}"
    );
    assert!(
        out.contains(", id: &str"),
        "path param expected in signature; got:\n{out}"
    );
}

#[test]
fn static_put_no_body_empty_response_uses_put_void_no_body() {
    let ep = fixture(
        HttpMethod::Put,
        "/foo/{id}/poke",
        "poke_foo",
        None,
        ResponseBodyKind::Empty,
    );
    let out = emit_static(&ep);
    assert!(
        out.contains("self.client.put_void_no_body("),
        "expected put_void_no_body; got:\n{out}"
    );
}

// ─────────────────── Static mode: DELETE permutations ───────────────────

#[test]
fn static_delete_no_response_uses_delete_helper() {
    let ep = fixture(
        HttpMethod::Delete,
        "/foo/{id}",
        "delete_foo",
        None,
        ResponseBodyKind::Empty,
    );
    let out = emit_static(&ep);
    assert!(
        out.contains("self.client.delete("),
        "expected .delete() call; got:\n{out}"
    );
    // DELETE never emits a `body:` argument in static mode even if
    // body_kind is set.
    assert!(
        !out.contains(", body: "),
        "DELETE must not carry a body arg; got:\n{out}"
    );
}

#[test]
fn static_delete_returning_entity_uses_delete_returning() {
    let mut ep = fixture(
        HttpMethod::Delete,
        "/foo/{id}",
        "delete_foo",
        None,
        ResponseBodyKind::Json {
            schema_ref: "FooEntity".to_string(),
        },
    );
    ep.response_type = Some("FooEntity".to_string());
    let out = emit_static(&ep);
    assert!(
        out.contains("self.client.delete_returning("),
        "expected delete_returning for DELETE with response_type; got:\n{out}"
    );
}

// ──────────────── Static mode: form-encoded is skipped ────────────────

#[test]
fn static_form_encoded_body_is_skipped() {
    // /access/token is the single form-encoded endpoint; its
    // emitter-side output is intentionally empty — the client has a
    // hand-written `login()` method instead.
    let ep = fixture(
        HttpMethod::Post,
        "/access/token",
        "create_access_token",
        Some(RequestBodyKind::FormEncoded),
        ResponseBodyKind::Text,
    );
    let out = emit_static(&ep);
    assert!(
        out.is_empty(),
        "form-encoded endpoints must not be emitted in static mode; got:\n{out}"
    );
}

// ─────────────────── Dynamic mode: core invariants ───────────────────

#[test]
fn dynamic_mode_emits_require_endpoint_gate() {
    let ep = fixture(
        HttpMethod::Get,
        "/foo",
        "get_foo",
        None,
        ResponseBodyKind::Json {
            schema_ref: "FooEntity".to_string(),
        },
    );
    let out = emit_dynamic(&ep, "GetFoo");
    assert!(
        out.contains("self.client.require_endpoint(Endpoint::GetFoo).await?;"),
        "dynamic mode must gate on require_endpoint; got:\n{out}"
    );
    assert!(
        out.contains("self.client.inner().get("),
        "dynamic GET JSON should call inner().get(); got:\n{out}"
    );
}

#[test]
fn dynamic_mode_get_octet_stream_streaming_returns_bytes_stream() {
    let ep = fixture(
        HttpMethod::Get,
        "/content",
        "download",
        None,
        ResponseBodyKind::OctetStream,
    );
    let out = emit_dynamic(&ep, "Download");
    assert!(
        out.contains("pub async fn download_stream("),
        "streaming variant missing; got:\n{out}"
    );
    assert!(
        out.contains("self.client.inner().get_bytes_stream("),
        "streaming variant should dispatch to get_bytes_stream; got:\n{out}"
    );
    assert!(
        out.contains("crate::BytesStream"),
        "streaming return type missing; got:\n{out}"
    );
}

#[test]
fn dynamic_mode_post_void_with_json_body_uses_post_void() {
    let mut ep = fixture(
        HttpMethod::Post,
        "/foo",
        "update_foo",
        Some(RequestBodyKind::Json),
        ResponseBodyKind::Empty,
    );
    ep.request_type = Some("FooEntity".to_string());
    let out = emit_dynamic(&ep, "UpdateFoo");
    assert!(
        out.contains("self.client.inner().post_void("),
        "dynamic POST JSON with empty response should dispatch to post_void; got:\n{out}"
    );
    assert!(
        out.contains("body: &crate::dynamic::types::FooEntity"),
        "dynamic JSON body uses crate::dynamic::types::; got:\n{out}"
    );
}
