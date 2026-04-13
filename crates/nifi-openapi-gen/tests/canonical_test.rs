use std::path::Path;

use nifi_openapi_gen::canonical::{
    CanonicalEndpoint, CanonicalField, CanonicalSpec, CanonicalType, EndpointKey, VersionSet,
    canonicalize, project,
};
use nifi_openapi_gen::content_type::ResponseBodyKind;
use nifi_openapi_gen::emit::{emit_api, emit_types};
use nifi_openapi_gen::parser::{
    ApiSpec, Endpoint, Field, FieldType, HttpMethod, TagGroup, TypeDef, TypeKind, load,
};

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
fn version_set_iter_is_semver_sorted_not_lexicographic() {
    // 2.10.0 lexicographically sorts BEFORE 2.6.0 but in semver order
    // it sorts AFTER 2.9.0. This test locks in semver ordering.
    let mut vs = VersionSet::new();
    vs.insert("2.6.0");
    vs.insert("2.10.0");
    vs.insert("2.9.0");
    let collected: Vec<&str> = vs.iter().map(String::as_str).collect();
    assert_eq!(collected, vec!["2.6.0", "2.9.0", "2.10.0"]);
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
    let a = EndpointKey {
        method: HttpMethod::Get,
        path: "/flow/about".into(),
    };
    let b = EndpointKey {
        method: HttpMethod::Get,
        path: "/flow/about".into(),
    };
    let c = EndpointKey {
        method: HttpMethod::Post,
        path: "/flow/about".into(),
    };
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
            endpoints: eps,
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
    let canonical = canonicalize(&[("2.8.0".to_string(), spec_a), ("2.9.0".to_string(), spec_b)]);
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
    let canonical = canonicalize(&[("2.8.0".to_string(), spec_a), ("2.9.0".to_string(), spec_b)]);
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

fn make_field(name: &str, ty: FieldType) -> Field {
    Field {
        rust_name: name.to_string(),
        serde_name: name.to_string(),
        ty,
        doc: None,
        read_only: false,
        deprecated: false,
    }
}

fn make_type(name: &str, fields: Vec<Field>) -> TypeDef {
    TypeDef {
        name: name.to_string(),
        kind: TypeKind::Dto,
        fields,
        doc: None,
    }
}

fn make_spec_with_types(types: Vec<TypeDef>) -> ApiSpec {
    ApiSpec {
        tags: vec![],
        all_types: types,
    }
}

#[test]
fn canonicalize_single_type_tracks_version() {
    let spec = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let canonical = canonicalize(&[("2.8.0".to_string(), spec)]);
    assert_eq!(canonical.types.len(), 1);
    let t = canonical.types.get("AboutDto").unwrap();
    assert_eq!(t.versions.to_vec(), vec!["2.8.0"]);
    assert_eq!(t.fields.len(), 1);
    assert_eq!(
        t.fields.get("title").unwrap().versions.to_vec(),
        vec!["2.8.0"]
    );
}

#[test]
fn canonicalize_type_present_in_two_versions_unions_versions() {
    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let spec_b = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let canonical = canonicalize(&[("2.8.0".to_string(), spec_a), ("2.9.0".to_string(), spec_b)]);
    let t = canonical.types.get("AboutDto").unwrap();
    assert_eq!(t.versions.to_vec(), vec!["2.8.0", "2.9.0"]);
    assert_eq!(
        t.fields.get("title").unwrap().versions.to_vec(),
        vec!["2.8.0", "2.9.0"]
    );
}

#[test]
fn canonicalize_new_field_in_later_version_has_later_version_only() {
    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let spec_b = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![
            make_field("title", FieldType::Str),
            make_field("version", FieldType::Str),
        ],
    )]);
    let canonical = canonicalize(&[("2.8.0".to_string(), spec_a), ("2.9.0".to_string(), spec_b)]);
    let t = canonical.types.get("AboutDto").unwrap();
    assert_eq!(t.fields.len(), 2);
    assert_eq!(
        t.fields.get("title").unwrap().versions.to_vec(),
        vec!["2.8.0", "2.9.0"]
    );
    assert_eq!(
        t.fields.get("version").unwrap().versions.to_vec(),
        vec!["2.9.0"]
    );
}

