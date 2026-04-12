use std::io::{self, Write};

use comfy_table::{Table, presets::UTF8_FULL_CONDENSED};
use serde_json::Value;

use crate::output::ColumnDef;

/// Extract a field from `val` using a JSON Pointer. Returns `"-"` for missing/null.
pub fn extract_field(val: &Value, pointer: &str) -> String {
    match val.pointer(pointer) {
        Some(v) => format_value(v),
        None => "-".to_string(),
    }
}

/// Format a JSON value as a human-readable string for table cells.
pub fn format_value(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "-".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    }
}

/// Render a single JSON value as a table.
///
/// - If `columns` is empty: key-value table (Field | Value) for objects, or pretty JSON.
/// - If `columns` is non-empty: single-row table with column headers.
pub fn render_single(val: &Value, columns: &[ColumnDef], writer: &mut dyn Write) -> io::Result<()> {
    if columns.is_empty() {
        // Fallback: key-value table for objects, pretty JSON otherwise.
        if let Value::Object(map) = val {
            let mut table = Table::new();
            table.load_preset(UTF8_FULL_CONDENSED);
            table.set_header(["Field", "Value"]);
            for (key, value) in map {
                table.add_row([key.as_str(), &format_value(value)]);
            }
            writeln!(writer, "{table}")
        } else {
            let s = serde_json::to_string_pretty(val).map_err(io::Error::other)?;
            writeln!(writer, "{s}")
        }
    } else {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL_CONDENSED);
        table.set_header(columns.iter().map(|c| c.header));
        let row: Vec<String> = columns
            .iter()
            .map(|c| extract_field(val, c.json_pointer))
            .collect();
        table.add_row(row);
        writeln!(writer, "{table}")
    }
}

/// Render a list of JSON values as a table.
///
/// - If `columns` is empty: fallback to pretty-print JSON array.
/// - If `columns` is non-empty: multi-row table.
pub fn render_list(
    items: &[Value],
    columns: &[ColumnDef],
    writer: &mut dyn Write,
) -> io::Result<()> {
    if columns.is_empty() {
        let s = serde_json::to_string_pretty(items).map_err(io::Error::other)?;
        writeln!(writer, "{s}")
    } else {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL_CONDENSED);
        table.set_header(columns.iter().map(|c| c.header));
        for item in items {
            let row: Vec<String> = columns
                .iter()
                .map(|c| extract_field(item, c.json_pointer))
                .collect();
            table.add_row(row);
        }
        writeln!(writer, "{table}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::ColumnDef;
    use serde_json::json;

    #[test]
    fn render_list_with_columns() {
        let items = vec![
            json!({"id": "abc", "name": "Foo", "state": "RUNNING"}),
            json!({"id": "def", "name": "Bar", "state": "STOPPED"}),
        ];
        let columns = [
            ColumnDef {
                header: "ID",
                json_pointer: "/id",
            },
            ColumnDef {
                header: "Name",
                json_pointer: "/name",
            },
            ColumnDef {
                header: "State",
                json_pointer: "/state",
            },
        ];
        let mut buf = Vec::new();
        render_list(&items, &columns, &mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("ID"), "expected header ID in: {s}");
        assert!(s.contains("Foo"), "expected Foo in: {s}");
        assert!(s.contains("STOPPED"), "expected STOPPED in: {s}");
    }

    #[test]
    fn missing_field_shows_dash() {
        let val = json!({"id": "abc"});
        assert_eq!(extract_field(&val, "/name"), "-");
        assert_eq!(extract_field(&val, "/id"), "abc");
    }

    #[test]
    fn single_without_columns_shows_key_value() {
        let val = json!({"id": "abc", "name": "Foo"});
        let mut buf = Vec::new();
        render_single(&val, &[], &mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("Field"), "expected 'Field' header in: {s}");
        assert!(s.contains("Value"), "expected 'Value' header in: {s}");
        assert!(s.contains("id"), "expected 'id' field in: {s}");
        assert!(s.contains("abc"), "expected 'abc' value in: {s}");
    }
}
