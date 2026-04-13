//! Canonical (superset) view of all supported NiFi specs.
//!
//! Produced by merging every parsed `ApiSpec` in semver order. Each
//! endpoint, type, field, and enum variant carries a `VersionSet`
//! indicating which supported versions declare it. This is the
//! generator-internal source of truth consumed by future code emitters
//! (not yet wired up in this plan).

use std::collections::BTreeSet;

/// Set of supported NiFi version strings (e.g. "2.8.0").
///
/// Kept as a sorted `BTreeSet<String>` so iteration order is stable
/// and deterministic, matching the existing pipeline's string-based
/// version keys (`generate.rs` uses `Vec<(String, ApiSpec)>`).
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Default for VersionSet {
    fn default() -> Self {
        Self::new()
    }
}
