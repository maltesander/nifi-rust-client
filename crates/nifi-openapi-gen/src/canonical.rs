//! Canonical (superset) view of all supported NiFi specs.
//!
//! Produced by merging every parsed `ApiSpec` in semver order. Each
//! endpoint, type, field, and enum variant carries a `VersionSet`
//! indicating which supported versions declare it. This is the
//! generator-internal source of truth consumed by future code emitters
//! (not yet wired up in this plan).

use std::collections::{BTreeMap, BTreeSet};

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

use crate::parser::{FieldType, HttpMethod, TypeKind};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EndpointKey {
    pub method: HttpMethod,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct CanonicalEndpoint {
    pub method: HttpMethod,
    pub path: String,
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
