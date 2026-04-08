use crate::parser::FieldType;
use crate::util::pascal_case;

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
}
