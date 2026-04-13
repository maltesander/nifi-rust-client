//! Canonical-superset dynamic-mode emitter.
//!
//! Produces the dispatch-free dynamic surface into `$OUT_DIR/dynamic/`.
//! This is the canonical emitter after Phase 4b renamed `dynamic` → `dynamic`.

mod api;
mod availability;
mod index;
mod types;

pub use api::emit_api;
pub use availability::{emit_availability, endpoint_variant_name};
pub use index::EndpointIndex;
pub use types::emit_types;

use std::path::PathBuf;

use crate::canonical::CanonicalSpec;

/// Emit every file that lives under `$OUT_DIR/dynamic/`.
///
/// Returns relative paths under `dynamic/` and their contents. The caller
/// (build_api.rs) is responsible for prepending `$OUT_DIR/dynamic/` and
/// writing the bytes.
pub fn emit_dynamic(canonical: &CanonicalSpec) -> Vec<(PathBuf, String)> {
    let index = index::EndpointIndex::build(canonical);
    let mut files: Vec<(PathBuf, String)> = Vec::new();

    // availability.rs
    let availability_src = availability::emit_availability(canonical, &index);
    files.push((PathBuf::from("availability.rs"), availability_src));

    // types/*.rs
    for (filename, content) in types::emit_types(canonical) {
        files.push((PathBuf::from("types").join(filename), content));
    }

    // api/*.rs
    for (filename, content) in api::emit_api(canonical, &index) {
        files.push((PathBuf::from("api").join(filename), content));
    }

    // mod.rs that ties it all together. Declares the hand-written `client`
    // submodule that build.rs copies from src/dynamic/client.rs.
    let mod_src = "pub mod api;\npub mod availability;\npub mod client;\npub mod strategy;\npub mod types;\n\npub use availability::{DetectedVersion, Endpoint, LATEST_NIFI_VERSION};\npub use client::DynamicClient;\npub use strategy::VersionResolutionStrategy;\n";
    files.push((PathBuf::from("mod.rs"), mod_src.to_string()));

    files
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::canonicalize;
    use crate::parser::{ApiSpec, Endpoint, HttpMethod, TagGroup};

    fn minimal() -> ApiSpec {
        ApiSpec {
            tags: vec![TagGroup {
                tag: "Flow".to_string(),
                struct_name: "FlowApi".to_string(),
                module_name: "flow".to_string(),
                accessor_fn: "flow_api".to_string(),
                types: vec![],
                root_endpoints: vec![Endpoint {
                    method: HttpMethod::Get,
                    path: "/flow/about".to_string(),
                    fn_name: "get_about_info".to_string(),
                    raw_operation_id: "getAboutInfo".to_string(),
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
                    query_params: vec![],
                    success_responses: vec![],
                    error_responses: vec![],
                    security: None,
                }],
                sub_groups: vec![],
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn emit_dynamic_produces_expected_layout() {
        let canonical = canonicalize(&[("2.8.0".to_string(), minimal())]);
        let files = emit_dynamic(&canonical);
        let names: Vec<String> = files.iter().map(|(p, _)| p.display().to_string()).collect();
        assert!(names.contains(&"availability.rs".to_string()));
        assert!(names.iter().any(|n| n.starts_with("types/")));
        assert!(names.iter().any(|n| n.starts_with("api/")));
        assert!(names.contains(&"mod.rs".to_string()));
    }
}
