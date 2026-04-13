//! Canonical-superset dynamic-mode emitter (Phase 4a parallel build).
//!
//! Produces the new dispatch-free dynamic surface into `$OUT_DIR/dynamic_v2/`.
//! Runs alongside the legacy dispatch emitter — both are written on every
//! build with the `dynamic` Cargo feature active. Phase 4b deletes the legacy
//! emitter and renames `dynamic_v2` → `dynamic`.

mod availability;
mod index;

pub use availability::{emit_availability, endpoint_variant_name};
pub use index::EndpointIndex;

use std::path::PathBuf;

use crate::canonical::CanonicalSpec;

/// Emit every file that lives under `$OUT_DIR/dynamic_v2/`.
///
/// Returns relative paths under `dynamic_v2/` and their contents. The caller
/// (build_api.rs) is responsible for prepending `$OUT_DIR/dynamic_v2/` and
/// writing the bytes.
pub fn emit_dynamic_v2(_canonical: &CanonicalSpec) -> Vec<(PathBuf, String)> {
    Vec::new()
}
