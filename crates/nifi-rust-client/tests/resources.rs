use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_resources_returns_list() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/resources"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "resources": [
                { "identifier": "/flow", "name": "Flow" },
                { "identifier": "/system", "name": "System" }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let entity = client.resources_api().get_resources().await.unwrap();

    assert!(entity.resources.as_deref().unwrap_or_default().len() >= 2);
}
