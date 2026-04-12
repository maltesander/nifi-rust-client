//! Method name stability for generated resource structs.
//!
//! This module runs as a post-pass after the parser has built per-version
//! `ApiSpec`s. It:
//!
//! 1. Applies `(version, operationId)` overrides from
//!    [`crate::naming_overrides`] to rewrite specific method names.
//! 2. Panics at build time if any two operations in the same tag would
//!    generate the same method name (same-tag collision — implemented in
//!    Task 5).
//! 3. Panics at build time if the same `(tag, method, path)` triple
//!    resolves to different method names in different versions
//!    (cross-version drift — implemented in Task 6).
//!
//! All three checks are strict — there are no ignore flags. The panics are
//! actionable and point at the exact override-table entry that would fix
//! each case.

use std::collections::HashMap;

use crate::naming_overrides::NAMING_OVERRIDES;
use crate::parser::{ApiSpec, Endpoint};

/// Apply override entries from the production `NAMING_OVERRIDES` table to a
/// single parsed spec, then run the per-version same-tag collision check.
///
/// This is the entry point used by the real generator. Tests that need to
/// inject a synthetic override map should call
/// [`apply_overrides_with_table`] directly.
pub fn apply_overrides_and_check_single(spec: &mut ApiSpec, version: &str) {
    let table: HashMap<(String, String), String> = NAMING_OVERRIDES
        .iter()
        .map(|((v, op), name)| ((v.to_string(), op.to_string()), name.to_string()))
        .collect();
    apply_overrides_with_table(spec, version, &table);
    check_collisions(spec, version);
}

/// Public test hook: apply overrides from a caller-supplied table.
pub fn apply_overrides_with_table(
    spec: &mut ApiSpec,
    version: &str,
    overrides: &HashMap<(String, String), String>,
) {
    for tag in &mut spec.tags {
        for ep in &mut tag.root_endpoints {
            maybe_override(ep, version, overrides);
        }
        for sg in &mut tag.sub_groups {
            for ep in &mut sg.endpoints {
                maybe_override(ep, version, overrides);
            }
        }
    }
}

fn maybe_override(
    ep: &mut Endpoint,
    version: &str,
    overrides: &HashMap<(String, String), String>,
) {
    let key = (version.to_string(), ep.raw_operation_id.clone());
    if let Some(name) = overrides.get(&key) {
        ep.fn_name = name.clone();
    }
}

/// Placeholder — filled in by Task 5 with the same-tag collision check.
pub fn check_collisions(_spec: &ApiSpec, _version: &str) {}
