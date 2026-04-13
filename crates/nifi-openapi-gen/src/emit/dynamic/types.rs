//! Dynamic types emitter for `dynamic/types/*.rs`.
//!
//! Inlined from the legacy `emit/dynamic/types.rs` as part of Phase 4b, making
//! this module fully self-contained. The public entry point `emit_types` accepts
//! a `&CanonicalSpec` and delegates to `emit_types_from_specs` (the renamed body
//! of the legacy `emit_dynamic_types`). Semantics are preserved exactly.
//!
//! Phase 4b Task 4 will delete `emit/dynamic/types.rs`; this module is the
//! canonical replacement.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::canonical::CanonicalSpec;
use crate::emit::common::{InlineEnumMode, field_type_to_rust};
use crate::parser::{ApiSpec, Field, FieldType, TagGroup, TypeKind};
use crate::util::{escape_keyword, wire_to_pascal};

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

/// Entry point: merge type definitions from all API versions in the canonical spec
/// and emit per-tag Rust source files containing union types.
///
/// Returns a list of `(filename, content)` pairs to write into `src/dynamic/types/`:
/// - `"mod.rs"` — module declarations + glob re-exports
/// - `"common.rs"` — types referenced by 0 or 2+ tags (across all versions)
/// - `"<tag>.rs"` — one file per tag for types used exclusively by that tag
pub fn emit_types(canonical: &CanonicalSpec) -> Vec<(String, String)> {
    let specs: Vec<(&str, &ApiSpec)> = canonical
        .per_version_specs
        .iter()
        .map(|(v, s)| (v.as_str(), s))
        .collect();
    emit_types_from_specs(&specs)
}

/// Merge type definitions from all API versions and emit per-tag Rust source files
/// containing union types where all fields are `Option<T>`.
///
/// Returns a list of `(filename, content)` pairs to write into `src/dynamic/types/`:
/// - `"mod.rs"` — module declarations + glob re-exports
/// - `"common.rs"` — types referenced by 0 or 2+ tags (across all versions)
/// - `"<tag>.rs"` — one file per tag for types used exclusively by that tag
fn emit_types_from_specs(specs: &[(&str, &ApiSpec)]) -> Vec<(String, String)> {
    let merged = merge_all_types(specs);

    // Build type_name -> set of tag module_names across ALL versions.
    let mut type_to_tags: HashMap<String, HashSet<String>> = HashMap::new();
    for (_version, spec) in specs {
        for tag in &spec.tags {
            for type_name in types_referenced_by_tag(tag) {
                type_to_tags
                    .entry(type_name)
                    .or_default()
                    .insert(tag.module_name.clone());
            }
        }
    }

    // Assign each merged type to exactly one tag or common.
    // BTreeMap for deterministic file order.
    let mut tag_types: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    let mut common_names: Vec<&str> = vec![];

    for name in merged.keys() {
        match type_to_tags.get(name.as_str()) {
            Some(tags) if tags.len() == 1 => {
                let tag = tags.iter().next().unwrap().clone();
                tag_types.entry(tag).or_default().push(name.as_str());
            }
            _ => common_names.push(name.as_str()),
        }
    }

    let mut files: Vec<(String, String)> = vec![];

    // common.rs
    files.push((
        "common.rs".into(),
        emit_dynamic_type_file(&common_names, &merged),
    ));

    // per-tag files
    let tag_names: Vec<String> = tag_types.keys().cloned().collect();
    for tag_name in &tag_names {
        let names = tag_types.get(tag_name).unwrap();
        files.push((
            format!("{tag_name}.rs"),
            emit_dynamic_type_file(names, &merged),
        ));
    }

    // mod.rs
    let mut mod_out = String::new();
    mod_out.push_str("pub mod common;\n");
    for tag_name in &tag_names {
        mod_out.push_str(&format!("pub mod {tag_name};\n"));
    }
    mod_out.push_str("\npub use common::*;\n");
    for tag_name in &tag_names {
        mod_out.push_str(&format!("pub use {tag_name}::*;\n"));
    }
    files.push(("mod.rs".into(), crate::util::format_source(&mod_out)));

    files
}

