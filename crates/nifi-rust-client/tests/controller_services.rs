use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── get_controller_service ────────────────────────────────────────────────────

#[tokio::test]
async fn get_controller_service_returns_name_and_type() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/controller-services/svc-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "svc-id",
            "component": {
                "id": "svc-id",
                "name": "my-service",
                "type": "org.apache.nifi.dbcp.DBCPConnectionPool",
                "state": "DISABLED"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let svc = client
        .controller_services_api()
        .get_controller_service("svc-id", None)
        .await
        .unwrap();

    assert_eq!(svc.id.as_deref(), Some("svc-id"));
    assert_eq!(
        svc.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("my-service")
    );
    assert_eq!(
        svc.component.as_ref().and_then(|c| c.r#type.as_deref()),
        Some("org.apache.nifi.dbcp.DBCPConnectionPool")
    );
}

// ── update_run_status ─────────────────────────────────────────────────────────

#[tokio::test]
async fn update_run_status_sends_request_to_correct_path() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/nifi-api/controller-services/svc-id/run-status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "svc-id",
            "component": { "id": "svc-id", "name": "my-service", "state": "ENABLED" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ControllerServiceRunStatusEntity::default();
    let result = client
        .controller_services_api()
        .run_status("svc-id")
        .update_run_status_1(&body)
        .await;

    assert!(result.is_ok(), "{:?}", result);
}
