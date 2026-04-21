#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::{NifiClientBuilder, NifiError};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// -- login stores token on success ─────────────────────────────────────────────

#[tokio::test]
async fn login_stores_token_on_success() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(201).set_body_string("eyJhbGciOiJSUzI1NiJ9.test-token"))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    assert!(client.token().await.is_none());
    client.login("admin", "password").await.unwrap();
    assert_eq!(
        client.token().await.as_deref(),
        Some("eyJhbGciOiJSUzI1NiJ9.test-token")
    );
}

// -- login returns NifiError::Auth on failure ──────────────────────────────────

#[tokio::test]
async fn login_returns_auth_error_on_401() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Bad credentials"))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.login("admin", "wrong").await.unwrap_err();

    assert!(
        matches!(err, NifiError::Auth { .. }),
        "expected NifiError::Auth, got: {err:?}"
    );
    assert!(client.token().await.is_none());
}

/// Covers the `delete` helper + the no-response-body return shape
/// (`log_out` → `Result<(), NifiError>`).
#[tokio::test]
async fn log_out_returns_ok_on_success() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/access/logout"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;
    client.access().log_out().await.unwrap();
}

/// Covers the `get_void` helper's redirect-as-success path:
/// NiFi responds with 302 once logout completes, and the helper
/// accepts it alongside 2xx.
#[tokio::test]
async fn log_out_complete_treats_302_as_success() {
    let mock_server = MockServer::start().await;
    // 302 with NO Location header — reqwest returns the 302 response
    // as-is rather than following, so the helper sees the redirect.
    Mock::given(method("GET"))
        .and(path("/nifi-api/access/logout/complete"))
        .respond_with(ResponseTemplate::new(302))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;
    client.access().log_out_complete().await.unwrap();
}
