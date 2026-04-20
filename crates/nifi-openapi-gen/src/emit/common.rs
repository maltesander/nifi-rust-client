use crate::content_type::ResponseBodyKind;
use crate::parser::{Endpoint, FieldType, RequestBodyKind};
use crate::util::pascal_case;

/// Rust return type for an endpoint, based on its `response_kind`.
///
/// The `types_prefix` controls where referenced JSON types live:
///
/// - For per-version inherent impls, pass `"crate"` so references resolve to
///   `crate::types::{Name}`.
/// - For per-version emit that references types under a different crate
///   path (e.g. `crate::v2_8_0`), pass that prefix so references expand to
///   `crate::v2_8_0::types::{Name}`.
/// - For dynamic emit that already lives in a module whose `use super::types;`
///   brings the local types into scope, pass `""` so references expand to
///   `types::{Name}`.
///
/// Non-JSON responses map to primitive Rust types with no prefix:
/// `Text`/`Xml` → `String`, `OctetStream`/`Wildcard` → `Vec<u8>`, `Empty` → `()`.
pub(crate) fn response_return_type(ep: &Endpoint, types_prefix: &str) -> String {
    // Helper: build the prefix for a JSON type reference. We keep the legacy
    // behavior (response_inner/response_type) so Entity-unwrapping downstream
    // continues to work unchanged.
    let json_ref = |name: &str| -> String {
        if types_prefix.is_empty() {
            format!("types::{name}")
        } else {
            format!("{types_prefix}::types::{name}")
        }
    };
    match &ep.response_kind {
        ResponseBodyKind::Json { .. } => {
            if let Some(inner) = &ep.response_inner {
                json_ref(inner)
            } else if let Some(ty) = &ep.response_type {
                json_ref(ty)
            } else {
                "serde_json::Value".into()
            }
        }
        ResponseBodyKind::Text | ResponseBodyKind::Xml => "String".into(),
        ResponseBodyKind::OctetStream | ResponseBodyKind::Wildcard => "Vec<u8>".into(),
        ResponseBodyKind::Empty => "()".into(),
    }
}

/// Streaming-variant return type. Only valid for endpoints where
/// `response_kind` is `OctetStream` or `Wildcard`.
pub(crate) fn response_stream_return_type(types_prefix: &str) -> String {
    if types_prefix.is_empty() {
        "crate::BytesStream".into()
    } else {
        format!("{types_prefix}::BytesStream")
    }
}

/// Body parameter signature fragment for non-JSON request bodies.
///
/// Returns the static `, name: type` fragment (including the leading `, `) that
/// gets appended to a generated function signature, or an empty string for body
/// kinds that don't contribute fixed parameters:
///
/// - `OctetStream` → `", filename: Option<&str>, data: Vec<u8>"`
/// - `Multipart`   → `", filename: &str, data: Vec<u8>"`
/// - `Json`        → `""` (the JSON type name is per-endpoint; callers build
///   the `", body: &...::types::{req_type}"` fragment themselves)
/// - `Wildcard | FormEncoded | None` → `""`
pub(crate) fn body_kind_signature(body_kind: Option<&RequestBodyKind>) -> &'static str {
    match body_kind {
        // OctetStream: only the raw bytes; any Filename header is emitted via
        // header_params (B.7) and no longer appears here.
        Some(RequestBodyKind::OctetStream) => ", data: Vec<u8>",
        Some(RequestBodyKind::Multipart) => ", filename: &str, data: Vec<u8>",
        Some(RequestBodyKind::Json)
        | Some(RequestBodyKind::Wildcard)
        | Some(RequestBodyKind::FormEncoded)
        | None => "",
    }
}

/// Descend through `Opt`/`List`/`Map` wrappers to find an inline string enum.
/// Returns the cloned variant list if found, `None` otherwise. Used by both
/// the static (per-version) and dynamic (canonical-superset) emitters.
pub(crate) fn extract_inline_enum_variants(ty: &FieldType) -> Option<Vec<String>> {
    match ty {
        FieldType::Enum(v) => Some(v.clone()),
        FieldType::Opt(inner) => extract_inline_enum_variants(inner),
        FieldType::List(inner) => extract_inline_enum_variants(inner),
        FieldType::Map(inner) => extract_inline_enum_variants(inner),
        _ => None,
    }
}

