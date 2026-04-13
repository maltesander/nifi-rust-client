//! Canonical (superset) view of all supported NiFi specs.
//!
//! Produced by merging every parsed `ApiSpec` in semver order. Each
//! endpoint, type, field, and enum variant carries a `VersionSet`
//! indicating which supported versions declare it. This is the
//! generator-internal source of truth consumed by future code emitters
//! (not yet wired up in this plan).

use std::collections::{BTreeMap, BTreeSet};

use crate::non_additive_detector::check;
use crate::non_additive_overrides::NonAdditiveOverrides;
use crate::parser::{ApiSpec, Endpoint, FieldType, HttpMethod, TagGroup, TypeDef, TypeKind};

/// Set of supported NiFi version strings (e.g. "2.8.0").
///
/// Kept as a sorted `BTreeSet<String>` so iteration order is stable
/// and deterministic, matching the existing pipeline's string-based
/// version keys (`generate.rs` uses `Vec<(String, ApiSpec)>`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VersionSet(BTreeSet<String>);

impl VersionSet {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn with(version: &str) -> Self {
        let mut set = BTreeSet::new();
        set.insert(version.to_string());
        Self(set)
    }

    pub fn insert(&mut self, version: &str) {
        self.0.insert(version.to_string());
    }

    pub fn contains(&self, version: &str) -> bool {
        self.0.contains(version)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.0.iter().cloned().collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EndpointKey {
    pub method: HttpMethod,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct CanonicalEndpoint {
    pub tag: String,
    pub raw_operation_id: String,
    pub versions: VersionSet,
}

#[derive(Debug, Clone)]
pub struct CanonicalField {
    pub name: String,
    pub ty: FieldType,
    pub versions: VersionSet,
}

#[derive(Debug, Clone)]
pub struct CanonicalVariant {
    pub wire_value: String,
    pub versions: VersionSet,
}

#[derive(Debug, Clone)]
pub struct CanonicalType {
    pub name: String,
    pub kind: TypeKind,
    pub fields: BTreeMap<String, CanonicalField>,
    pub variants: BTreeMap<String, CanonicalVariant>,
    pub versions: VersionSet,
}

#[derive(Debug, Clone, Default)]
pub struct CanonicalSpec {
    pub endpoints: BTreeMap<EndpointKey, CanonicalEndpoint>,
    pub types: BTreeMap<String, CanonicalType>,
}

impl CanonicalSpec {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Merge all supported specs into a single canonical superset.
///
/// Input is pre-ordered by caller (typically semver-sorted via
/// `util::discover_spec_versions`).
pub fn canonicalize(all_parsed: &[(String, ApiSpec)]) -> CanonicalSpec {
    let mut canonical = CanonicalSpec::new();
    for (version, spec) in all_parsed {
        merge_spec(&mut canonical, version, spec);
    }
    canonical
}

fn merge_spec(canonical: &mut CanonicalSpec, version: &str, spec: &ApiSpec) {
    for tag in &spec.tags {
        for endpoint in &tag.root_endpoints {
            merge_endpoint(canonical, version, tag, endpoint);
        }
        for sub in &tag.sub_groups {
            for endpoint in &sub.endpoints {
                merge_endpoint(canonical, version, tag, endpoint);
            }
        }
    }
    for type_def in &spec.all_types {
        merge_type(canonical, version, type_def);
    }
}

fn merge_endpoint(
    canonical: &mut CanonicalSpec,
    version: &str,
    tag: &TagGroup,
    endpoint: &Endpoint,
) {
    let key = EndpointKey {
        method: endpoint.method.clone(),
        path: endpoint.path.clone(),
    };
    canonical
        .endpoints
        .entry(key)
        .and_modify(|e| e.versions.insert(version))
        .or_insert_with(|| CanonicalEndpoint {
            tag: tag.tag.clone(),
            raw_operation_id: endpoint.raw_operation_id.clone(),
            versions: VersionSet::with(version),
        });
}

fn merge_type(canonical: &mut CanonicalSpec, version: &str, type_def: &TypeDef) {
    let entry = canonical
        .types
        .entry(type_def.name.clone())
        .or_insert_with(|| CanonicalType {
            name: type_def.name.clone(),
            kind: type_def.kind.clone(),
            fields: BTreeMap::new(),
            variants: BTreeMap::new(),
            versions: VersionSet::new(),
        });
    entry.versions.insert(version);

    for field in &type_def.fields {
        entry
            .fields
            .entry(field.rust_name.clone())
            .and_modify(|f| f.versions.insert(version))
            .or_insert_with(|| CanonicalField {
                name: field.rust_name.clone(),
                ty: field.ty.clone(),
                versions: VersionSet::with(version),
            });
    }

    if let TypeKind::StringEnum(variants) = &type_def.kind {
        for variant in variants {
            entry
                .variants
                .entry(variant.clone())
                .and_modify(|v| v.versions.insert(version))
                .or_insert_with(|| CanonicalVariant {
                    wire_value: variant.clone(),
                    versions: VersionSet::with(version),
                });
        }
    }
}

/// Canonicalize with non-additive-change safety net.
///
/// Runs the detector between each merge step. If any non-overridden
/// change is found, panics with a formatted message that names the
/// exact override entry the human should add.
///
/// `spec_path_of` maps a version string to a human-readable path (used
/// in panic messages). In production this is typically
/// `|v| format!("crates/nifi-openapi-gen/specs/{v}/nifi-api.json")`.
pub fn canonicalize_or_panic<F>(
    all_parsed: &[(String, ApiSpec)],
    spec_path_of: F,
    overrides: &NonAdditiveOverrides,
) -> CanonicalSpec
where
    F: Fn(&str) -> String,
{
    let mut canonical = CanonicalSpec::new();
    for (version, spec) in all_parsed {
        let residual: Vec<_> = check(&canonical, version, spec)
            .into_iter()
            .filter(|c| !overrides.allows(c))
            .collect();
        if !residual.is_empty() {
            let spec_path = spec_path_of(version);
            let msgs: Vec<String> =
                residual.iter().map(|c| c.panic_message(&spec_path)).collect();
            panic!("{}", msgs.join("\n\n"));
        }
        merge_spec(&mut canonical, version, spec);
    }
    canonical
}
