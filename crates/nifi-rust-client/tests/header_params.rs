//! Wiremock tests that assert header params emitted by the
//! generator are forwarded on the outgoing request.
//!
//! Covers two distinct headers introduced by the B.5-B.7 emitter changes:
//!   - `Filename` — `POST /connectors/{id}/assets` (octet-stream upload)
//!   - `Range`    — `GET  /provenance-events/{id}/content/input` (byte-range download)

#![cfg(not(feature = "dynamic"))]

use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Asserts that `create_asset` forwards the `Filename` header to the server.
///
/// Accessor: `client.connectors()` → `Connectors<'_>`
/// Signature: `create_asset(&self, id: &str, filename: Option<&str>, data: Vec<u8>)`
#[tokio::test]
async fn create_asset_forwards_filename_header() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/nifi-api/connectors/connector-1/assets"))
        .and(header("Filename", "my-asset.nar"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"asset": {}}))
                .insert_header("content-type", "application/json"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = NifiClientBuilder::new(&server.uri())
        .unwrap()
        .build()
        .unwrap();

    client
        .connectors()
        .create_asset("connector-1", Some("my-asset.nar"), vec![0u8; 16])
        .await
        .expect("request should succeed with Filename header");
}

/// Asserts that `get_input_content` forwards the `Range` header to the server.
///
/// Accessor: `client.provenanceevents()` → `ProvenanceEvents<'_>`
/// Signature: `get_input_content(&self, id: &str, cluster_node_id: Option<&str>, range: Option<&str>)`
#[tokio::test]
async fn get_input_content_forwards_range_header() {
    let server = MockServer::start().await;

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

    let client = NifiClientBuilder::new(&server.uri())
        .unwrap()
        .build()
        .unwrap();

    let bytes = client
        .provenanceevents()
        .get_input_content("event-1", None, Some("bytes=0-1023"))
        .await
        .expect("request should succeed with Range header");

    assert_eq!(bytes.len(), 1024);
}
