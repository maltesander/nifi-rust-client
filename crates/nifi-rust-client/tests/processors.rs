use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::NifiError;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── get_processor ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn get_processor_returns_name_and_type() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/processors/proc-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "proc-id",
            "component": {
                "id": "proc-id",
                "name": "GenerateFlowFile",
                "type": "org.apache.nifi.processors.standard.GenerateFlowFile"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let proc = client
        .processors_api()
        .get_processor("proc-id")
        .await
        .unwrap();

    assert_eq!(proc.id.as_deref(), Some("proc-id"));
    assert_eq!(
        proc.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("GenerateFlowFile")
    );
    assert_eq!(
        proc.component.as_ref().and_then(|c| c.r#type.as_deref()),
        Some("org.apache.nifi.processors.standard.GenerateFlowFile")
    );
}

// ── update_processor ──────────────────────────────────────────────────────────

#[tokio::test]
async fn update_processor_sends_body_and_returns_entity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/nifi-api/processors/proc-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "proc-id",
            "component": { "id": "proc-id", "name": "RenameMe" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ProcessorEntity::default();
    let updated = client
        .processors_api()
        .update_processor("proc-id", &body)
        .await
        .unwrap();

    assert_eq!(updated.id.as_deref(), Some("proc-id"));
}

// ── behavior: 409 on running processor ───────────────────────────────────────

#[tokio::test]
async fn delete_running_processor_returns_409() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/processors/proc-id"))
        .respond_with(
            ResponseTemplate::new(409)
                .set_body_json(serde_json::json!({"message": "Cannot delete a running processor"})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client
        .processors_api()
        .delete_processor("proc-id", Some("1"), None, None)
        .await
        .unwrap_err();

    assert!(matches!(err, NifiError::Api { status: 409, .. }));
}
