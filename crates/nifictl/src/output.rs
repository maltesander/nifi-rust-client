use std::io::{self, IsTerminal, Write};

use serde_json::Value;

use crate::table;

/// Output format selected by the user (may include `Auto`).
#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Auto,
    Json,
    JsonCompact,
    Yaml,
    /// Reachable only via `Auto.resolve()` on a TTY. `parse("table")` is
    /// rejected (C4-drop) so this variant cannot be constructed from
    /// user input — the `#[allow]` silences a dead-code warning that
    /// would otherwise fire because the only construction sites are
    /// the `Auto` resolution arm and the unit test below.
    #[allow(dead_code)]
    Table,
    Raw,
}

impl OutputFormat {
    /// Parse the user's `-o`/`--output` value.
    ///
    /// `table` is intentionally rejected here even though `OutputFormat::Table`
    /// still exists internally — the `Table` variant is reachable only via
    /// `Auto` resolving on a TTY (where the comfy-table renderer produces a
    /// best-effort key-value display). It was never a real columnar table
    /// for arbitrary DTOs and exposing it via `-o table` confused users.
    /// See C4-drop in the audit follow-ups for the rationale.
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "auto" => Ok(Self::Auto),
            "json" => Ok(Self::Json),
            "json-compact" => Ok(Self::JsonCompact),
            "yaml" => Ok(Self::Yaml),
            "raw" => Ok(Self::Raw),
            other => Err(format!(
                "unknown output format '{other}'; valid values: auto, json, json-compact, yaml, raw"
            )),
        }
    }

    /// Resolve `Auto` to a concrete format based on whether stdout is a TTY.
    ///
    /// **C5:** `Auto` on a non-TTY now resolves to `Json` (JSON array for
    /// lists, pretty-printed object for singles) rather than `JsonCompact`
    /// (which emits NDJSON for lists). The previous behaviour broke
    /// downstream `jq` invocations expecting a single document — `jq '.'`
    /// over an NDJSON stream errors with "trailing garbage". Pipelines that
    /// genuinely want NDJSON keep working with explicit `-o json-compact`.
    pub fn resolve(&self) -> ResolvedFormat {
        match self {
            Self::Auto => {
                if std::io::stdout().is_terminal() {
                    ResolvedFormat::Table
                } else {
                    ResolvedFormat::Json
                }
            }
            Self::Json => ResolvedFormat::Json,
            Self::JsonCompact => ResolvedFormat::JsonCompact,
            Self::Yaml => ResolvedFormat::Yaml,
            Self::Table => ResolvedFormat::Table,
            Self::Raw => ResolvedFormat::Raw,
        }
    }
}

/// Concrete output format (no `Auto` variant).
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedFormat {
    Json,
    JsonCompact,
    Yaml,
    Table,
    Raw,
}

/// The logical output of a CLI command.
#[derive(Debug)]
#[allow(dead_code)]
pub enum CliOutput {
    /// A single resource.
    Single(Value),
    /// A collection of resources.
    List(Vec<Value>),
    /// No output.
    Empty,
}

/// Describes how to extract and label one column from a JSON value.
pub struct ColumnDef {
    pub header: &'static str,
    /// JSON Pointer (RFC 6901) to the field, e.g. `"/component/name"`.
    pub json_pointer: &'static str,
}

