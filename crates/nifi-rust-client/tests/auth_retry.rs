#![cfg(not(feature = "dynamic"))]

use nifi_rust_client::config::credentials::StaticCredentials;
use nifi_rust_client::{NifiClientBuilder, NifiError};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// -- auto-refreshes token on 401 ──────────────────────────────────────────────

#[tokio::test]
async fn auto_refreshes_token_on_401() {
    let mock_server = MockServer::start().await;

    // First GET /flow/about returns 401 (expired token).
    // Then POST /access/token returns a fresh token.
    // Then retry of GET /flow/about returns 200.
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Token expired"))
        .up_to_n_times(1)
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(201).set_body_string("fresh-jwt-token"))
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": {
                "title": "NiFi",
                "version": "2.8.0",
                "uri": "https://localhost:8443/nifi-api",
                "contentViewerUrl": "../nifi-content-viewer/",
                "timezone": "UTC",
                "buildTag": "nifi-2.8.0",
                "buildTimestamp": "2024-01-01T00:00:00Z"
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .credential_provider(StaticCredentials::new("admin", "password"))
        .build()
        .unwrap();

    // Set an expired token.
    client.set_token("expired-jwt".to_string()).await;

    // The call should auto-refresh and succeed.
    let result = client.flow_api().get_about_info().await;
    assert!(result.is_ok(), "expected Ok, got: {result:?}");

    // Token should be the fresh one.
    assert_eq!(client.token().await.as_deref(), Some("fresh-jwt-token"));
}

// -- no retry without credential provider ─────────────────────────────────────

#[tokio::test]
async fn no_retry_without_credential_provider() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Token expired"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    client.set_token("expired-jwt".to_string()).await;

    // Without a credential provider, the 401 should propagate immediately.
    let err = client.flow_api().get_about_info().await.unwrap_err();
    assert!(
        matches!(err, NifiError::Unauthorized { .. }),
        "expected Unauthorized, got: {err:?}"
    );
}

// -- no infinite loop when refresh fails ──────────────────────────────────────

#[tokio::test]
async fn no_infinite_loop_when_refresh_fails() {
    let mock_server = MockServer::start().await;

    // GET returns 401.
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Token expired"))
        .expect(1) // only once: login_with_provider fails, so no retry of the original call
        .mount(&mock_server)
        .await;

    // POST /access/token also fails (e.g. credentials changed).
    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Bad credentials"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .credential_provider(StaticCredentials::new("admin", "wrong-password"))
        .build()
        .unwrap();

    client.set_token("expired-jwt".to_string()).await;

    // The refresh attempt fails with Auth error (login returns Auth, not Unauthorized).
    let err = client.flow_api().get_about_info().await.unwrap_err();
    assert!(
        matches!(err, NifiError::Auth { .. }),
        "expected Auth error from failed re-login, got: {err:?}"
    );
}

// -- login_with_provider uses credential provider ─────────────────────────────

#[tokio::test]
async fn login_with_provider_uses_credential_provider() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(201).set_body_string("provider-jwt-token"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .credential_provider(StaticCredentials::new("admin", "secret"))
        .build()
        .unwrap();

    client.login_with_provider().await.unwrap();
    assert_eq!(client.token().await.as_deref(), Some("provider-jwt-token"));
}

// -- login still works with explicit credentials ──────────────────────────────

#[tokio::test]
async fn login_still_works_with_explicit_credentials() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(201).set_body_string("explicit-jwt-token"))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Builder with no credential provider.
    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    // Explicit login still works.
    client.login("admin", "password").await.unwrap();
    assert_eq!(client.token().await.as_deref(), Some("explicit-jwt-token"));
}
