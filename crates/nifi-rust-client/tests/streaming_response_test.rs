#![cfg(not(feature = "dynamic"))]

use bytes::Bytes;
use futures_util::StreamExt;
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Asserts a streamed download concatenates to the same bytes as the
/// buffered variant. Uses `/controller/nar-manager/nars/{id}/content`
/// because its emitted accessor chain is the simplest bytes path.
#[tokio::test]
async fn stream_chunks_concat_match_payload() {
    let server = MockServer::start().await;
    let payload: Vec<u8> = (0..=255u8).cycle().take(16_384).collect();
    Mock::given(method("GET"))
        .and(path_regex(
            r"^/nifi-api/controller/nar-manager/nars/[^/]+/content$",
        ))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(payload.clone())
                .insert_header("content-type", "application/octet-stream"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = NifiClientBuilder::new(&server.uri())
        .unwrap()
        .build()
        .unwrap();

    let mut stream = client
        .controller()
        .download_nar_stream("nar-id-123")
        .await
        .expect("start stream");
    let mut collected: Vec<u8> = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk: Bytes = chunk.expect("stream chunk");
        collected.extend_from_slice(&chunk);
    }
    assert_eq!(collected, payload);
}
