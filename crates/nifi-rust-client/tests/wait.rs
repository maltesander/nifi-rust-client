#![cfg(not(feature = "dynamic"))]

use std::time::Duration;

use nifi_rust_client::wait::{
    self, ControllerServiceTargetState, ProcessorTargetState, WaitConfig,
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

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
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

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
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
        .respond_with(
            ResponseTemplate::new(200).set_body_json(controller_service_entity("ENABLING")),
        )
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/controller-services/cs-1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(controller_service_entity("ENABLED")),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
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
        .respond_with(
            ResponseTemplate::new(200).set_body_json(controller_service_entity("DISABLED")),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
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

// ── parameter_context_update ────────────────────────────────────────────────

fn update_request_entity(complete: bool, failure: Option<&str>) -> serde_json::Value {
    let mut req = json!({ "complete": complete });
    if let Some(reason) = failure {
        req["failureReason"] = json!(reason);
    }
    json!({ "request": req })
}

#[tokio::test]
async fn parameter_context_update_succeeds_and_cleans_up() {
    let mock_server = MockServer::start().await;

    // First GET: incomplete. Second: complete.
    Mock::given(method("GET"))
        .and(path(
            "/nifi-api/parameter-contexts/ctx-1/update-requests/req-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(update_request_entity(false, None)))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path(
            "/nifi-api/parameter-contexts/ctx-1/update-requests/req-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(update_request_entity(true, None)))
        .mount(&mock_server)
        .await;

    // DELETE expected exactly once (cleanup: true).
    Mock::given(method("DELETE"))
        .and(path(
            "/nifi-api/parameter-contexts/ctx-1/update-requests/req-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(update_request_entity(true, None)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let entity = wait::parameter_context_update(&client, "ctx-1", "req-1", fast_config(1000))
        .await
        .unwrap();
    assert_eq!(entity.request.and_then(|r| r.complete), Some(true));
}

#[tokio::test]
async fn parameter_context_update_reports_failure() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/nifi-api/parameter-contexts/ctx-1/update-requests/req-1",
        ))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(update_request_entity(true, Some("validation failed"))),
        )
        .mount(&mock_server)
        .await;

    // DELETE still runs on failure when cleanup is on.
    Mock::given(method("DELETE"))
        .and(path(
            "/nifi-api/parameter-contexts/ctx-1/update-requests/req-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(update_request_entity(true, None)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let err = wait::parameter_context_update(&client, "ctx-1", "req-1", fast_config(1000))
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
async fn parameter_context_update_no_cleanup_when_disabled() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/nifi-api/parameter-contexts/ctx-1/update-requests/req-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(update_request_entity(true, None)))
        .mount(&mock_server)
        .await;

    // DELETE must NOT be called when cleanup: false.
    Mock::given(method("DELETE"))
        .and(path(
            "/nifi-api/parameter-contexts/ctx-1/update-requests/req-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(update_request_entity(true, None)))
        .expect(0)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let config = WaitConfig {
        timeout: Duration::from_millis(500),
        poll_interval: Duration::from_millis(10),
        initial_delay: Duration::ZERO,
        cleanup: false,
    };
    let entity = wait::parameter_context_update(&client, "ctx-1", "req-1", config)
        .await
        .unwrap();
    assert!(entity.request.and_then(|r| r.complete).unwrap_or(false));
}

// ── provenance_query ────────────────────────────────────────────────────────

fn provenance_entity(finished: bool) -> serde_json::Value {
    json!({
        "provenance": {
            "id": "q-1",
            "finished": finished,
            "percentCompleted": if finished { 100 } else { 50 },
        }
    })
}

#[tokio::test]
async fn provenance_query_succeeds_and_cleans_up() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/provenance/q-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(provenance_entity(false)))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/provenance/q-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(provenance_entity(true)))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/nifi-api/provenance/q-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(provenance_entity(true)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let dto = wait::provenance_query(&client, "q-1", fast_config(1000))
        .await
        .unwrap();
    assert_eq!(dto.finished, Some(true));
}

#[tokio::test]
async fn provenance_query_propagates_fetch_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/provenance/q-1"))
        .respond_with(ResponseTemplate::new(500).set_body_string("internal error"))
        .mount(&mock_server)
        .await;

    // Cleanup is still attempted regardless of poll outcome.
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/provenance/q-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(provenance_entity(true)))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let err = wait::provenance_query(&client, "q-1", fast_config(200))
        .await
        .unwrap_err();
    // Error might be Api(500) (first fetch) or Timeout (if retries keep hitting 500).
    // Accept both — the exact outcome depends on whether a retry policy is configured
    // (default is None, so Api(500) is most likely).
    assert!(
        matches!(err, NifiError::Api { status: 500, .. })
            || matches!(err, NifiError::Timeout { .. }),
        "expected Api(500) or Timeout, got: {err:?}"
    );
}

// ── flowfile_drop ───────────────────────────────────────────────────────────

fn drop_request_entity(finished: bool, failure: Option<&str>) -> serde_json::Value {
    let mut req = json!({
        "id": "drop-1",
        "finished": finished,
        "percentCompleted": if finished { 100 } else { 50 },
    });
    if let Some(reason) = failure {
        req["failureReason"] = json!(reason);
    }
    json!({ "dropRequest": req })
}

#[tokio::test]
async fn flowfile_drop_succeeds_and_cleans_up() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flowfile-queues/q-1/drop-requests/drop-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(drop_request_entity(false, None)))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flowfile-queues/q-1/drop-requests/drop-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(drop_request_entity(true, None)))
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/flowfile-queues/q-1/drop-requests/drop-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(drop_request_entity(true, None)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let dto = wait::flowfile_drop(&client, "q-1", "drop-1", fast_config(1000))
        .await
        .unwrap();
    assert_eq!(dto.finished, Some(true));
}

