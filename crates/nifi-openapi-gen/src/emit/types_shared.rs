//! Shared helpers between the static (`emit::types`) and dynamic
//! (`emit::dynamic::types`) type emitters.
//!
//! Both emitters produce a `src/types/` tree consisting of `common.rs` plus
//! one file per OpenAPI tag and a `mod.rs` that declares + glob-re-exports
//! every sub-module. The assignment of types to files and the `mod.rs`
//! skeleton are identical across the two code paths; only the per-type
//! emission logic differs (single-version vs canonical-union).
//!
//! This module factors out the mechanical pieces so the two emitters share
//! a single implementation:
//!
//! - [`types_referenced_by_tag`] — shallow dependency walk over a tag's
//!   endpoints collecting every referenced type name (request, response,
//!   response-inner, query-param enum).
//! - [`emit_mod_rs`] — deterministic `pub mod` + `pub use` re-export list
//!   for the generated `mod.rs`.

use std::collections::HashSet;

use crate::parser::TagGroup;

/// Collect all type names directly referenced by a tag's endpoints.
///
/// Shallow — no transitive follow: only the types that appear directly on
/// endpoint signatures (`request_type`, `response_type`, `response_inner`)
/// and query-parameter enum types are included. The returned set is used
/// by both emitters to decide whether a type belongs in a single tag file
/// or in `common.rs`.
pub(crate) fn types_referenced_by_tag(tag: &TagGroup) -> HashSet<String> {
    let mut names = HashSet::new();
    for ep in tag.endpoints.iter() {
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

/// Emit the generated `types/mod.rs` skeleton.
///
/// The layout is identical between static and dynamic emitters:
/// `pub mod common;` first, then every per-tag module in the order given,
/// a blank line, then `pub use common::*;` followed by per-tag glob
/// re-exports for backward-compatible flat access.
///
/// The caller is responsible for supplying `tag_names` in deterministic
/// order (both existing callers iterate a `BTreeMap`, which is sorted).
pub(crate) fn emit_mod_rs(tag_names: &[String]) -> String {
    let mut out = String::new();
    out.push_str("pub mod common;\n");
    for tag_name in tag_names {
        out.push_str(&format!("pub mod {tag_name};\n"));
    }
    out.push_str("\npub use common::*;\n");
    for tag_name in tag_names {
        out.push_str(&format!("pub use {tag_name}::*;\n"));
    }
    crate::util::format_source(&out)
}
