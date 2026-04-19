#![cfg(not(feature = "dynamic"))]

use std::time::Duration;

use nifi_rust_client::wait::{self, ControllerServiceTargetState, ProcessorTargetState, WaitConfig};
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

fn processor_entity(state: &str) -> serde_json::Value {
    json!({
        "component": {
            "id": "abc",
            "state": state,
        }
    })
}

fn controller_service_entity(state: &str) -> serde_json::Value {
    json!({
        "component": {
            "id": "cs-1",
            "state": state,
        }
    })
}

// ── processor_state ─────────────────────────────────────────────────────────

#[tokio::test]
async fn processor_state_reaches_target() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/processors/abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(processor_entity("STOPPED")))
        .up_to_n_times(2)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/processors/abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(processor_entity("RUNNING")))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri()).unwrap().build().unwrap();
    client.set_token("jwt".to_string()).await;

    let entity = wait::processor_state(
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
async fn processor_state_times_out() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/processors/abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(processor_entity("STOPPED")))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri()).unwrap().build().unwrap();
    client.set_token("jwt".to_string()).await;

    let err = wait::processor_state(
        &client,
        "abc",
        ProcessorTargetState::Running,
        fast_config(50),
    )
    .await
    .unwrap_err();
    match err {
        NifiError::Timeout { operation } => {
            assert!(operation.contains("wait_for_processor_state"));
            assert!(operation.contains("RUNNING"));
        }
        other => panic!("expected Timeout, got {other:?}"),
    }
}

// ── controller_service_state ────────────────────────────────────────────────

#[tokio::test]
async fn controller_service_state_reaches_target() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/controller-services/cs-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(controller_service_entity("ENABLING")))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/controller-services/cs-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(controller_service_entity("ENABLED")))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri()).unwrap().build().unwrap();
    client.set_token("jwt".to_string()).await;

    let entity = wait::controller_service_state(
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
async fn controller_service_state_times_out() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/controller-services/cs-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(controller_service_entity("DISABLED")))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri()).unwrap().build().unwrap();
    client.set_token("jwt".to_string()).await;

    let err = wait::controller_service_state(
        &client,
        "cs-1",
        ControllerServiceTargetState::Enabled,
        fast_config(50),
    )
    .await
    .unwrap_err();
    assert!(matches!(err, NifiError::Timeout { .. }));
}
