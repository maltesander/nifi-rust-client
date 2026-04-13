//! Non-additive-change detector.
//!
//! Runs during canonicalization to enforce the monotonic-additive
//! assumption on supported NiFi specs. When a new spec removes or
//! semantically changes something that a previous spec declared, the
//! detector emits a `NonAdditiveChange` variant. The caller panics
//! unless the change is accepted by `NON_ADDITIVE_OVERRIDES`.

use std::collections::BTreeSet;

use crate::canonical::{CanonicalSpec, EndpointKey};
use crate::parser::{ApiSpec, Field, FieldType, HttpMethod, TypeDef, TypeKind};

/// Returns `true` if the `spec` field type is a compatible evolution
/// of the `canonical` field type. Compatible means:
/// - Identical, or
/// - Both inline `Enum(a)` and `Enum(b)` where every variant in `a` is still
///   present in `b` (variant additions are additive, removals/renames are not), or
/// - Both `Opt<T>`, `List<T>`, or `Map<T>` and their inner types are compatible.
fn field_types_compatible(canonical: &FieldType, spec: &FieldType) -> bool {
    use FieldType::*;
    match (canonical, spec) {
        (Enum(a), Enum(b)) => a.iter().all(|v| b.contains(v)),
        (Opt(a), Opt(b)) => field_types_compatible(a, b),
        (List(a), List(b)) => field_types_compatible(a, b),
        (Map(a), Map(b)) => field_types_compatible(a, b),
        _ => canonical == spec,
    }
}

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
pub fn check(canonical: &CanonicalSpec, version: &str, spec: &ApiSpec) -> Vec<NonAdditiveChange> {
    let mut out = Vec::new();
    check_endpoint_removed(canonical, version, spec, &mut out);
    check_type_removed(canonical, version, spec, &mut out);
    check_field_removed_or_retyped(canonical, version, spec, &mut out);
    check_enum_variant_removed(canonical, version, spec, &mut out);
    out
}