/// Render `output` in `format` to `writer`, using `columns` for table output.
pub fn render(
    output: &CliOutput,
    format: &ResolvedFormat,
    columns: &[ColumnDef],
    writer: &mut dyn Write,
) -> io::Result<()> {
    match output {
        CliOutput::Empty => Ok(()),

        CliOutput::Single(val) => match format {
            ResolvedFormat::Json => {
                serde_json::to_writer_pretty(&mut *writer, val).map_err(io::Error::other)?;
                writeln!(writer)
            }
            ResolvedFormat::JsonCompact | ResolvedFormat::Raw => {
                serde_json::to_writer(&mut *writer, val).map_err(io::Error::other)?;
                writeln!(writer)
            }
            ResolvedFormat::Yaml => {
                let s = serde_yml::to_string(val).map_err(io::Error::other)?;
                write!(writer, "{s}")
            }
            ResolvedFormat::Table => table::render_single(val, columns, writer),
        },

        CliOutput::List(items) => match format {
            ResolvedFormat::Json => {
                serde_json::to_writer_pretty(&mut *writer, items).map_err(io::Error::other)?;
                writeln!(writer)
            }
            ResolvedFormat::JsonCompact | ResolvedFormat::Raw => {
                // NDJSON: one JSON object per line
                for item in items {
                    serde_json::to_writer(&mut *writer, item).map_err(io::Error::other)?;
                    writeln!(writer)?;
                }
                Ok(())
            }
            ResolvedFormat::Yaml => {
                let s = serde_yml::to_string(items).map_err(io::Error::other)?;
                write!(writer, "{s}")
            }
            ResolvedFormat::Table => table::render_list(items, columns, writer),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn explicit_formats_resolve_to_themselves() {
        assert_eq!(OutputFormat::Json.resolve(), ResolvedFormat::Json);
        assert_eq!(
            OutputFormat::JsonCompact.resolve(),
            ResolvedFormat::JsonCompact
        );
        assert_eq!(OutputFormat::Yaml.resolve(), ResolvedFormat::Yaml);
        assert_eq!(OutputFormat::Table.resolve(), ResolvedFormat::Table);
        assert_eq!(OutputFormat::Raw.resolve(), ResolvedFormat::Raw);
    }

    /// C4-drop: `parse("table")` is rejected so the only way to reach
    /// `OutputFormat::Table` is via `Auto` resolving on a TTY. The
    /// "valid values" list in the message must NOT advertise table any
    /// more — we check that explicitly so a future regression that
    /// reintroduces table doesn't slip past the test.
    #[test]
    fn parse_table_is_rejected() {
        let err = OutputFormat::parse("table").expect_err("table must not parse");
        // The error echoes the input ('table'); inspect only the
        // "valid values:" portion to confirm table is no longer listed.
        let valid_list = err
            .split("valid values:")
            .nth(1)
            .expect("error should contain 'valid values:' suffix");
        assert!(
            !valid_list.contains("table"),
            "'table' must not appear in valid values: {err}"
        );
        assert!(valid_list.contains("auto") && valid_list.contains("json"));
    }

    #[test]
    fn auto_resolves_to_table_or_json() {
        // Auto depends on whether stdout is a TTY:
        // - TTY: Table (key-value pretty render)
        // - non-TTY: Json (array for lists; previously JsonCompact/NDJSON)
        let resolved = OutputFormat::Auto.resolve();
        assert!(
            resolved == ResolvedFormat::Table || resolved == ResolvedFormat::Json,
            "Auto should resolve to Table or Json, got {resolved:?}"
        );
    }

    /// C5: Auto on non-TTY + List output produces a single JSON array,
    /// not NDJSON. We can't directly force a non-TTY in this unit test,
    /// but we can simulate the resolution result and confirm that
    /// `ResolvedFormat::Json` for a list emits one document with a
    /// top-level `[` / `]`.
    #[test]
    fn auto_pipe_list_renders_json_array() {
        let output = CliOutput::List(vec![json!({"id": "1"}), json!({"id": "2"})]);
        let mut buf = Vec::new();
        render(&output, &ResolvedFormat::Json, &[], &mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(
            s.trim_start().starts_with('['),
            "expected JSON array, got: {s}"
        );
        assert!(s.trim_end().ends_with(']'), "expected JSON array, got: {s}");
        // Confirm it's NOT NDJSON: parsing the whole blob as a single
        // serde_json::Value must succeed.
        let parsed: Vec<Value> = serde_json::from_str(&s).expect("must parse as JSON array");
        assert_eq!(parsed.len(), 2);
    }

    #[test]
    fn render_single_json() {
        let output = CliOutput::Single(json!({"id": "abc", "name": "test"}));
        let mut buf = Vec::new();
        render(&output, &ResolvedFormat::Json, &[], &mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        // Pretty JSON should contain newlines and indentation.
        assert!(s.contains("\"id\""));
        assert!(s.contains("\"abc\""));
        assert!(s.contains('\n'));
    }

    #[test]
    fn render_list_ndjson() {
        let output = CliOutput::List(vec![json!({"id": "1"}), json!({"id": "2"})]);
        let mut buf = Vec::new();
        render(&output, &ResolvedFormat::JsonCompact, &[], &mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines.len(), 2, "expected 2 NDJSON lines, got: {s:?}");
    }

    #[test]
    fn render_empty_produces_nothing() {
        let output = CliOutput::Empty;
        let mut buf = Vec::new();
        render(&output, &ResolvedFormat::Json, &[], &mut buf).unwrap();
        assert!(buf.is_empty());
    }
}
