//! Wiremock tests that assert header params emitted by the dynamic
//! emitter are forwarded on the outgoing request. Mirror of
//! header_params.rs but exercising the DynamicClient path.

#![cfg(feature = "dynamic")]

use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Helper: mount the about mock and return a DynamicClient pointed at `server`.
#[allow(clippy::unwrap_used)]
async fn dynamic_client_on(server: &MockServer, version: &str) -> nifi_rust_client::dynamic::DynamicClient {
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": { "title": "NiFi", "version": version }
        })))
        .mount(server)
        .await;

    let client = NifiClientBuilder::new(&server.uri())
        .unwrap()
        .build()
        .unwrap();
    nifi_rust_client::dynamic::DynamicClient::from_client(client)
        .await
        .unwrap()
}

/// Asserts that `create_asset` on the dynamic path forwards the `Filename` header.
///
/// Mirrors the static `create_asset_forwards_filename_header` test.
#[tokio::test]
async fn dynamic_create_asset_forwards_filename_header() {
    let server = MockServer::start().await;
    let client = dynamic_client_on(&server, "2.9.0").await;

    Mock::given(method("POST"))
        .and(path("/nifi-api/connectors/connector-1/assets"))
        .and(header("Filename", "my-asset.nar"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({}))
                .insert_header("content-type", "application/json"),
        )
        .expect(1)
        .mount(&server)
        .await;

    client
        .connectors()
        .create_asset("connector-1", Some("my-asset.nar"), vec![0u8; 16])
        .await
        .expect("request should succeed with Filename header");
}

/// Asserts that `get_input_content` on the dynamic path forwards the `Range` header.
///
/// Mirrors the static `get_input_content_forwards_range_header` test.
#[tokio::test]
async fn dynamic_get_input_content_forwards_range_header() {
    let server = MockServer::start().await;
    let client = dynamic_client_on(&server, "2.9.0").await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/provenance-events/event-1/content/input"))
        .and(header("Range", "bytes=0-1023"))
        .respond_with(
            ResponseTemplate::new(206)
                .set_body_bytes(vec![0u8; 1024])
                .insert_header("content-type", "application/octet-stream"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let bytes = client
        .provenanceevents()
        .get_input_content("event-1", None, Some("bytes=0-1023"))
        .await
        .expect("request should succeed with Range header");

    assert_eq!(bytes.len(), 1024);
}
