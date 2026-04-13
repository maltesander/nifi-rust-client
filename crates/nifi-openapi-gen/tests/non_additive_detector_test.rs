use nifi_openapi_gen::canonical::{canonicalize, CanonicalSpec};
use nifi_openapi_gen::content_type::ResponseBodyKind;
use nifi_openapi_gen::non_additive_detector::{check, NonAdditiveChange};
use nifi_openapi_gen::parser::{ApiSpec, Endpoint, HttpMethod, TagGroup};

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