fn make_enum_type(name: &str, variants: Vec<&str>) -> TypeDef {
    TypeDef {
        name: name.to_string(),
        kind: TypeKind::StringEnum(variants.into_iter().map(String::from).collect()),
        fields: vec![],
        doc: None,
    }
}

#[test]
fn canonicalize_enum_variants_union_across_versions() {
    let spec_a = make_spec_with_types(vec![make_enum_type(
        "ComponentType",
        vec!["PROCESSOR", "CONTROLLER_SERVICE"],
    )]);
    let spec_b = make_spec_with_types(vec![make_enum_type(
        "ComponentType",
        vec!["PROCESSOR", "CONTROLLER_SERVICE", "CONNECTOR"],
    )]);
    let canonical = canonicalize(&[("2.8.0".to_string(), spec_a), ("2.9.0".to_string(), spec_b)]);
    let t = canonical.types.get("ComponentType").unwrap();
    assert_eq!(t.variants.len(), 3);
    assert_eq!(
        t.variants.get("PROCESSOR").unwrap().versions.to_vec(),
        vec!["2.8.0", "2.9.0"]
    );
    assert_eq!(
        t.variants.get("CONNECTOR").unwrap().versions.to_vec(),
        vec!["2.9.0"]
    );
}

#[test]
fn canonicalize_real_spec_chain_monotonic_growth() {
    let specs_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("specs");
    let versions = ["2.6.0", "2.7.2", "2.8.0", "2.9.0"];
    let all_parsed: Vec<(String, ApiSpec)> = versions
        .iter()
        .map(|v| {
            let p = specs_dir.join(v).join("nifi-api.json");
            (v.to_string(), load(p.to_str().unwrap()))
        })
        .collect();

    let canonical = canonicalize(&all_parsed);

    // Sanity: we have hundreds of endpoints and types in the canonical spec.
    assert!(
        canonical.endpoints.len() >= 200,
        "expected at least 200 canonical endpoints, got {}",
        canonical.endpoints.len()
    );
    assert!(
        canonical.types.len() >= 200,
        "expected at least 200 canonical types, got {}",
        canonical.types.len()
    );

    // Every endpoint belongs to at least one version.
    for (key, ep) in &canonical.endpoints {
        assert!(
            !ep.versions.is_empty(),
            "endpoint {:?} {} has empty version set",
            key.method,
            key.path
        );
    }

    // Every type belongs to at least one version.
    for (name, t) in &canonical.types {
        assert!(
            !t.versions.is_empty(),
            "type {} has empty version set",
            name
        );
    }

    // At least one endpoint should be present in all four versions
    // (e.g. /flow/about has existed since 2.6.0).
    let flow_about_key = EndpointKey {
        method: HttpMethod::Get,
        path: "/flow/about".into(),
    };
    let flow_about = canonical
        .endpoints
        .get(&flow_about_key)
        .expect("GET /flow/about should be canonical");
    assert_eq!(
        flow_about.versions.to_vec(),
        vec!["2.6.0", "2.7.2", "2.8.0", "2.9.0"]
    );

    // At least one endpoint should be NEW in 2.9.0 (connectors).
    let connectors_key = EndpointKey {
        method: HttpMethod::Get,
        path: "/connectors".into(),
    };
    if let Some(connectors_ep) = canonical.endpoints.get(&connectors_key) {
        // Per NIFI_API_CHANGES.md, /connectors appeared in 2.9.0 only.
        assert!(connectors_ep.versions.contains("2.9.0"));
        assert!(!connectors_ep.versions.contains("2.6.0"));
    }
}

#[test]
fn api_spec_is_cloneable() {
    let original = ApiSpec {
        tags: vec![TagGroup {
            tag: "Flow".to_string(),
            struct_name: "FlowApi".to_string(),
            module_name: "flow".to_string(),
            accessor_fn: "flow_api".to_string(),
            types: vec![],
            endpoints: vec![],
        }],
        all_types: vec![],
    };
    let cloned = original.clone();
    assert_eq!(cloned.tags.len(), 1);
    assert_eq!(cloned.all_types.len(), 0);
    assert_eq!(cloned.tags[0].tag, "Flow");
}

#[test]
fn canonical_spec_new_has_empty_per_version_specs() {
    let canonical = CanonicalSpec::new();
    assert!(canonical.per_version_specs.is_empty());
}

