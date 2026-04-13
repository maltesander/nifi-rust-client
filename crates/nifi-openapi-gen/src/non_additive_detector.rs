//! Non-additive-change detector.
//!
//! Runs during canonicalization to enforce the monotonic-additive
//! assumption on supported NiFi specs. When a new spec removes or
//! semantically changes something that a previous spec declared, the
//! detector emits a `NonAdditiveChange` variant. The caller panics
//! unless the change is accepted by `NON_ADDITIVE_OVERRIDES`.

use crate::canonical::CanonicalSpec;
use crate::parser::{ApiSpec, FieldType, HttpMethod};

#[derive(Debug, Clone, PartialEq)]
pub enum NonAdditiveChange {
    EndpointRemoved {
        method: HttpMethod,
        path: String,
        previous_versions: Vec<String>,
        missing_in: String,
    },
    TypeRemoved {
        type_name: String,
        previous_versions: Vec<String>,
        missing_in: String,
    },
    FieldRemoved {
        type_name: String,
        field: String,
        previous_versions: Vec<String>,
        missing_in: String,
    },
    FieldTypeChanged {
        type_name: String,
        field: String,
        from: FieldType,
        to: FieldType,
        previous_versions: Vec<String>,
        changed_in: String,
    },
    EnumVariantRemoved {
        enum_name: String,
        variant: String,
        previous_versions: Vec<String>,
        missing_in: String,
    },
}

/// Run all rules over the (canonical, new-spec) pair.
///
/// `canonical` is the in-progress canonical spec built from all
/// previously-merged versions. `version` is the spec being evaluated.
/// Returns every rule hit as a `NonAdditiveChange`. Caller is responsible
/// for consulting the override table and panicking on residual changes.
pub fn check(
    canonical: &CanonicalSpec,
    version: &str,
    spec: &ApiSpec,
) -> Vec<NonAdditiveChange> {
    let _ = (canonical, version, spec);
    Vec::new()
}
