use std::io::{self, Write};

use comfy_table::Table;
use comfy_table::presets::{ASCII_BORDERS_ONLY_CONDENSED, UTF8_FULL_CONDENSED};
use serde_json::Value;

use crate::output::ColumnDef;

/// Pick the comfy-table preset honouring the [NO_COLOR] convention.
///
/// Today our tables don't apply any ANSI color codes, so the strict
/// "no ANSI escapes" rule is already met by either preset. NO_COLOR
/// also commonly signals "I'm in a non-decorative environment" — log
/// scrapers, CI logs, terminals without Unicode font support — so we
/// downgrade to an ASCII-borders preset when the variable is set.
///
/// [NO_COLOR]: https://no-color.org/
fn preset() -> &'static str {
    if std::env::var_os("NO_COLOR").is_some() {
        ASCII_BORDERS_ONLY_CONDENSED
    } else {
        UTF8_FULL_CONDENSED
    }
}

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
            table.load_preset(preset());
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
        table.load_preset(preset());
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
        table.load_preset(preset());
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

    /// C13: when `NO_COLOR` is set, table renders use the ASCII-borders
    /// preset instead of the UTF-8 box-drawing one. There are no ANSI
    /// color codes in either case (we never apply cell colours), but the
    /// ASCII preset is more friendly to log scrapers and CI pipelines —
    /// which is the spirit of NO_COLOR.
    #[test]
    fn no_color_uses_ascii_preset() {
        // Serialize this single env-var-mutating test so it can't race
        // with other tests in this module under cargo's parallel runner.
        // No other table tests touch NO_COLOR, so a local mutex is fine.
        use std::sync::Mutex;
        static ENV_LOCK: Mutex<()> = Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        // SAFETY: ENV_LOCK serializes this test; no internal threads.
        unsafe {
            std::env::set_var("NO_COLOR", "1");
        }
        let val = json!({"id": "abc"});
        let mut buf = Vec::new();
        render_single(&val, &[], &mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        // ASCII preset has no UTF-8 box-drawing chars (e.g. │ ─ ┼).
        assert!(
            !s.contains('│') && !s.contains('─') && !s.contains('┼'),
            "NO_COLOR render should not contain UTF-8 box chars: {s}"
        );
        unsafe {
            std::env::remove_var("NO_COLOR");
        }
    }
}