/// Controls how `FieldType::Enum` is mapped to a Rust type name.
pub(crate) enum InlineEnumMode<'a> {
    /// Static mode: generate an inline enum type named `{struct_name}{PascalCase(field_name)}`.
    Extract {
        struct_name: &'a str,
        field_name: &'a str,
    },
    /// Dynamic mode: map enums to `String`.
    AsString,
}

/// Convert a [`FieldType`] to its Rust type string.
///
/// The `struct_name` parameter is used for self-referential `Ref` types (emitted as `Box<T>`).
/// The `mode` parameter controls how inline `Enum` variants are rendered.
pub(crate) fn field_type_to_rust(
    ty: &FieldType,
    struct_name: &str,
    mode: InlineEnumMode<'_>,
) -> String {
    match ty {
        FieldType::Str => "String".into(),
        FieldType::Bool => "bool".into(),
        FieldType::I32 => "i32".into(),
        FieldType::I64 => "i64".into(),
        FieldType::F64 => "f64".into(),
        FieldType::Opt(inner) => {
            format!(
                "Option<{}>",
                field_type_to_rust(inner, struct_name, reborrow_mode(&mode))
            )
        }
        FieldType::List(inner) => {
            format!(
                "Vec<{}>",
                field_type_to_rust(inner, struct_name, reborrow_mode(&mode))
            )
        }
        FieldType::Ref(name) => {
            if name == struct_name {
                format!("Box<{name}>")
            } else {
                name.clone()
            }
        }
        FieldType::Enum(_) => match &mode {
            InlineEnumMode::Extract {
                struct_name,
                field_name,
            } => {
                format!("{}{}", struct_name, pascal_case(field_name))
            }
            InlineEnumMode::AsString => "String".into(),
        },
        FieldType::Map(value_ty) => format!(
            "std::collections::HashMap<String, {}>",
            field_type_to_rust(value_ty, struct_name, reborrow_mode(&mode))
        ),
    }
}

fn reborrow_mode<'a>(mode: &'a InlineEnumMode<'a>) -> InlineEnumMode<'a> {
    match mode {
        InlineEnumMode::Extract {
            struct_name,
            field_name,
        } => InlineEnumMode::Extract {
            struct_name,
            field_name,
        },
        InlineEnumMode::AsString => InlineEnumMode::AsString,
    }
}

