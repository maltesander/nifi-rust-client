#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn submit_provenance_request_returns_id() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/provenance"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "provenance": {
                "id": "prov-query-id",
                "uri": "http://localhost/nifi-api/provenance/prov-query-id",
                "submissionTime": "01/01/2026 00:00:00 UTC",
                "expiration": "01/01/2026 01:00:00 UTC",
                "percentCompleted": 0,
                "finished": false,
                "request": { "maxResults": 1000 },
                "results": { "provenanceEvents": [], "total": "0", "totalCount": 0 }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ProvenanceEntity::default();
    let prov = client
        .provenance_api()
        .submit_provenance_request(&body)
        .await
        .unwrap();

    assert_eq!(prov.id.as_deref(), Some("prov-query-id"));
}
