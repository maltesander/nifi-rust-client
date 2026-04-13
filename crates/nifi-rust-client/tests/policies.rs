#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_access_policy_returns_resource_and_action() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/policies/policy-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "policy-id",
            "component": {
                "id": "policy-id",
                "resource": "/flow",
                "action": "read",
                "users": [],
                "userGroups": []
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let policy = client
        .policies()
        .get_access_policy("policy-id")
        .await
        .unwrap();

    assert_eq!(policy.id.as_deref(), Some("policy-id"));
    assert_eq!(
        policy
            .component
            .as_ref()
            .and_then(|c| c.resource.as_deref()),
        Some("/flow")
    );
}
