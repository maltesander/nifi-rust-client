#![cfg(feature = "dynamic")]

use nifi_rust_client::NifiClientBuilder;

fn nifi_url() -> String {
    std::env::var("NIFI_URL").unwrap_or_else(|_| "https://localhost:8443".to_string())
}

fn nifi_username() -> String {
    std::env::var("NIFI_USERNAME").unwrap_or_else(|_| "admin".to_string())
}

fn nifi_password() -> String {
    std::env::var("NIFI_PASSWORD").unwrap_or_else(|_| "adminpassword123".to_string())
}

async fn dynamic_logged_in_client() -> nifi_rust_client::dynamic::DynamicClient {
    let mut client = NifiClientBuilder::new(&nifi_url())
        .expect("failed to parse NiFi URL")
        .danger_accept_invalid_certs(true)
        .build_dynamic()
        .await
        .expect("failed to build dynamic client");
    client
        .login(&nifi_username(), &nifi_password())
        .await
        .expect("failed to log in");
    client
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn dynamic_detects_version() {
    let client = dynamic_logged_in_client().await;
    let version = client.detected_version().to_string();
    assert!(
        version.starts_with("2."),
        "expected NiFi 2.x, got {version}"
    );
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn dynamic_about_returns_info() {
    let client = dynamic_logged_in_client().await;
    let about = client
        .flow_api()
        .get_about_info()
        .await
        .expect("failed to get about");
    assert!(about.title.is_some());
    assert!(about.version.is_some());
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn dynamic_resources_returns_ok() {
    let client = dynamic_logged_in_client().await;
    client
        .resources_api()
        .get_resources()
        .await
        .expect("failed to get resources");
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn dynamic_system_diagnostics_returns_ok() {
    let client = dynamic_logged_in_client().await;
    client
        .systemdiagnostics_api()
        .get_system_diagnostics(None, None, None)
        .await
        .expect("failed to get system diagnostics");
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn dynamic_current_user_returns_identity() {
    let client = dynamic_logged_in_client().await;
    let user = client
        .flow_api()
        .get_current_user()
        .await
        .expect("failed to get current user");
    assert!(user.identity.is_some());
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn dynamic_get_process_group_root() {
    let client = dynamic_logged_in_client().await;
    let pg = client
        .processgroups_api()
        .get_process_group("root")
        .await
        .expect("failed to get root process group");
    assert!(pg.id.is_some());
}
