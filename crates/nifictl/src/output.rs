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
    Table,
    Raw,
}

impl OutputFormat {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "auto" => Ok(Self::Auto),
            "json" => Ok(Self::Json),
            "json-compact" => Ok(Self::JsonCompact),
            "yaml" => Ok(Self::Yaml),
            "table" => Ok(Self::Table),
            "raw" => Ok(Self::Raw),
            other => Err(format!(
                "unknown output format '{other}'; valid values: auto, json, json-compact, yaml, table, raw"
            )),
        }
    }

    /// Resolve `Auto` to a concrete format based on whether stdout is a TTY.
    pub fn resolve(&self) -> ResolvedFormat {
        match self {
            Self::Auto => {
                if std::io::stdout().is_terminal() {
                    ResolvedFormat::Table
                } else {
                    ResolvedFormat::JsonCompact
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
                let s = serde_yaml::to_string(val).map_err(io::Error::other)?;
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
                let s = serde_yaml::to_string(items).map_err(io::Error::other)?;
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

    #[test]
    fn auto_resolves_to_table_or_json_compact() {
        // Auto depends on whether stdout is a TTY — both are valid.
        let resolved = OutputFormat::Auto.resolve();
        assert!(
            resolved == ResolvedFormat::Table || resolved == ResolvedFormat::JsonCompact,
            "Auto should resolve to Table or JsonCompact, got {resolved:?}"
        );
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
