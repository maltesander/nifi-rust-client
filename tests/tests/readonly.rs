/// Smoke tests for read-only API groups.
/// Each test just calls the endpoint and asserts it returns Ok —
/// no mutations, no setup required beyond a running NiFi instance.
mod helpers;

// ── systemdiagnostics ─────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn system_diagnostics_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .systemdiagnostics_api()
        .get_system_diagnostics(None, None, None)
        .await
        .expect("failed to get system diagnostics");
}

// ── resources ─────────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn resources_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .resources_api()
        .get_resources()
        .await
        .expect("failed to get resources");
}

// ── sitetosite ────────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn site_to_site_details_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .sitetosite_api()
        .get_site_to_site_details()
        .await
        .expect("failed to get site-to-site details");
}

// ── counters ──────────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn counters_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .counters_api()
        .get_counters(None, None)
        .await
        .expect("failed to get counters");
}

// ── authentication ────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn authentication_configuration_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .authentication_api()
        .get_authentication_configuration()
        .await
        .expect("failed to get authentication configuration");
}
