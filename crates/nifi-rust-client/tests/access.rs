#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::{NifiClientBuilder, NifiError};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── login stores token on success ─────────────────────────────────────────────

#[tokio::test]
async fn login_stores_token_on_success() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(201).set_body_string("eyJhbGciOiJSUzI1NiJ9.test-token"))
        .mount(&mock_server)
        .await;

    let mut client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    assert!(client.token().is_none());
    client.login("admin", "password").await.unwrap();
    assert_eq!(client.token(), Some("eyJhbGciOiJSUzI1NiJ9.test-token"));
}

// ── login returns NifiError::Auth on failure ──────────────────────────────────

#[tokio::test]
async fn login_returns_auth_error_on_401() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Bad credentials"))
        .mount(&mock_server)
        .await;

    let mut client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.login("admin", "wrong").await.unwrap_err();

    assert!(
        matches!(err, NifiError::Auth { .. }),
        "expected NifiError::Auth, got: {err:?}"
    );
    assert!(client.token().is_none());
}