/// Collect all type names directly referenced by a tag's endpoints (shallow — no transitive follow).
fn types_referenced_by_tag(tag: &TagGroup) -> HashSet<String> {
    let mut names = HashSet::new();
    let all_eps = tag
        .root_endpoints
        .iter()
        .chain(tag.sub_groups.iter().flat_map(|sg| sg.endpoints.iter()));
    for ep in all_eps {
        if let Some(t) = &ep.request_type {
            names.insert(t.clone());
        }
        if let Some(t) = &ep.response_type {
            names.insert(t.clone());
        }
        if let Some(t) = &ep.response_inner {
            names.insert(t.clone());
        }
        for qp in &ep.query_params {
            if let Some(type_name) = &qp.enum_type_name {
                names.insert(type_name.clone());
            }
        }
    }
    names
}

fn emit_dynamic_type_file(names: &[&str], merged: &BTreeMap<String, MergedType>) -> String {
    let mut out = String::new();
    out.push_str("#![allow(dead_code, private_interfaces, unused_imports)]\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n");
    out.push_str("use super::*;\n\n");

    for name in names {
        let mt = match merged.get(*name) {
            Some(m) => m,
            None => continue,
        };
        emit_merged_type(&mut out, name, mt);
    }

    crate::util::format_source(&out)
}

