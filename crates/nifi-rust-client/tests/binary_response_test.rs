#![cfg(not(feature = "dynamic"))]

use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Verifies that an `application/octet-stream` GET endpoint surfaces its raw
/// bytes to the caller instead of the pre-Task-3.2 `()` return type.
///
/// Uses `GET /controller/nar-manager/nars/{id}/content` because its accessor
/// chain (`controller().download_nar(id)`) is the simplest bytes-returning
/// path in the generated API.
#[tokio::test]
async fn nar_content_returns_bytes() {
    let server = MockServer::start().await;
    let payload: Vec<u8> = vec![0x01, 0x02, 0x03, 0x42];
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

    let bytes = client
        .controller()
        .download_nar("nar-id-123")
        .await
        .unwrap();
    assert_eq!(bytes, payload);
}
