//! Build-time index over `CanonicalSpec` for dynamic emission.
//!
//! Each `CanonicalEndpoint` carries only `(tag, raw_operation_id, versions)`.
//! The emitters need the full per-version `Endpoint` struct (params, return
//! types, body kinds) — this module looks up the latest supporting version
//! and exposes it as `IndexedEndpoint`. It also computes per-query-param
//! `VersionSet`s by walking every `Endpoint` for the same `(method, path)`
//! across `per_version_specs`.

use std::collections::BTreeMap;

use crate::canonical::{CanonicalSpec, EndpointKey, VersionSet};
use crate::parser::{ApiSpec, Endpoint};

/// One canonical endpoint with the data needed to emit a method body.
#[derive(Debug)]
pub struct IndexedEndpoint<'a> {
    pub key: EndpointKey,
    /// Canonical tag (matches the tag in CanonicalEndpoint).
    pub tag: &'a str,
    /// Endpoint struct from the latest version that defines this endpoint.
    pub endpoint: &'a Endpoint,
    /// Versions that declare this endpoint.
    pub versions: VersionSet,
    /// Per-query-param `VersionSet` (only populated for params present in
    /// at least one supporting version). Keyed by the wire name.
    pub query_param_versions: BTreeMap<String, VersionSet>,
}

#[derive(Debug)]
pub struct EndpointIndex<'a> {
    pub endpoints: Vec<IndexedEndpoint<'a>>,
}

impl<'a> EndpointIndex<'a> {
    pub fn build(canonical: &'a CanonicalSpec) -> Self {
        let mut endpoints = Vec::new();
        for (key, canon) in &canonical.endpoints {
            let endpoint =
                find_latest_endpoint(canonical, key).expect("indexed endpoint exists in some spec");
            let qp_versions = collect_query_param_versions(canonical, key);
            endpoints.push(IndexedEndpoint {
                key: key.clone(),
                tag: &canon.tag,
                endpoint,
                versions: canon.versions.clone(),
                query_param_versions: qp_versions,
            });
        }
        Self { endpoints }
    }
}

/// Look up the `Endpoint` from the latest version that defines this `(method, path)`.
fn find_latest_endpoint<'a>(
    canonical: &'a CanonicalSpec,
    key: &EndpointKey,
) -> Option<&'a Endpoint> {
    let canon = canonical.endpoints.get(key)?;
    let versions: Vec<String> = canon.versions.to_vec();
    for version in versions.iter().rev() {
        let spec = canonical.per_version_specs.get(version)?;
        if let Some(found) = scan_spec(spec, key) {
            return Some(found);
        }
    }
    None
}

fn scan_spec<'a>(spec: &'a ApiSpec, key: &EndpointKey) -> Option<&'a Endpoint> {
    for tag in &spec.tags {
        for ep in &tag.endpoints {
            if ep.method == key.method && ep.path == key.path {
                return Some(ep);
            }
        }
    }
    None
}

fn collect_query_param_versions(
    canonical: &CanonicalSpec,
    key: &EndpointKey,
) -> BTreeMap<String, VersionSet> {
    let mut out: BTreeMap<String, VersionSet> = BTreeMap::new();
    for (version, spec) in &canonical.per_version_specs {
        if let Some(ep) = scan_spec(spec, key) {
            for qp in &ep.query_params {
                out.entry(qp.name.clone()).or_default().insert(version);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::canonicalize;
    use crate::parser::{ApiSpec, Endpoint, HttpMethod, QueryParam, QueryParamType, TagGroup};

    fn ep(method: HttpMethod, path: &str, qps: Vec<&str>) -> Endpoint {
        Endpoint {
            method,
            path: path.to_string(),
            fn_name: "stub".to_string(),
            raw_operation_id: "stub".to_string(),
            doc: None,
            description: None,
            path_params: vec![],
            request_type: None,
            body_kind: None,
            body_doc: None,
            response_type: None,
            response_inner: None,
            response_field: None,
            response_kind: crate::content_type::ResponseBodyKind::Empty,
            query_params: qps
                .into_iter()
                .map(|n| QueryParam {
                    name: n.to_string(),
                    rust_name: n.to_string(),
                    ty: QueryParamType::Str,
                    required: false,
                    doc: None,
                    enum_type_name: None,
                })
                .collect(),
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        }
    }

    fn spec_with(eps: Vec<Endpoint>) -> ApiSpec {
        ApiSpec {
            tags: vec![TagGroup {
                tag: "Flow".to_string(),
                struct_name: "Flow".to_string(),
                module_name: "flow".to_string(),
                accessor_fn: "flow".to_string(),
                types: vec![],
                endpoints: eps,
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn index_picks_latest_version_endpoint_and_collects_query_param_versions() {
        let v26 = spec_with(vec![ep(HttpMethod::Get, "/flow/about", vec![])]);
        let v29 = spec_with(vec![ep(HttpMethod::Get, "/flow/about", vec!["registries"])]);
        let canonical = canonicalize(&[("2.6.0".to_string(), v26), ("2.9.0".to_string(), v29)]);
        let index = EndpointIndex::build(&canonical);
        assert_eq!(index.endpoints.len(), 1);
        let ep = &index.endpoints[0];
        assert_eq!(ep.endpoint.query_params.len(), 1);
        assert_eq!(ep.endpoint.query_params[0].name, "registries");
        let registries_versions = ep.query_param_versions.get("registries").unwrap();
        assert!(!registries_versions.contains("2.6.0"));
        assert!(registries_versions.contains("2.9.0"));
    }
}
