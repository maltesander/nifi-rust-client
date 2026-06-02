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

#[tokio::test]
async fn get_access_policy_for_resource_preserves_multi_segment_resource() {
    // NiFi's `{resource}` path param is a multi-segment path (it carries
    // `pattern: ".+"` in the spec). Its `/` separators must reach NiFi
    // intact — they must NOT be percent-encoded to `%2F`, or NiFi won't
    // match the route. Regression test for the 0.13.0 path-encoding change.
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/policies/read/process-groups/root"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "policy-id",
            "component": {
                "id": "policy-id",
                "resource": "/process-groups/root",
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
        .get_access_policy_for_resource("read", "process-groups/root")
        .await
        .unwrap();

    assert_eq!(policy.id.as_deref(), Some("policy-id"));
    assert_eq!(
        policy
            .component
            .as_ref()
            .and_then(|c| c.resource.as_deref()),
        Some("/process-groups/root")
    );
}
