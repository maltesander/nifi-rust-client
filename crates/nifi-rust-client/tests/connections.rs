#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::NifiError;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── get_connection ────────────────────────────────────────────────────────────

#[tokio::test]
async fn get_connection_returns_source_and_destination() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/connections/conn-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "conn-id",
            "sourceType": "PROCESSOR",
            "destinationType": "PROCESSOR",
            "component": {
                "id": "conn-id",
                "source": { "id": "src-id", "groupId": "pg-id", "type": "PROCESSOR" },
                "destination": { "id": "dst-id", "groupId": "pg-id", "type": "PROCESSOR" },
                "selectedRelationships": ["success"]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let conn = client
        .connections_api()
        .get_connection("conn-id")
        .await
        .unwrap();

    assert_eq!(conn.id.as_deref(), Some("conn-id"));
    assert_eq!(
        conn.component
            .as_ref()
            .and_then(|c| c.source.as_ref())
            .map(|s| s.id.as_str()),
        Some("src-id")
    );
    assert_eq!(
        conn.component
            .as_ref()
            .and_then(|c| c.destination.as_ref())
            .map(|d| d.id.as_str()),
        Some("dst-id")
    );
}

// ── update_connection ─────────────────────────────────────────────────────────

#[tokio::test]
async fn update_connection_sends_body_and_returns_entity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/nifi-api/connections/conn-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "conn-id",
            "sourceType": "PROCESSOR",
            "destinationType": "PROCESSOR",
            "component": { "id": "conn-id" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ConnectionEntity::default();
    let result = client
        .connections_api()
        .update_connection("conn-id", &body)
        .await;

    assert!(result.is_ok(), "{:?}", result);
}

// ── behavior: 409 on non-empty queue ─────────────────────────────────────────

#[tokio::test]
async fn delete_connection_with_queued_data_returns_409() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/connections/conn-id"))
        .respond_with(
            ResponseTemplate::new(409)
                .set_body_json(serde_json::json!({"message": "Connection contains queued data"})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client
        .connections_api()
        .delete_connection("conn-id", Some("1"), None, None)
        .await
        .unwrap_err();

    assert!(matches!(err, NifiError::Api { status: 409, .. }));
}
