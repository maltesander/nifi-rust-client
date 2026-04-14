//! End-to-end regression test: setting a version-specific query
//! parameter on a DynamicClient whose detected version does not
//! support it must fail with NifiError::UnsupportedQueryParam.
//!
//! Pins the current canonical-superset contract: query params
//! set to a non-None value on an unsupporting version error out,
//! instead of silently sending the request.
//!
//! Target: Flow::get_flow_metrics on GET /flow/metrics/{producer}.
//! The `flowMetricsReportingStrategy` param was added in NiFi 2.7.2;
//! detecting a 2.6.0 server and passing Some(..) must fail the
//! availability check before any HTTP request is made.

#![cfg(feature = "dynamic")]

use nifi_rust_client::dynamic::DynamicClient;
use nifi_rust_client::dynamic::types::FlowMetricsReportingStrategy;
use nifi_rust_client::{NifiClientBuilder, NifiError};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Mount just enough mocks for DynamicClient to detect NiFi 2.6.0.
/// Only GET /flow/about is needed for detect_version().
async fn mock_nifi_2_6_0(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": {
                "title": "NiFi",
                "version": "2.6.0",
                "uri": "https://localhost/nifi-api",
                "contentViewerUrl": "",
                "timezone": "UTC",
                "buildTag": "",
                "buildRevision": "",
                "buildBranch": "",
                "buildTimestamp": ""
            }
        })))
        .mount(server)
        .await;
}

#[tokio::test]
async fn setting_unsupported_query_param_returns_unsupported_query_param_error() {
    let server = MockServer::start().await;
    mock_nifi_2_6_0(&server).await;

    let inner = NifiClientBuilder::new(&server.uri())
        .expect("builder")
        .build()
        .expect("client");
    let client = DynamicClient::new(inner);

    // Set a fake token so authenticated helpers don't warn about missing credentials.
    client.inner().set_token("fake-jwt".to_string()).await;

    // Prime the version cache: this calls GET /flow/about and records "2.6.0".
    client.detect_version().await.expect("detect_version");

    // Call get_flow_metrics with flow_metrics_reporting_strategy set to Some(..),
    // a param only available in NiFi 2.7.2 and later.
    // The availability check fires before any HTTP request is made.
    let result = client
        .flow()
        .get_flow_metrics(
            "prometheus", // producer path param — value irrelevant, no HTTP call made
            None,         // included_registries
            None,         // sample_name
            None,         // sample_label_value
            None,         // root_field_name
            Some(FlowMetricsReportingStrategy::AllProcessGroups),
        )
        .await;

    match result {
        Err(NifiError::UnsupportedQueryParam {
            param,
            detected_version,
            ..
        }) => {
            assert_eq!(detected_version, "2.6.0");
            assert_eq!(param, "flowMetricsReportingStrategy");
        }
        other => panic!("expected UnsupportedQueryParam, got {other:?}"),
    }
}
