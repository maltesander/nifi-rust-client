use std::collections::{BTreeMap, BTreeSet};

use crate::parser::{ApiSpec, Field, FieldType, TypeKind};

/// Intermediate representation of a merged type across all API versions.
enum MergedType {
    Dto {
        /// Union of all fields across versions, preserving insertion order via BTreeMap on rust_name.
        fields: BTreeMap<String, Field>,
        /// Fields that exist in every version with the same Rust type string.
        /// These can be emitted without extra Option wrapping.
        universal_fields: BTreeSet<String>,
        /// Doc comment from the first version's TypeDef, if present.
        doc: Option<String>,
    },
    Entity {
        field: String,
        inner: String,
    },
    StringEnum {
        variants: BTreeSet<String>,
    },
}

/// Merge type definitions from all API versions and emit a single Rust source file
/// containing union types where all fields are `Option<T>`.
pub fn emit_dynamic_types(specs: &[(&str, &ApiSpec)]) -> String {
    let merged = merge_all_types(specs);

    let mut out = String::new();
    out.push_str("#![allow(dead_code, private_interfaces)]\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n\n");

    for (name, mt) in &merged {
        match mt {
            MergedType::Dto {
                fields,
                universal_fields,
                doc,
            } => {
                if let Some(d) = doc {
                    for line in d.lines() {
                        out.push_str(&format!("/// {line}\n"));
                    }
                }
                out.push_str("#[non_exhaustive]\n");
                out.push_str("#[derive(Debug, Clone, Default, Deserialize, Serialize)]\n");
                out.push_str("#[serde(rename_all = \"camelCase\")]\n");
                out.push_str(&format!("pub struct {name} {{\n"));
                for field in fields.values() {
                    let rust_ty = field_type_to_rust(&field.ty, name);
                    let is_universal = universal_fields.contains(&field.rust_name);
                    let final_ty = if is_universal {
                        // Field exists in all versions with the same type — emit as-is
                        rust_ty
                    } else {
                        // Field missing in some version or type differs — wrap in Option
                        wrap_in_option(&field.ty, &rust_ty)
                    };
                    if let Some(field_doc) = &field.doc {
                        for line in field_doc.lines() {
                            out.push_str(&format!("    /// {line}\n"));
                        }
                    }
                    // Use serde rename if it differs from rust_name
                    if field.serde_name != field.rust_name {
                        out.push_str(&format!(
                            "    #[serde(rename = \"{}\")]\n",
                            field.serde_name
                        ));
                    }
                    // Non-universal fields need default on deserialization for missing keys
                    if !is_universal {
                        out.push_str("    #[serde(default)]\n");
                    }
                    // Add skip_serializing_if for Option fields
                    if final_ty.starts_with("Option<") {
                        out.push_str(
                            "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                        );
                    }
                    out.push_str(&format!(
                        "    pub {}: {final_ty},\n",
                        escape_keyword(&field.rust_name)
                    ));
                }
                out.push_str("}\n\n");
            }
            MergedType::Entity { field, inner } => {
                out.push_str("#[derive(Debug, Clone, Default, Deserialize, Serialize)]\n");
                out.push_str("#[serde(rename_all = \"camelCase\")]\n");
                out.push_str(&format!("pub struct {name} {{\n"));
                out.push_str(&format!(
                    "    pub {}: Option<{inner}>,\n",
                    escape_keyword(field)
                ));
                out.push_str("}\n\n");
            }
            MergedType::StringEnum { variants } => {
                out.push_str("#[non_exhaustive]\n");
                out.push_str(
                    "#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]\n",
                );
                out.push_str(&format!("pub enum {name} {{\n"));
                for wire in variants {
                    let pascal = wire_to_pascal(wire);
                    out.push_str(&format!("    #[serde(rename = \"{wire}\")]\n"));
                    out.push_str(&format!("    {pascal},\n"));
                }
                out.push_str("}\n\n");

                // Display impl — outputs the wire value
                out.push_str(&format!("impl std::fmt::Display for {name} {{\n"));
                out.push_str(
                    "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n",
                );
                out.push_str("        match self {\n");
                for wire in variants {
                    let pascal = wire_to_pascal(wire);
                    out.push_str(&format!(
                        "            {name}::{pascal} => write!(f, \"{wire}\"),\n"
                    ));
                }
                out.push_str("        }\n");
                out.push_str("    }\n");
                out.push_str("}\n\n");
            }
        }
    }

    out
}

/// Returns a map of type_name -> sorted field names (rust_name) for all merged types.
/// Dto types get their field names; Entity and StringEnum types get empty field lists
/// (their conversions are handled differently but they still need From impls).
pub fn collect_merged_field_names(specs: &[(&str, &ApiSpec)]) -> BTreeMap<String, Vec<String>> {
    let merged = merge_all_types(specs);
    let mut result = BTreeMap::new();
    for (name, mt) in merged {
        match mt {
            MergedType::Dto { fields, .. } => {
                result.insert(name, fields.keys().cloned().collect());
            }
            MergedType::Entity { .. } | MergedType::StringEnum { .. } => {
                result.insert(name, vec![]);
            }
        }
    }
    result
}

/// Returns the set of field names that are "universal" for a given type:
/// present in every version with the same Rust type string.
pub fn collect_universal_fields(specs: &[(&str, &ApiSpec)]) -> BTreeMap<String, BTreeSet<String>> {
    let merged = merge_all_types(specs);
    let mut result = BTreeMap::new();
    for (name, mt) in merged {
        if let MergedType::Dto {
            universal_fields, ..
        } = mt
        {
            result.insert(name, universal_fields);
        }
    }
    result
}

fn merge_all_types(specs: &[(&str, &ApiSpec)]) -> BTreeMap<String, MergedType> {
    let mut merged: BTreeMap<String, MergedType> = BTreeMap::new();

    // Pass 1: collect all fields from all versions (union)
    // Also track per-type: which versions contain this type, and per-field: presence count + type string
    let num_versions = specs.len();

    // Track: type_name -> (field_name -> (presence_count, rust_type_string, consistent))
    let mut field_presence: BTreeMap<String, BTreeMap<String, (usize, String, bool)>> =
        BTreeMap::new();
    // Track: type_name -> number of versions this type appears in
    let mut type_version_count: BTreeMap<String, usize> = BTreeMap::new();

    for (_version, spec) in specs {
        for td in &spec.all_types {
            match &td.kind {
                TypeKind::Dto => {
                    *type_version_count.entry(td.name.clone()).or_default() += 1;

                    let entry = merged
                        .entry(td.name.clone())
                        .or_insert_with(|| MergedType::Dto {
                            fields: BTreeMap::new(),
                            universal_fields: BTreeSet::new(),
                            doc: td.doc.clone(),
                        });
                    if let MergedType::Dto { fields, .. } = entry {
                        for field in &td.fields {
                            fields
                                .entry(field.rust_name.clone())
                                .or_insert_with(|| field.clone());
                        }
                    }

                    // Track field presence and type consistency
                    let type_fields = field_presence.entry(td.name.clone()).or_default();
                    for field in &td.fields {
                        let rust_ty = field_type_to_rust(&field.ty, &td.name);
                        let entry = type_fields
                            .entry(field.rust_name.clone())
                            .or_insert_with(|| (0, rust_ty.clone(), true));
                        entry.0 += 1;
                        if entry.1 != rust_ty {
                            entry.2 = false; // type mismatch
                        }
                    }
                }
                TypeKind::Entity { field, inner } => {
                    merged
                        .entry(td.name.clone())
                        .or_insert_with(|| MergedType::Entity {
                            field: field.clone(),
                            inner: inner.clone(),
                        });
                }
                TypeKind::StringEnum(variants) => {
                    let entry =
                        merged
                            .entry(td.name.clone())
                            .or_insert_with(|| MergedType::StringEnum {
                                variants: BTreeSet::new(),
                            });
                    if let MergedType::StringEnum { variants: existing } = entry {
                        for v in variants {
                            existing.insert(v.clone());
                        }
                    }
                }
            }
        }
    }

    // Pass 2: compute universal_fields for each Dto type.
    // A field is universal if:
    // 1. The type itself exists in all versions (type_version_count == num_versions)
    // 2. The field appears in every version where the type exists
    // 3. The Rust type string is consistent across all versions
    for (type_name, mt) in merged.iter_mut() {
        if let MergedType::Dto {
            universal_fields, ..
        } = mt
        {
            let versions_with_type = type_version_count.get(type_name).copied().unwrap_or(0);
            if versions_with_type < num_versions {
                // Type doesn't exist in all versions — no fields can be universal
                continue;
            }
            if let Some(fields_info) = field_presence.get(type_name) {
                for (field_name, (count, _ty, consistent)) in fields_info {
                    if *count == versions_with_type && *consistent {
                        universal_fields.insert(field_name.clone());
                    }
                }
            }
        }
    }

    merged
}

fn escape_keyword(name: &str) -> String {
    match name {
        "type" | "ref" | "use" | "mod" | "fn" | "let" | "match" | "for" | "if" | "else"
        | "return" | "struct" | "enum" | "impl" | "trait" | "pub" | "super" | "self" | "crate"
        | "where" | "true" | "false" | "in" | "loop" | "while" | "break" | "continue" | "mut"
        | "move" | "async" | "await" | "dyn" | "box" | "const" | "static" | "extern" | "unsafe"
        | "as" => format!("r#{name}"),
        _ => name.to_string(),
    }
}

fn field_type_to_rust(ty: &FieldType, struct_name: &str) -> String {
    match ty {
        FieldType::Str => "String".to_string(),
        FieldType::Bool => "bool".to_string(),
        FieldType::I32 => "i32".to_string(),
        FieldType::I64 => "i64".to_string(),
        FieldType::F64 => "f64".to_string(),
        FieldType::Opt(inner) => format!("Option<{}>", field_type_to_rust(inner, struct_name)),
        FieldType::List(inner) => format!("Vec<{}>", field_type_to_rust(inner, struct_name)),
        FieldType::Enum(_) => "String".to_string(),
        FieldType::Ref(name) => {
            if name == struct_name {
                format!("Box<{name}>")
            } else {
                name.clone()
            }
        }
        FieldType::Map(inner) => {
            format!(
                "std::collections::HashMap<String, {}>",
                field_type_to_rust(inner, struct_name)
            )
        }
    }
}

/// Wrap the type in `Option<T>` unless it is already `Option<T>` (i.e. `FieldType::Opt`).
fn wrap_in_option(ty: &FieldType, rust_str: &str) -> String {
    match ty {
        FieldType::Opt(_) => rust_str.to_string(),
        _ => format!("Option<{rust_str}>"),
    }
}

/// Convert a SCREAMING_SNAKE wire value to PascalCase.
/// `KEEP_EXISTING` -> `KeepExisting`, `JVM` -> `Jvm`
fn wire_to_pascal(wire: &str) -> String {
    wire.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let mut s = c.to_uppercase().to_string();
                    s.extend(chars.map(|ch| ch.to_ascii_lowercase()));
                    s
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{ApiSpec, Field, FieldType, TypeDef, TypeKind};

    fn make_spec(types: Vec<TypeDef>) -> ApiSpec {
        ApiSpec {
            tags: vec![],
            all_types: types,
        }
    }

    fn make_field(name: &str, ty: FieldType) -> Field {
        Field {
            rust_name: name.to_string(),
            serde_name: name.to_string(),
            ty,
            doc: None,
            read_only: false,
        }
    }

    #[test]
    fn test_merge_identical_types() {
        let td = TypeDef {
            name: "AboutDto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![
                make_field("title", FieldType::Opt(Box::new(FieldType::Str))),
                make_field("version", FieldType::Opt(Box::new(FieldType::Str))),
            ],
            doc: None,
        };
        let spec_a = make_spec(vec![td.clone()]);
        let spec_b = make_spec(vec![td]);
        let output = emit_dynamic_types(&[("2.7.2", &spec_a), ("2.8.0", &spec_b)]);
        assert!(output.contains("pub struct AboutDto"));
        assert!(output.contains("pub title: Option<String>"));
        assert!(output.contains("pub version: Option<String>"));
    }

    #[test]
    fn test_merge_union_adds_new_field() {
        let td_old = TypeDef {
            name: "ProcessorDto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![make_field("name", FieldType::Opt(Box::new(FieldType::Str)))],
            doc: None,
        };
        let td_new = TypeDef {
            name: "ProcessorDto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![
                make_field("name", FieldType::Opt(Box::new(FieldType::Str))),
                make_field("new_field", FieldType::Opt(Box::new(FieldType::Bool))),
            ],
            doc: None,
        };
        let spec_a = make_spec(vec![td_old]);
        let spec_b = make_spec(vec![td_new]);
        let output = emit_dynamic_types(&[("2.7.2", &spec_a), ("2.8.0", &spec_b)]);
        assert!(output.contains("pub struct ProcessorDto"));
        assert!(output.contains("pub name: Option<String>"));
        assert!(output.contains("pub new_field: Option<bool>"));
    }

    #[test]
    fn test_merge_string_enum_union() {
        let td_old = TypeDef {
            name: "IncludedRegistries".to_string(),
            kind: TypeKind::StringEnum(vec!["NIFI".into(), "JVM".into()]),
            fields: vec![],
            doc: None,
        };
        let td_new = TypeDef {
            name: "IncludedRegistries".to_string(),
            kind: TypeKind::StringEnum(vec!["NIFI".into(), "JVM".into(), "VERSION_INFO".into()]),
            fields: vec![],
            doc: None,
        };
        let spec_a = make_spec(vec![td_old]);
        let spec_b = make_spec(vec![td_new]);
        let output = emit_dynamic_types(&[("2.7.2", &spec_a), ("2.8.0", &spec_b)]);
        assert!(output.contains("pub enum IncludedRegistries"));
        assert!(output.contains("Nifi"));
        assert!(output.contains("Jvm"));
        assert!(output.contains("VersionInfo"));
    }

    #[test]
    fn test_type_only_in_one_version() {
        let td = TypeDef {
            name: "NewInV2Dto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![make_field("id", FieldType::Opt(Box::new(FieldType::Str)))],
            doc: None,
        };
        let spec_a = make_spec(vec![]);
        let spec_b = make_spec(vec![td]);
        let output = emit_dynamic_types(&[("2.7.2", &spec_a), ("2.8.0", &spec_b)]);
        assert!(output.contains("pub struct NewInV2Dto"));
        assert!(output.contains("pub id: Option<String>"));
    }

    #[test]
    fn test_collect_merged_field_names() {
        let td_a = TypeDef {
            name: "Dto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![make_field("a", FieldType::Str)],
            doc: None,
        };
        let td_b = TypeDef {
            name: "Dto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![
                make_field("a", FieldType::Str),
                make_field("b", FieldType::Bool),
            ],
            doc: None,
        };
        let spec_a = make_spec(vec![td_a]);
        let spec_b = make_spec(vec![td_b]);
        let result = collect_merged_field_names(&[("2.7.2", &spec_a), ("2.8.0", &spec_b)]);
        assert_eq!(
            result.get("Dto").unwrap(),
            &vec!["a".to_string(), "b".to_string()]
        );
    }

    fn make_field_with_doc(name: &str, ty: FieldType, doc: &str) -> Field {
        Field {
            rust_name: name.to_string(),
            serde_name: name.to_string(),
            ty,
            doc: Some(doc.to_string()),
            read_only: false,
        }
    }

    #[test]
    fn dynamic_types_emit_non_exhaustive_on_struct() {
        let td = TypeDef {
            name: "AboutDto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![
                make_field_with_doc("version", FieldType::Opt(Box::new(FieldType::Str)), "The NiFi version"),
                make_field("title", FieldType::Str),
            ],
            doc: Some("Information about the NiFi instance".to_string()),
        };
        let spec = make_spec(vec![td]);
        let output = emit_dynamic_types(&[("2.8.0", &spec)]);
        // non_exhaustive attribute on struct
        assert!(output.contains("#[non_exhaustive]"), "missing #[non_exhaustive] on struct");
        // struct-level doc comment
        assert!(output.contains("/// Information about the NiFi instance"), "missing struct doc comment");
        // field-level doc comment
        assert!(output.contains("/// The NiFi version"), "missing field doc comment");
        // skip_serializing_if on Option fields
        assert!(output.contains("#[serde(skip_serializing_if = \"Option::is_none\")]"), "missing skip_serializing_if");
    }

    #[test]
    fn dynamic_types_emit_non_exhaustive_on_enum() {
        let td = TypeDef {
            name: "DiagnosticLevel".to_string(),
            kind: TypeKind::StringEnum(vec!["BASIC".into(), "VERBOSE".into()]),
            fields: vec![],
            doc: None,
        };
        let spec = make_spec(vec![td]);
        let output = emit_dynamic_types(&[("2.8.0", &spec)]);
        assert!(output.contains("pub enum DiagnosticLevel"), "missing enum declaration");
        assert!(output.contains("#[non_exhaustive]"), "missing #[non_exhaustive] on enum");
    }
}