fn check_type_removed(
    canonical: &CanonicalSpec,
    version: &str,
    spec: &ApiSpec,
    out: &mut Vec<NonAdditiveChange>,
) {
    let spec_type_names: BTreeSet<&str> = spec.all_types.iter().map(|t| t.name.as_str()).collect();
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

fn check_field_removed_or_retyped(
    canonical: &CanonicalSpec,
    version: &str,
    spec: &ApiSpec,
    out: &mut Vec<NonAdditiveChange>,
) {
    let spec_types: std::collections::BTreeMap<&str, &TypeDef> = spec
        .all_types
        .iter()
        .map(|t| (t.name.as_str(), t))
        .collect();

    for (type_name, canonical_type) in &canonical.types {
        let Some(spec_type) = spec_types.get(type_name.as_str()) else {
            continue;
        };
        let spec_fields: std::collections::BTreeMap<&str, &Field> = spec_type
            .fields
            .iter()
            .map(|f| (f.rust_name.as_str(), f))
            .collect();

        for (field_name, canonical_field) in &canonical_type.fields {
            match spec_fields.get(field_name.as_str()) {
                None => out.push(NonAdditiveChange::FieldRemoved {
                    type_name: type_name.clone(),
                    field: field_name.clone(),
                    previous_versions: canonical_field.versions.to_vec(),
                    missing_in: version.to_string(),
                }),
                Some(spec_field)
                    if !field_types_compatible(&canonical_field.ty, &spec_field.ty) =>
                {
                    out.push(NonAdditiveChange::FieldTypeChanged {
                        type_name: type_name.clone(),
                        field: field_name.clone(),
                        from: canonical_field.ty.clone(),
                        to: spec_field.ty.clone(),
                        previous_versions: canonical_field.versions.to_vec(),
                        changed_in: version.to_string(),
                    });
                }
                _ => {}
            }
        }
    }
}

fn check_enum_variant_removed(
    canonical: &CanonicalSpec,
    version: &str,
    spec: &ApiSpec,
    out: &mut Vec<NonAdditiveChange>,
) {
    let spec_types: std::collections::BTreeMap<&str, &TypeDef> = spec
        .all_types
        .iter()
        .map(|t| (t.name.as_str(), t))
        .collect();

    for (type_name, canonical_type) in &canonical.types {
        let Some(spec_type) = spec_types.get(type_name.as_str()) else {
            continue;
        };
        let TypeKind::StringEnum(spec_variants) = &spec_type.kind else {
            continue;
        };
        let spec_variant_set: BTreeSet<&str> = spec_variants.iter().map(String::as_str).collect();

        for (variant_name, canonical_variant) in &canonical_type.variants {
            if !spec_variant_set.contains(variant_name.as_str()) {
                out.push(NonAdditiveChange::EnumVariantRemoved {
                    enum_name: type_name.clone(),
                    variant: variant_name.clone(),
                    previous_versions: canonical_variant.versions.to_vec(),
                    missing_in: version.to_string(),
                });
            }
        }
    }
}

impl NonAdditiveChange {
    pub fn rule_name(&self) -> &'static str {
        match self {
            NonAdditiveChange::EndpointRemoved { .. } => "EndpointRemoved",
            NonAdditiveChange::TypeRemoved { .. } => "TypeRemoved",
            NonAdditiveChange::FieldRemoved { .. } => "FieldRemoved",
            NonAdditiveChange::FieldTypeChanged { .. } => "FieldTypeChanged",
            NonAdditiveChange::EnumVariantRemoved { .. } => "EnumVariantRemoved",
        }
    }

    pub fn panic_message(&self, spec_path: &str) -> String {
        let rule = self.rule_name();
        let (location, from, to, override_hint) = match self {
            NonAdditiveChange::EndpointRemoved {
                method,
                path,
                previous_versions,
                missing_in,
            } => (
                format!("{} {}", method.as_str(), path),
                format!("present in {}", previous_versions.join(", ")),
                format!("absent in {} ({})", missing_in, spec_path),
                format!(
                    "EndpointRemoved {{\n               method: \"{}\",\n               path:   \"{}\",\n               reason: \"<why this is safe>\",\n           }}",
                    method.as_str(),
                    path,
                ),
            ),
            NonAdditiveChange::TypeRemoved {
                type_name,
                previous_versions,
                missing_in,
            } => (
                type_name.clone(),
                format!("present in {}", previous_versions.join(", ")),
                format!("absent in {} ({})", missing_in, spec_path),
                format!(
                    "TypeRemoved {{\n               type_name: \"{}\",\n               reason:    \"<why this is safe>\",\n           }}",
                    type_name,
                ),
            ),
            NonAdditiveChange::FieldRemoved {
                type_name,
                field,
                previous_versions,
                missing_in,
            } => (
                format!("{}.{}", type_name, field),
                format!("present in {}", previous_versions.join(", ")),
                format!("absent in {} ({})", missing_in, spec_path),
                format!(
                    "FieldRemoved {{\n               type_name: \"{}\",\n               field:     \"{}\",\n               reason:    \"<why this is safe>\",\n           }}",
                    type_name, field,
                ),
            ),
            NonAdditiveChange::FieldTypeChanged {
                type_name,
                field,
                from,
                to,
                previous_versions,
                changed_in,
            } => (
                format!("{}.{}", type_name, field),
                format!("{:?} in {}", from, previous_versions.join(", ")),
                format!("{:?} in {} ({})", to, changed_in, spec_path),
                format!(
                    "FieldTypeChanged {{\n               type_name: \"{}\",\n               field:     \"{}\",\n               reason:    \"<why this is safe>\",\n           }}",
                    type_name, field,
                ),
            ),
            NonAdditiveChange::EnumVariantRemoved {
                enum_name,
                variant,
                previous_versions,
                missing_in,
            } => (
                format!("{}::{}", enum_name, variant),
                format!("present in {}", previous_versions.join(", ")),
                format!("absent in {} ({})", missing_in, spec_path),
                format!(
                    "EnumVariantRemoved {{\n               enum_name: \"{}\",\n               variant:   \"{}\",\n               reason:    \"<why this is safe>\",\n           }}",
                    enum_name, variant,
                ),
            ),
        };

        format!(
            "nifi-openapi-gen: non-additive change detected\n  \
             rule:     {rule}\n  \
             location: {location}\n  \
             from:     {from}\n  \
             to:       {to}\n\n  \
             The canonical superset codegen assumes NiFi's API evolves monotonically.\n  \
             This change is not additive and needs a human decision.\n\n  \
             How to proceed:\n    \
             1. If this is intentional and the canonical union should keep the old shape,\n       \
                add an override to crates/nifi-openapi-gen/src/non_additive_overrides.rs:\n           \
                    {override_hint}\n    \
             2. If the change is truly breaking and requires per-version dispatch for this\n       \
                endpoint or type, open an issue before proceeding — the canonical model\n       \
                cannot represent it."
        )
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