#[tokio::test]
async fn flowfile_drop_reports_failure() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flowfile-queues/q-1/drop-requests/drop-1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(drop_request_entity(true, Some("queue locked"))),
        )
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/flowfile-queues/q-1/drop-requests/drop-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(drop_request_entity(true, None)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let err = wait::flowfile_drop(&client, "q-1", "drop-1", fast_config(1000))
        .await
        .unwrap_err();
    match err {
        NifiError::Api { status, message } => {
            assert_eq!(status, 500);
            assert!(message.contains("drop request failed"));
            assert!(message.contains("queue locked"));
        }
        other => panic!("expected Api, got {other:?}"),
    }
}

#[tokio::test]
async fn flowfile_drop_no_cleanup_when_disabled() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flowfile-queues/q-1/drop-requests/drop-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(drop_request_entity(true, None)))
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/flowfile-queues/q-1/drop-requests/drop-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(drop_request_entity(true, None)))
        .expect(0)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let config = WaitConfig {
        timeout: Duration::from_millis(500),
        poll_interval: Duration::from_millis(10),
        initial_delay: Duration::ZERO,
        cleanup: false,
    };
    let dto = wait::flowfile_drop(&client, "q-1", "drop-1", config)
        .await
        .unwrap();
    assert_eq!(dto.finished, Some(true));
}

// ── flowfile_listing ────────────────────────────────────────────────────────

fn listing_request_entity(finished: bool, failure: Option<&str>) -> serde_json::Value {
    let mut req = json!({
        "id": "list-1",
        "finished": finished,
        "percentCompleted": if finished { 100 } else { 50 },
    });
    if let Some(reason) = failure {
        req["failureReason"] = json!(reason);
    }
    json!({ "listingRequest": req })
}

#[tokio::test]
async fn flowfile_listing_succeeds_and_cleans_up() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flowfile-queues/q-1/listing-requests/list-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(listing_request_entity(true, None)))
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/flowfile-queues/q-1/listing-requests/list-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(listing_request_entity(true, None)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let dto = wait::flowfile_listing(&client, "q-1", "list-1", fast_config(1000))
        .await
        .unwrap();
    assert_eq!(dto.finished, Some(true));
}

