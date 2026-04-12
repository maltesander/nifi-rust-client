#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn about_json() -> serde_json::Value {
    serde_json::json!({
        "about": {
            "title": "NiFi",
            "version": "2.8.0",
            "uri": "https://localhost:8443/nifi-api",
            "contentViewerUrl": "../nifi-content-viewer/",
            "timezone": "UTC",
            "buildTag": "nifi-2.8.0",
            "buildTimestamp": "2024-01-01T00:00:00Z"
        }
    })
}

#[tokio::test]
async fn proxied_entities_chain_header_sent_on_requests() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .and(header("X-ProxiedEntitiesChain", "<alice><bob>"))
        .respond_with(ResponseTemplate::new(200).set_body_json(about_json()))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .proxied_entities_chain("<alice><bob>")
        .build()
        .unwrap();
    client.set_token("some-jwt".to_string()).await;

    let result = client.flow_api().get_about_info().await;
    assert!(result.is_ok(), "expected success but got: {:?}", result.err());
}

#[tokio::test]
async fn no_proxied_entities_header_by_default() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(about_json()))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("some-jwt".to_string()).await;

    let result = client.flow_api().get_about_info().await;
    assert!(result.is_ok(), "expected success but got: {:?}", result.err());
}
