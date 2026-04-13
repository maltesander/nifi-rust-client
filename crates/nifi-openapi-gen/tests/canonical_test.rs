use nifi_openapi_gen::canonical::{
    canonicalize, CanonicalEndpoint, CanonicalField, CanonicalSpec, CanonicalType, EndpointKey,
    VersionSet,
};
use nifi_openapi_gen::content_type::ResponseBodyKind;
use nifi_openapi_gen::parser::{ApiSpec, Endpoint, FieldType, HttpMethod, TagGroup, TypeKind};

#[test]
fn version_set_new_is_empty() {
    let vs = VersionSet::new();
    assert!(vs.is_empty());
    assert_eq!(vs.len(), 0);
}

#[test]
fn version_set_with_single_version() {
    let vs = VersionSet::with("2.8.0");
    assert!(vs.contains("2.8.0"));
    assert!(!vs.contains("2.9.0"));
    assert_eq!(vs.len(), 1);
}

#[test]
fn version_set_insert_and_iter_sorted() {
    let mut vs = VersionSet::new();
    vs.insert("2.9.0");
    vs.insert("2.6.0");
    vs.insert("2.8.0");
    let collected: Vec<&str> = vs.iter().map(String::as_str).collect();
    assert_eq!(collected, vec!["2.6.0", "2.8.0", "2.9.0"]);
}

#[test]
fn version_set_insert_duplicate_is_noop() {
    let mut vs = VersionSet::with("2.8.0");
    vs.insert("2.8.0");
    assert_eq!(vs.len(), 1);
}

#[test]
fn canonical_spec_empty_construction() {
    let canonical = CanonicalSpec::new();
    assert!(canonical.endpoints.is_empty());
    assert!(canonical.types.is_empty());
}

#[test]
fn endpoint_key_equality_on_method_and_path() {
    let a = EndpointKey { method: HttpMethod::Get, path: "/flow/about".into() };
    let b = EndpointKey { method: HttpMethod::Get, path: "/flow/about".into() };
    let c = EndpointKey { method: HttpMethod::Post, path: "/flow/about".into() };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn canonical_endpoint_tracks_versions() {
    let mut ep = CanonicalEndpoint {
        tag: "Flow".into(),
        raw_operation_id: "getAboutInfo".into(),
        versions: nifi_openapi_gen::canonical::VersionSet::with("2.6.0"),
    };
    ep.versions.insert("2.7.2");
    assert_eq!(ep.versions.len(), 2);
}

#[test]
fn canonical_type_has_fields_and_variants_maps() {
    let t = CanonicalType {
        name: "AboutDto".into(),
        kind: TypeKind::Dto,
        fields: std::collections::BTreeMap::new(),
        variants: std::collections::BTreeMap::new(),
        versions: nifi_openapi_gen::canonical::VersionSet::new(),
    };
    assert_eq!(t.name, "AboutDto");
    assert!(t.fields.is_empty());
}

#[test]
fn canonical_field_stores_field_type_and_versions() {
    let f = CanonicalField {
        name: "title".into(),
        ty: FieldType::Str,
        versions: nifi_openapi_gen::canonical::VersionSet::with("2.6.0"),
    };
    assert_eq!(f.ty, FieldType::Str);
}

fn make_endpoint(method: HttpMethod, path: &str, op_id: &str) -> Endpoint {
    Endpoint {
        method,
        path: path.to_string(),
        fn_name: "stub".to_string(),
        raw_operation_id: op_id.to_string(),
        doc: None,
        description: None,
        path_params: vec![],
        request_type: None,
        body_kind: None,
        body_doc: None,
        response_type: None,
        response_inner: None,
        response_field: None,
        response_kind: ResponseBodyKind::Empty,
        query_params: vec![],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    }
}

fn make_spec_with_endpoints(eps: Vec<Endpoint>) -> ApiSpec {
    ApiSpec {
        tags: vec![TagGroup {
            tag: "Flow".to_string(),
            struct_name: "FlowApi".to_string(),
            module_name: "flow".to_string(),
            accessor_fn: "flow_api".to_string(),
            types: vec![],
            root_endpoints: eps,
            sub_groups: vec![],
        }],
        all_types: vec![],
    }
}

#[test]
fn canonicalize_single_spec_keeps_endpoint_with_one_version() {
    let spec = make_spec_with_endpoints(vec![make_endpoint(
        HttpMethod::Get,
        "/flow/about",
        "getAboutInfo",
    )]);
    let canonical = canonicalize(&[("2.8.0".to_string(), spec)]);
    assert_eq!(canonical.endpoints.len(), 1);
    let key = EndpointKey {
        method: HttpMethod::Get,
        path: "/flow/about".into(),
    };
    let ep = canonical.endpoints.get(&key).unwrap();
    assert_eq!(ep.versions.to_vec(), vec!["2.8.0"]);
    assert_eq!(ep.tag, "Flow");
}

#[test]
fn canonicalize_two_specs_unions_endpoint_versions() {
    let spec_a = make_spec_with_endpoints(vec![make_endpoint(
        HttpMethod::Get,
        "/flow/about",
        "getAboutInfo",
    )]);
    let spec_b = make_spec_with_endpoints(vec![make_endpoint(
        HttpMethod::Get,
        "/flow/about",
        "getAboutInfo",
    )]);
    let canonical = canonicalize(&[
        ("2.8.0".to_string(), spec_a),
        ("2.9.0".to_string(), spec_b),
    ]);
    assert_eq!(canonical.endpoints.len(), 1);
    let ep = canonical
        .endpoints
        .get(&EndpointKey {
            method: HttpMethod::Get,
            path: "/flow/about".into(),
        })
        .unwrap();
    assert_eq!(ep.versions.to_vec(), vec!["2.8.0", "2.9.0"]);
}

#[test]
fn canonicalize_new_endpoint_in_later_version_only_has_later_version() {
    let spec_a = make_spec_with_endpoints(vec![make_endpoint(
        HttpMethod::Get,
        "/flow/about",
        "getAboutInfo",
    )]);
    let spec_b = make_spec_with_endpoints(vec![
        make_endpoint(HttpMethod::Get, "/flow/about", "getAboutInfo"),
        make_endpoint(HttpMethod::Get, "/connectors", "listConnectors"),
    ]);
    let canonical = canonicalize(&[
        ("2.8.0".to_string(), spec_a),
        ("2.9.0".to_string(), spec_b),
    ]);
    assert_eq!(canonical.endpoints.len(), 2);
    let new_ep = canonical
        .endpoints
        .get(&EndpointKey {
            method: HttpMethod::Get,
            path: "/connectors".into(),
        })
        .unwrap();
    assert_eq!(new_ep.versions.to_vec(), vec!["2.9.0"]);
}