fn emit_merged_type(out: &mut String, name: &str, mt: &MergedType) {
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
                let rust_ty = field_type_to_rust(&field.ty, name, InlineEnumMode::AsString);
                let is_universal = universal_fields.contains(&field.rust_name);
                let final_ty = if is_universal {
                    rust_ty
                } else {
                    wrap_in_option(&field.ty, &rust_ty)
                };
                if let Some(field_doc) = &field.doc {
                    for line in field_doc.lines() {
                        out.push_str(&format!("    /// {line}\n"));
                    }
                }
                if field.serde_name != field.rust_name {
                    out.push_str(&format!(
                        "    #[serde(rename = \"{}\")]\n",
                        field.serde_name
                    ));
                }
                if !is_universal {
                    out.push_str("    #[serde(default)]\n");
                }
                if final_ty.starts_with("Option<") {
                    out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
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
            out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]\n");
            out.push_str(&format!("pub enum {name} {{\n"));
            for wire in variants {
                let pascal = wire_to_pascal(wire);
                out.push_str(&format!("    #[serde(rename = \"{wire}\")]\n"));
                out.push_str(&format!("    {pascal},\n"));
            }
            out.push_str("}\n\n");

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

/// Returns a map of type_name -> sorted field names (rust_name) for all merged types.
/// Dto types get their field names; Entity and StringEnum types get empty field lists
/// (their conversions are handled differently but they still need From impls).
pub(crate) fn collect_merged_field_names(
    specs: &[(&str, &ApiSpec)],
) -> BTreeMap<String, Vec<String>> {
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
pub(crate) fn collect_universal_fields(
    specs: &[(&str, &ApiSpec)],
) -> BTreeMap<String, BTreeSet<String>> {
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
                        let rust_ty =
                            field_type_to_rust(&field.ty, &td.name, InlineEnumMode::AsString);
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

/// Wrap the type in `Option<T>` unless it is already `Option<T>` (i.e. `FieldType::Opt`).
fn wrap_in_option(ty: &FieldType, rust_str: &str) -> String {
    match ty {
        FieldType::Opt(_) => rust_str.to_string(),
        _ => format!("Option<{rust_str}>"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::canonicalize;
    use crate::parser::{ApiSpec, Endpoint, Field, FieldType, HttpMethod, TagGroup, TypeDef, TypeKind};

    fn make_spec(types: Vec<TypeDef>) -> ApiSpec {
        ApiSpec {
            tags: vec![],
            all_types: types,
        }
    }

    fn make_spec_with_tags(types: Vec<TypeDef>, tags: Vec<TagGroup>) -> ApiSpec {
        ApiSpec {
            tags,
            all_types: types,
        }
    }

    fn make_tag(module_name: &str, response_types: Vec<&str>) -> TagGroup {
        let endpoints = response_types
            .into_iter()
            .map(|t| Endpoint {
                fn_name: format!("get_{t}"),
                raw_operation_id: String::new(),
                path: format!("/{t}"),
                method: HttpMethod::Get,
                doc: None,
                description: None,
                path_params: vec![],
                query_params: vec![],
                request_type: None,
                body_kind: None,
                body_doc: None,
                response_type: Some(t.to_string()),
                response_inner: None,
                response_field: None,
                response_kind: crate::content_type::ResponseBodyKind::Json {
                    schema_ref: t.to_string(),
                },
                success_responses: vec![],
                error_responses: vec![],
                security: None,
            })
            .collect();
        TagGroup {
            tag: module_name.to_string(),
            struct_name: format!("{}Api", module_name),
            module_name: module_name.to_string(),
            accessor_fn: format!("{module_name}_api"),
            types: vec![],
            root_endpoints: endpoints,
            sub_groups: vec![],
        }
    }

    fn make_field(name: &str, ty: FieldType) -> Field {
        Field {
            rust_name: name.to_string(),
            serde_name: name.to_string(),
            ty,
            doc: None,
            read_only: false,
            deprecated: false,
        }
    }

    /// Concatenate all file contents from emit_types_from_specs output.
    fn all_content(files: &[(String, String)]) -> String {
        files
            .iter()
            .map(|(_, c)| c.as_str())
            .collect::<Vec<_>>()
            .join("\n")
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
        let files = emit_types_from_specs(&[("2.7.2", &spec_a), ("2.8.0", &spec_b)]);
        let output = all_content(&files);
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
        let files = emit_types_from_specs(&[("2.7.2", &spec_a), ("2.8.0", &spec_b)]);
        let output = all_content(&files);
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
        let files = emit_types_from_specs(&[("2.7.2", &spec_a), ("2.8.0", &spec_b)]);
        let output = all_content(&files);
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
        let files = emit_types_from_specs(&[("2.7.2", &spec_a), ("2.8.0", &spec_b)]);
        let output = all_content(&files);
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
            deprecated: false,
        }
    }

    #[test]
    fn dynamic_types_emit_non_exhaustive_on_struct() {
        let td = TypeDef {
            name: "AboutDto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![
                make_field_with_doc(
                    "version",
                    FieldType::Opt(Box::new(FieldType::Str)),
                    "The NiFi version",
                ),
                make_field("title", FieldType::Str),
            ],
            doc: Some("Information about the NiFi instance".to_string()),
        };
        let spec = make_spec(vec![td]);
        let files = emit_types_from_specs(&[("2.8.0", &spec)]);
        let output = all_content(&files);
        // non_exhaustive attribute on struct
        assert!(
            output.contains("#[non_exhaustive]"),
            "missing #[non_exhaustive] on struct"
        );
        // struct-level doc comment
        assert!(
            output.contains("/// Information about the NiFi instance"),
            "missing struct doc comment"
        );
        // field-level doc comment
        assert!(
            output.contains("/// The NiFi version"),
            "missing field doc comment"
        );
        // skip_serializing_if on Option fields
        assert!(
            output.contains("#[serde(skip_serializing_if = \"Option::is_none\")]"),
            "missing skip_serializing_if"
        );
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
        let files = emit_types_from_specs(&[("2.8.0", &spec)]);
        let output = all_content(&files);
        assert!(
            output.contains("pub enum DiagnosticLevel"),
            "missing enum declaration"
        );
        assert!(
            output.contains("#[non_exhaustive]"),
            "missing #[non_exhaustive] on enum"
        );
    }

    #[test]
    fn dynamic_types_split_per_tag() {
        // AboutDto and AboutEntity are referenced by the "flow" tag
        // PositionDto is not referenced by any tag — goes to common.rs
        let about_dto = TypeDef {
            name: "AboutDto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![make_field(
                "version",
                FieldType::Opt(Box::new(FieldType::Str)),
            )],
            doc: None,
        };
        let about_entity = TypeDef {
            name: "AboutEntity".to_string(),
            kind: TypeKind::Entity {
                field: "about".into(),
                inner: "AboutDto".into(),
            },
            fields: vec![],
            doc: None,
        };
        let position_dto = TypeDef {
            name: "PositionDto".to_string(),
            kind: TypeKind::Dto,
            fields: vec![make_field("x", FieldType::F64)],
            doc: None,
        };

        let flow_tag = make_tag("flow", vec!["AboutEntity"]);
        let spec = make_spec_with_tags(vec![about_dto, about_entity, position_dto], vec![flow_tag]);

        let files = emit_types_from_specs(&[("2.8.0", &spec)]);
        let filenames: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();

        // Must contain mod.rs, flow.rs, and common.rs
        assert!(
            filenames.contains(&"mod.rs"),
            "missing mod.rs; got: {filenames:?}"
        );
        assert!(
            filenames.contains(&"flow.rs"),
            "missing flow.rs; got: {filenames:?}"
        );
        assert!(
            filenames.contains(&"common.rs"),
            "missing common.rs; got: {filenames:?}"
        );

        let flow_content = files
            .iter()
            .find(|(n, _)| n == "flow.rs")
            .unwrap()
            .1
            .as_str();
        let common_content = files
            .iter()
            .find(|(n, _)| n == "common.rs")
            .unwrap()
            .1
            .as_str();

        // AboutEntity is exclusively referenced by flow -> flow.rs
        assert!(
            flow_content.contains("AboutEntity"),
            "flow.rs should contain AboutEntity"
        );
        // PositionDto is not referenced by any tag -> common.rs
        assert!(
            common_content.contains("PositionDto"),
            "common.rs should contain PositionDto"
        );
        // AboutDto is not directly referenced (AboutEntity is, not AboutDto itself) -> common.rs
        assert!(
            common_content.contains("AboutDto"),
            "common.rs should contain AboutDto"
        );
    }

    // --- Phase 4a canonical entry point test ---

    fn spec_with_about() -> ApiSpec {
        ApiSpec {
            tags: vec![TagGroup {
                tag: "Flow".to_string(),
                struct_name: "FlowApi".to_string(),
                module_name: "flow".to_string(),
                accessor_fn: "flow_api".to_string(),
                types: vec![],
                root_endpoints: vec![Endpoint {
                    method: HttpMethod::Get,
                    path: "/flow/about".to_string(),
                    fn_name: "get_about_info".to_string(),
                    raw_operation_id: "getAboutInfo".to_string(),
                    doc: None,
                    description: None,
                    path_params: vec![],
                    request_type: None,
                    body_kind: None,
                    body_doc: None,
                    response_type: Some("AboutEntity".to_string()),
                    response_inner: Some("AboutDto".to_string()),
                    response_field: Some("about".to_string()),
                    response_kind: crate::content_type::ResponseBodyKind::Json {
                        schema_ref: "AboutEntity".to_string(),
                    },
                    query_params: vec![],
                    success_responses: vec![],
                    error_responses: vec![],
                    security: None,
                }],
                sub_groups: vec![],
            }],
            all_types: vec![
                TypeDef {
                    name: "AboutEntity".to_string(),
                    kind: TypeKind::Entity {
                        field: "about".to_string(),
                        inner: "AboutDto".to_string(),
                    },
                    fields: vec![],
                    doc: None,
                },
                TypeDef {
                    name: "AboutDto".to_string(),
                    kind: TypeKind::Dto,
                    fields: vec![Field {
                        rust_name: "version".to_string(),
                        serde_name: "version".to_string(),
                        ty: FieldType::Opt(Box::new(FieldType::Str)),
                        doc: None,
                        read_only: false,
                        deprecated: false,
                    }],
                    doc: None,
                },
            ],
        }
    }

    #[test]
    fn emit_types_produces_per_tag_files() {
        let canonical = canonicalize(&[("2.8.0".to_string(), spec_with_about())]);
        let files = emit_types(&canonical);
        let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"mod.rs"));
        assert!(names.contains(&"common.rs") || names.iter().any(|n| n.starts_with("flow")));
        let all_content: String = files.iter().map(|(_, c)| c.as_str()).collect();
        assert!(all_content.contains("AboutDto"));
        assert!(all_content.contains("version"));
    }
}
