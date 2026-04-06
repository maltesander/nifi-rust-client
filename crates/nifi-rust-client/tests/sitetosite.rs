#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_site_to_site_details_returns_controller_id() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/site-to-site"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "controller": {
                "id": "ctrl-id",
                "name": "NiFi",
                "inputPortCount": 0,
                "outputPortCount": 0,
                "inputPorts": [],
                "outputPorts": []
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let ctrl = client
        .sitetosite_api()
        .get_site_to_site_details()
        .await
        .unwrap();

    assert_eq!(ctrl.id.as_deref(), Some("ctrl-id"));
    assert_eq!(ctrl.name.as_deref(), Some("NiFi"));
}
