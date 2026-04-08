use std::collections::{BTreeMap, BTreeSet};

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
/// `serde_json::Value` refs are opaque and don't need conversion.
fn needs_conversion(ty: &FieldType) -> bool {
    match ty {
        FieldType::Ref(name) => name != "serde_json::Value",
        FieldType::Enum(_) => true,
        FieldType::Opt(inner) | FieldType::List(inner) | FieldType::Map(inner) => {
            needs_conversion(inner)
        }
        _ => false,
    }
}

/// Generate the conversion expression for a field from a per-version type to the dynamic type.
/// If `is_universal` is true, the dynamic field has the same type as the per-version field
/// (no extra Option wrapping). Otherwise, non-Optional per-version fields get wrapped in `Some(...)`.
fn field_conversion_expr(
    escaped_name: &str,
    ty: &FieldType,
    struct_name: &str,
    is_universal: bool,
) -> String {
    match ty {
        FieldType::Opt(_) => {
            // Already optional — convert the inner value
            if !needs_conversion(ty) {
                format!("v.{escaped_name}")
            } else {
                convert_expr(&format!("v.{escaped_name}"), ty, struct_name)
            }
        }
        _ if is_universal => {
            // Universal field: same type in dynamic struct, no Some wrapping needed
            if !needs_conversion(ty) {
                format!("v.{escaped_name}")
            } else {
                convert_expr(&format!("v.{escaped_name}"), ty, struct_name)
            }
        }
        _ => {
            // Non-universal, non-optional in the per-version type → wrap in Some for the dynamic type
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
            format!("enum_to_string(&{expr})")
        }
        FieldType::Opt(inner) => {
            if needs_conversion(inner) {
                match inner.as_ref() {
                    FieldType::Ref(name) if name == struct_name => {
                        format!("{expr}.map(|v| Box::new((*v).into()))")
                    }
                    FieldType::Ref(_) => format!("{expr}.map(Into::into)"),
                    FieldType::Enum(_) => {
                        format!("{expr}.map(|v| enum_to_string(&v))")
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
                format!("{expr}.into_iter().map(|v| enum_to_string(&v)).collect()")
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
/// Returns a `Vec<(filename, content)>` — one entry per version (e.g. `"v2_7_2.rs"`) plus a
/// `"mod.rs"` that declares all version sub-modules.
///
/// `specs` is `[(version_str, mod_name, spec)]` — e.g. `("2.7.2", "v2_7_2", &spec)`.
/// `merged_type_names` maps type_name → ordered list of all field names in the merged type.
/// `universal_fields` maps type_name → set of field names present in all versions with same type.
pub fn emit_dynamic_conversions(
    specs: &[(&str, &str, &ApiSpec)],
    merged_type_names: &BTreeMap<String, Vec<String>>,
    universal_fields: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = Vec::new();

    for (_version, mod_name, spec) in specs {
        let mut out = String::new();
        out.push_str("#![allow(clippy::useless_conversion, clippy::redundant_closure)]\n\n");

        // Helper to convert a serde-serializable enum to its wire string representation.
        out.push_str("fn enum_to_string(v: &impl serde::Serialize) -> String {\n");
        out.push_str("    serde_json::to_value(v)\n");
        out.push_str("        .ok()\n");
        out.push_str("        .and_then(|v| v.as_str().map(|s| s.to_string()))\n");
        out.push_str("        .unwrap_or_default()\n");
        out.push_str("}\n\n");

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
                "impl From<crate::{}::types::{}> for super::super::types::{} {{\n",
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

            let uf = universal_fields
                .get(type_name.as_str())
                .cloned()
                .unwrap_or_default();

            match kind {
                TypeKind::Dto => {
                    let version_fields = field_lookup.get(type_name.as_str());
                    out.push_str("        Self {\n");
                    for field_name in merged_fields {
                        let escaped = escape_keyword(field_name);
                        let is_universal = uf.contains(field_name.as_str());
                        let field_ty =
                            version_fields.and_then(|f| f.get(field_name.as_str()).copied());
                        match field_ty {
                            Some(ty) => {
                                let conversion =
                                    field_conversion_expr(&escaped, ty, type_name, is_universal);
                                out.push_str(&format!("            {escaped}: {conversion},\n"));
                            }
                            None => {
                                // Field not present in this version — must be non-universal (Option)
                                out.push_str(&format!("            {escaped}: None,\n"));
                            }
                        }
                    }
                    out.push_str("        }\n");
                }
                TypeKind::Entity { field, inner: _ } => {
                    let escaped_field = escape_keyword(field);
                    // Per-version entity inner field is Option<T>; unwrap with unwrap_or_default
                    // before converting so we don't need From<Option<T>>.
                    out.push_str(&format!(
                        "        Self {{ {escaped_field}: Some(v.{escaped_field}.unwrap_or_default().into()) }}\n"
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

        // Generate TryFrom<dynamic_enum> -> version_enum (narrowing) for each StringEnum.
        emit_enum_narrowing_conversions_for_version(&mut out, _version, mod_name, spec);

        // Generate TryFrom<dynamic_dto> -> version_dto (narrowing) for request body types.
        emit_dto_narrowing_conversions_for_version(
            &mut out,
            _version,
            mod_name,
            spec,
            merged_type_names,
            universal_fields,
        );

        files.push((format!("{mod_name}.rs"), out));
    }

    // Emit mod.rs declaring all version sub-modules.
    let mut mod_rs = String::new();
    for (_version, mod_name, _spec) in specs {
        mod_rs.push_str(&format!("pub(super) mod {mod_name};\n"));
    }
    files.push(("mod.rs".to_string(), mod_rs));

    files
}

/// Convert a SCREAMING_SNAKE wire value to PascalCase.
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

/// Emit TryFrom<dynamic_enum> -> version_enum narrowing conversions for a single version.
fn emit_enum_narrowing_conversions_for_version(
    out: &mut String,
    version: &str,
    mod_name: &str,
    spec: &ApiSpec,
) {
    // Collect the union of all variants for enums in this version's spec (single-version view).
    // The narrowing still needs to match all variants in the dynamic (merged) enum, so we re-use
    // the full merged list when it is available. Since we only have a single spec here, the
    // merged variants are the same as the version's variants — any extra variants from other
    // versions are unknown to this file. To handle this correctly the caller should pass the
    // merged variants; for now we use the version's own variants as the match arms (all Ok).
    for td in &spec.all_types {
        let TypeKind::StringEnum(version_variants) = &td.kind else {
            continue;
        };
        let type_name = &td.name;
        let version_set: BTreeSet<&str> = version_variants.iter().map(|s| s.as_str()).collect();

        // TryFrom<dynamic_enum> for version_enum (narrowing)
        out.push_str(&format!(
            "impl TryFrom<super::super::types::{type_name}> for crate::{mod_name}::types::{type_name} {{\n"
        ));
        out.push_str("    type Error = crate::NifiError;\n");
        out.push_str(&format!(
            "    fn try_from(v: super::super::types::{type_name}) -> Result<Self, Self::Error> {{\n"
        ));
        out.push_str("        #[allow(unreachable_patterns)]\n");
        out.push_str("        match v {\n");
        for wire in version_set.iter() {
            let pascal = wire_to_pascal(wire);
            out.push_str(&format!(
                "            super::super::types::{type_name}::{pascal} => Ok(crate::{mod_name}::types::{type_name}::{pascal}),\n"
            ));
            let _ = version; // used below for non-matching variants
        }
        // Catch-all for dynamic variants not present in this version (emitted as unreachable arms
        // for variants that are in the merged enum but not this version — handled via a wildcard).
        out.push_str(&format!(
            "            _ => Err(crate::NifiError::UnsupportedEnumVariant {{ variant: format!(\"{{v:?}}\"), type_name: \"{type_name}\".to_string(), version: \"{version}\".to_string() }}),\n"
        ));
        out.push_str("        }\n");
        out.push_str("    }\n");
        out.push_str("}\n\n");
    }
}

/// Emit TryFrom<dynamic_dto> -> version_dto narrowing conversions for a single version.
fn emit_dto_narrowing_conversions_for_version(
    out: &mut String,
    version: &str,
    mod_name: &str,
    spec: &ApiSpec,
    merged_type_names: &BTreeMap<String, Vec<String>>,
    universal_fields: &BTreeMap<String, BTreeSet<String>>,
) {
    // Build field lookup for this version
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

    let type_map: BTreeMap<&str, &TypeKind> = spec
        .all_types
        .iter()
        .map(|td| (td.name.as_str(), &td.kind))
        .collect();

    for (type_name, merged_fields) in merged_type_names {
        let Some(kind) = type_map.get(type_name.as_str()) else {
            continue;
        };

        // Only generate TryFrom for Dto and Entity types (not StringEnum — those are already handled)
        match kind {
            TypeKind::Dto => {}
            TypeKind::Entity { .. } => {
                // Entity narrowing: unwrap Option inner, convert via TryFrom
                let TypeKind::Entity { field, inner } = kind else {
                    unreachable!()
                };
                let escaped_field = escape_keyword(field);
                out.push_str(&format!(
                    "impl TryFrom<super::super::types::{type_name}> for crate::{mod_name}::types::{type_name} {{\n"
                ));
                out.push_str("    type Error = crate::NifiError;\n");
                out.push_str(&format!(
                    "    fn try_from(v: super::super::types::{type_name}) -> Result<Self, Self::Error> {{\n"
                ));
                out.push_str(&format!(
                    "        Ok(Self {{ {escaped_field}: v.{escaped_field}.map(crate::{mod_name}::types::{inner}::try_from).transpose()? }})\n"
                ));
                out.push_str("    }\n");
                out.push_str("}\n\n");
                continue;
            }
            TypeKind::StringEnum(_) => continue,
        }

        let version_fields = match field_lookup.get(type_name.as_str()) {
            Some(f) => f,
            None => continue,
        };

        let uf = universal_fields
            .get(type_name.as_str())
            .cloned()
            .unwrap_or_default();

        out.push_str(&format!(
            "impl TryFrom<super::super::types::{type_name}> for crate::{mod_name}::types::{type_name} {{\n"
        ));
        out.push_str("    type Error = crate::NifiError;\n");
        // Use _v if the version type has no fields to avoid unused variable warning
        let param_name = if version_fields.is_empty() { "_v" } else { "v" };
        out.push_str(&format!(
            "    fn try_from({param_name}: super::super::types::{type_name}) -> Result<Self, Self::Error> {{\n"
        ));
        out.push_str("        Ok(Self {\n");

        // Iterate over the VERSION's fields (not merged fields), since we're constructing the version type
        for (field_name, field_ty) in version_fields.iter() {
            let escaped = escape_keyword(field_name);
            let is_universal = uf.contains(*field_name);

            // Check if this field exists in the merged type (it should, since merged is a superset)
            if !merged_fields.contains(&field_name.to_string()) {
                // Field in version but not in merged type — shouldn't happen
                continue;
            }

            let expr = narrowing_field_expr(&escaped, field_ty, type_name, is_universal, version);
            out.push_str(&format!("            {escaped}: {expr},\n"));
        }

        out.push_str("        })\n");
        out.push_str("    }\n");
        out.push_str("}\n\n");
    }
}

/// Generate the narrowing conversion expression for a single field.
/// This converts from dynamic union type → version-specific type.
fn narrowing_field_expr(
    escaped_name: &str,
    version_ty: &FieldType,
    struct_name: &str,
    is_universal: bool,
    version: &str,
) -> String {
    match version_ty {
        FieldType::Opt(_) => {
            // Version field is optional — dynamic field is also optional (same or Option-wrapped)
            narrowing_convert(&format!("v.{escaped_name}"), version_ty, struct_name)
        }
        _ if is_universal => {
            // Universal field: same type in dynamic struct, direct assignment
            narrowing_convert(&format!("v.{escaped_name}"), version_ty, struct_name)
        }
        _ => {
            // Non-universal, required in version but Option<T> in dynamic.
            // Need to unwrap with MissingRequiredField error, then convert the inner value.
            if needs_conversion(version_ty) {
                let inner_conv = narrowing_convert_fallible("v", version_ty, struct_name);
                format!(
                    "v.{escaped_name}.map(|v| {inner_conv}).ok_or_else(|| crate::NifiError::MissingRequiredField {{ field: \"{escaped_name}\".to_string(), type_name: \"{struct_name}\".to_string(), version: \"{version}\".to_string() }})??",
                )
            } else {
                format!(
                    "v.{escaped_name}.ok_or_else(|| crate::NifiError::MissingRequiredField {{ field: \"{escaped_name}\".to_string(), type_name: \"{struct_name}\".to_string(), version: \"{version}\".to_string() }})?",
                )
            }
        }
    }
}

/// Generate a narrowing conversion expression for an arbitrary FieldType.
/// Handles nesting recursively (Opt, List, Map of any depth).
fn narrowing_convert(expr: &str, ty: &FieldType, struct_name: &str) -> String {
    if !needs_conversion(ty) {
        return expr.to_string();
    }
    match ty {
        FieldType::Ref(name) if name == struct_name => {
            // Self-referential Box<T>: unbox, try_into, rebox
            format!("(*{expr}).try_into().map(Box::new)?")
        }
        FieldType::Ref(_) => format!("{expr}.try_into()?"),
        FieldType::Enum(_) => {
            // Dynamic has String, version has enum — parse via serde
            format!(
                "serde_json::from_value(serde_json::Value::String({expr})).map_err(|e| crate::NifiError::Api {{ status: 0, message: format!(\"enum parse: {{}}\", e) }})?",
            )
        }
        FieldType::Opt(inner) => {
            if !needs_conversion(inner) {
                return expr.to_string();
            }
            // Generate a fallible closure: .map(|v| -> Result<T>) then .transpose()?
            let inner_conv = narrowing_convert_fallible("v", inner, struct_name);
            format!("{expr}.map(|v| {inner_conv}).transpose()?")
        }
        FieldType::List(inner) => {
            if !needs_conversion(inner) {
                return expr.to_string();
            }
            let inner_conv = narrowing_convert_fallible("v", inner, struct_name);
            format!("{expr}.into_iter().map(|v| {inner_conv}).collect::<Result<Vec<_>, _>>()?")
        }
        FieldType::Map(inner) => {
            if !needs_conversion(inner) {
                return expr.to_string();
            }
            let inner_conv = narrowing_convert_fallible("v", inner, struct_name);
            format!(
                "{expr}.into_iter().map(|(k, v)| Ok::<_, crate::NifiError>((k, {inner_conv}?))).collect::<Result<std::collections::HashMap<_, _>, _>>()?"
            )
        }
        _ => expr.to_string(),
    }
}

/// Like `narrowing_convert`, but always returns a `Result<T, NifiError>` expression.
/// Used inside closures where `.transpose()` or `.collect::<Result<...>>()` will handle the error.
fn narrowing_convert_fallible(expr: &str, ty: &FieldType, struct_name: &str) -> String {
    if !needs_conversion(ty) {
        return format!("Ok::<_, crate::NifiError>({expr})");
    }
    match ty {
        FieldType::Ref(name) if name == struct_name => {
            format!("(*{expr}).try_into().map(Box::new)")
        }
        FieldType::Ref(_) => format!("{expr}.try_into()"),
        FieldType::Enum(_) => {
            format!(
                "serde_json::from_value::<_>(serde_json::Value::String({expr})).map_err(|e| crate::NifiError::Api {{ status: 0, message: format!(\"enum parse: {{}}\", e) }})",
            )
        }
        FieldType::Opt(inner) => {
            if !needs_conversion(inner) {
                return format!("Ok::<_, crate::NifiError>({expr})");
            }
            let inner_conv = narrowing_convert_fallible("v", inner, struct_name);
            format!("{expr}.map(|v| {inner_conv}).transpose()")
        }
        FieldType::List(inner) => {
            if !needs_conversion(inner) {
                return format!("Ok::<_, crate::NifiError>({expr})");
            }
            let inner_conv = narrowing_convert_fallible("v", inner, struct_name);
            format!("{expr}.into_iter().map(|v| {inner_conv}).collect::<Result<Vec<_>, _>>()")
        }
        FieldType::Map(inner) => {
            if !needs_conversion(inner) {
                return format!("Ok::<_, crate::NifiError>({expr})");
            }
            let inner_conv = narrowing_convert_fallible("v", inner, struct_name);
            format!(
                "{expr}.into_iter().map(|(k, v)| Ok::<_, crate::NifiError>((k, {inner_conv}?))).collect::<Result<std::collections::HashMap<_, _>, _>>()"
            )
        }
        _ => format!("Ok::<_, crate::NifiError>({expr})"),
    }
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

    fn get_file<'a>(files: &'a [(String, String)], name: &str) -> &'a str {
        files
            .iter()
            .find(|(f, _)| f == name)
            .map(|(_, c)| c.as_str())
            .unwrap_or_else(|| panic!("file {name} not found in output"))
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

        let files =
            emit_dynamic_conversions(&[("2.7.2", "v2_7_2", &spec)], &merged, &BTreeMap::new());
        let output = get_file(&files, "v2_7_2.rs");
        assert!(output.contains(
            "impl From<crate::v2_7_2::types::AboutDto> for super::super::types::AboutDto"
        ));
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

        let files =
            emit_dynamic_conversions(&[("2.8.0", "v2_8_0", &spec)], &merged, &BTreeMap::new());
        let output = get_file(&files, "v2_8_0.rs");
        assert!(
            output.contains(
                "impl From<crate::v2_8_0::types::MyEnum> for super::super::types::MyEnum"
            )
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

        let files =
            emit_dynamic_conversions(&[("2.7.2", "v2_7_2", &spec)], &merged, &BTreeMap::new());
        let output = get_file(&files, "v2_7_2.rs");
        assert!(!output.contains("MissingType"));
    }

    #[test]
    fn dynamic_conversions_split_per_version() {
        let td_v1 = TypeDef {
            name: "AboutDto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![make_field(
                "title",
                FieldType::Opt(Box::new(FieldType::Str)),
            )],
            doc: None,
        };
        let spec_v1 = ApiSpec {
            tags: vec![],
            all_types: vec![td_v1],
        };

        let td_v2 = TypeDef {
            name: "AboutDto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![
                make_field("title", FieldType::Opt(Box::new(FieldType::Str))),
                make_field("version", FieldType::Opt(Box::new(FieldType::Str))),
            ],
            doc: None,
        };
        let spec_v2 = ApiSpec {
            tags: vec![],
            all_types: vec![td_v2],
        };

        let mut merged = BTreeMap::new();
        merged.insert(
            "AboutDto".to_string(),
            vec!["title".to_string(), "version".to_string()],
        );

        let files = emit_dynamic_conversions(
            &[("2.7.2", "v2_7_2", &spec_v1), ("2.8.0", "v2_8_0", &spec_v2)],
            &merged,
            &BTreeMap::new(),
        );

        // All expected filenames are present
        let filenames: Vec<&str> = files.iter().map(|(f, _)| f.as_str()).collect();
        assert!(
            filenames.contains(&"mod.rs"),
            "expected mod.rs, got {filenames:?}"
        );
        assert!(filenames.contains(&"v2_7_2.rs"), "expected v2_7_2.rs");
        assert!(filenames.contains(&"v2_8_0.rs"), "expected v2_8_0.rs");

        // mod.rs declares both version modules
        let mod_rs = get_file(&files, "mod.rs");
        assert!(mod_rs.contains("pub(super) mod v2_7_2;"));
        assert!(mod_rs.contains("pub(super) mod v2_8_0;"));

        // Each version file contains From impl only for that version
        let v1 = get_file(&files, "v2_7_2.rs");
        assert!(v1.contains("impl From<crate::v2_7_2::types::AboutDto>"));
        assert!(!v1.contains("v2_8_0"));

        let v2 = get_file(&files, "v2_8_0.rs");
        assert!(v2.contains("impl From<crate::v2_8_0::types::AboutDto>"));
        assert!(!v2.contains("v2_7_2"));
    }
}
