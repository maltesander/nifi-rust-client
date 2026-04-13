#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_counters_returns_aggregate_snapshot() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/counters"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "counters": {
                "aggregateSnapshot": {
                    "generated": "01/01/2026 00:00:00 UTC",
                    "counters": []
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let counters = client.counters().get_counters(None, None).await.unwrap();

    assert!(counters.aggregate_snapshot.is_some());
}
