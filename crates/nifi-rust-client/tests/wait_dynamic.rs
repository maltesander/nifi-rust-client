#![cfg(feature = "dynamic")]

use std::time::Duration;

use nifi_rust_client::dynamic::DynamicClient;
use nifi_rust_client::wait::{
    ControllerServiceTargetState, ProcessorTargetState, WaitConfig,
    controller_service_state_dynamic, parameter_context_update_dynamic, processor_state_dynamic,
    provenance_query_dynamic,
};
use nifi_rust_client::{NifiClientBuilder, NifiError};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn fast_config(timeout_ms: u64) -> WaitConfig {
    WaitConfig {
        timeout: Duration::from_millis(timeout_ms),
        poll_interval: Duration::from_millis(10),
        initial_delay: Duration::ZERO,
        cleanup: true,
    }
}

async fn mount_about(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "about": {
                "title": "NiFi",
                "version": "2.9.0",
                "uri": "https://localhost:8443/nifi-api",
                "contentViewerUrl": "../nifi-content-viewer/",
                "timezone": "UTC",
                "buildTag": "nifi-2.9.0",
                "buildTimestamp": "2024-01-01T00:00:00Z"
            }
        })))
        .mount(server)
        .await;
}

async fn dynamic_client(server: &MockServer) -> DynamicClient {
    let client = NifiClientBuilder::new(&server.uri())
        .unwrap()
        .build_dynamic()
        .unwrap();
    client.detect_version().await.unwrap();
    client.inner().set_token("jwt".to_string()).await;
    client
}

#[tokio::test]
async fn processor_state_dynamic_reaches_target() {
    let mock_server = MockServer::start().await;
    mount_about(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/processors/abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "component": { "id": "abc", "state": "RUNNING" }
        })))
        .mount(&mock_server)
        .await;

    let client = dynamic_client(&mock_server).await;

    let entity = processor_state_dynamic(
        &client,
        "abc",
        ProcessorTargetState::Running,
        fast_config(1000),
    )
    .await
    .unwrap();
    assert!(entity.component.is_some());
}

#[tokio::test]
async fn controller_service_state_dynamic_reaches_target() {
    let mock_server = MockServer::start().await;
    mount_about(&mock_server).await;

    // First ENABLING, then ENABLED — helper must poll through the transient state.
    Mock::given(method("GET"))
        .and(path("/nifi-api/controller-services/cs-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "component": { "id": "cs-1", "state": "ENABLING" }
        })))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/controller-services/cs-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "component": { "id": "cs-1", "state": "ENABLED" }
        })))
        .mount(&mock_server)
        .await;

    let client = dynamic_client(&mock_server).await;

    let entity = controller_service_state_dynamic(
        &client,
        "cs-1",
        ControllerServiceTargetState::Enabled,
        fast_config(1000),
    )
    .await
    .unwrap();
    assert!(entity.component.is_some());
}

#[tokio::test]
async fn parameter_context_update_dynamic_succeeds_and_cleans_up() {
    let mock_server = MockServer::start().await;
    mount_about(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/nifi-api/parameter-contexts/ctx-1/update-requests/req-1",
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "request": { "complete": false } })),
        )
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path(
            "/nifi-api/parameter-contexts/ctx-1/update-requests/req-1",
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "request": { "complete": true } })),
        )
        .mount(&mock_server)
        .await;

    // cleanup: true — DELETE fires exactly once after poll resolves.
    Mock::given(method("DELETE"))
        .and(path(
            "/nifi-api/parameter-contexts/ctx-1/update-requests/req-1",
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "request": { "complete": true } })),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = dynamic_client(&mock_server).await;

    let entity = parameter_context_update_dynamic(&client, "ctx-1", "req-1", fast_config(1000))
        .await
        .unwrap();
    assert_eq!(entity.request.and_then(|r| r.complete), Some(true));
}

#[tokio::test]
async fn parameter_context_update_dynamic_reports_failure() {
    let mock_server = MockServer::start().await;
    mount_about(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/nifi-api/parameter-contexts/ctx-1/update-requests/req-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "request": { "complete": true, "failureReason": "validation failed" }
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path(
            "/nifi-api/parameter-contexts/ctx-1/update-requests/req-1",
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "request": { "complete": true } })),
        )
        .mount(&mock_server)
        .await;

    let client = dynamic_client(&mock_server).await;

    let err = parameter_context_update_dynamic(&client, "ctx-1", "req-1", fast_config(1000))
        .await
        .unwrap_err();
    match err {
        NifiError::Api { status, message } => {
            assert_eq!(status, 500);
            assert!(message.contains("validation failed"));
        }
        other => panic!("expected Api, got {other:?}"),
    }
}

#[tokio::test]
async fn provenance_query_dynamic_succeeds_and_cleans_up() {
    let mock_server = MockServer::start().await;
    mount_about(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/provenance/q-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "provenance": { "id": "q-1", "finished": false, "percentCompleted": 50 }
        })))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/provenance/q-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "provenance": { "id": "q-1", "finished": true, "percentCompleted": 100 }
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/nifi-api/provenance/q-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "provenance": { "id": "q-1", "finished": true }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = dynamic_client(&mock_server).await;

    let dto = provenance_query_dynamic(&client, "q-1", fast_config(1000))
        .await
        .unwrap();
    assert_eq!(dto.finished, Some(true));
}
