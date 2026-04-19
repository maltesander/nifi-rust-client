#![cfg(not(feature = "dynamic"))]

use futures_util::StreamExt;
use nifi_rust_client::NifiClientBuilder;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn about_json() -> serde_json::Value {
    json!({
        "about": {
            "title": "NiFi",
            "version": "2.9.0",
            "uri": "https://localhost:8443/nifi-api",
            "contentViewerUrl": "../nifi-content-viewer/",
            "timezone": "UTC",
            "buildTag": "nifi-2.9.0",
            "buildTimestamp": "2024-01-01T00:00:00Z"
        }
    })
}

#[tokio::test]
async fn dropping_stream_mid_body_does_not_poison_client() {
    let mock_server = MockServer::start().await;

    // Serve a 1 MiB body so chunked streaming is possible.
    let body = vec![0u8; 1 << 20];
    Mock::given(method("GET"))
        .and(path("/nifi-api/provenance-events/42/content/input"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(body)
                .insert_header("Content-Type", "application/octet-stream"),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    // Follow-up request to verify the client still works.
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(about_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri()).unwrap().build().unwrap();
    client.set_token("jwt".to_string()).await;

    let mut stream = client
        .provenanceevents()
        .get_input_content_stream("42", None, None)
        .await
        .expect("get input content stream");

    // Consume at least one chunk, then drop.
    let first_chunk = stream.next().await;
    assert!(first_chunk.is_some(), "expected at least one chunk");
    drop(stream);

    // Follow-up request must succeed — mid-body cancel should not poison the connection pool.
    let about = client.flow().get_about_info().await.unwrap();
    assert_eq!(about.version.as_deref(), Some("2.9.0"));
}
