use crate::parser::{FieldType, RequestBodyKind};
use crate::util::pascal_case;

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
        Some(RequestBodyKind::OctetStream) => ", filename: Option<&str>, data: Vec<u8>",
        Some(RequestBodyKind::Multipart) => ", filename: &str, data: Vec<u8>",
        Some(RequestBodyKind::Json)
        | Some(RequestBodyKind::Wildcard)
        | Some(RequestBodyKind::FormEncoded)
        | None => "",
    }
}

/// Forward-args fragment for delegating a call from a wrapper to the underlying
/// inherent method. Returns the `", a, b"` portion (including the leading `, `)
/// or an empty string. For builders that need a `Vec<String>`, see
/// [`body_kind_forward_arg_names`].
///
/// - `Json`        → `", body"`
/// - `OctetStream` → `", filename, data"`
/// - `Multipart`   → `", filename, data"`
/// - `Wildcard | FormEncoded | None` → `""`
pub(crate) fn body_kind_forward_args(body_kind: Option<&RequestBodyKind>) -> &'static str {
    match body_kind {
        Some(RequestBodyKind::Json) => ", body",
        Some(RequestBodyKind::OctetStream) | Some(RequestBodyKind::Multipart) => {
            ", filename, data"
        }
        Some(RequestBodyKind::Wildcard) | Some(RequestBodyKind::FormEncoded) | None => "",
    }
}

/// Individual forward-arg names for a given `body_kind`. Used by call sites
/// that build up a `Vec<String>` of arguments rather than a single fragment.
/// Mirrors [`body_kind_forward_args`] but returns the names as a slice with
/// no leading comma.
pub(crate) fn body_kind_forward_arg_names(
    body_kind: Option<&RequestBodyKind>,
) -> &'static [&'static str] {
    match body_kind {
        Some(RequestBodyKind::Json) => &["body"],
        Some(RequestBodyKind::OctetStream) | Some(RequestBodyKind::Multipart) => {
            &["filename", "data"]
        }
        Some(RequestBodyKind::Wildcard) | Some(RequestBodyKind::FormEncoded) | None => &[],
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
        assert_eq!(
            body_kind_signature(Some(&RequestBodyKind::OctetStream)),
            ", filename: Option<&str>, data: Vec<u8>"
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
    fn body_kind_forward_args_json() {
        assert_eq!(
            body_kind_forward_args(Some(&RequestBodyKind::Json)),
            ", body"
        );
    }

    #[test]
    fn body_kind_forward_args_octet_stream() {
        assert_eq!(
            body_kind_forward_args(Some(&RequestBodyKind::OctetStream)),
            ", filename, data"
        );
    }

    #[test]
    fn body_kind_forward_args_multipart() {
        assert_eq!(
            body_kind_forward_args(Some(&RequestBodyKind::Multipart)),
            ", filename, data"
        );
    }

    #[test]
    fn body_kind_forward_args_no_contribution() {
        assert_eq!(body_kind_forward_args(None), "");
        assert_eq!(
            body_kind_forward_args(Some(&RequestBodyKind::Wildcard)),
            ""
        );
        assert_eq!(
            body_kind_forward_args(Some(&RequestBodyKind::FormEncoded)),
            ""
        );
    }

    #[test]
    fn body_kind_forward_arg_names_variants() {
        assert_eq!(
            body_kind_forward_arg_names(Some(&RequestBodyKind::Json)),
            &["body"]
        );
        assert_eq!(
            body_kind_forward_arg_names(Some(&RequestBodyKind::OctetStream)),
            &["filename", "data"]
        );
        assert_eq!(
            body_kind_forward_arg_names(Some(&RequestBodyKind::Multipart)),
            &["filename", "data"]
        );
        assert!(body_kind_forward_arg_names(None).is_empty());
        assert!(body_kind_forward_arg_names(Some(&RequestBodyKind::Wildcard)).is_empty());
        assert!(body_kind_forward_arg_names(Some(&RequestBodyKind::FormEncoded)).is_empty());
    }
}
