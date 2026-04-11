//! Strict-parsing panic helpers. A panic from here means the OpenAPI spec
//! contains a shape nifi-openapi-gen has never seen before — investigate,
//! then teach the parser about it (see `content_type.rs` for request/response
//! bodies, `parser.rs` scalar-type match arms for field and query types).

pub fn unknown_shape_message(kind: &str, json_pointer: &str, detail: &str) -> String {
    format!(
        "nifi-openapi-gen: unknown {kind} at {json_pointer} ({detail}). \
         This means the OpenAPI spec has a shape the generator doesn't handle. \
         Investigate in crates/nifi-openapi-gen/src/parser.rs and, if the shape \
         is legitimate, add a new match arm. Do NOT paper over this by \
         extending the catch-all — the whole point is to surface new shapes."
    )
}

#[track_caller]
pub fn panic_unknown(kind: &str, json_pointer: &str, detail: &str) -> ! {
    panic!("{}", unknown_shape_message(kind, json_pointer, detail));
}
