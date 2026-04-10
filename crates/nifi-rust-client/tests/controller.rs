#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_controller_config_returns_max_timer_driven_thread_count() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/controller/config"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "component": {
                "maxTimerDrivenThreadCount": 10
            },
            "revision": { "version": 0 }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let config = client
        .controller_api()
        .get_controller_config()
        .await
        .unwrap();

    assert_eq!(
        config
            .component
            .as_ref()
            .and_then(|c| c.max_timer_driven_thread_count),
        Some(10)
    );
}

#[tokio::test]
async fn upload_nar_sends_octet_stream_content_type() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/controller/nar-manager/nars/content"))
        .and(header("content-type", "application/octet-stream"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"narSummary": {}})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .controller_api()
        .upload_nar(Some("test.nar"), vec![1u8, 2, 3])
        .await;
    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn upload_nar_sets_filename_header() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/controller/nar-manager/nars/content"))
        .and(header("Filename", "my-bundle.nar"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"narSummary": {}})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .controller_api()
        .upload_nar(Some("my-bundle.nar"), vec![0u8])
        .await;
    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn upload_nar_without_filename_still_sends_data() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/controller/nar-manager/nars/content"))
        .and(header("content-type", "application/octet-stream"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"narSummary": {}})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .controller_api()
        .upload_nar(None, vec![0xca, 0xfe])
        .await;
    assert!(result.is_ok(), "{:?}", result);
}

// ── bulletin-clear endpoints ──────────────────────────────────────────────────

#[tokio::test]
async fn clear_flow_analysis_rule_bulletins_returns_cleared_count() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/controller/flow-analysis-rules/some-id/bulletins/clear-requests"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bulletinsCleared": 2,
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
        .controller_api()
        .bulletins("some-id")
        .clear_flow_analysis_rule_bulletins(&body)
        .await;

    assert!(result.is_ok(), "clear_flow_analysis_rule_bulletins failed: {:?}", result.err());
}

#[tokio::test]
async fn clear_parameter_provider_bulletins_returns_cleared_count() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/controller/parameter-providers/some-id/bulletins/clear-requests"))
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
        .controller_api()
        .bulletins("some-id")
        .clear_parameter_provider_bulletins(&body)
        .await;

    assert!(result.is_ok(), "clear_parameter_provider_bulletins failed: {:?}", result.err());
}

#[tokio::test]
async fn clear_registry_client_bulletins_returns_cleared_count() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/controller/registry-clients/some-id/bulletins/clear-requests"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bulletinsCleared": 3,
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
        .controller_api()
        .bulletins("some-id")
        .clear_registry_client_bulletins(&body)
        .await;

    assert!(result.is_ok(), "clear_registry_client_bulletins failed: {:?}", result.err());
}