/// Emit a `///` doc-comment block into `out`, prefixing each line with `indent`.
pub(crate) fn emit_doc_comment(out: &mut String, doc: &str, indent: &str) {
    for line in doc.lines() {
        let line = line.trim();
        if line.is_empty() {
            out.push_str(&format!("{indent}///\n"));
        } else {
            out.push_str(&format!("{indent}/// {line}\n"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::FieldType;

    #[test]
    fn field_type_str() {
        let result = field_type_to_rust(&FieldType::Str, "Foo", InlineEnumMode::AsString);
        assert_eq!(result, "String");
    }

    #[test]
    fn field_type_bool() {
        let result = field_type_to_rust(&FieldType::Bool, "Foo", InlineEnumMode::AsString);
        assert_eq!(result, "bool");
    }

    #[test]
    fn field_type_i32() {
        let result = field_type_to_rust(&FieldType::I32, "Foo", InlineEnumMode::AsString);
        assert_eq!(result, "i32");
    }

    #[test]
    fn field_type_i64() {
        let result = field_type_to_rust(&FieldType::I64, "Foo", InlineEnumMode::AsString);
        assert_eq!(result, "i64");
    }

    #[test]
    fn field_type_f64() {
        let result = field_type_to_rust(&FieldType::F64, "Foo", InlineEnumMode::AsString);
        assert_eq!(result, "f64");
    }

    #[test]
    fn field_type_opt() {
        let ty = FieldType::Opt(Box::new(FieldType::Str));
        let result = field_type_to_rust(&ty, "Foo", InlineEnumMode::AsString);
        assert_eq!(result, "Option<String>");
    }

    #[test]
    fn field_type_list() {
        let ty = FieldType::List(Box::new(FieldType::I32));
        let result = field_type_to_rust(&ty, "Foo", InlineEnumMode::AsString);
        assert_eq!(result, "Vec<i32>");
    }

    #[test]
    fn field_type_ref_other() {
        let ty = FieldType::Ref("BarDto".into());
        let result = field_type_to_rust(&ty, "Foo", InlineEnumMode::AsString);
        assert_eq!(result, "BarDto");
    }

    #[test]
    fn field_type_ref_self() {
        let ty = FieldType::Ref("Foo".into());
        let result = field_type_to_rust(&ty, "Foo", InlineEnumMode::AsString);
        assert_eq!(result, "Box<Foo>");
    }

    #[test]
    fn field_type_enum_as_string() {
        let ty = FieldType::Enum(vec!["A".into(), "B".into()]);
        let result = field_type_to_rust(&ty, "Foo", InlineEnumMode::AsString);
        assert_eq!(result, "String");
    }

    #[test]
    fn field_type_enum_extract() {
        let ty = FieldType::Enum(vec!["A".into(), "B".into()]);
        let result = field_type_to_rust(
            &ty,
            "Foo",
            InlineEnumMode::Extract {
                struct_name: "MyStruct",
                field_name: "my_field",
            },
        );
        assert_eq!(result, "MyStructMyField");
    }

    #[test]
    fn field_type_map() {
        let ty = FieldType::Map(Box::new(FieldType::Str));
        let result = field_type_to_rust(&ty, "Foo", InlineEnumMode::AsString);
        assert_eq!(result, "std::collections::HashMap<String, String>");
    }

    #[test]
    fn doc_comment_single_line() {
        let mut out = String::new();
        emit_doc_comment(&mut out, "Hello world", "    ");
        assert_eq!(out, "    /// Hello world\n");
    }

    #[test]
    fn doc_comment_multi_line() {
        let mut out = String::new();
        emit_doc_comment(&mut out, "Line one\nLine two", "");
        assert_eq!(out, "/// Line one\n/// Line two\n");
    }

    #[test]
    fn doc_comment_blank_line() {
        let mut out = String::new();
        emit_doc_comment(&mut out, "Before\n\nAfter", "");
        assert_eq!(out, "/// Before\n///\n/// After\n");
    }

    #[test]
    fn body_kind_signature_octet_stream() {
        // filename is now emitted via header_params (B.7); OctetStream only contributes data.
        assert_eq!(
            body_kind_signature(Some(&RequestBodyKind::OctetStream)),
            ", data: Vec<u8>"
        );
    }

    #[test]
    fn body_kind_signature_multipart() {
        assert_eq!(
            body_kind_signature(Some(&RequestBodyKind::Multipart)),
            ", filename: &str, data: Vec<u8>"
        );
    }

    #[test]
    fn body_kind_signature_json_is_empty() {
        // JSON body type names are per-endpoint and built by callers.
        assert_eq!(body_kind_signature(Some(&RequestBodyKind::Json)), "");
    }

    #[test]
    fn body_kind_signature_no_contribution() {
        assert_eq!(body_kind_signature(None), "");
        assert_eq!(body_kind_signature(Some(&RequestBodyKind::Wildcard)), "");
        assert_eq!(body_kind_signature(Some(&RequestBodyKind::FormEncoded)), "");
    }

    #[test]
    fn extract_inline_enum_variants_finds_through_opt() {
        let ty = FieldType::Opt(Box::new(FieldType::Enum(vec![
            "RUNNING".into(),
            "STOPPED".into(),
        ])));
        let got = extract_inline_enum_variants(&ty);
        assert_eq!(
            got,
            Some(vec!["RUNNING".to_string(), "STOPPED".to_string()])
        );
    }

    #[test]
    fn extract_inline_enum_variants_finds_through_list() {
        let ty = FieldType::List(Box::new(FieldType::Enum(vec!["A".into()])));
        let got = extract_inline_enum_variants(&ty);
        assert_eq!(got, Some(vec!["A".to_string()]));
    }

    #[test]
    fn extract_inline_enum_variants_finds_through_map() {
        let ty = FieldType::Map(Box::new(FieldType::Enum(vec!["K".into()])));
        let got = extract_inline_enum_variants(&ty);
        assert_eq!(got, Some(vec!["K".to_string()]));
    }

    #[test]
    fn extract_inline_enum_variants_returns_none_for_str() {
        assert_eq!(extract_inline_enum_variants(&FieldType::Str), None);
    }
}
