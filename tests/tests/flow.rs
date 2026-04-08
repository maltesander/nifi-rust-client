/// Integration tests require a running NiFi instance.
/// Use tests/run.sh to start one, or set env vars manually:
///   NIFI_URL          (default: https://localhost:8443)
///   NIFI_USERNAME     (default: admin)
///   NIFI_PASSWORD     (default: adminpassword123)
///   NIFI_CA_CERT_PATH (optional: path to PEM CA cert for TLS verification)
fn nifi_url() -> String {
    std::env::var("NIFI_URL").unwrap_or_else(|_| "https://localhost:8443".to_string())
}

fn nifi_username() -> String {
    std::env::var("NIFI_USERNAME").unwrap_or_else(|_| "admin".to_string())
}

fn nifi_password() -> String {
    std::env::var("NIFI_PASSWORD").unwrap_or_else(|_| "adminpassword123".to_string())
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn current_user_identity_matches_login() {
    let client = nifi_rust_client::NifiClientBuilder::new(&nifi_url())
        .expect("failed to parse NiFi URL")
        .danger_accept_invalid_certs(true)
        .build()
        .expect("failed to build NiFi client");

    client
        .login(&nifi_username(), &nifi_password())
        .await
        .expect("failed to log in to NiFi");

    let user = client
        .flow_api()
        .get_current_user()
        .await
        .expect("failed to get current user");

    assert_eq!(user.identity.as_deref(), Some(nifi_username().as_str()));
    assert!(!user.anonymous.unwrap_or(false));
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn flow_status_succeeds_on_fresh_instance() {
    let client = nifi_rust_client::NifiClientBuilder::new(&nifi_url())
        .expect("failed to parse NiFi URL")
        .danger_accept_invalid_certs(true)
        .build()
        .expect("failed to build NiFi client");

    client
        .login(&nifi_username(), &nifi_password())
        .await
        .expect("failed to log in to NiFi");

    let status = client
        .flow_api()
        .get_controller_status()
        .await
        .expect("failed to get flow status");

    // Fresh NiFi instance has no processors or queued data.
    assert_eq!(status.flow_files_queued, Some(0));
    assert_eq!(status.bytes_queued, Some(0));
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn logout_clears_token_and_invalidates_session() {
    let client = nifi_rust_client::NifiClientBuilder::new(&nifi_url())
        .expect("failed to parse NiFi URL")
        .danger_accept_invalid_certs(true)
        .build()
        .expect("failed to build NiFi client");

    client
        .login(&nifi_username(), &nifi_password())
        .await
        .expect("failed to log in");

    // Sanity check: a call with the token succeeds.
    client
        .flow_api()
        .get_about_info()
        .await
        .expect("expected authenticated call to succeed before logout");

    client.logout().await.expect("logout failed");

    // After logout the token is cleared on the client.
    assert!(
        client.token().await.is_none(),
        "expected token to be None after logout"
    );
}
