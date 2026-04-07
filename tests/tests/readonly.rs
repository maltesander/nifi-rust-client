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

// ── flow — basic info ────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn about_info_returns_nifi_version() {
    let client = helpers::logged_in_client().await;
    let about = client
        .flow_api()
        .get_about_info()
        .await
        .expect("failed to get about info");
    assert!(
        about.title.as_deref().unwrap_or("").len() > 0,
        "expected non-empty title in AboutDto"
    );
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn banners_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .flow_api()
        .get_banners()
        .await
        .expect("failed to get banners");
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn bulletin_board_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .flow_api()
        .get_bulletin_board(None, None, None, None, None, None)
        .await
        .expect("failed to get bulletin board");
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn bulletins_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .flow_api()
        .get_bulletins()
        .await
        .expect("failed to get bulletins");
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn flow_config_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .flow_api()
        .get_flow_config()
        .await
        .expect("failed to get flow config");
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn cluster_summary_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .flow_api()
        .get_cluster_summary()
        .await
        .expect("failed to get cluster summary");
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn generate_client_id_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .flow_api()
        .generate_client_id()
        .await
        .expect("failed to generate client id");
}

// ── flow — component types ───────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn processor_types_returns_ok() {
    let client = helpers::logged_in_client().await;
    let entity = client
        .flow_api()
        .get_processor_types(None, None, None)
        .await
        .expect("failed to get processor types");
    assert!(
        entity.processor_types.as_deref().unwrap_or_default().len() > 0,
        "expected at least one processor type"
    );
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn controller_service_types_returns_ok() {
    let client = helpers::logged_in_client().await;
    let entity = client
        .flow_api()
        .get_controller_service_types(None, None, None, None, None, None, None)
        .await
        .expect("failed to get controller service types");
    assert!(
        entity
            .controller_service_types
            .as_deref()
            .unwrap_or_default()
            .len()
            > 0,
        "expected at least one controller service type"
    );
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn reporting_task_types_returns_ok() {
    let client = helpers::logged_in_client().await;
    let entity = client
        .flow_api()
        .get_reporting_task_types(None, None, None)
        .await
        .expect("failed to get reporting task types");
    assert!(
        entity
            .reporting_task_types
            .as_deref()
            .unwrap_or_default()
            .len()
            > 0,
        "expected at least one reporting task type"
    );
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn prioritizers_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .flow_api()
        .get_prioritizers()
        .await
        .expect("failed to get prioritizers");
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn content_viewers_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .flow_api()
        .get_content_viewers()
        .await
        .expect("failed to get content viewers");
}

// ── flow — process group and search ─────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn get_root_flow_returns_ok() {
    let client = helpers::logged_in_client().await;
    let entity = client
        .flow_api()
        .get_flow("root", None)
        .await
        .expect("failed to get root flow");
    assert!(
        entity.process_group_flow.is_some(),
        "expected process_group_flow to be Some"
    );
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn search_flow_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .flow_api()
        .search_flow(Some("NiFi"), None)
        .await
        .expect("failed to search flow");
}

// ── system diagnostics — jmx metrics ────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn jmx_metrics_returns_ok() {
    let client = helpers::logged_in_client().await;
    client
        .systemdiagnostics_api()
        .get_jmx_metrics(None)
        .await
        .expect("failed to get jmx metrics");
}
