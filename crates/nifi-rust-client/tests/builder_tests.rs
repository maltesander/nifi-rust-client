use std::time::Duration;

use nifi_rust_client::NifiClientBuilder;
use url::Url;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── Construction ────────────────────────────────────────────────────────────

#[test]
fn builder_rejects_invalid_base_url() {
    let result = NifiClientBuilder::new("not a url");
    assert!(result.is_err());
}

#[test]
fn builder_builds_with_defaults() {
    let result = NifiClientBuilder::new("https://nifi.example.com:8443")
        .unwrap()
        .build();
    assert!(result.is_ok(), "{:?}", result);
}

#[test]
fn builder_builds_with_timeout_and_connect_timeout() {
    let result = NifiClientBuilder::new("https://nifi.example.com:8443")
        .unwrap()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(5))
        .build();
    assert!(result.is_ok(), "{:?}", result);
}

#[test]
fn builder_danger_accept_invalid_certs() {
    let result = NifiClientBuilder::new("https://nifi.example.com:8443")
        .unwrap()
        .danger_accept_invalid_certs(true)
        .build();
    assert!(result.is_ok(), "{:?}", result);
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

#[test]
fn builder_proxy_all_with_valid_url() {
    let proxy_url = Url::parse("http://proxy.example.com:3128").unwrap();
    let result = NifiClientBuilder::new("https://nifi.example.com:8443")
        .unwrap()
        .proxy(proxy_url)
        .build();
    assert!(result.is_ok(), "{:?}", result);
}

#[test]
fn builder_http_and_https_proxy() {
    let http = Url::parse("http://proxy.example.com:3128").unwrap();
    let https = Url::parse("http://proxy.example.com:3129").unwrap();
    let result = NifiClientBuilder::new("https://nifi.example.com:8443")
        .unwrap()
        .http_proxy(http)
        .https_proxy(https)
        .build();
    assert!(result.is_ok(), "{:?}", result);
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

    let result = client.flow_api().get_about_info().await;
    assert!(result.is_ok(), "{:?}", result);
}
