//! Regression tests for the wire-shape drift covered by
//! [`nifi_rust_client::FlexibleString`].
//!
//! NiFi 2.9.0+ returns `StatusSnapshotDTO.timestamp` as a JSON integer
//! (Unix milliseconds) even though the OpenAPI spec declares it as
//! `string + format: date-time`. Earlier versions return a formatted
//! string. Both shapes must deserialize through the generated DTO.

#![cfg(feature = "dynamic")]

use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[allow(clippy::unwrap_used)]
async fn dynamic_client_on(
    mock: &MockServer,
    version: &str,
) -> nifi_rust_client::dynamic::DynamicClient {
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": { "title": "NiFi", "version": version }
        })))
        .mount(mock)
        .await;

    let client = NifiClientBuilder::new(&mock.uri())
        .unwrap()
        .build()
        .unwrap();
    nifi_rust_client::dynamic::DynamicClient::from_client(client)
        .await
        .unwrap()
}

#[tokio::test]
async fn status_snapshot_timestamp_accepts_integer_wire_shape() {
    let mock = MockServer::start().await;
    let dynamic = dynamic_client_on(&mock, "2.9.0").await;

    let processor_id = "d443858d-019d-1000-ffff-ffffc5e26250";

    Mock::given(method("GET"))
        .and(path(format!(
            "/nifi-api/flow/processors/{processor_id}/status/history"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "statusHistory": {
                "generated": "04/27/2026 10:00:00 UTC",
                "componentDetails": { "Type": "GenerateFlowFile" },
                "fieldDescriptors": [],
                "aggregateSnapshots": [
                    { "timestamp": 1777382685086_i64, "statusMetrics": { "bytesRead": 0 } },
                    { "timestamp": 1777382715086_i64, "statusMetrics": { "bytesRead": 4096 } }
                ]
            }
        })))
        .mount(&mock)
        .await;

    let history = dynamic
        .flow()
        .get_processor_status_history(processor_id)
        .await
        .expect("status history must deserialize integer-shape timestamps");

    let snapshots = history
        .status_history
        .as_ref()
        .and_then(|s| s.aggregate_snapshots.as_deref())
        .unwrap_or_default();
    assert_eq!(snapshots.len(), 2);
    assert_eq!(
        snapshots[0].timestamp.as_deref(),
        Some("1777382685086"),
        "integer wire value renders as decimal string"
    );
    assert_eq!(snapshots[1].timestamp.as_deref(), Some("1777382715086"));
}

#[tokio::test]
async fn status_snapshot_timestamp_accepts_string_wire_shape() {
    let mock = MockServer::start().await;
    let dynamic = dynamic_client_on(&mock, "2.6.0").await;

    let processor_id = "abc";

    Mock::given(method("GET"))
        .and(path(format!(
            "/nifi-api/flow/processors/{processor_id}/status/history"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "statusHistory": {
                "aggregateSnapshots": [
                    { "timestamp": "04/27/2026 10:00:00 UTC", "statusMetrics": {} }
                ]
            }
        })))
        .mount(&mock)
        .await;

    let history = dynamic
        .flow()
        .get_processor_status_history(processor_id)
        .await
        .unwrap();

    let snapshots = history
        .status_history
        .as_ref()
        .and_then(|s| s.aggregate_snapshots.as_deref())
        .unwrap_or_default();
    assert_eq!(
        snapshots[0].timestamp.as_deref(),
        Some("04/27/2026 10:00:00 UTC")
    );
}
