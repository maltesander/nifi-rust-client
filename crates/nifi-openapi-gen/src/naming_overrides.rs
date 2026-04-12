//! Explicit `fn_name` pins for specific `(spec_version, operationId)` pairs.
//!
//! This table is consulted by the `naming` module after the parser derives a
//! base `fn_name` from the `operationId`. Two situations require an entry
//! here:
//!
//! 1. **Same-tag collision.** When a future NiFi version introduces two
//!    operations in the same tag whose stripped `operationId`s collide, the
//!    generator panics. The fix is to pick a descriptive name for one (or
//!    both) operations and pin it via an override entry.
//!
//! 2. **Upstream operationId rename.** When a NiFi version renames an
//!    `operationId` between releases, the cross-version drift check panics.
//!    The fix is to either pin the older name on the newer version (keeping
//!    the Rust method name stable) or accept the rename and update the
//!    golden file.
//!
//! Today this table is empty. Every supported NiFi 2.x operation produces a
//! clean `fn_name` with no intervention. The panics below will tell you
//! exactly which entry to add if that changes.

use std::collections::HashMap;
use std::sync::LazyLock;

/// Keyed by `(spec_version, raw_operationId)`, valued by the explicit
/// `fn_name` to use for that operation. Consulted after suffix stripping
/// but before collision and drift checks.
pub static NAMING_OVERRIDES: LazyLock<HashMap<(&'static str, &'static str), &'static str>> =
    LazyLock::new(|| {
        let mut m: HashMap<(&'static str, &'static str), &'static str> = HashMap::new();
        // Example (not active):
        // m.insert(
        //     ("2.9.0", "updateRunStatus_9"),
        //     "update_run_status_for_new_endpoint",
        // );
        let _ = &mut m;
        m
    });

/// Returns the override `fn_name` for `(version, operation_id)` if one is
/// registered, otherwise `None`.
pub fn override_for(version: &str, operation_id: &str) -> Option<&'static str> {
    NAMING_OVERRIDES.get(&(version, operation_id)).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_table_returns_none() {
        // The production table ships empty; this test guards against
        // accidentally committing a real entry.
        assert_eq!(NAMING_OVERRIDES.len(), 0);
        assert_eq!(override_for("2.9.0", "anything"), None);
    }
}
