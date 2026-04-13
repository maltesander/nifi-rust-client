#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_version_information_returns_state() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/versions/process-groups/pg-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "versionControlInformation": {
                "groupId": "pg-id",
                "registryId": "reg-id",
                "registryName": "My Registry",
                "bucketId": "bucket-id",
                "bucketName": "My Bucket",
                "flowId": "flow-id",
                "flowName": "My Flow",
                "version": "1",
                "state": "UP_TO_DATE",
                "stateExplanation": "Flow is current"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let entity = client
        .versions()
        .get_version_information("pg-id")
        .await
        .unwrap();

    let vci = entity.version_control_information.as_ref().unwrap();
    assert_eq!(vci.group_id.as_deref(), Some("pg-id"));
    assert_eq!(vci.flow_name.as_deref(), Some("My Flow"));
    assert_eq!(vci.version.as_deref(), Some("1"));
}
