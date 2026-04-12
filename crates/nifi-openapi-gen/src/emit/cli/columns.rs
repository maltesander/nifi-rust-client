use crate::parser::TypeDef;

/// Priority fields to include as table columns, in order.
const PRIORITY_FIELDS: &[(&str, &str)] = &[
    ("id", "/id"),
    ("name", "/name"),
    ("r#type", "/type"),
    ("state", "/state"),
    ("status", "/status"),
    ("run_status", "/runStatus"),
    ("uri", "/uri"),
];

/// Maximum number of columns to emit.
const MAX_COLUMNS: usize = 6;

/// Emit a static `&[ColumnDef]` array for a response type.
///
/// Returns `None` if the type is not found or no priority fields match.
pub fn emit_column_defs(response_type: Option<&str>, all_types: &[TypeDef]) -> Option<String> {
    let type_name = response_type?;
    let typedef = all_types.iter().find(|t| t.name == type_name)?;

    let mut columns = Vec::new();
    for &(rust_name, json_pointer) in PRIORITY_FIELDS {
        if columns.len() >= MAX_COLUMNS {
            break;
        }
        if typedef.fields.iter().any(|f| f.rust_name == rust_name) {
            columns.push((rust_name, json_pointer));
        }
    }

    if columns.is_empty() {
        return None;
    }

    let mut out = String::new();
    out.push_str("&[\n");
    for (rust_name, json_pointer) in &columns {
        // Use the rust_name (without r# prefix) as the header, uppercased
        let header = rust_name
            .strip_prefix("r#")
            .unwrap_or(rust_name)
            .to_uppercase();
        out.push_str(&format!(
            "    crate::output::ColumnDef {{ header: \"{header}\", json_pointer: \"{json_pointer}\" }},\n"
        ));
    }
    out.push(']');

    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Field, FieldType, TypeKind};

    fn make_type(name: &str, field_names: &[&str]) -> TypeDef {
        TypeDef {
            name: name.to_string(),
            kind: TypeKind::Dto,
            fields: field_names
                .iter()
                .map(|n| Field {
                    rust_name: n.to_string(),
                    serde_name: n.to_string(),
                    ty: FieldType::Opt(Box::new(FieldType::Str)),
                    doc: None,
                    read_only: false,
                    deprecated: false,
                })
                .collect(),
            doc: None,
        }
    }

    #[test]
    fn picks_standard_columns() {
        let types = vec![make_type(
            "ProcessorDto",
            &["id", "name", "state", "uri", "other_field"],
        )];
        let result = emit_column_defs(Some("ProcessorDto"), &types).unwrap();
        assert!(result.contains("/id"));
        assert!(result.contains("/name"));
        assert!(result.contains("/state"));
        assert!(result.contains("/uri"));
        assert!(!result.contains("other_field"));
    }

    #[test]
    fn returns_none_for_unknown_type() {
        let types = vec![make_type("ProcessorDto", &["id", "name"])];
        assert!(emit_column_defs(Some("UnknownType"), &types).is_none());
    }

    #[test]
    fn returns_none_for_no_matching_fields() {
        let types = vec![make_type("WeirdDto", &["foo", "bar", "baz"])];
        assert!(emit_column_defs(Some("WeirdDto"), &types).is_none());
    }
}
