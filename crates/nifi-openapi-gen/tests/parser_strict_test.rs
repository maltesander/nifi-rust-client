use nifi_openapi_gen::parser_strict::unknown_shape_message;

#[test]
fn unknown_shape_message_is_actionable() {
    let msg = unknown_shape_message("field_type", "/paths/~1foo/get", "type=\"frobnitz\"");
    assert!(msg.contains("nifi-openapi-gen: unknown field_type"));
    assert!(msg.contains("/paths/~1foo/get"));
    assert!(msg.contains("type=\"frobnitz\""));
    assert!(msg.contains("parser.rs"));
}
