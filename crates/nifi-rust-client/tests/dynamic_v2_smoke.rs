//! Phase 4a smoke tests for the parallel canonical dynamic client.
//!
//! Reaches `DynamicClientV2` via the `#[doc(hidden)]` `dynamic_v2` module.
//! Verifies end-to-end: build → link → call → response decode.

#![cfg(feature = "dynamic")]

use nifi_rust_client::dynamic_v2::DynamicClientV2;
use nifi_rust_client::NifiClientBuilder;
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
