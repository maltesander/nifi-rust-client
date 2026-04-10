#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_input_port_returns_name() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/input-ports/port-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "port-id",
            "component": { "id": "port-id", "name": "my-input-port" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let port = client
        .inputports_api()
        .get_input_port("port-id")
        .await
        .unwrap();

    assert_eq!(port.id.as_deref(), Some("port-id"));
    assert_eq!(
        port.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("my-input-port")
    );
}

#[tokio::test]
async fn create_input_port_returns_id() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/process-groups/pg-id/input-ports"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "new-port-id",
            "component": { "id": "new-port-id", "name": "in-port" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::PortEntity::default();
    let port = client
        .processgroups_api()
        .input_ports("pg-id")
        .create_input_port(&body)
        .await
        .unwrap();

    assert_eq!(port.id.as_deref(), Some("new-port-id"));
}

// ── clear_bulletins ───────────────────────────────────────────────────────────

#[tokio::test]
async fn clear_input_port_bulletins_returns_cleared_count() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/input-ports/some-id/bulletins/clear-requests"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bulletinsCleared": 1,
            "componentId": "some-id"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ClearBulletinsRequestEntity::default();
    let result = client
        .inputports_api()
        .bulletins("some-id")
        .clear_bulletins_2(&body)
        .await;

    assert!(result.is_ok(), "clear_input_port_bulletins failed: {:?}", result.err());
}