#[tokio::test]
async fn flowfile_listing_reports_failure() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flowfile-queues/q-1/listing-requests/list-1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(listing_request_entity(true, Some("queue too large"))),
        )
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/flowfile-queues/q-1/listing-requests/list-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(listing_request_entity(true, None)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let err = wait::flowfile_listing(&client, "q-1", "list-1", fast_config(1000))
        .await
        .unwrap_err();
    match err {
        NifiError::Api { status, message } => {
            assert_eq!(status, 500);
            assert!(message.contains("listing request failed"));
            assert!(message.contains("queue too large"));
        }
        other => panic!("expected Api, got {other:?}"),
    }
}

// ── empty_all_connections ───────────────────────────────────────────────────

#[tokio::test]
async fn empty_all_connections_succeeds_and_cleans_up() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/process-groups/pg-1/empty-all-connections-requests/drop-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(drop_request_entity(true, None)))
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/process-groups/pg-1/empty-all-connections-requests/drop-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(drop_request_entity(true, None)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let dto = wait::empty_all_connections(&client, "pg-1", "drop-1", fast_config(1000))
        .await
        .unwrap();
    assert_eq!(dto.finished, Some(true));
}

#[tokio::test]
async fn empty_all_connections_reports_failure() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/process-groups/pg-1/empty-all-connections-requests/drop-1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(drop_request_entity(true, Some("connection in use"))),
        )
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/process-groups/pg-1/empty-all-connections-requests/drop-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(drop_request_entity(true, None)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let err = wait::empty_all_connections(&client, "pg-1", "drop-1", fast_config(1000))
        .await
        .unwrap_err();
    assert!(matches!(err, NifiError::Api { status: 500, .. }));
}

// ── provenance_lineage ──────────────────────────────────────────────────────

fn lineage_entity(finished: bool) -> serde_json::Value {
    json!({
        "lineage": {
            "id": "lin-1",
            "finished": finished,
            "percentCompleted": if finished { 100 } else { 50 },
        }
    })
}

#[tokio::test]
async fn provenance_lineage_succeeds_and_cleans_up() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/provenance/lineage/lin-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(lineage_entity(false)))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/provenance/lineage/lin-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(lineage_entity(true)))
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/provenance/lineage/lin-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(lineage_entity(true)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let dto = wait::provenance_lineage(&client, "lin-1", fast_config(1000))
        .await
        .unwrap();
    assert_eq!(dto.finished, Some(true));
}

// ── *_verify_config helpers (shared fixture) ────────────────────────────────

fn verify_config_request_entity(complete: bool, failure: Option<&str>) -> serde_json::Value {
    let mut req = json!({
        "requestId": "req-1",
        "complete": complete,
        "percentCompleted": if complete { 100 } else { 50 },
    });
    if let Some(reason) = failure {
        req["failureReason"] = json!(reason);
    }
    json!({ "request": req })
}

#[tokio::test]
async fn processor_verify_config_succeeds_and_cleans_up() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/processors/p-1/config/verification-requests/req-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(verify_config_request_entity(true, None)))
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/processors/p-1/config/verification-requests/req-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(verify_config_request_entity(true, None)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let dto = wait::processor_verify_config(&client, "p-1", "req-1", fast_config(1000))
        .await
        .unwrap();
    assert_eq!(dto.complete, Some(true));
}

#[tokio::test]
async fn processor_verify_config_reports_failure() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/processors/p-1/config/verification-requests/req-1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(verify_config_request_entity(true, Some("invalid prop"))),
        )
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/processors/p-1/config/verification-requests/req-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(verify_config_request_entity(true, None)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;

    let err = wait::processor_verify_config(&client, "p-1", "req-1", fast_config(1000))
        .await
        .unwrap_err();
    match err {
        NifiError::Api { status, message } => {
            assert_eq!(status, 500);
            assert!(message.contains("verification failed"));
            assert!(message.contains("invalid prop"));
        }
        other => panic!("expected Api, got {other:?}"),
    }
}
