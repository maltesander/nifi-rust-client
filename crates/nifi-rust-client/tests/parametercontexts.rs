#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── create_parameter_context ─────────────────────────────────────────────────

#[tokio::test]
async fn create_parameter_context_returns_id_and_name() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/parameter-contexts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "ctx-id",
            "component": {
                "id": "ctx-id",
                "name": "my-params",
                "description": "test context",
                "parameters": []
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ParameterContextEntity::default();
    let ctx = client
        .parametercontexts()
        .create_parameter_context(&body)
        .await
        .unwrap();

    assert_eq!(ctx.id.as_deref(), Some("ctx-id"));
    assert_eq!(
        ctx.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("my-params")
    );
}

// ── get_parameter_context ─────────────────────────────────────────────────────

#[tokio::test]
async fn get_parameter_context_returns_name_and_description() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/parameter-contexts/ctx-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "ctx-id",
            "component": {
                "id": "ctx-id",
                "name": "my-params",
                "description": "test context",
                "parameters": []
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let ctx = client
        .parametercontexts()
        .get_parameter_context("ctx-id", None)
        .await
        .unwrap();

    assert_eq!(
        ctx.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("my-params")
    );
    assert_eq!(
        ctx.component
            .as_ref()
            .and_then(|c| c.description.as_deref()),
        Some("test context")
    );
}
