use std::collections::BTreeMap;

use crate::parser::{ApiSpec, FieldType, TypeKind};

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

/// Determine if a field type needs `.into()` conversion (i.e., contains a Ref or Enum).
fn needs_conversion(ty: &FieldType) -> bool {
    match ty {
        FieldType::Ref(_) | FieldType::Enum(_) => true,
        FieldType::Opt(inner) | FieldType::List(inner) | FieldType::Map(inner) => {
            needs_conversion(inner)
        }
        _ => false,
    }
}

/// Generate the conversion expression for a field from a per-version type to the dynamic type.
/// All dynamic fields are `Option<T>`, so non-Optional per-version fields get wrapped in `Some(...)`.
fn field_conversion_expr(escaped_name: &str, ty: &FieldType, struct_name: &str) -> String {
    match ty {
        FieldType::Opt(_) => {
            // Already optional — convert the inner value
            if !needs_conversion(ty) {
                format!("v.{escaped_name}")
            } else {
                convert_expr(&format!("v.{escaped_name}"), ty, struct_name)
            }
        }
        _ => {
            // Non-optional in the per-version type → wrap in Some for the dynamic type
            if !needs_conversion(ty) {
                format!("Some(v.{escaped_name})")
            } else {
                let converted = convert_expr(&format!("v.{escaped_name}"), ty, struct_name);
                format!("Some({converted})")
            }
        }
    }
}

fn convert_expr(expr: &str, ty: &FieldType, struct_name: &str) -> String {
    match ty {
        FieldType::Ref(name) if name == struct_name => {
            // Self-referential field — wrapped in Box in the dynamic type
            format!("{expr}.map(|v| Box::new((*v).into()))")
        }
        FieldType::Ref(_) => format!("{expr}.into()"),
        FieldType::Enum(_) => {
            // Inline string enums are emitted as String in the dynamic type;
            // use serde round-trip since per-version enums may not impl Display
            format!(
                "serde_json::to_value(&{expr}).ok().and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default()"
            )
        }
        FieldType::Opt(inner) => {
            if needs_conversion(inner) {
                match inner.as_ref() {
                    FieldType::Ref(name) if name == struct_name => {
                        format!("{expr}.map(|v| Box::new((*v).into()))")
                    }
                    FieldType::Ref(_) => format!("{expr}.map(Into::into)"),
                    FieldType::Enum(_) => {
                        format!(
                            "{expr}.map(|v| serde_json::to_value(&v).ok().and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default())"
                        )
                    }
                    FieldType::List(item) if needs_conversion(item) => {
                        let list_conv =
                            convert_expr("v", &FieldType::List(item.clone()), struct_name);
                        format!("{expr}.map(|v| {list_conv})")
                    }
                    FieldType::Map(val) if needs_conversion(val) => {
                        let val_conversion = convert_expr("v", val, struct_name);
                        format!(
                            "{expr}.map(|m| m.into_iter().map(|(k, v)| (k, {val_conversion})).collect())"
                        )
                    }
                    _ => format!("{expr}.map(Into::into)"),
                }
            } else {
                expr.to_string()
            }
        }
        FieldType::List(inner) if needs_conversion(inner) => match inner.as_ref() {
            FieldType::Enum(_) => {
                format!(
                    "{expr}.into_iter().map(|v| serde_json::to_value(&v).ok().and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default()).collect()"
                )
            }
            FieldType::Ref(name) if name == struct_name => {
                format!("{expr}.into_iter().map(|v| Box::new((*v).into())).collect()")
            }
            _ => {
                let item_conversion = convert_expr("v", inner, struct_name);
                format!("{expr}.into_iter().map(|v| {item_conversion}).collect()")
            }
        },
        FieldType::Map(inner) if needs_conversion(inner) => {
            let val_conversion = convert_expr("v", inner, struct_name);
            format!("{expr}.into_iter().map(|(k, v)| (k, {val_conversion})).collect()")
        }
        _ => expr.to_string(),
    }
}

