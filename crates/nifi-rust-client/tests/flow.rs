#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::NifiError;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn about_returns_version_and_title() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": {
                "title": "NiFi",
                "version": "2.8.0",
                "uri": "http://localhost/nifi-api/",
                "contentViewerUrl": "/nifi-content-viewer/",
                "timezone": "UTC",
                "buildTag": "nifi-2.8.0",
                "buildTimestamp": "01/01/2026 00:00:00 UTC"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let about = client.flow_api().get_about_info().await.unwrap();

    assert_eq!(about.version.as_deref(), Some("2.8.0"));
    assert_eq!(about.title.as_deref(), Some("NiFi"));
    assert_eq!(about.timezone.as_deref(), Some("UTC"));
}

#[tokio::test]
async fn current_user_returns_identity_and_anonymous_flag() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/current-user"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "identity": "admin",
            "anonymous": false,
            "canVersionFlows": false
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let user = client.flow_api().get_current_user().await.unwrap();

    assert_eq!(user.identity.as_deref(), Some("admin"));
    assert!(!user.anonymous.unwrap_or(false));
}

#[tokio::test]
async fn flow_status_returns_component_counts() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "controllerStatus": {
                "activeThreadCount": 2,
                "terminatedThreadCount": 0,
                "queued": "5 (1 KB)",
                "flowFilesQueued": 5,
                "bytesQueued": 1024,
                "runningCount": 3,
                "stoppedCount": 1,
                "invalidCount": 0,
                "disabledCount": 2,
                "activeRemotePortCount": 0,
                "inactiveRemotePortCount": 0
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let status = client.flow_api().get_controller_status().await.unwrap();

    assert_eq!(status.active_thread_count, Some(2));
    assert_eq!(status.running_count, Some(3));
    assert_eq!(status.stopped_count, Some(1));
    assert_eq!(status.flow_files_queued, Some(5));
    assert_eq!(status.queued.as_deref(), Some("5 (1 KB)"));
}

#[tokio::test]
async fn logout_invalidates_token_on_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/nifi-api/access/logout"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("some-jwt".to_string()).await;
    assert!(client.token().await.is_some());

    client.logout().await.unwrap();

    assert!(client.token().await.is_none());
}

#[tokio::test]
async fn logout_clears_token_even_on_server_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/nifi-api/access/logout"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("expired-jwt".to_string()).await;

    let err = client.logout().await.unwrap_err();

    assert!(matches!(err, NifiError::Unauthorized { .. }));
    assert!(client.token().await.is_none());
}

#[tokio::test]
async fn get_flow_config_returns_supports_managed_authorizer() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/config"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "flowConfiguration": {
                "supportsManagedAuthorizer": true,
                "supportsConfigurableUsersAndGroups": true,
                "supportsConfigurableAuthorizer": true,
                "defaultBackPressureObjectThreshold": 10000,
                "defaultBackPressureDataSizeThreshold": "1 GB"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let config = client.flow_api().get_flow_config().await.unwrap();

    assert_eq!(config.supports_managed_authorizer, Some(true));
    assert_eq!(config.supports_configurable_users_and_groups, Some(true));
}

#[tokio::test]
async fn get_cluster_summary_returns_connected_node_count() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/cluster/summary"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "clusterSummary": {
                "clustered": false,
                "connectedToCluster": false,
                "connectedNodeCount": 0,
                "totalNodeCount": 0,
                "connectedNodes": "0 / 0"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let summary = client.flow_api().get_cluster_summary().await.unwrap();

    assert_eq!(summary.clustered, Some(false));
    assert_eq!(summary.connected_to_cluster, Some(false));
    assert_eq!(summary.connected_node_count, Some(0));
}

#[tokio::test]
async fn get_flow_returns_process_group_id() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/process-groups/root"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "processGroupFlow": {
                "id": "root-pg-id",
                "uri": "http://localhost/nifi-api/flow/process-groups/root",
                "parentGroupId": null,
                "lastRefreshed": "01/01/2026 00:00:00 UTC",
                "breadcrumb": {},
                "flow": {
                    "processGroups": [],
                    "remoteProcessGroups": [],
                    "processors": [],
                    "inputPorts": [],
                    "outputPorts": [],
                    "connections": [],
                    "labels": [],
                    "funnels": []
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let entity = client.flow_api().get_flow("root", None).await.unwrap();

    assert_eq!(
        entity
            .process_group_flow
            .as_ref()
            .and_then(|f| f.id.as_deref()),
        Some("root-pg-id")
    );
}

