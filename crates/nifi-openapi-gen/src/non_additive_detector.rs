//! Non-additive-change detector.
//!
//! Runs during canonicalization to enforce the monotonic-additive
//! assumption on supported NiFi specs. When a new spec removes or
//! semantically changes something that a previous spec declared, the
//! detector emits a `NonAdditiveChange` variant. The caller panics
//! unless the change is accepted by `NON_ADDITIVE_OVERRIDES`.

use std::collections::BTreeSet;

use crate::canonical::{CanonicalSpec, EndpointKey};
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
    let mut out = Vec::new();
    check_endpoint_removed(canonical, version, spec, &mut out);
    check_type_removed(canonical, version, spec, &mut out);
    out
}

fn check_type_removed(
    canonical: &CanonicalSpec,
    version: &str,
    spec: &ApiSpec,
    out: &mut Vec<NonAdditiveChange>,
) {
    let spec_type_names: BTreeSet<&str> =
        spec.all_types.iter().map(|t| t.name.as_str()).collect();
    for (name, canonical_type) in &canonical.types {
        if !spec_type_names.contains(name.as_str()) {
            out.push(NonAdditiveChange::TypeRemoved {
                type_name: name.clone(),
                previous_versions: canonical_type.versions.to_vec(),
                missing_in: version.to_string(),
            });
        }
    }
}

fn collect_spec_endpoint_keys(spec: &ApiSpec) -> BTreeSet<EndpointKey> {
    let mut keys = BTreeSet::new();
    for tag in &spec.tags {
        for endpoint in &tag.root_endpoints {
            keys.insert(EndpointKey {
                method: endpoint.method.clone(),
                path: endpoint.path.clone(),
            });
        }
        for sub in &tag.sub_groups {
            for endpoint in &sub.endpoints {
                keys.insert(EndpointKey {
                    method: endpoint.method.clone(),
                    path: endpoint.path.clone(),
                });
            }
        }
    }
    keys
}

fn check_endpoint_removed(
    canonical: &CanonicalSpec,
    version: &str,
    spec: &ApiSpec,
    out: &mut Vec<NonAdditiveChange>,
) {
    let spec_keys = collect_spec_endpoint_keys(spec);
    for (key, canonical_ep) in &canonical.endpoints {
        if !spec_keys.contains(key) {
            out.push(NonAdditiveChange::EndpointRemoved {
                method: key.method.clone(),
                path: key.path.clone(),
                previous_versions: canonical_ep.versions.to_vec(),
                missing_in: version.to_string(),
            });
        }
    }
}
