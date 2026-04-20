#![cfg(not(feature = "dynamic"))]

use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::config::auth::PasswordAuth;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn about_json() -> serde_json::Value {
    json!({
        "about": {
            "title": "NiFi",
            "version": "2.9.0",
            "uri": "https://localhost:8443/nifi-api",
            "contentViewerUrl": "../nifi-content-viewer/",
            "timezone": "UTC",
            "buildTag": "nifi-2.9.0",
            "buildTimestamp": "2024-01-01T00:00:00Z"
        }
    })
}

#[tokio::test]
async fn concurrent_401s_trigger_single_reauth() {
    let mock_server = MockServer::start().await;

    // First two GETs get 401 (with 50ms delay to force overlap), subsequent GETs get 200.
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_string("Token expired")
                .set_delay(std::time::Duration::from_millis(50)),
        )
        .up_to_n_times(2)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(about_json()))
        .mount(&mock_server)
        .await;

    // CRITICAL: login endpoint should be hit exactly ONCE despite two concurrent 401s.
    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(201).set_body_string("fresh-jwt"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .auth_provider(PasswordAuth::new("admin", "pw"))
        .build()
        .unwrap();
    client.set_token("expired".to_string()).await;

    let flow = client.flow();
    let (r1, r2) = tokio::join!(flow.get_about_info(), flow.get_about_info(),);
    assert!(r1.is_ok(), "first call failed: {r1:?}");
    assert!(r2.is_ok(), "second call failed: {r2:?}");
    assert_eq!(client.token().await.as_deref(), Some("fresh-jwt"));
}
