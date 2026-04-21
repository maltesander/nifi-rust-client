//! Type-reachability: associate types with the tag that references them.

use crate::parser::ApiSpec;
use std::collections::HashMap;

/// Build a mapping from type name to the sorted, deduplicated list of tags that use it.
///
/// Walks endpoint request/response types and their fields transitively to
/// associate types with their API tags.
pub(super) fn type_tag_index(spec: &ApiSpec) -> HashMap<&str, Vec<String>> {
    let type_map: HashMap<&str, &crate::parser::TypeDef> = spec
        .all_types
        .iter()
        .map(|t| (t.name.as_str(), t))
        .collect();
    let mut index: HashMap<&str, Vec<String>> = HashMap::new();

    for tag_group in &spec.tags {
        // Collect direct endpoint types
        let mut frontier: Vec<&str> = Vec::new();
        for ep in tag_group.endpoints.iter() {
            if let Some(t) = &ep.request_type {
                frontier.push(t.as_str());
            }
            if let Some(t) = &ep.response_type {
                frontier.push(t.as_str());
            }
            if let Some(t) = &ep.response_inner {
                frontier.push(t.as_str());
            }
        }

        // Walk fields transitively
        let mut visited: std::collections::HashSet<&str> = std::collections::HashSet::new();
        while let Some(name) = frontier.pop() {
            if !visited.insert(name) {
                continue;
            }
            if let Some(td) = type_map.get(name) {
                for f in &td.fields {
                    if let Some(ref_name) = field_type_ref_name(&f.ty) {
                        frontier.push(ref_name);
                    }
                }
            }
        }

        for name in visited {
            index.entry(name).or_default().push(tag_group.tag.clone());
        }
    }
    // Deduplicate and sort
    for tags in index.values_mut() {
        tags.sort();
        tags.dedup();
    }
    index
}

/// Extract the type name from a field type if it's a named reference.
fn field_type_ref_name(ty: &crate::parser::FieldType) -> Option<&str> {
    use crate::parser::FieldType;
    match ty {
        FieldType::Ref(name) => Some(name.as_str()),
        FieldType::List(inner) | FieldType::Opt(inner) | FieldType::Map(inner) => {
            field_type_ref_name(inner)
        }
        _ => None,
    }
}
