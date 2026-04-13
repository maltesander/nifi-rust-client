use nifi_openapi_gen::canonical::{canonicalize, CanonicalSpec};
use nifi_openapi_gen::content_type::ResponseBodyKind;
use nifi_openapi_gen::non_additive_detector::{check, NonAdditiveChange};
use nifi_openapi_gen::parser::{ApiSpec, Endpoint, Field, FieldType, HttpMethod, TagGroup, TypeDef, TypeKind};

#[test]
fn detector_on_empty_canonical_returns_empty() {
    let canonical = CanonicalSpec::new();
    let spec = ApiSpec {
        tags: vec![],
        all_types: vec![],
    };
    let changes: Vec<NonAdditiveChange> = check(&canonical, "2.6.0", &spec);
    assert!(changes.is_empty());
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
fn detects_endpoint_removed_in_next_version() {
    let spec_a = make_spec_with_endpoints(vec![make_endpoint(
        HttpMethod::Get,
        "/flow/about",
        "getAboutInfo",
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_endpoints(vec![]);
    let changes = check(&canonical, "2.7.2", &spec_b);

    assert_eq!(changes.len(), 1);
    match &changes[0] {
        NonAdditiveChange::EndpointRemoved {
            method,
            path,
            previous_versions,
            missing_in,
        } => {
            assert_eq!(*method, HttpMethod::Get);
            assert_eq!(path, "/flow/about");
            assert_eq!(previous_versions, &vec!["2.6.0".to_string()]);
            assert_eq!(missing_in, "2.7.2");
        }
        other => panic!("expected EndpointRemoved, got {other:?}"),
    }
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
fn does_not_report_added_endpoint() {
    let spec_a = make_spec_with_endpoints(vec![make_endpoint(
        HttpMethod::Get,
        "/flow/about",
        "getAboutInfo",
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_endpoints(vec![
        make_endpoint(HttpMethod::Get, "/flow/about", "getAboutInfo"),
        make_endpoint(HttpMethod::Get, "/connectors", "listConnectors"),
    ]);
    let changes = check(&canonical, "2.9.0", &spec_b);
    assert!(changes.is_empty());
}

#[test]
fn detects_type_removed_in_next_version() {
    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_types(vec![]);
    let changes = check(&canonical, "2.7.2", &spec_b);

    assert_eq!(changes.len(), 1);
    match &changes[0] {
        NonAdditiveChange::TypeRemoved {
            type_name,
            previous_versions,
            missing_in,
        } => {
            assert_eq!(type_name, "AboutDto");
            assert_eq!(previous_versions, &vec!["2.6.0".to_string()]);
            assert_eq!(missing_in, "2.7.2");
        }
        other => panic!("expected TypeRemoved, got {other:?}"),
    }
}
