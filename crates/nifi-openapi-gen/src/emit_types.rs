use std::collections::{BTreeMap, HashMap, HashSet};

use crate::parser::{ApiSpec, FieldType, TagGroup, TypeDef, TypeKind};

/// Returns a list of `(filename, content)` pairs to write into `src/types/`.
///
/// - `"mod.rs"` — module declarations + glob re-exports for backward compat
/// - `"common.rs"` — types referenced by 0 or 2+ tags
/// - `"<tag>.rs"` — one file per tag for types used exclusively by that tag
pub fn emit_types(spec: &ApiSpec) -> Vec<(String, String)> {
    // Map type name → set of tag module_names that directly reference it.
    let mut type_to_tags: HashMap<String, HashSet<String>> = HashMap::new();
    for tag in &spec.tags {
        for type_name in types_referenced_by_tag(tag) {
            type_to_tags
                .entry(type_name)
                .or_default()
                .insert(tag.module_name.clone());
        }
    }

    // Sort types into per-tag buckets or common.
    // BTreeMap for deterministic file order.
    let mut tag_types: BTreeMap<String, Vec<&TypeDef>> = BTreeMap::new();
    let mut common_types: Vec<&TypeDef> = vec![];

    for typedef in &spec.all_types {
        match type_to_tags.get(&typedef.name) {
            Some(tags) if tags.len() == 1 => {
                let tag = tags.iter().next().unwrap().clone();
                tag_types.entry(tag).or_default().push(typedef);
            }
            _ => common_types.push(typedef),
        }
    }

    let mut files: Vec<(String, String)> = vec![];

    // common.rs
    files.push(("common.rs".into(), emit_type_file(&common_types)));

    // per-tag files
    let tag_names: Vec<String> = tag_types.keys().cloned().collect();
    for tag_name in &tag_names {
        let types = tag_types.get(tag_name).unwrap();
        files.push((format!("{tag_name}.rs"), emit_type_file(types)));
    }

    // mod.rs — declares modules and re-exports all for backward compat
    let mut mod_out = String::new();
    mod_out.push_str("pub mod common;\n");
    for tag_name in &tag_names {
        mod_out.push_str(&format!("pub mod {tag_name};\n"));
    }
    mod_out.push_str("\npub use common::*;\n");
    for tag_name in &tag_names {
        mod_out.push_str(&format!("pub use {tag_name}::*;\n"));
    }
    files.push(("mod.rs".into(), format_source(&mod_out)));

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
        // Include query param enum types so they land in this tag's types file
        for qp in &ep.query_params {
            if let Some(type_name) = &qp.enum_type_name {
                names.insert(type_name.clone());
            }
        }
    }
    names
}

fn emit_type_file(types: &[&TypeDef]) -> String {
    let mut out = String::new();
    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("use serde::{Deserialize, Serialize};\n");
    // Import sibling modules' types so cross-file $ref fields compile.
    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("use super::*;\n\n");
    for typedef in types {
        out.push_str(&emit_type(typedef));
        out.push('\n');
    }
    format_source(&out)
}

fn emit_type(t: &TypeDef) -> String {
    match &t.kind {
        TypeKind::Dto => emit_dto(t),
        TypeKind::Entity { field, inner } => emit_entity(t, field, inner),
        TypeKind::StringEnum(variants) => emit_standalone_string_enum(&t.name, variants),
    }
}

fn emit_doc_comment(out: &mut String, doc: &str, indent: &str) {
    for line in doc.lines() {
        let line = line.trim();
        if line.is_empty() {
            out.push_str(&format!("{indent}///\n"));
        } else {
            out.push_str(&format!("{indent}/// {line}\n"));
        }
    }
}

fn emit_dto(t: &TypeDef) -> String {
    let mut out = String::new();
    for field in &t.fields {
        if let Some(variants) = extract_enum(&field.ty) {
            let enum_name = format!("{}{}", t.name, pascal_case(&field.rust_name));
            out.push_str(&emit_string_enum(&enum_name, &variants));
            out.push('\n');
        }
    }
    if let Some(doc) = &t.doc {
        emit_doc_comment(&mut out, doc, "");
    }
    out.push_str("#[derive(Debug, Clone, Default, Deserialize, Serialize)]\n");
    out.push_str("#[serde(rename_all = \"camelCase\")]\n");
    out.push_str(&format!("pub struct {} {{\n", t.name));
    for field in &t.fields {
        if let Some(doc) = &field.doc {
            if field.read_only {
                let mut full_doc = doc.clone();
                full_doc.push_str(" Read-only — set by NiFi.");
                emit_doc_comment(&mut out, &full_doc, "    ");
            } else {
                emit_doc_comment(&mut out, doc, "    ");
            }
        } else if field.read_only {
            out.push_str("    /// Read-only — set by NiFi.\n");
        }
        let ty_str = field_type_to_rust(&field.ty, &t.name, &field.rust_name);
        let field_ident = escape_keyword(&field.rust_name);
        let is_option = matches!(&field.ty, crate::parser::FieldType::Opt(_));
        if field_ident != field.rust_name && is_option {
            out.push_str(&format!(
                "    #[serde(rename = \"{}\", skip_serializing_if = \"Option::is_none\")]\n",
                field.serde_name
            ));
        } else if field_ident != field.rust_name {
            out.push_str(&format!(
                "    #[serde(rename = \"{}\")]\n",
                field.serde_name
            ));
        } else if is_option {
            out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
        }
        out.push_str(&format!("    pub {field_ident}: {ty_str},\n"));
    }
    out.push_str("}\n");
    out
}

