#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_funnel_returns_id() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/funnels/funnel-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "funnel-id",
            "component": { "id": "funnel-id", "position": { "x": 0.0, "y": 0.0 } }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let funnel = client.funnels().get_funnel("funnel-id").await.unwrap();

    assert_eq!(funnel.id.as_deref(), Some("funnel-id"));
}

#[tokio::test]
async fn create_funnel_returns_id() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/process-groups/pg-id/funnels"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "new-funnel-id",
            "component": { "id": "new-funnel-id" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::FunnelEntity::default();
    let funnel = client
        .processgroups()
        .create_funnel("pg-id", &body)
        .await
        .unwrap();

    assert_eq!(funnel.id.as_deref(), Some("new-funnel-id"));
}
