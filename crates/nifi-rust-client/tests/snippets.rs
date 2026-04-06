use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn create_snippet_returns_id() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/snippets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "snippet": {
                "id": "snippet-id",
                "parentGroupId": "pg-id",
                "processors": {},
                "connections": {},
                "inputPorts": {},
                "outputPorts": {},
                "processGroups": {},
                "remoteProcessGroups": {},
                "funnels": {},
                "labels": {}
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::SnippetEntity::default();
    let entity = client.snippets_api().create_snippet(&body).await.unwrap();

    assert_eq!(
        entity.snippet.as_ref().and_then(|s| s.id.as_deref()),
        Some("snippet-id")
    );
}
