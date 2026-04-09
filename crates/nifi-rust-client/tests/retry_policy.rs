#![cfg(not(feature = "dynamic"))]

use std::time::Duration;

use nifi_rust_client::config::credentials::StaticCredentials;
use nifi_rust_client::config::retry::RetryPolicy;
use nifi_rust_client::{NifiClientBuilder, NifiError};
use wiremock::matchers::{method, path};
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

fn fast_policy() -> RetryPolicy {
    RetryPolicy {
        max_retries: 3,
        initial_backoff: Duration::from_millis(10),
        max_backoff: Duration::from_millis(50),
    }
}

// -- retries on 503 then succeeds ─────────────────────────────────────────────

#[tokio::test]
async fn retries_on_503_then_succeeds() {
    let mock_server = MockServer::start().await;

    // First call returns 503, second returns 200.
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(503).set_body_string("Service Unavailable"))
        .up_to_n_times(1)
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(about_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .retry_policy(fast_policy())
        .build()
        .unwrap();

    let result = client.flow_api().get_about_info().await;
    assert!(result.is_ok(), "expected Ok after retry, got: {result:?}");
}

// -- stops after max retries ──────────────────────────────────────────────────

#[tokio::test]
async fn stops_after_max_retries() {
    let mock_server = MockServer::start().await;

    // Always returns 503. Expect 1 initial + 3 retries = 4 total calls.
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(503).set_body_string("Service Unavailable"))
        .expect(4)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .retry_policy(fast_policy())
        .build()
        .unwrap();

    let err = client.flow_api().get_about_info().await.unwrap_err();
    assert!(
        matches!(err, NifiError::Api { status, .. } if status == 503),
        "expected Api error with status 503, got: {err:?}"
    );
}

// -- no retry without policy ──────────────────────────────────────────────────

#[tokio::test]
async fn no_retry_without_policy() {
    let mock_server = MockServer::start().await;

    // 503 with no retry policy: expect exactly 1 call.
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(503).set_body_string("Service Unavailable"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let err = client.flow_api().get_about_info().await.unwrap_err();
    assert!(
        matches!(err, NifiError::Api { status, .. } if status == 503),
        "expected Api error with status 503, got: {err:?}"
    );
}

// -- does not retry non-retryable errors ──────────────────────────────────────

#[tokio::test]
async fn does_not_retry_non_retryable_errors() {
    let mock_server = MockServer::start().await;

    // 400 is not retryable: expect exactly 1 call.
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(400).set_body_string("Bad Request"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .retry_policy(fast_policy())
        .build()
        .unwrap();

    let err = client.flow_api().get_about_info().await.unwrap_err();
    assert!(
        matches!(err, NifiError::Api { status, .. } if status == 400),
        "expected Api error with status 400, got: {err:?}"
    );
}

// -- retry composes with auth refresh ─────────────────────────────────────────

#[tokio::test]
async fn retry_composes_with_auth_refresh() {
    let mock_server = MockServer::start().await;

    // Sequence: 503 → 401 → (login) → 200
    // Call 1: 503 (transient, triggers retry)
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(503).set_body_string("Service Unavailable"))
        .up_to_n_times(1)
        .expect(1)
        .mount(&mock_server)
        .await;

    // Call 2 (retry attempt 1): 401 (triggers auth refresh inside with_auth_retry)
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Token expired"))
        .up_to_n_times(1)
        .expect(1)
        .mount(&mock_server)
        .await;

    // Login to refresh token
    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(201).set_body_string("fresh-jwt-token"))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Call 3 (auth retry of call 2): 200
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(about_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .credential_provider(StaticCredentials::new("admin", "password"))
        .retry_policy(fast_policy())
        .build()
        .unwrap();

    client.set_token("expired-jwt".to_string()).await;

    let result = client.flow_api().get_about_info().await;
    assert!(
        result.is_ok(),
        "expected Ok after retry + auth refresh, got: {result:?}"
    );
    assert_eq!(client.token().await.as_deref(), Some("fresh-jwt-token"));
}
