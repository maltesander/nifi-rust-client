#![cfg(feature = "dynamic")]

use bytes::Bytes;
use futures_util::StreamExt;
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn dynamic_stream_chunks_concat_match_payload() {
    let server = MockServer::start().await;

    // /flow/about for version detection
    Mock::given(method("GET"))
        .and(path_regex(r"^/nifi-api/flow/about$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": { "version": "2.9.0" }
        })))
        .mount(&server)
        .await;

    let payload: Vec<u8> = (0..=255u8).cycle().take(8_192).collect();
    Mock::given(method("GET"))
        .and(path_regex(
            r"^/nifi-api/controller/nar-manager/nars/[^/]+/content$",
        ))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(payload.clone())
                .insert_header("content-type", "application/octet-stream"),
        )
        .mount(&server)
        .await;

    let client = NifiClientBuilder::new(&server.uri())
        .unwrap()
        .build_dynamic()
        .unwrap();
    client.detect_version().await.unwrap();

    let mut stream = client
        .controller()
        .download_nar_stream("nar-id-123")
        .await
        .unwrap();
    let mut collected = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk: Bytes = chunk.unwrap();
        collected.extend_from_slice(&chunk);
    }
    assert_eq!(collected, payload);
}
