//! Unit tests for HeaderParam parser collection.

use std::path::PathBuf;

fn spec_path(version: &str) -> PathBuf {
    let manifest = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR set when running cargo test");
    PathBuf::from(manifest)
        .join("specs")
        .join(version)
        .join("nifi-api.json")
}

#[test]
fn header_params_are_collected_for_filename_endpoint() {
    let spec = nifi_openapi_gen::parser::load(spec_path("2.9.0").to_str().unwrap());
    let endpoint = spec
        .tags
        .iter()
        .flat_map(|t| &t.endpoints)
        .find(|e| {
            e.path == "/connectors/{id}/assets"
                && matches!(e.method, nifi_openapi_gen::parser::HttpMethod::Post)
        })
        .expect("POST /connectors/{id}/assets should exist in 2.9.0");
    assert_eq!(endpoint.header_params.len(), 1);
    let hp = &endpoint.header_params[0];
    assert_eq!(hp.name, "Filename");
    assert_eq!(hp.rust_name, "filename");
    assert!(!hp.required);
}

#[test]
fn header_params_are_collected_for_range_endpoint() {
    let spec = nifi_openapi_gen::parser::load(spec_path("2.9.0").to_str().unwrap());
    let endpoint = spec
        .tags
        .iter()
        .flat_map(|t| &t.endpoints)
        .find(|e| {
            e.path == "/provenance-events/{id}/content/input"
                && matches!(e.method, nifi_openapi_gen::parser::HttpMethod::Get)
        })
        .expect("GET /provenance-events/{id}/content/input should exist in 2.9.0");
    assert_eq!(endpoint.header_params.len(), 1);
    let hp = &endpoint.header_params[0];
    assert_eq!(hp.name, "Range");
    assert_eq!(hp.rust_name, "range");
    assert!(!hp.required);
}

#[test]
fn endpoints_without_header_params_have_empty_vec() {
    let spec = nifi_openapi_gen::parser::load(spec_path("2.9.0").to_str().unwrap());
    // GET /flow/about has no header params
    let endpoint = spec
        .tags
        .iter()
        .flat_map(|t| &t.endpoints)
        .find(|e| {
            e.path == "/flow/about" && matches!(e.method, nifi_openapi_gen::parser::HttpMethod::Get)
        })
        .expect("GET /flow/about should exist in 2.9.0");
    assert!(endpoint.header_params.is_empty());
}

#[test]
fn all_filename_header_endpoints_collected() {
    let spec = nifi_openapi_gen::parser::load(spec_path("2.9.0").to_str().unwrap());
    let filename_endpoints: Vec<_> = spec
        .tags
        .iter()
        .flat_map(|t| &t.endpoints)
        .filter(|e| e.header_params.iter().any(|h| h.name == "Filename"))
        .collect();
    // POST /connectors/{id}/assets, POST /controller/nar-manager/nars/content,
    // POST /parameter-contexts/{contextId}/assets
    assert_eq!(filename_endpoints.len(), 3);
}