#[test]
fn canonicalize_populates_per_version_specs() {
    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let spec_b = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![
            make_field("title", FieldType::Str),
            make_field("version", FieldType::Str),
        ],
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a), ("2.7.2".to_string(), spec_b)]);

    assert_eq!(canonical.per_version_specs.len(), 2);
    assert!(canonical.per_version_specs.contains_key("2.6.0"));
    assert!(canonical.per_version_specs.contains_key("2.7.2"));

    let stored_26 = canonical.per_version_specs.get("2.6.0").unwrap();
    assert_eq!(stored_26.all_types[0].fields.len(), 1);
    let stored_27 = canonical.per_version_specs.get("2.7.2").unwrap();
    assert_eq!(stored_27.all_types[0].fields.len(), 2);
}

#[test]
fn project_returns_spec_for_known_version() {
    let spec = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let canonical = canonicalize(&[("2.8.0".to_string(), spec)]);

    let projected = project(&canonical, "2.8.0").expect("version 2.8.0 present");
    assert_eq!(projected.all_types.len(), 1);
    assert_eq!(projected.all_types[0].name, "AboutDto");
}

#[test]
fn project_returns_none_for_unknown_version() {
    let spec = make_spec_with_types(vec![]);
    let canonical = canonicalize(&[("2.8.0".to_string(), spec)]);

    assert!(project(&canonical, "99.0.0").is_none());
}

#[test]
fn project_single_version_round_trip_byte_identical_emit_api() {
    let specs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs");
    let versions = ["2.6.0", "2.7.2", "2.8.0", "2.9.0"];

    for version in versions {
        let spec_path = specs_dir.join(version).join("nifi-api.json");
        let original = load(spec_path.to_str().unwrap());

        let canonical = canonicalize(&[(version.to_string(), original.clone())]);
        let projected = project(&canonical, version)
            .unwrap_or_else(|| panic!("project returned None for {version}"));

        let original_emit = emit_api(&original);
        let projected_emit = emit_api(projected);

        assert_eq!(
            original_emit, projected_emit,
            "emit_api output diverged for {version}"
        );
    }
}

#[test]
fn project_multi_version_round_trip_byte_identical_all_specs() {
    let specs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs");
    let versions = ["2.6.0", "2.7.2", "2.8.0", "2.9.0"];

    // Load each spec twice: once for the "original" baseline and once
    // for the canonicalize input. Avoids mutating a single parse and
    // comparing against itself.
    let baselines: Vec<(String, ApiSpec)> = versions
        .iter()
        .map(|v| {
            let path = specs_dir.join(v).join("nifi-api.json");
            (v.to_string(), load(path.to_str().unwrap()))
        })
        .collect();
    let inputs: Vec<(String, ApiSpec)> = versions
        .iter()
        .map(|v| {
            let path = specs_dir.join(v).join("nifi-api.json");
            (v.to_string(), load(path.to_str().unwrap()))
        })
        .collect();

    let canonical = canonicalize(&inputs);

    for (version, original) in &baselines {
        let projected = project(&canonical, version)
            .unwrap_or_else(|| panic!("project returned None for {version}"));

        let original_api = emit_api(original);
        let projected_api = emit_api(projected);
        assert_eq!(
            original_api, projected_api,
            "multi-version emit_api diverged for {version}"
        );

        let original_types = emit_types(original);
        let projected_types = emit_types(projected);
        assert_eq!(
            original_types, projected_types,
            "multi-version emit_types diverged for {version}"
        );
    }
}

#[test]
fn project_single_version_round_trip_byte_identical_emit_types() {
    let specs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs");
    let versions = ["2.6.0", "2.7.2", "2.8.0", "2.9.0"];

    for version in versions {
        let spec_path = specs_dir.join(version).join("nifi-api.json");
        let original = load(spec_path.to_str().unwrap());

        let canonical = canonicalize(&[(version.to_string(), original.clone())]);
        let projected = project(&canonical, version)
            .unwrap_or_else(|| panic!("project returned None for {version}"));

        let original_emit = emit_types(&original);
        let projected_emit = emit_types(projected);

        assert_eq!(
            original_emit, projected_emit,
            "emit_types output diverged for {version}"
        );
    }
}
