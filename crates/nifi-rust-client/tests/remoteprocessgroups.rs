#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_remote_process_group_returns_target_uri() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/remote-process-groups/rpg-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "rpg-id",
            "component": {
                "id": "rpg-id",
                "name": "Remote NiFi",
                "targetUri": "https://remote-nifi:8443/nifi",
                "targetSecure": true,
                "activeRemoteInputPortCount": 0,
                "activeRemoteOutputPortCount": 0,
                "inactiveRemoteInputPortCount": 0,
                "inactiveRemoteOutputPortCount": 0
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let rpg = client
        .remoteprocessgroups()
        .get_remote_process_group("rpg-id")
        .await
        .unwrap();

    assert_eq!(rpg.id.as_deref(), Some("rpg-id"));
    assert_eq!(
        rpg.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("Remote NiFi")
    );
}

// ── clear_bulletins ───────────────────────────────────────────────────────────
// Added in NiFi 2.7.2 — gate so nifi-2-6-0 builds stay green.

#[cfg(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0"))]
#[tokio::test]
async fn clear_remote_process_group_bulletins_returns_cleared_count() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/nifi-api/remote-process-groups/some-id/bulletins/clear-requests",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bulletinsCleared": 5,
            "componentId": "some-id"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ClearBulletinsRequestEntity::default();
    let result = client
        .remoteprocessgroups()
        .clear_bulletins("some-id", &body)
        .await;

    assert!(
        result.is_ok(),
        "clear_remote_process_group_bulletins failed: {:?}",
        result.err()
    );
}
