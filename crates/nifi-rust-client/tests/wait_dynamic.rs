#![cfg(feature = "dynamic")]

use std::time::Duration;

use nifi_rust_client::wait::{processor_state_dynamic, ProcessorTargetState, WaitConfig};
use nifi_rust_client::NifiClientBuilder;
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

#[tokio::test]
async fn processor_state_dynamic_reaches_target() {
    let mock_server = MockServer::start().await;

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
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/processors/abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "component": { "id": "abc", "state": "RUNNING" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build_dynamic()
        .unwrap();
    client.detect_version().await.unwrap();
    client.inner().set_token("jwt".to_string()).await;

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
