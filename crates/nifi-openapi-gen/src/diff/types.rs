//! Type-level diff (struct fields, enum variants).

use crate::parser::{ApiSpec, Field, FieldType, TypeDef};
use std::collections::HashMap;

use super::{FieldChange, FieldChangeKind, TypeChanges, TypeEntry, TypesDiff};

/// Collects all types from a spec as name → TypeDef ref.
fn type_map(spec: &ApiSpec) -> HashMap<&str, &TypeDef> {
    spec.all_types
        .iter()
        .map(|t| (t.name.as_str(), t))
        .collect()
}

fn is_optional(ft: &FieldType) -> bool {
    matches!(ft, FieldType::Opt(_))
}

pub(super) fn diff_types(from: &ApiSpec, to: &ApiSpec) -> TypesDiff {
    let from_map = type_map(from);
    let to_map = type_map(to);
    let to_tags = super::index::type_tag_index(to);
    let from_tags = super::index::type_tag_index(from);

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();

    for (name, to_type) in &to_map {
        if !from_map.contains_key(name) {
            added.push(TypeEntry {
                name: name.to_string(),
                tags: to_tags.get(name).cloned().unwrap_or_default(),
            });
        } else {
            let from_type = from_map[name];
            let tc = diff_type_fields(name, from_type, to_type);
            if !tc.added_fields.is_empty()
                || !tc.removed_fields.is_empty()
                || !tc.changed_fields.is_empty()
                || !tc.added_variants.is_empty()
                || !tc.removed_variants.is_empty()
            {
                changed.push(tc);
            }
        }
    }

    for name in from_map.keys() {
        if !to_map.contains_key(name) {
            removed.push(TypeEntry {
                name: name.to_string(),
                tags: from_tags.get(name).cloned().unwrap_or_default(),
            });
        }
    }

    added.sort_by(|a, b| a.name.cmp(&b.name));
    removed.sort_by(|a, b| a.name.cmp(&b.name));
    changed.sort_by(|a, b| a.name.cmp(&b.name));

    TypesDiff {
        added,
        removed,
        changed,
    }
}

fn unwrap_opt(ty: &FieldType) -> &FieldType {
    match ty {
        FieldType::Opt(inner) => inner.as_ref(),
        other => other,
    }
}

fn field_type_display(ty: &FieldType) -> String {
    match ty {
        FieldType::Str => "String".to_string(),
        FieldType::DateTimeStr => "crate::compat::FlexibleString".to_string(),
        FieldType::Bool => "bool".to_string(),
        FieldType::I32 => "i32".to_string(),
        FieldType::I64 => "i64".to_string(),
        FieldType::F64 => "f64".to_string(),
        FieldType::Opt(inner) => format!("Option<{}>", field_type_display(inner)),
        FieldType::List(inner) => format!("Vec<{}>", field_type_display(inner)),
        FieldType::Ref(name) => name.clone(),
        FieldType::Map(inner) => format!("HashMap<String, {}>", field_type_display(inner)),
        FieldType::Enum(variants) => {
            let mut sorted = variants.clone();
            sorted.sort();
            format!("Enum({})", sorted.join(" | "))
        }
    }
}

fn diff_type_fields(name: &str, from_type: &TypeDef, to_type: &TypeDef) -> TypeChanges {
    use crate::parser::TypeKind;
    use std::collections::HashSet;

    // StringEnum: compare variant lists directly (no fields to diff)
    let (added_variants, removed_variants) = match (&from_type.kind, &to_type.kind) {
        (TypeKind::StringEnum(from_vals), TypeKind::StringEnum(to_vals)) => {
            let from_set: HashSet<&str> = from_vals.iter().map(String::as_str).collect();
            let to_set: HashSet<&str> = to_vals.iter().map(String::as_str).collect();
            let mut added: Vec<String> = to_set
                .difference(&from_set)
                .map(|s| s.to_string())
                .collect();
            let mut removed: Vec<String> = from_set
                .difference(&to_set)
                .map(|s| s.to_string())
                .collect();
            added.sort();
            removed.sort();
            (added, removed)
        }
        _ => (vec![], vec![]),
    };

    // Dto / Entity: compare fields
    let from_fields: HashMap<&str, &Field> = from_type
        .fields
        .iter()
        .map(|f| (f.rust_name.as_str(), f))
        .collect();
    let to_fields: HashMap<&str, &Field> = to_type
        .fields
        .iter()
        .map(|f| (f.rust_name.as_str(), f))
        .collect();

    let mut added_fields = Vec::new();
    let mut removed_fields = Vec::new();
    let mut changed_fields = Vec::new();

    for (fname, to_f) in &to_fields {
        if !from_fields.contains_key(fname) {
            added_fields.push(fname.to_string());
        } else {
            let from_f = from_fields[fname];
            let from_opt = is_optional(&from_f.ty);
            let to_opt = is_optional(&to_f.ty);
            let from_base = unwrap_opt(&from_f.ty);
            let to_base = unwrap_opt(&to_f.ty);

            if from_opt != to_opt {
                changed_fields.push(FieldChange {
                    name: fname.to_string(),
                    kind: if to_opt {
                        FieldChangeKind::BecameOptional
                    } else {
                        FieldChangeKind::BecameRequired
                    },
                });
            }
            if from_f.deprecated != to_f.deprecated {
                changed_fields.push(FieldChange {
                    name: fname.to_string(),
                    kind: FieldChangeKind::DeprecationChanged {
                        now_deprecated: to_f.deprecated,
                    },
                });
            }
            // For inline enum fields, compare variant sets (order-insensitive).
            let types_differ = match (from_base, to_base) {
                (FieldType::Enum(fv), FieldType::Enum(tv)) => {
                    let mut a = fv.clone();
                    let mut b = tv.clone();
                    a.sort();
                    b.sort();
                    a != b
                }
                _ => from_base != to_base,
            };
            if types_differ {
                let kind = match (from_base, to_base) {
                    (FieldType::Enum(fv), FieldType::Enum(tv)) => {
                        let fset: std::collections::HashSet<_> = fv.iter().collect();
                        let tset: std::collections::HashSet<_> = tv.iter().collect();
                        let mut added: Vec<String> =
                            tset.difference(&fset).map(|s| s.to_string()).collect();
                        let mut removed: Vec<String> =
                            fset.difference(&tset).map(|s| s.to_string()).collect();
                        added.sort();
                        removed.sort();
                        FieldChangeKind::InlineEnumChanged { added, removed }
                    }
                    (FieldType::Str, FieldType::Enum(tv)) => {
                        let mut variants = tv.clone();
                        variants.sort();
                        FieldChangeKind::NarrowedToEnum { variants }
                    }
                    _ => FieldChangeKind::TypeChanged {
                        from: field_type_display(from_base),
                        to: field_type_display(to_base),
                    },
                };
                changed_fields.push(FieldChange {
                    name: fname.to_string(),
                    kind,
                });
            }
        }
    }

    for fname in from_fields.keys() {
        if !to_fields.contains_key(fname) {
            removed_fields.push(fname.to_string());
        }
    }

    added_fields.sort();
    removed_fields.sort();
    changed_fields.sort_by(|a, b| a.name.cmp(&b.name));

    TypeChanges {
        name: name.to_string(),
        added_fields,
        removed_fields,
        changed_fields,
        added_variants,
        removed_variants,
    }
}
