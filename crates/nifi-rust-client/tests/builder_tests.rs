#![cfg(not(feature = "dynamic"))]
use std::time::Duration;

use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── Construction ────────────────────────────────────────────────────────────

#[test]
fn builder_rejects_invalid_base_url() {
    let result = NifiClientBuilder::new("not a url");
    assert!(result.is_err());
}

const INVALID_PEM: &[u8] =
    b"-----BEGIN CERTIFICATE-----\nnot!valid!base64!!!\n-----END CERTIFICATE-----\n";

#[test]
fn builder_rejects_invalid_pem_certificate() {
    let result = NifiClientBuilder::new("https://nifi.example.com:8443")
        .unwrap()
        .add_root_certificate(INVALID_PEM)
        .build();
    assert!(result.is_err());
}

// ── Functional: builder-produced client can make requests ───────────────────

#[tokio::test]
async fn builder_client_can_make_requests() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": {}
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let result = client.flow().get_about_info().await;
    assert!(result.is_ok(), "{:?}", result);
}
