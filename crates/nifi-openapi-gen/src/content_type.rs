//! Central allow-list for OpenAPI content types. If a new shape appears in
//! a NiFi spec and the parser panics, the fix usually belongs here: add a
//! variant, add a match arm, teach the emitters how to render it. Do NOT
//! add unknown content types to the catch-all.

use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestBodyKind {
    /// application/json with a $ref schema.
    Json,
    /// application/octet-stream — raw bytes.
    OctetStream,
    /// application/x-www-form-urlencoded — skipped by emitters, handled by
    /// hand-written client code (currently only /access/token).
    FormEncoded,
    /// multipart/form-data — a file upload with optional metadata fields.
    Multipart,
    /// */* — NiFi's idiom for "body allowed but shape unspecified".
    /// Emitted as a no-body variant unless the endpoint is overridden.
    Wildcard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseBodyKind {
    /// application/json, schema via $ref.
    Json { schema_ref: String },
    /// text/plain — returned as String.
    Text,
    /// application/octet-stream — returned as Vec<u8>.
    OctetStream,
    /// application/xml — returned as String. No deserialization.
    Xml,
    /// */* — returned as Vec<u8>. Caller interprets.
    Wildcard,
    /// No body (2xx without content).
    Empty,
}

pub fn resolve_request_body(content: &Value, json_pointer: &str) -> Option<RequestBodyKind> {
    let map = content.as_object()?;
    if map.is_empty() {
        return None;
    }
    // Precedence: json > octet-stream > multipart > form-encoded > wildcard.
    // If multiple are present (rare in NiFi), prefer the strongest typing.
    if map.contains_key("application/json") {
        return Some(RequestBodyKind::Json);
    }
    if map.contains_key("application/octet-stream") {
        return Some(RequestBodyKind::OctetStream);
    }
    if map.contains_key("multipart/form-data") {
        return Some(RequestBodyKind::Multipart);
    }
    if map.contains_key("application/x-www-form-urlencoded") {
        return Some(RequestBodyKind::FormEncoded);
    }
    if map.contains_key("*/*") {
        return Some(RequestBodyKind::Wildcard);
    }
    let keys: Vec<&str> = map.keys().map(String::as_str).collect();
    crate::parser_strict::panic_unknown(
        "request_body_content_type",
        json_pointer,
        &format!("content keys={keys:?}"),
    );
}

pub fn resolve_response_body(content: &Value, json_pointer: &str) -> ResponseBodyKind {
    let Some(map) = content.as_object() else {
        return ResponseBodyKind::Empty;
    };
    if map.is_empty() {
        return ResponseBodyKind::Empty;
    }
    if let Some(schema) = map.get("application/json").and_then(|v| v.get("schema")) {
        if let Some(r) = schema.get("$ref").and_then(|r| r.as_str()) {
            let name = r.trim_start_matches("#/components/schemas/").to_string();
            return ResponseBodyKind::Json { schema_ref: name };
        }
        // application/json without a $ref: inline schema — treat as opaque
        // serde_json::Value.
        return ResponseBodyKind::Json {
            schema_ref: "serde_json::Value".to_string(),
        };
    }
    if map.contains_key("text/plain") {
        return ResponseBodyKind::Text;
    }
    if map.contains_key("application/octet-stream") {
        return ResponseBodyKind::OctetStream;
    }
    if map.contains_key("application/xml") {
        return ResponseBodyKind::Xml;
    }
    if map.contains_key("*/*") {
        return ResponseBodyKind::Wildcard;
    }
    let keys: Vec<&str> = map.keys().map(String::as_str).collect();
    crate::parser_strict::panic_unknown(
        "response_body_content_type",
        json_pointer,
        &format!("content keys={keys:?}"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn resolves_json_request_body() {
        let content = json!({ "application/json": { "schema": { "$ref": "#/components/schemas/Foo" } } });
        let kind = resolve_request_body(&content, "/paths/~1x/post").unwrap();
        assert!(matches!(kind, RequestBodyKind::Json));
    }

    #[test]
    fn resolves_multipart_request_body() {
        let content = json!({ "multipart/form-data": { "schema": { "type": "object" } } });
        let kind = resolve_request_body(&content, "/paths/~1x/post").unwrap();
        assert!(matches!(kind, RequestBodyKind::Multipart));
    }

    #[test]
    fn resolves_wildcard_request_body() {
        let content = json!({ "*/*": {} });
        let kind = resolve_request_body(&content, "/paths/~1x/post").unwrap();
        assert!(matches!(kind, RequestBodyKind::Wildcard));
    }

    #[test]
    #[should_panic(expected = "nifi-openapi-gen: unknown request_body_content_type")]
    fn unknown_content_type_panics() {
        let content = json!({ "application/frobnitz": {} });
        let _ = resolve_request_body(&content, "/paths/~1x/post");
    }

    #[test]
    fn picks_json_when_multiple_content_types_present() {
        let content = json!({
            "application/json": { "schema": { "$ref": "#/components/schemas/Foo" } },
            "*/*": {}
        });
        let kind = resolve_request_body(&content, "/paths/~1x/post").unwrap();
        assert!(matches!(kind, RequestBodyKind::Json));
    }

    #[test]
    fn resolves_json_response_with_ref() {
        let content = json!({ "application/json": { "schema": { "$ref": "#/components/schemas/AboutEntity" } } });
        let kind = resolve_response_body(&content, "/paths/~1x/get/responses/200");
        assert!(matches!(&kind, ResponseBodyKind::Json { schema_ref } if schema_ref == "AboutEntity"));
    }

    #[test]
    fn resolves_text_response() {
        let content = json!({ "text/plain": {} });
        let kind = resolve_response_body(&content, "/paths/~1x/get/responses/200");
        assert!(matches!(kind, ResponseBodyKind::Text));
    }

    #[test]
    fn resolves_octet_stream_response() {
        let content = json!({ "application/octet-stream": {} });
        let kind = resolve_response_body(&content, "/paths/~1x/get/responses/200");
        assert!(matches!(kind, ResponseBodyKind::OctetStream));
    }

    #[test]
    fn resolves_xml_response() {
        let content = json!({ "application/xml": {} });
        let kind = resolve_response_body(&content, "/paths/~1x/get/responses/200");
        assert!(matches!(kind, ResponseBodyKind::Xml));
    }

    #[test]
    fn resolves_wildcard_response() {
        let content = json!({ "*/*": {} });
        let kind = resolve_response_body(&content, "/paths/~1x/get/responses/200");
        assert!(matches!(kind, ResponseBodyKind::Wildcard));
    }

    #[test]
    fn empty_response_content_is_empty() {
        let content = json!({});
        let kind = resolve_response_body(&content, "/paths/~1x/get/responses/204");
        assert!(matches!(kind, ResponseBodyKind::Empty));
    }

    #[test]
    #[should_panic(expected = "nifi-openapi-gen: unknown response_body_content_type")]
    fn unknown_response_content_type_panics() {
        let content = json!({ "application/frobnitz": {} });
        let _ = resolve_response_body(&content, "/paths/~1x/get/responses/200");
    }
}
