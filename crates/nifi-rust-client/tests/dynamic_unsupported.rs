//! End-to-end regression tests for dynamic-mode availability gating.
//!
//! Two distinct failure modes are pinned here:
//!
//! 1. Setting a version-specific query parameter on a server that
//!    predates it → `NifiError::UnsupportedQueryParam`.
//!    Target: `Flow::get_flow_metrics` with `flowMetricsReportingStrategy`
//!    (added in NiFi 2.7.2) against a detected 2.6.0 server.
//!
//! 2. Calling an endpoint that doesn't exist in the detected version →
//!    `NifiError::UnsupportedEndpoint`.
//!    Target: `Connectors::get_flow` (added in NiFi 2.9.0) against a
//!    detected 2.6.0 server.
//!
//! Both assertions verify the availability check fires before any HTTP
//! request is made — the mock server is provisioned only with
//! `/flow/about` so a mistaken request would show up as a wiremock miss.

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
    client.set_token("fake-jwt".to_string()).await;

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

#[tokio::test]
async fn calling_unsupported_endpoint_returns_unsupported_endpoint_error() {
    let server = MockServer::start().await;
    mock_nifi_2_6_0(&server).await;

    let inner = NifiClientBuilder::new(&server.uri())
        .expect("builder")
        .build()
        .expect("client");
    let client = DynamicClient::new(inner);

    client.set_token("fake-jwt".to_string()).await;
    client.detect_version().await.expect("detect_version");

    // `GET /connectors/{connectorId}/flow/process-groups/{processGroupId}`
    // is only available on NiFi 2.9.0. Against a detected 2.6.0 server
    // the availability check must fire before any HTTP call.
    let result = client
        .connectors()
        .get_flow("connector-id", "pg-id", None)
        .await;

    match result {
        Err(NifiError::UnsupportedEndpoint { endpoint, version }) => {
            assert_eq!(version, "2.6.0");
            assert!(
                endpoint.contains("connectors"),
                "endpoint string should reference connectors path: {endpoint}"
            );
        }
        other => panic!("expected UnsupportedEndpoint, got {other:?}"),
    }
}
