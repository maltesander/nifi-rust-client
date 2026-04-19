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

#[tokio::test]
async fn stream_forwards_range_header_and_accepts_206() {
    use wiremock::matchers::header;

    let server = MockServer::start().await;
    let payload: Vec<u8> = (0..128u8).collect();

    Mock::given(method("GET"))
        .and(path_regex(
            r"^/nifi-api/provenance-events/\d+/content/input$",
        ))
        .and(header("Range", "bytes=0-63"))
        .respond_with(
            ResponseTemplate::new(206)
                .set_body_bytes(payload[..64].to_vec())
                .insert_header("content-type", "application/octet-stream")
                .insert_header("content-range", "bytes 0-63/128"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = NifiClientBuilder::new(&server.uri())
        .unwrap()
        .build()
        .unwrap();

    let mut stream = client
        .provenanceevents()
        .get_input_content_stream("12345", None, Some("bytes=0-63"))
        .await
        .expect("partial content stream");
    let mut collected = Vec::new();
    while let Some(chunk) = stream.next().await {
        collected.extend_from_slice(&chunk.unwrap());
    }
    assert_eq!(collected, payload[..64]);
}
