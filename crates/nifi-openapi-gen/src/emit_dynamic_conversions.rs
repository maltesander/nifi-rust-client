use std::collections::BTreeMap;

use crate::parser::{ApiSpec, TypeKind};

/// Generates From impls converting each version's types into the common dynamic types.
/// Returns the content for `dynamic/conversions.rs`.
///
/// `specs` is `[(version_str, mod_name, spec)]` — e.g. `("2.7.2", "v2_7_2", &spec)`.
/// `merged_type_names` maps type_name → ordered list of all field names in the merged type.
pub fn emit_dynamic_conversions(
    specs: &[(&str, &str, &ApiSpec)],
    merged_type_names: &BTreeMap<String, Vec<String>>,
) -> String {
    let mut out = String::from(
        "// @generated — do not edit; run `cargo run -p nifi-openapi-gen`\n\n",
    );

    for (_version, mod_name, spec) in specs {
        let type_map: BTreeMap<&str, &TypeKind> = spec
            .all_types
            .iter()
            .map(|td| (td.name.as_str(), &td.kind))
            .collect();

        // Build a lookup from field rust_name to its presence for this version's types
        let field_lookup: BTreeMap<&str, BTreeMap<&str, bool>> = spec
            .all_types
            .iter()
            .map(|td| {
                let fields: BTreeMap<&str, bool> = td
                    .fields
                    .iter()
                    .map(|f| (f.rust_name.as_str(), true))
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
            out.push_str(&format!(
                "    fn from(v: crate::{}::types::{}) -> Self {{\n",
                mod_name, type_name
            ));

            match kind {
                TypeKind::Dto => {
                    let version_fields = field_lookup.get(type_name.as_str());
                    out.push_str("        Self {\n");
                    for field_name in merged_fields {
                        let present = version_fields
                            .map(|f| f.contains_key(field_name.as_str()))
                            .unwrap_or(false);
                        if present {
                            out.push_str(&format!("            {}: v.{},\n", field_name, field_name));
                        } else {
                            out.push_str(&format!("            {}: None,\n", field_name));
                        }
                    }
                    out.push_str("        }\n");
                }
                TypeKind::Entity { field, inner: _ } => {
                    out.push_str(&format!(
                        "        Self {{ {field}: v.{field}.into() }}\n"
                    ));
                }
                TypeKind::StringEnum(_) => {
                    out.push_str(
                        "        let s = serde_json::to_string(&v).expect(\"serialize enum\");\n",
                    );
                    out.push_str(
                        "        serde_json::from_str(&s).expect(\"deserialize enum\")\n",
                    );
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
        }
    }

    #[test]
    fn test_from_impl_maps_matching_fields() {
        let td = TypeDef {
            name: "AboutDto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![make_field("title", FieldType::Opt(Box::new(FieldType::Str)))],
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
        assert!(output.contains("impl From<crate::v2_7_2::types::AboutDto> for super::types::AboutDto"));
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
        assert!(output.contains("impl From<crate::v2_8_0::types::MyEnum> for super::types::MyEnum"));
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
