#![cfg(feature = "dynamic")]

mod helpers;

use helpers::dynamic_logged_in_client;

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn dynamic_detects_version() {
    let client = dynamic_logged_in_client().await;
    let version = client
        .detected_version()
        .expect("login should have populated version")
        .to_string();
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
