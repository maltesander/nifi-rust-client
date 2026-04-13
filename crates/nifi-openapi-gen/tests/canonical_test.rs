use nifi_openapi_gen::canonical::{
    CanonicalEndpoint, CanonicalField, CanonicalSpec, CanonicalType, EndpointKey, VersionSet,
};
use nifi_openapi_gen::parser::{FieldType, HttpMethod, TypeKind};

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
