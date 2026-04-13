#![cfg(not(feature = "dynamic"))]

use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn flow_client_id_returns_text() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/client-id"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("abc-123")
                .insert_header("content-type", "text/plain"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = NifiClientBuilder::new(&server.uri())
        .unwrap()
        .build()
        .unwrap();

    let id = client.flow().generate_client_id().await.unwrap();
    assert_eq!(id, "abc-123");
}
