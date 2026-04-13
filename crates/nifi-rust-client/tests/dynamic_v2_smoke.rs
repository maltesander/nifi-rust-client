//! Phase 4a smoke tests for the parallel canonical dynamic client.
//!
//! Reaches `DynamicClientV2` via the `#[doc(hidden)]` `dynamic_v2` module.
//! Verifies end-to-end: build → link → call → response decode.

#![cfg(feature = "dynamic")]

use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::dynamic_v2::DynamicClientV2;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn make_client(server: &MockServer) -> DynamicClientV2 {
    let inner = NifiClientBuilder::new(&server.uri())
        .expect("builder")
        .build()
        .expect("client");
    DynamicClientV2::new(inner)
}

#[tokio::test]
async fn dynamic_v2_get_about_info_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": { "version": "2.8.0", "title": "NiFi" }
        })))
        .mount(&server)
        .await;

    let client = make_client(&server).await;
    // detect_version primes the version cache via GET /flow/about.
    let v = client.detect_version().await.expect("detect");
    assert_eq!(v, "2.8.0");

    // get_about_info re-uses the cached version (require_endpoint) and
    // issues GET /flow/about, returning the canonical AboutDto.
    let about = client
        .flow_api()
        .get_about_info()
        .await
        .expect("get_about_info");
    // Canonical AboutDto has every field as Option<T>.
    assert_eq!(about.version.as_deref(), Some("2.8.0"));
    assert_eq!(about.title.as_deref(), Some("NiFi"));
}

#[tokio::test]
async fn dynamic_v2_unsupported_endpoint_error() {
    // Pick an endpoint that exists in newer spec(s) but not in 2.6.0. The exact
    // endpoint depends on the supported spec set; walk the availability table
    // to find one dynamically so the assertion is robust across spec bumps.
    use nifi_rust_client::dynamic_v2::availability::{ENDPOINT_AVAILABILITY, Endpoint};

    let target: Option<Endpoint> = ENDPOINT_AVAILABILITY
        .iter()
        .find(|(_, versions)| !versions.iter().any(|v| *v == "2.6.0"))
        .map(|(e, _)| *e);
    let Some(target) = target else {
        // No version-skewed endpoint in the current spec set; nothing to assert.
        eprintln!("skipping: no endpoint missing in 2.6.0");
        return;
    };

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": { "version": "2.6.0" }
        })))
        .mount(&server)
        .await;

    let client = make_client(&server).await;
    client.detect_version().await.expect("detect");

    // Use the public supports() helper rather than calling the method
    // directly — calling it would require enumerating every possible target
    // endpoint signature. supports() exercises the same lookup path.
    assert!(
        !client.supports(target),
        "expected {target:?} unsupported in 2.6.0"
    );

    // Manual require_endpoint round-trip: should produce UnsupportedEndpoint.
    let err = client
        .require_endpoint(target)
        .await
        .expect_err("expected UnsupportedEndpoint");
    match err {
        nifi_rust_client::NifiError::UnsupportedEndpoint { version, .. } => {
            assert_eq!(version, "2.6.0");
        }
        other => panic!("expected UnsupportedEndpoint, got {other:?}"),
    }
}

#[tokio::test]
async fn dynamic_v2_unsupported_query_param_error() {
    use nifi_rust_client::dynamic_v2::availability::{
        QUERY_PARAM_AVAILABILITY, query_param_supported,
    };

    // Find any (endpoint, param) pair where 2.6.0 is NOT in the supported set.
    let target = QUERY_PARAM_AVAILABILITY
        .iter()
        .find(|((_, _), versions)| !versions.iter().any(|v| *v == "2.6.0"))
        .copied();
    let Some(((endpoint, param), _versions)) = target else {
        eprintln!("skipping: no version-skewed query param in current spec set");
        return;
    };

    // Exercise the helper directly. The generated emit_method in Task 7
    // already proves the guard is wired into method bodies at compile time.
    let supported = query_param_supported(endpoint, param, "2.6.0");
    assert!(
        !supported,
        "expected {param} unsupported on {endpoint:?} in 2.6.0"
    );

    // Sanity check: the same helper returns true for a version that supports it.
    let supported_in_29 = query_param_supported(endpoint, param, "2.9.0");
    assert!(
        supported_in_29,
        "expected {param} supported on {endpoint:?} in 2.9.0"
    );
}