/// Generates From impls converting each version's types into the common dynamic types.
/// Returns the content for `dynamic/conversions.rs`.
///
/// `specs` is `[(version_str, mod_name, spec)]` — e.g. `("2.7.2", "v2_7_2", &spec)`.
/// `merged_type_names` maps type_name → ordered list of all field names in the merged type.
pub fn emit_dynamic_conversions(
    specs: &[(&str, &str, &ApiSpec)],
    merged_type_names: &BTreeMap<String, Vec<String>>,
) -> String {
    let mut out = String::new();
    out.push_str("#![allow(clippy::useless_conversion)]\n\n");

    for (_version, mod_name, spec) in specs {
        let type_map: BTreeMap<&str, &TypeKind> = spec
            .all_types
            .iter()
            .map(|td| (td.name.as_str(), &td.kind))
            .collect();

        // Build a lookup from (type_name, field_rust_name) -> &FieldType
        let field_lookup: BTreeMap<&str, BTreeMap<&str, &FieldType>> = spec
            .all_types
            .iter()
            .map(|td| {
                let fields: BTreeMap<&str, &FieldType> = td
                    .fields
                    .iter()
                    .map(|f| (f.rust_name.as_str(), &f.ty))
                    .collect();
                (td.name.as_str(), fields)
            })
            .collect();

        for (type_name, merged_fields) in merged_type_names {
            let Some(kind) = type_map.get(type_name.as_str()) else {
                continue;
            };

            out.push_str(&format!(
                "impl From<crate::{}::types::{}> for super::types::{} {{\n",
                mod_name, type_name, type_name
            ));
            // Use _v for Dto types with no fields in this version (avoids unused variable warning)
            let uses_param = match kind {
                TypeKind::Dto => field_lookup
                    .get(type_name.as_str())
                    .is_some_and(|f| !f.is_empty()),
                _ => true, // Entity and StringEnum always use the parameter
            };
            let param_name = if uses_param { "v" } else { "_v" };
            out.push_str(&format!(
                "    fn from({param_name}: crate::{}::types::{}) -> Self {{\n",
                mod_name, type_name
            ));

            match kind {
                TypeKind::Dto => {
                    let version_fields = field_lookup.get(type_name.as_str());
                    out.push_str("        Self {\n");
                    for field_name in merged_fields {
                        let escaped = escape_keyword(field_name);
                        let field_ty =
                            version_fields.and_then(|f| f.get(field_name.as_str()).copied());
                        match field_ty {
                            Some(ty) => {
                                let conversion = field_conversion_expr(&escaped, ty, type_name);
                                out.push_str(&format!("            {escaped}: {conversion},\n"));
                            }
                            None => {
                                // Field not present in this version
                                out.push_str(&format!("            {escaped}: None,\n"));
                            }
                        }
                    }
                    out.push_str("        }\n");
                }
                TypeKind::Entity { field, inner: _ } => {
                    let escaped_field = escape_keyword(field);
                    // Per-version entity has non-Optional inner field; dynamic has Option
                    out.push_str(&format!(
                        "        Self {{ {escaped_field}: Some(v.{escaped_field}.into()) }}\n"
                    ));
                }
                TypeKind::StringEnum(_) => {
                    out.push_str(
                        "        let s = serde_json::to_string(&v).expect(\"serialize enum\");\n",
                    );
                    out.push_str("        serde_json::from_str(&s).expect(\"deserialize enum\")\n");
                }
            }

            out.push_str("    }\n");
            out.push_str("}\n\n");
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{ApiSpec, Field, FieldType, TypeDef, TypeKind};

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
    fn test_from_impl_maps_matching_fields() {
        let td = TypeDef {
            name: "AboutDto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![make_field(
                "title",
                FieldType::Opt(Box::new(FieldType::Str)),
            )],
            doc: None,
        };
        let spec = ApiSpec {
            tags: vec![],
            all_types: vec![td],
        };

        let mut merged = BTreeMap::new();
        merged.insert(
            "AboutDto".to_string(),
            vec!["title".to_string(), "version".to_string()],
        );

        let output = emit_dynamic_conversions(&[("2.7.2", "v2_7_2", &spec)], &merged);
        assert!(
            output.contains("impl From<crate::v2_7_2::types::AboutDto> for super::types::AboutDto")
        );
        assert!(output.contains("title: v.title"));
        assert!(output.contains("version: None"));
    }

    #[test]
    fn test_from_impl_string_enum() {
        let td = TypeDef {
            name: "MyEnum".to_string(),
            kind: TypeKind::StringEnum(vec!["A".into(), "B".into()]),
            fields: vec![],
            doc: None,
        };
        let spec = ApiSpec {
            tags: vec![],
            all_types: vec![td],
        };
        let mut merged = BTreeMap::new();
        merged.insert("MyEnum".to_string(), vec![]);

        let output = emit_dynamic_conversions(&[("2.8.0", "v2_8_0", &spec)], &merged);
        assert!(
            output.contains("impl From<crate::v2_8_0::types::MyEnum> for super::types::MyEnum")
        );
        assert!(output.contains("serde_json"));
    }

    #[test]
    fn test_type_not_in_version_skipped() {
        let spec = ApiSpec {
            tags: vec![],
            all_types: vec![],
        };
        let mut merged = BTreeMap::new();
        merged.insert("MissingType".to_string(), vec!["field".to_string()]);

        let output = emit_dynamic_conversions(&[("2.7.2", "v2_7_2", &spec)], &merged);
        assert!(!output.contains("MissingType"));
    }
}
