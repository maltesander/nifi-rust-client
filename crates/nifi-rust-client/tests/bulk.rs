#![cfg(not(feature = "dynamic"))]

use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::bulk;
use serde_json::json;
use wiremock::matchers::{body_partial_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn start_process_group_sends_running_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/nifi-api/flow/process-groups/pg-1"))
        .and(body_partial_json(
            json!({ "id": "pg-1", "state": "RUNNING" }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "pg-1", "state": "RUNNING"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let result = bulk::start_process_group(&client, "pg-1").await.unwrap();
    assert_eq!(result.id.as_deref(), Some("pg-1"));
}

#[tokio::test]
async fn stop_process_group_sends_stopped_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/nifi-api/flow/process-groups/pg-1"))
        .and(body_partial_json(
            json!({ "id": "pg-1", "state": "STOPPED" }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "pg-1", "state": "STOPPED"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let result = bulk::stop_process_group(&client, "pg-1").await.unwrap();
    assert_eq!(result.id.as_deref(), Some("pg-1"));
}

#[tokio::test]
async fn enable_all_controller_services_sends_enabled_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(
            "/nifi-api/flow/process-groups/pg-1/controller-services",
        ))
        .and(body_partial_json(
            json!({ "id": "pg-1", "state": "ENABLED" }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "pg-1", "state": "ENABLED"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let result = bulk::enable_all_controller_services(&client, "pg-1")
        .await
        .unwrap();
    assert_eq!(result.id.as_deref(), Some("pg-1"));
}

#[tokio::test]
async fn disable_all_controller_services_sends_disabled_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(
            "/nifi-api/flow/process-groups/pg-1/controller-services",
        ))
        .and(body_partial_json(
            json!({ "id": "pg-1", "state": "DISABLED" }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "pg-1", "state": "DISABLED"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let result = bulk::disable_all_controller_services(&client, "pg-1")
        .await
        .unwrap();
    assert_eq!(result.id.as_deref(), Some("pg-1"));
}
