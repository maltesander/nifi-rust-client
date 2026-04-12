#![cfg(not(feature = "dynamic"))]
#![allow(clippy::panic)]

use nifi_rust_client::config::auth::{EnvPasswordAuth, PasswordAuth, Redacted, StaticTokenAuth};
use nifi_rust_client::{NifiClientBuilder, NifiError};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── Redacted<T> ──────────────────────────────────────────────────────────────

#[test]
fn redacted_hides_value_in_debug() {
    let secret = Redacted::new("super-secret");
    assert_eq!(format!("{secret:?}"), "[REDACTED]");
    assert_eq!(*secret.inner(), "super-secret");
}

// ── PasswordAuth ─────────────────────────────────────────────────────────────

#[test]
fn password_auth_debug_hides_password() {
    let auth = PasswordAuth::new("admin", "s3cret");
    let dbg = format!("{auth:?}");
    assert!(dbg.contains("admin"), "expected username in debug: {dbg}");
    assert!(
        dbg.contains("[REDACTED]"),
        "expected [REDACTED] in debug: {dbg}"
    );
    assert!(!dbg.contains("s3cret"), "password leaked into debug: {dbg}");
}

#[tokio::test]
async fn password_auth_calls_login() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(201).set_body_string("test-jwt"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .auth_provider(PasswordAuth::new("admin", "password"))
        .build()
        .unwrap();

    client.authenticate().await.unwrap();
    assert_eq!(client.token().await.as_deref(), Some("test-jwt"));
}

// ── StaticTokenAuth ──────────────────────────────────────────────────────────

#[tokio::test]
async fn static_token_auth_sets_token() {
    let client = NifiClientBuilder::new("https://localhost:8443")
        .unwrap()
        .danger_accept_invalid_certs(true)
        .auth_provider(StaticTokenAuth::new("pre-obtained-jwt"))
        .build()
        .unwrap();

    client.authenticate().await.unwrap();
    assert_eq!(client.token().await.as_deref(), Some("pre-obtained-jwt"));
}

// ── EnvPasswordAuth ──────────────────────────────────────────────────────────

#[tokio::test]
async fn env_password_auth_reads_env_and_logs_in() {
    let u_var = "TEST_AUTH_PROVIDER_USER";
    let p_var = "TEST_AUTH_PROVIDER_PASS";
    // SAFETY: test runs in isolation; no other thread reads these variables.
    unsafe {
        std::env::set_var(u_var, "env_admin");
        std::env::set_var(p_var, "env_pass");
    }

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(201).set_body_string("env-jwt"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .auth_provider(EnvPasswordAuth::with_vars(u_var, p_var))
        .build()
        .unwrap();

    client.authenticate().await.unwrap();
    assert_eq!(client.token().await.as_deref(), Some("env-jwt"));

    // SAFETY: test cleanup; no other thread reads these variables.
    unsafe {
        std::env::remove_var(u_var);
        std::env::remove_var(p_var);
    }
}

#[tokio::test]
async fn env_password_auth_error_when_var_missing() {
    let client = NifiClientBuilder::new("https://localhost:8443")
        .unwrap()
        .danger_accept_invalid_certs(true)
        .auth_provider(EnvPasswordAuth::with_vars(
            "NONEXISTENT_AUTH_USER_XYZZY",
            "NONEXISTENT_AUTH_PASS_XYZZY",
        ))
        .build()
        .unwrap();

    let result = client.authenticate().await;
    assert!(result.is_err());
    match result.unwrap_err() {
        NifiError::Auth { message } => {
            assert!(
                message.contains("NONEXISTENT_AUTH_USER_XYZZY"),
                "expected env var name in error: {message}"
            );
        }
        other => panic!("expected Auth error, got: {other:?}"),
    }
}

#[test]
fn env_password_auth_default_uses_nifi_vars() {
    let auth = EnvPasswordAuth::new();
    assert_eq!(auth.username_var(), "NIFI_USERNAME");
    assert_eq!(auth.password_var(), "NIFI_PASSWORD");

    let default_auth = EnvPasswordAuth::default();
    assert_eq!(default_auth.username_var(), "NIFI_USERNAME");
    assert_eq!(default_auth.password_var(), "NIFI_PASSWORD");
}
