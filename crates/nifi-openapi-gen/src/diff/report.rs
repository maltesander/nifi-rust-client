//! Report/analysis methods on [`VersionDiff`]: human-readable summary,
//! backwards-compat check, and semver bump suggestion.

use super::{FieldChangeKind, SemverBump, VersionDiff};

impl VersionDiff {
    /// Produces a short human-readable summary for use in the versions table.
    /// Example: "+4 endpoints, -1 endpoints, +12 fields, +2 enum values"
    pub fn summary(&self) -> String {
        let added_ep = self.endpoints.added.len() as i64;
        let removed_ep = self.endpoints.removed.len() as i64;

        let added_fields: usize = self
            .types
            .changed
            .iter()
            .map(|tc| tc.added_fields.len())
            .sum();
        let removed_fields: usize = self
            .types
            .changed
            .iter()
            .map(|tc| tc.removed_fields.len())
            .sum();

        let added_enum_vals: usize = self
            .endpoints
            .changed
            .iter()
            .flat_map(|ec| ec.changed_params.iter())
            .map(|pc| pc.added_enum_values.len())
            .sum::<usize>()
            + self
                .types
                .changed
                .iter()
                .map(|tc| tc.added_variants.len())
                .sum::<usize>();

        let removed_enum_vals: usize = self
            .endpoints
            .changed
            .iter()
            .flat_map(|ec| ec.changed_params.iter())
            .map(|pc| pc.removed_enum_values.len())
            .sum::<usize>()
            + self
                .types
                .changed
                .iter()
                .map(|tc| tc.removed_variants.len())
                .sum::<usize>();

        let added_types = self.types.added.len() as i64;
        let removed_types = self.types.removed.len() as i64;

        let mut parts: Vec<String> = Vec::new();

        if added_ep != 0 || removed_ep != 0 {
            if added_ep > 0 {
                parts.push(format!("+{added_ep} endpoints"));
            }
            if removed_ep > 0 {
                parts.push(format!("-{removed_ep} endpoints"));
            }
        }
        if added_types > 0 {
            parts.push(format!("+{added_types} types"));
        }
        if removed_types > 0 {
            parts.push(format!("-{removed_types} types"));
        }
        if added_fields > 0 {
            parts.push(format!("+{added_fields} fields"));
        }
        if removed_fields > 0 {
            parts.push(format!("-{removed_fields} fields"));
        }
        if added_enum_vals > 0 {
            parts.push(format!("+{added_enum_vals} enum values"));
        }
        if removed_enum_vals > 0 {
            parts.push(format!("-{removed_enum_vals} enum values"));
        }

        if parts.is_empty() {
            format!("no API changes vs {}", self.from)
        } else {
            format!("{} vs {}", parts.join(", "), self.from)
        }
    }

    /// Returns `true` if the diff contains any backwards-incompatible change.
    pub fn is_breaking(&self) -> bool {
        // Removed endpoints or types
        if !self.endpoints.removed.is_empty() || !self.types.removed.is_empty() {
            return true;
        }

        // Changes to existing endpoints
        for ec in &self.endpoints.changed {
            // Removed query or path params, or added path params (breaking)
            if !ec.removed_params.is_empty()
                || !ec.removed_path_params.is_empty()
                || !ec.added_path_params.is_empty()
            {
                return true;
            }
            // Contract changes (request/response type, body kind)
            if !ec.contract_changes.is_empty() {
                return true;
            }
            // Required query param added
            if ec.added_params.iter().any(|p| p.required) {
                return true;
            }
            // Query param enum values removed or type changed
            if ec
                .changed_params
                .iter()
                .any(|pc| !pc.removed_enum_values.is_empty() || pc.type_changed.is_some())
            {
                return true;
            }
        }

        // Changes to existing types
        for tc in &self.types.changed {
            // Removed fields or enum variants
            if !tc.removed_fields.is_empty() || !tc.removed_variants.is_empty() {
                return true;
            }
            // Fields that became required, changed type, or lost enum variants
            if tc.changed_fields.iter().any(|fc| match &fc.kind {
                FieldChangeKind::BecameRequired
                | FieldChangeKind::TypeChanged { .. }
                | FieldChangeKind::NarrowedToEnum { .. } => true,
                FieldChangeKind::InlineEnumChanged { removed, .. } => !removed.is_empty(),
                _ => false,
            }) {
                return true;
            }
        }

        false
    }

    /// Suggests a semantic version bump level for this diff.
    pub fn semver_bump(&self) -> SemverBump {
        if self.is_breaking() {
            return SemverBump::Major;
        }
        // Additive changes → Minor
        let is_additive = !self.endpoints.added.is_empty()
            || !self.types.added.is_empty()
            || self
                .types
                .changed
                .iter()
                .any(|tc| !tc.added_fields.is_empty() || !tc.added_variants.is_empty())
            || self
                .endpoints
                .changed
                .iter()
                .any(|ec| ec.added_params.iter().any(|p| !p.required));
        if is_additive {
            SemverBump::Minor
        } else {
            SemverBump::Patch
        }
    }
}
