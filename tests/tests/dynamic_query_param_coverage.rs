#![cfg(feature = "dynamic")]

mod helpers;

#[cfg(feature = "nifi-2-7-2")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn param_flow_get_flow_metrics_flow_metrics_reporting_strategy_accepted() {
    use nifi_rust_client::dynamic::traits::FlowApi;
    use nifi_rust_client::dynamic::types::FlowMetricsReportingStrategy;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .flow_api()
        .get_flow_metrics(
            "prometheus",
            None,
            None,
            None,
            None,
            Some(FlowMetricsReportingStrategy::AllProcessGroups),
        )
        .await;
    assert!(
        result.is_ok(),
        "expected param flowMetricsReportingStrategy to be accepted, got: {:?}",
        result.unwrap_err()
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn param_flow_get_flow_metrics_flow_metrics_reporting_strategy_ignored_on_older() {
    use nifi_rust_client::dynamic::traits::FlowApi;
    use nifi_rust_client::dynamic::types::FlowMetricsReportingStrategy;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .flow_api()
        .get_flow_metrics(
            "prometheus",
            None,
            None,
            None,
            None,
            Some(FlowMetricsReportingStrategy::AllProcessGroups),
        )
        .await;
    // Param should be silently skipped — no error expected
    assert!(
        result.is_ok(),
        "expected param to be silently skipped, got: {:?}",
        result.unwrap_err()
    );
}
