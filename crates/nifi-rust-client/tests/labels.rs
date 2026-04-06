use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_label_returns_label_text() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/labels/label-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "label-id",
            "component": { "id": "label-id", "label": "Hello World", "width": 150.0, "height": 50.0 }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let label = client.labels_api().get_label("label-id").await.unwrap();

    assert_eq!(label.id.as_deref(), Some("label-id"));
    assert_eq!(
        label.component.as_ref().and_then(|c| c.label.as_deref()),
        Some("Hello World")
    );
}

#[tokio::test]
async fn create_label_returns_id() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/process-groups/pg-id/labels"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "new-label-id",
            "component": { "id": "new-label-id", "label": "test" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::LabelEntity::default();
    let label = client
        .processgroups_api()
        .labels("pg-id")
        .create_label(&body)
        .await
        .unwrap();

    assert_eq!(label.id.as_deref(), Some("new-label-id"));
}