fn emit_entity(t: &TypeDef, field: &str, inner: &str) -> String {
    let mut out = String::new();
    if let Some(doc) = &t.doc {
        emit_doc_comment(&mut out, doc, "");
    }
    out.push_str("#[derive(Debug, Clone, Default, Deserialize, Serialize)]\n");
    out.push_str("#[serde(rename_all = \"camelCase\")]\n");
    out.push_str(&format!("pub struct {} {{\n", t.name));
    // The inner field is `Option<T>` so that NiFi responses which omit the field
    // (e.g. empty-body 200/202 replies) deserialize without error.
    out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
    out.push_str(&format!("    pub {field}: Option<{inner}>,\n"));
    out.push_str("}\n");
    out
}

fn emit_string_enum(name: &str, variants: &[String]) -> String {
    let mut out = String::new();
    out.push_str("#[derive(Debug, Clone, Default, Deserialize, Serialize)]\n");
    out.push_str(&format!("pub enum {name} {{\n"));
    for (i, v) in variants.iter().enumerate() {
        let variant = pascal_case(v);
        if i == 0 {
            out.push_str("    #[default]\n");
        }
        out.push_str(&format!("    #[serde(rename = \"{v}\")]\n    {variant},\n"));
    }
    out.push_str("}\n");
    out
}

fn emit_standalone_string_enum(name: &str, variants: &[String]) -> String {
    let mut out = String::new();
    // No Default derive: standalone enums are always used as Option<T> query params
    out.push_str("#[derive(Debug, Clone, Deserialize, Serialize)]\n");
    out.push_str(&format!("pub enum {name} {{\n"));
    for v in variants {
        let variant = pascal_case(v);
        out.push_str(&format!("    #[serde(rename = \"{v}\")]\n    {variant},\n"));
    }
    out.push_str("}\n");
    out.push('\n');
    // Display impl so .to_string() produces the wire value in query param building
    out.push_str(&format!("impl std::fmt::Display for {name} {{\n"));
    out.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    out.push_str("        let s = match self {\n");
    for v in variants {
        let variant = pascal_case(v);
        out.push_str(&format!("            {name}::{variant} => \"{v}\",\n"));
    }
    out.push_str("        };\n");
    out.push_str("        f.write_str(s)\n");
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

fn field_type_to_rust(ty: &FieldType, struct_name: &str, field_name: &str) -> String {
    match ty {
        FieldType::Str => "String".into(),
        FieldType::Bool => "bool".into(),
        FieldType::I32 => "i32".into(),
        FieldType::I64 => "i64".into(),
        FieldType::F64 => "f64".into(),
        FieldType::Opt(inner) => {
            format!(
                "Option<{}>",
                field_type_to_rust(inner, struct_name, field_name)
            )
        }
        FieldType::List(inner) => {
            format!(
                "Vec<{}>",
                field_type_to_rust(inner, struct_name, field_name)
            )
        }
        FieldType::Ref(name) => {
            if name == struct_name {
                format!("Box<{name}>")
            } else {
                name.clone()
            }
        }
        FieldType::Enum(_) => format!("{}{}", struct_name, pascal_case(field_name)),
        FieldType::Map(value_ty) => format!(
            "std::collections::HashMap<String, {}>",
            field_type_to_rust(value_ty, struct_name, field_name)
        ),
    }
}

fn extract_enum(ty: &FieldType) -> Option<Vec<String>> {
    match ty {
        FieldType::Enum(v) => Some(v.clone()),
        FieldType::Opt(inner) => extract_enum(inner),
        FieldType::List(inner) => extract_enum(inner),
        FieldType::Map(inner) => extract_enum(inner),
        _ => None,
    }
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

pub(crate) fn pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut c = part.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
            }
        })
        .collect()
}

fn format_source(src: &str) -> String {
    match syn::parse_file(src) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => src.to_string(),
    }
}
