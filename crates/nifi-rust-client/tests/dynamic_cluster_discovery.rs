#![cfg(feature = "dynamic")]
//! Audit follow-up A8: `DynamicClient::discover_cluster` must not poison
//! the `cluster_node_id` `OnceCell` on error. A transient failure (e.g. a
//! 401 during a JWT-refresh window, an unauthorized `/controller/cluster`
//! endpoint) should leave the cell empty so the next call can retry, not
//! cache `Some(None)` ("definitively standalone") forever.

use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[allow(clippy::unwrap_used)]
async fn build_dynamic_client(mock: &MockServer) -> nifi_rust_client::dynamic::DynamicClient {
    let client = NifiClientBuilder::new(&mock.uri())
        .unwrap()
        .build()
        .unwrap();
    nifi_rust_client::dynamic::DynamicClient::new(client)
}

#[allow(clippy::unwrap_used)]
fn about_response(version: &str) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "about": { "title": "NiFi", "version": version }
    }))
}

/// First `login` call: cluster-summary returns 401 (e.g. mid-rotation token
/// hiccup). The discovery is silently abandoned, but the `OnceCell` must
/// stay empty.
///
/// Second `login` call: cluster-summary returns success and
/// `/controller/cluster` returns one connected node. Discovery must retry
/// and populate `cluster_node_id` with `Some("node-abc")`.
///
/// Pre-fix this test fails because the first failed discovery cached
/// `Some(None)` and the second call's `get_or_init` is a no-op.
#[tokio::test]
async fn cluster_discovery_does_not_poison_oncecell_on_error() {
    let mock = MockServer::start().await;

    // /access/token used by both `login` calls.
    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(200).set_body_string("jwt-token"))
        .mount(&mock)
        .await;

    // /flow/about used by version detection (idempotent — both logins hit it).
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(about_response("2.9.0"))
        .mount(&mock)
        .await;

    // First /flow/cluster/summary call returns 401. wiremock checks mocks
    // in registration order; once `up_to_n_times(1)` is exhausted, the
    // subsequent 200 mount handles further requests.
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/cluster/summary"))
        .respond_with(ResponseTemplate::new(401).set_body_string(""))
        .up_to_n_times(1)
        .mount(&mock)
        .await;

    // After the first 401, subsequent summary calls return clustered: true.
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/cluster/summary"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "clusterSummary": { "clustered": true, "connectedToCluster": true }
        })))
        .mount(&mock)
        .await;

    // /controller/cluster: only the second login reaches it (the first
    // login bails after the 401 from summary).
    Mock::given(method("GET"))
        .and(path("/nifi-api/controller/cluster"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "cluster": {
                "nodes": [
                    { "nodeId": "node-abc", "status": "CONNECTED" }
                ]
            }
        })))
        .mount(&mock)
        .await;

    let dynamic = build_dynamic_client(&mock).await;

    // First login — discovery hits the 401-mount and silently fails.
    dynamic.login("admin", "pw").await.unwrap();
    assert!(
        dynamic.cluster_node_id().is_none(),
        "first failed discovery must NOT cache anything (cell stays empty)"
    );

    // Second login — discovery should retry and succeed, populating the cell.
    dynamic.login("admin", "pw").await.unwrap();
    assert_eq!(
        dynamic.cluster_node_id(),
        Some("node-abc"),
        "after a successful retry, cluster_node_id must reflect the discovered node"
    );
}