#[tokio::test]
async fn search_flow_returns_processor_results() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/search-results"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "searchResultsDTO": {
                "processorResults": [
                    { "id": "proc-abc", "name": "GenerateFlowFile", "groupId": "root" }
                ],
                "connectionResults": [],
                "processGroupResults": [],
                "inputPortResults": [],
                "outputPortResults": [],
                "remoteProcessGroupResults": [],
                "funnelResults": [],
                "labelResults": [],
                "controllerServiceNodeResults": [],
                "parameterProviderNodeResults": [],
                "parameterContextResults": [],
                "parameterResults": []
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let results = client
        .flow_api()
        .search_flow(Some("GenerateFlowFile"), None)
        .await
        .unwrap();

    let first = results.processor_results.as_deref().and_then(|r| r.first());
    assert_eq!(first.and_then(|r| r.id.as_deref()), Some("proc-abc"));
    assert_eq!(
        first.and_then(|r| r.name.as_deref()),
        Some("GenerateFlowFile")
    );
}

// ── clear_bulletins (process-group) ──────────────────────────────────────────
// Added in NiFi 2.7.2 — gate so nifi-2-6-0 builds stay green.

#[cfg(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0"))]
#[tokio::test]
async fn clear_process_group_bulletins_returns_cleared_count() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/nifi-api/flow/process-groups/some-id/bulletins/clear-requests",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bulletinsCleared": 7
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ClearBulletinsForGroupRequestEntity::default();
    let result = client
        .flow_api()
        .bulletins("some-id")
        .clear_bulletins_1(&body)
        .await;

    assert!(
        result.is_ok(),
        "clear_process_group_bulletins failed: {:?}",
        result.err()
    );
}

// ── listen-ports ──────────────────────────────────────────────────────────────

// Version-feature-gated test pattern — these tests exercise endpoints that
// only exist in NiFi 2.7.2+. The `#[cfg]` guard ensures they are compiled
// out when built against nifi-2-6-0 features, keeping cross-version builds
// green. Use this pattern for any test that touches endpoints or fields
// added in a specific version.
#[cfg(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0"))]
#[tokio::test]
async fn get_listen_ports_returns_port_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/listen-ports"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "listenPorts": [
                {
                    "portNumber": 8080,
                    "portName": "HTTP",
                    "transportProtocol": "TCP",
                    "componentId": "abc-123",
                    "componentName": "ListenHTTP",
                    "componentType": "Processor",
                    "componentClass": "org.apache.nifi.processors.standard.ListenHTTP",
                    "parentGroupId": "root"
                }
            ]
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client.flow_api().get_listen_ports().await.unwrap();

    let ports = result.listen_ports.unwrap_or_default();
    assert_eq!(ports.len(), 1);
    assert_eq!(ports[0].port_number, Some(8080));
    assert_eq!(ports[0].transport_protocol.as_deref(), Some("TCP"));
}

// ── flow-registry-client-definition ──────────────────────────────────────────

#[cfg(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0"))]
#[tokio::test]
async fn get_flow_registry_client_definition_returns_artifact_info() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/nifi-api/flow/flow-registry-client-definition/org.apache.nifi/nifi-flow-registry-client-nar/2.8.0/StandardNiFiRegistryFlowRegistryClient",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "group": "org.apache.nifi",
            "artifact": "nifi-flow-registry-client-nar",
            "version": "2.8.0",
            "type": "StandardNiFiRegistryFlowRegistryClient",
            "deprecated": false,
            "additionalDetails": false
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .flow_api()
        .get_flow_registry_client_definition(
            "org.apache.nifi",
            "nifi-flow-registry-client-nar",
            "2.8.0",
            "StandardNiFiRegistryFlowRegistryClient",
        )
        .await
        .unwrap();

    assert_eq!(
        result.artifact.as_deref(),
        Some("nifi-flow-registry-client-nar")
    );
    assert_eq!(result.group.as_deref(), Some("org.apache.nifi"));
    assert!(!result.deprecated.unwrap_or(true));
}
