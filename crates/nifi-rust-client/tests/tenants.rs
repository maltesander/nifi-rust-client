use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── create_user ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn create_user_returns_id_and_identity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/tenants/users"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "user-abc",
            "component": { "id": "user-abc", "identity": "alice@example.com" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::UserEntity::default();
    let user = client.tenants_api().create_user(&body).await.unwrap();

    assert_eq!(user.id.as_deref(), Some("user-abc"));
    assert_eq!(
        user.component.as_ref().and_then(|c| c.identity.as_deref()),
        Some("alice@example.com")
    );
}

// ── get_user ──────────────────────────────────────────────────────────────────

#[tokio::test]
async fn get_user_returns_identity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/tenants/users/user-abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "user-abc",
            "component": { "id": "user-abc", "identity": "alice@example.com" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let user = client.tenants_api().get_user("user-abc").await.unwrap();

    assert_eq!(
        user.component.as_ref().and_then(|c| c.identity.as_deref()),
        Some("alice@example.com")
    );
}

// ── get_user_group ────────────────────────────────────────────────────────────

#[tokio::test]
async fn get_user_group_returns_identity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/tenants/user-groups/group-xyz"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "group-xyz",
            "component": { "id": "group-xyz", "identity": "ops-team" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let group = client
        .tenants_api()
        .get_user_group("group-xyz")
        .await
        .unwrap();

    assert_eq!(
        group.component.as_ref().and_then(|c| c.identity.as_deref()),
        Some("ops-team")
    );
}
