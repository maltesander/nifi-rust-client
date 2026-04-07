#![cfg(all(feature = "nifi-2-7-2", not(feature = "dynamic")))]

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};
#[cfg(test)]
mod access_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_log_out() {
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
        let result = client.access_api().log_out().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_log_out_complete() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/access/logout/complete"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.access_api().log_out_complete().await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod authentication_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_get_authentication_configuration() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/authentication/configuration"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::AuthenticationConfigurationEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .authentication_api()
            .get_authentication_configuration()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod connections_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_delete_connection() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/connections/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ConnectionEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .connections_api()
            .delete_connection("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_connection() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/connections/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ConnectionEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.connections_api().get_connection("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_connection() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/connections/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ConnectionEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ConnectionEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .connections_api()
            .update_connection("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod controller_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_create_bulletin() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/controller/bulletin"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::BulletinEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::BulletinEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().create_bulletin(&body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_cluster() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller/cluster"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ClusterEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().get_cluster().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_node() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/controller/cluster/nodes/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::NodeEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().delete_node("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_node() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller/cluster/nodes/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::NodeEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().get_node("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_node() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/controller/cluster/nodes/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::NodeEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::NodeEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().update_node("test-id", &body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_controller_config() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller/config"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ControllerConfigurationEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().get_controller_config().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_controller_config() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/controller/config"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ControllerConfigurationEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ControllerConfigurationEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .update_controller_config(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_controller_service() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/controller/controller-services"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ControllerServiceEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ControllerServiceEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .create_controller_service(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_analysis_rules() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller/flow-analysis-rules"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowAnalysisRulesEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().get_flow_analysis_rules().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_flow_analysis_rule() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/controller/flow-analysis-rules"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::FlowAnalysisRuleEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::FlowAnalysisRuleEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .create_flow_analysis_rule(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_remove_flow_analysis_rule() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/controller/flow-analysis-rules/test-id"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::FlowAnalysisRuleEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .remove_flow_analysis_rule("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_analysis_rule() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller/flow-analysis-rules/test-id"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::FlowAnalysisRuleEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .get_flow_analysis_rule("test-id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_flow_analysis_rule() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/controller/flow-analysis-rules/test-id"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::FlowAnalysisRuleEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::FlowAnalysisRuleEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .update_flow_analysis_rule("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_history() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/controller/history"))
            .and(query_param("endDate", "test-value"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::HistoryEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().delete_history("test-value").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_nar_summaries() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller/nar-manager/nars"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::NarSummariesEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().get_nar_summaries().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_upload_nar() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/controller/nar-manager/nars/content"))
            .and(header("content-type", "application/octet-stream"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::NarSummaryEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .upload_nar(Some("test.nar"), vec![1u8, 2, 3])
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_nar() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/controller/nar-manager/nars/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::NarSummaryEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .delete_nar("test-id", None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_nar_summary() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller/nar-manager/nars/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::NarDetailsEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().get_nar_summary("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_parameter_provider() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/controller/parameter-providers"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterProviderEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ParameterProviderEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .create_parameter_provider(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_registry_clients() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller/registry-clients"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowRegistryClientsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().get_flow_registry_clients().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_flow_registry_client() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/controller/registry-clients"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowRegistryClientEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::FlowRegistryClientEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .create_flow_registry_client(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_flow_registry_client() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/controller/registry-clients/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowRegistryClientEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .delete_flow_registry_client("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_registry_client() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller/registry-clients/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowRegistryClientEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .get_flow_registry_client("test-id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_flow_registry_client() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/controller/registry-clients/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowRegistryClientEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::FlowRegistryClientEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .update_flow_registry_client("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_registry_client_types() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller/registry-types"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowRegistryClientTypesEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().get_registry_client_types().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_reporting_task() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/controller/reporting-tasks"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ReportingTaskEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ReportingTaskEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().create_reporting_task(&body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_import_reporting_task_snapshot() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/controller/reporting-tasks/import"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::VersionedReportingTaskImportResponseEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::VersionedReportingTaskImportRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .import_reporting_task_snapshot(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_node_status_history() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller/status/history"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ComponentHistoryEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.controller_api().get_node_status_history().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_flow_analysis_rule_bulletins() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/controller/flow-analysis-rules/test-id/bulletins/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ClearBulletinsResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ClearBulletinsRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .bulletins("test-id")
            .clear_flow_analysis_rule_bulletins(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_parameter_provider_bulletins() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/controller/parameter-providers/test-id/bulletins/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ClearBulletinsResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ClearBulletinsRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .bulletins("test-id")
            .clear_parameter_provider_bulletins(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_registry_client_bulletins() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/controller/registry-clients/test-id/bulletins/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ClearBulletinsResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ClearBulletinsRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .bulletins("test-id")
            .clear_registry_client_bulletins(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_analyze_flow_analysis_rule_configuration() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/controller/flow-analysis-rules/test-id/config/analysis",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ConfigurationAnalysisEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ConfigurationAnalysisEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .config("test-id")
            .analyze_flow_analysis_rule_configuration(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_submit_flow_analysis_rule_config_verification_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/controller/flow-analysis-rules/test-id/config/verification-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::VerifyConfigRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .config("test-id")
            .submit_flow_analysis_rule_config_verification_request(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_flow_analysis_rule_verification_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(
                path(
                    "/nifi-api/controller/flow-analysis-rules/test-id/config/verification-requests/test-request_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .config("test-id")
            .delete_flow_analysis_rule_verification_request("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_analysis_rule_verification_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/controller/flow-analysis-rules/test-id/config/verification-requests/test-request_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .config("test-id")
            .get_flow_analysis_rule_verification_request("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_analyze_flow_registry_client_configuration() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/controller/registry-clients/test-id/config/analysis",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ConfigurationAnalysisEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ConfigurationAnalysisEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .config("test-id")
            .analyze_flow_registry_client_configuration(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_submit_registry_client_config_verification_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/controller/registry-clients/test-id/config/verification-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::VerifyConfigRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .config("test-id")
            .submit_registry_client_config_verification_request(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_registry_client_verification_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(
                path(
                    "/nifi-api/controller/registry-clients/test-id/config/verification-requests/test-request_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .config("test-id")
            .delete_registry_client_verification_request("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_registry_client_verification_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/controller/registry-clients/test-id/config/verification-requests/test-request_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .config("test-id")
            .get_registry_client_verification_request("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_download_nar() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/controller/nar-manager/nars/test-id/content",
            ))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .content("test-id")
            .download_nar()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_analysis_rule_property_descriptor() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/controller/flow-analysis-rules/test-id/descriptors",
            ))
            .and(query_param("propertyName", "test-value"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::PropertyDescriptorEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .descriptors("test-id")
            .get_flow_analysis_rule_property_descriptor("test-value", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_property_descriptor() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/controller/registry-clients/test-id/descriptors",
            ))
            .and(query_param("propertyName", "test-value"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::PropertyDescriptorEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .descriptors("test-id")
            .get_property_descriptor("test-value", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_nar_details() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/controller/nar-manager/nars/test-id/details",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::NarDetailsEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .details("test-id")
            .get_nar_details()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_run_status() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/controller/flow-analysis-rules/test-id/run-status"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::FlowAnalysisRuleEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::FlowAnalysisRuleRunStatusEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .run_status("test-id")
            .update_run_status(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_analysis_rule_state() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/controller/flow-analysis-rules/test-id/state",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ComponentStateEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .state("test-id")
            .get_flow_analysis_rule_state()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_state() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/controller/flow-analysis-rules/test-id/state/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ComponentStateEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ComponentStateEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_api()
            .state("test-id")
            .clear_state(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod controller_services_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_remove_controller_service() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/controller-services/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ControllerServiceEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .remove_controller_service("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_controller_service() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller-services/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ControllerServiceEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .get_controller_service("test-id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_controller_service() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/controller-services/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ControllerServiceEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ControllerServiceEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .update_controller_service("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_bulletins() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/controller-services/test-id/bulletins/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ClearBulletinsResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ClearBulletinsRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .bulletins("test-id")
            .clear_bulletins(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_analyze_configuration() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/controller-services/test-id/config/analysis",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ConfigurationAnalysisEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ConfigurationAnalysisEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .config("test-id")
            .analyze_configuration(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_submit_config_verification_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/controller-services/test-id/config/verification-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::VerifyConfigRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .config("test-id")
            .submit_config_verification_request(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_verification_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(
                path(
                    "/nifi-api/controller-services/test-id/config/verification-requests/test-request_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .config("test-id")
            .delete_verification_request("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_verification_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/controller-services/test-id/config/verification-requests/test-request_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .config("test-id")
            .get_verification_request("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_property_descriptor_1() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller-services/test-id/descriptors"))
            .and(query_param("propertyName", "test-value"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::PropertyDescriptorEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .descriptors("test-id")
            .get_property_descriptor_1("test-value", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_controller_service_references() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller-services/test-id/references"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ControllerServiceReferencingComponentsEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .references("test-id")
            .get_controller_service_references()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_controller_service_references() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/controller-services/test-id/references"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ControllerServiceReferencingComponentsEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::UpdateControllerServiceReferenceRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .references("test-id")
            .update_controller_service_references(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_run_status_1() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/controller-services/test-id/run-status"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ControllerServiceEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ControllerServiceRunStatusEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .run_status("test-id")
            .update_run_status_1(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_state() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/controller-services/test-id/state"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ComponentStateEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .state("test-id")
            .get_state()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_state_1() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/controller-services/test-id/state/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ComponentStateEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ComponentStateEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .controller_services_api()
            .state("test-id")
            .clear_state_1(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod counters_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_get_counters() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/counters"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::CountersEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.counters_api().get_counters(None, None).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_all_counters() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/counters"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::CountersEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.counters_api().update_all_counters().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_counter() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/counters/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::CounterEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.counters_api().update_counter("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod datatransfer_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_commit_input_port_transaction() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/nifi-api/data-transfer/input-ports/test-port_id/transactions/test-transaction_id",
            ))
            .and(query_param("responseCode", "0"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::TransactionResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .datatransfer_api()
            .transactions("test-port_id")
            .commit_input_port_transaction("test-transaction_id", 0_i32)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_extend_input_port_transaction_t_t_l() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/nifi-api/data-transfer/input-ports/test-port_id/transactions/test-transaction_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::TransactionResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .datatransfer_api()
            .transactions("test-port_id")
            .extend_input_port_transaction_t_t_l("test-transaction_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_receive_flow_files() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(
                path(
                    "/nifi-api/data-transfer/input-ports/test-port_id/transactions/test-transaction_id/flow-files",
                ),
            )
            .and(header("content-type", "application/octet-stream"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .datatransfer_api()
            .transactions("test-port_id")
            .receive_flow_files("test-transaction_id", Some("test.nar"), vec![1u8, 2, 3])
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_commit_output_port_transaction() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(
                path(
                    "/nifi-api/data-transfer/output-ports/test-port_id/transactions/test-transaction_id",
                ),
            )
            .and(query_param("responseCode", "0"))
            .and(query_param("checksum", "test-value"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::TransactionResultEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .datatransfer_api()
            .transactions("test-port_id")
            .commit_output_port_transaction("test-transaction_id", 0_i32, "test-value")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_extend_output_port_transaction_t_t_l() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(
                path(
                    "/nifi-api/data-transfer/output-ports/test-port_id/transactions/test-transaction_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::TransactionResultEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .datatransfer_api()
            .transactions("test-port_id")
            .extend_output_port_transaction_t_t_l("test-transaction_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_transfer_flow_files() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/data-transfer/output-ports/test-port_id/transactions/test-transaction_id/flow-files",
                ),
            )
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .datatransfer_api()
            .transactions("test-port_id")
            .transfer_flow_files("test-transaction_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_port_transaction() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/data-transfer/test-port_type/test-port_id/transactions",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::TransactionResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .datatransfer_api()
            .transactions("test-port_id")
            .create_port_transaction("test-port_type")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod flow_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_get_about_info() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/about"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::AboutEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_about_info().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_additional_details() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/flow/additional-details/test-group/test-artifact/test-version/test-type",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::AdditionalDetailsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_additional_details("test-group", "test-artifact", "test-version", "test-type")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_banners() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/banners"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::BannerEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_banners().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_bulletin_board() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/bulletin-board"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::BulletinBoardEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_bulletin_board(None, None, None, None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_generate_client_id() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/client-id"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().generate_client_id().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_search_cluster() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/cluster/search-results"))
            .and(query_param("q", "test-value"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ClusterSearchResultsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().search_cluster("test-value").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_cluster_summary() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/cluster/summary"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ClusterSummaryEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_cluster_summary().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_config() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/config"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowConfigurationEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_flow_config().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_content_viewers() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/content-viewers"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ContentViewerEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_content_viewers().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_controller_service_definition() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/flow/controller-service-definition/test-group/test-artifact/test-version/test-type",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ControllerServiceDefinition::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_controller_service_definition(
                "test-group",
                "test-artifact",
                "test-version",
                "test-type",
            )
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_controller_service_types() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/controller-service-types"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ControllerServiceTypesEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_controller_service_types(None, None, None, None, None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_bulletins() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/controller/bulletins"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ControllerBulletinsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_bulletins().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_controller_services_from_controller() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/controller/controller-services"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ControllerServicesEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_controller_services_from_controller(None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_current_user() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/current-user"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::CurrentUserEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_current_user().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_analysis_rule_definition() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/flow/flow-analysis-rule-definition/test-group/test-artifact/test-version/test-type",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::FlowAnalysisRuleDefinition::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_flow_analysis_rule_definition(
                "test-group",
                "test-artifact",
                "test-version",
                "test-type",
            )
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_analysis_rule_types() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/flow-analysis-rule-types"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowAnalysisRuleTypesEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_flow_analysis_rule_types(None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_all_flow_analysis_results() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/flow-analysis/results"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowAnalysisResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_all_flow_analysis_results().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_analysis_results() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/flow/flow-analysis/results/test-process_group_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowAnalysisResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_flow_analysis_results("test-process_group_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_registry_client_definition() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/flow/flow-registry-client-definition/test-group/test-artifact/test-version/test-type",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::FlowRegistryClientDefinition::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_flow_registry_client_definition(
                "test-group",
                "test-artifact",
                "test-version",
                "test-type",
            )
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_query_history() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/history"))
            .and(query_param("offset", "test-value"))
            .and(query_param("count", "test-value"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::HistoryEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .query_history(
                "test-value",
                "test-value",
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_component_history() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/history/components/test-component_id"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ComponentHistoryEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_component_history("test-component_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_action() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/history/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ActionEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_action("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_listen_ports() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/listen-ports"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ListenPortsEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_listen_ports().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_metrics() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/metrics/test-producer"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_flow_metrics("test-producer", None, None, None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_parameter_contexts() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/parameter-contexts"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterContextsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_parameter_contexts().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_parameter_provider_definition() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/flow/parameter-provider-definition/test-group/test-artifact/test-version/test-type",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ParameterProviderDefinition::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_parameter_provider_definition(
                "test-group",
                "test-artifact",
                "test-version",
                "test-type",
            )
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_parameter_provider_types() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/parameter-provider-types"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterProviderTypesEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_parameter_provider_types(None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_parameter_providers() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/parameter-providers"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterProvidersEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_parameter_providers().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_prioritizers() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/prioritizers"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::PrioritizerTypesEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_prioritizers().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/process-groups/test-id"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ProcessGroupFlowEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_flow("test-id", None).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_schedule_components() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ScheduleComponentsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ScheduleComponentsEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .schedule_components("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_processor_definition() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/flow/processor-definition/test-group/test-artifact/test-version/test-type",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ProcessorDefinition::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_processor_definition("test-group", "test-artifact", "test-version", "test-type")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_processor_types() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/processor-types"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ProcessorTypesEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_processor_types(None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_registry_clients() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/registries"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowRegistryClientsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_registry_clients().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_reporting_task_definition() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/flow/reporting-task-definition/test-group/test-artifact/test-version/test-type",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ReportingTaskDefinition::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_reporting_task_definition(
                "test-group",
                "test-artifact",
                "test-version",
                "test-type",
            )
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_reporting_task_types() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/reporting-task-types"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ReportingTaskTypesEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .get_reporting_task_types(None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_reporting_tasks() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/reporting-tasks"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ReportingTasksEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_reporting_tasks().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_download_reporting_task_snapshot() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/reporting-tasks/download"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .download_reporting_task_snapshot(None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_reporting_task_snapshot() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/reporting-tasks/snapshot"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VersionedReportingTaskSnapshot::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_reporting_task_snapshot(None).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_runtime_manifest() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/runtime-manifest"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::RuntimeManifestEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_runtime_manifest().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_search_flow() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/search-results"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::SearchResultsEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().search_flow(None, None).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_controller_status() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/status"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ControllerStatusEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().get_controller_status().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_branches() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/registries/test-id/branches"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowRegistryBranchesEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().branches("test-id").get_branches().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_version_differences() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/flow/registries/test-registry_id/branches/test-branch_id_a/buckets/test-bucket_id_a/flows/test-flow_id_a/test-version_a/diff/branches/test-branch_id_b/buckets/test-bucket_id_b/flows/test-flow_id_b/test-version_b",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::FlowComparisonEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .branches("test-id")
            .get_version_differences(
                "test-registry_id",
                "test-branch_id_a",
                "test-bucket_id_a",
                "test-flow_id_a",
                "test-version_a",
                "test-branch_id_b",
                "test-bucket_id_b",
                "test-flow_id_b",
                "test-version_b",
                None,
                None,
            )
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_breadcrumbs() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/process-groups/test-id/breadcrumbs"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::FlowBreadcrumbEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .breadcrumbs("test-id")
            .get_breadcrumbs()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_buckets() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/registries/test-id/buckets"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::FlowRegistryBucketsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.flow_api().buckets("test-id").get_buckets(None).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flows() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/flow/registries/test-registry_id/buckets/test-bucket_id/flows",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::VersionedFlowsEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .buckets("test-id")
            .get_flows("test-registry_id", "test-bucket_id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_details() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/flow/registries/test-registry_id/buckets/test-bucket_id/flows/test-flow_id/details",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::VersionedFlowEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .buckets("test-id")
            .get_details("test-registry_id", "test-bucket_id", "test-flow_id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_versions() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/flow/registries/test-registry_id/buckets/test-bucket_id/flows/test-flow_id/versions",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::VersionedFlowSnapshotMetadataSetEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .buckets("test-id")
            .get_versions("test-registry_id", "test-bucket_id", "test-flow_id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_bulletins_1() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/flow/process-groups/test-id/bulletins/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ClearBulletinsForGroupResultsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ClearBulletinsForGroupRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .bulletins("test-id")
            .clear_bulletins_1(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_controller_services_from_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/flow/process-groups/test-id/controller-services",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ControllerServicesEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .controller_services("test-id")
            .get_controller_services_from_group(None, None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_activate_controller_services() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/nifi-api/flow/process-groups/test-id/controller-services",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ActivateControllerServicesEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ActivateControllerServicesEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .controller_services("test-id")
            .activate_controller_services(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_connection_statistics() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/connections/test-id/statistics"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ConnectionStatisticsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .statistics("test-id")
            .get_connection_statistics(None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_connection_status() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/connections/test-id/status"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ConnectionStatusEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .status("test-id")
            .get_connection_status(None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_connection_status_history() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/connections/test-id/status/history"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::StatusHistoryEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .status("test-id")
            .get_connection_status_history()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_input_port_status() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/input-ports/test-id/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::PortStatusEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .status("test-id")
            .get_input_port_status(None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_output_port_status() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/output-ports/test-id/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::PortStatusEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .status("test-id")
            .get_output_port_status(None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_process_group_status() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/process-groups/test-id/status"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ProcessGroupStatusEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .status("test-id")
            .get_process_group_status(None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_process_group_status_history() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/process-groups/test-id/status/history"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::StatusHistoryEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .status("test-id")
            .get_process_group_status_history()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_processor_status() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/processors/test-id/status"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ProcessorStatusEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .status("test-id")
            .get_processor_status(None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_processor_status_history() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/processors/test-id/status/history"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::StatusHistoryEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .status("test-id")
            .get_processor_status_history()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_remote_process_group_status() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/remote-process-groups/test-id/status"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::RemoteProcessGroupStatusEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .status("test-id")
            .get_remote_process_group_status(None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_remote_process_group_status_history() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/flow/remote-process-groups/test-id/status/history",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::StatusHistoryEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flow_api()
            .status("test-id")
            .get_remote_process_group_status_history()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod flowfilequeues_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_create_drop_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/flowfile-queues/test-id/drop-requests"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::DropRequestEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flowfilequeues_api()
            .drop_requests("test-id")
            .create_drop_request()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_remove_drop_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/nifi-api/flowfile-queues/test-id/drop-requests/test-drop_request_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::DropRequestEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flowfilequeues_api()
            .drop_requests("test-id")
            .remove_drop_request("test-drop_request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_drop_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/flowfile-queues/test-id/drop-requests/test-drop_request_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::DropRequestEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flowfilequeues_api()
            .drop_requests("test-id")
            .get_drop_request("test-drop_request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_flow_file() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/flowfile-queues/test-id/flowfiles/test-flowfile_uuid",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::FlowFileEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flowfilequeues_api()
            .flowfiles("test-id")
            .get_flow_file("test-flowfile_uuid", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_download_flow_file_content() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/flowfile-queues/test-id/flowfiles/test-flowfile_uuid/content",
            ))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flowfilequeues_api()
            .flowfiles("test-id")
            .download_flow_file_content("test-flowfile_uuid", None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_flow_file_listing() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/flowfile-queues/test-id/listing-requests"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ListingRequestEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flowfilequeues_api()
            .listing_requests("test-id")
            .create_flow_file_listing()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_listing_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/nifi-api/flowfile-queues/test-id/listing-requests/test-listing_request_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ListingRequestEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flowfilequeues_api()
            .listing_requests("test-id")
            .delete_listing_request("test-listing_request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_listing_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/flowfile-queues/test-id/listing-requests/test-listing_request_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ListingRequestEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .flowfilequeues_api()
            .listing_requests("test-id")
            .get_listing_request("test-listing_request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod funnels_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_remove_funnel() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/funnels/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::FunnelEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .funnels_api()
            .remove_funnel("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_funnel() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/funnels/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::FunnelEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.funnels_api().get_funnel("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_funnel() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/funnels/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::FunnelEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::FunnelEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.funnels_api().update_funnel("test-id", &body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod inputports_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_remove_input_port() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/input-ports/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::PortEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .inputports_api()
            .remove_input_port("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_input_port() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/input-ports/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::PortEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.inputports_api().get_input_port("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_input_port() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/input-ports/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::PortEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::PortEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .inputports_api()
            .update_input_port("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_bulletins_2() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/input-ports/test-id/bulletins/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ClearBulletinsResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ClearBulletinsRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .inputports_api()
            .bulletins("test-id")
            .clear_bulletins_2(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_run_status_2() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/input-ports/test-id/run-status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProcessorEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::PortRunStatusEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .inputports_api()
            .run_status("test-id")
            .update_run_status_2(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod labels_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_remove_label() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/labels/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::LabelEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .labels_api()
            .remove_label("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_label() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/labels/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::LabelEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.labels_api().get_label("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_label() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/labels/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::LabelEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::LabelEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.labels_api().update_label("test-id", &body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod outputports_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_remove_output_port() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/output-ports/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::PortEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .outputports_api()
            .remove_output_port("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_output_port() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/output-ports/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::PortEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.outputports_api().get_output_port("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_output_port() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/output-ports/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::PortEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::PortEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .outputports_api()
            .update_output_port("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_bulletins_3() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/output-ports/test-id/bulletins/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ClearBulletinsResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ClearBulletinsRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .outputports_api()
            .bulletins("test-id")
            .clear_bulletins_3(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_run_status_3() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/output-ports/test-id/run-status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProcessorEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::PortRunStatusEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .outputports_api()
            .run_status("test-id")
            .update_run_status_3(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod parametercontexts_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_create_parameter_context() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/parameter-contexts"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ParameterContextEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ParameterContextEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .create_parameter_context(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_parameter_context() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/parameter-contexts/test-id"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ParameterContextEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .delete_parameter_context("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_parameter_context() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/parameter-contexts/test-id"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ParameterContextEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .get_parameter_context("test-id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_parameter_context() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/parameter-contexts/test-id"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ParameterContextEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ParameterContextEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .update_parameter_context("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_assets() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/parameter-contexts/test-context_id/assets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::AssetsEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .assets("test-context_id")
            .get_assets()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_asset() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/parameter-contexts/test-context_id/assets"))
            .and(header("content-type", "application/octet-stream"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::AssetEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .assets("test-context_id")
            .create_asset(Some("test.nar"), vec![1u8, 2, 3])
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_asset() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/nifi-api/parameter-contexts/test-context_id/assets/test-asset_id",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::AssetEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .assets("test-context_id")
            .delete_asset("test-asset_id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_asset_content() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/parameter-contexts/test-context_id/assets/test-asset_id",
            ))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .assets("test-context_id")
            .get_asset_content("test-asset_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_submit_parameter_context_update() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/parameter-contexts/test-context_id/update-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterContextUpdateRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ParameterContextEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .update_requests("test-context_id")
            .submit_parameter_context_update(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_update_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/nifi-api/parameter-contexts/test-context_id/update-requests/test-request_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterContextUpdateRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .update_requests("test-context_id")
            .delete_update_request("test-request_id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_parameter_context_update() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/parameter-contexts/test-context_id/update-requests/test-request_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterContextUpdateRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .update_requests("test-context_id")
            .get_parameter_context_update("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_submit_validation_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/parameter-contexts/test-context_id/validation-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterContextValidationRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ParameterContextValidationRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .validation_requests("test-context_id")
            .submit_validation_request(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_validation_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/nifi-api/parameter-contexts/test-context_id/validation-requests/test-id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterContextValidationRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .validation_requests("test-context_id")
            .delete_validation_request("test-id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_validation_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/parameter-contexts/test-context_id/validation-requests/test-id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterContextValidationRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parametercontexts_api()
            .validation_requests("test-context_id")
            .get_validation_request("test-id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod parameterproviders_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_remove_parameter_provider() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/parameter-providers/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterProviderEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .remove_parameter_provider("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_parameter_provider() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/parameter-providers/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterProviderEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .get_parameter_provider("test-id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_parameter_provider() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/parameter-providers/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterProviderEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ParameterProviderEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .update_parameter_provider("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_submit_apply_parameters() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(
                path(
                    "/nifi-api/parameter-providers/test-provider_id/apply-parameters-requests",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ParameterProviderApplyParametersRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ParameterProviderParameterApplicationEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .apply_parameters_requests("test-provider_id")
            .submit_apply_parameters(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_apply_parameters_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(
                path(
                    "/nifi-api/parameter-providers/test-provider_id/apply-parameters-requests/test-request_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ParameterProviderApplyParametersRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .apply_parameters_requests("test-provider_id")
            .delete_apply_parameters_request("test-request_id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_parameter_provider_apply_parameters_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/parameter-providers/test-provider_id/apply-parameters-requests/test-request_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ParameterProviderApplyParametersRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .apply_parameters_requests("test-provider_id")
            .get_parameter_provider_apply_parameters_request("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_bulletins_4() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/parameter-providers/test-id/bulletins/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ClearBulletinsResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ClearBulletinsRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .bulletins("test-id")
            .clear_bulletins_4(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_analyze_configuration_1() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/parameter-providers/test-id/config/analysis",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ConfigurationAnalysisEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ConfigurationAnalysisEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .config("test-id")
            .analyze_configuration_1(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_submit_config_verification_request_1() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/parameter-providers/test-id/config/verification-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::VerifyConfigRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .config("test-id")
            .submit_config_verification_request_1(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_verification_request_1() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(
                path(
                    "/nifi-api/parameter-providers/test-id/config/verification-requests/test-request_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .config("test-id")
            .delete_verification_request_1("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_verification_request_1() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/parameter-providers/test-id/config/verification-requests/test-request_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .config("test-id")
            .get_verification_request_1("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_property_descriptor_2() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/parameter-providers/test-id/descriptors"))
            .and(query_param("propertyName", "test-value"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::PropertyDescriptorEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .descriptors("test-id")
            .get_property_descriptor_2("test-value")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_fetch_parameters() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/parameter-providers/test-id/parameters/fetch-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ParameterProviderEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ParameterProviderParameterFetchEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .parameters("test-id")
            .fetch_parameters(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_parameter_provider_references() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/parameter-providers/test-id/references"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::ParameterProviderReferencingComponentsEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .references("test-id")
            .get_parameter_provider_references()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_state_1() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/parameter-providers/test-id/state"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ComponentStateEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .state("test-id")
            .get_state_1()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_state_2() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/parameter-providers/test-id/state/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ComponentStateEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ComponentStateEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .parameterproviders_api()
            .state("test-id")
            .clear_state_2(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod policies_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_create_access_policy() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/policies"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::AccessPolicyEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::AccessPolicyEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.policies_api().create_access_policy(&body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_access_policy_for_resource() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/policies/test-action/test-resource"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::AccessPolicyEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .policies_api()
            .get_access_policy_for_resource("test-action", "test-resource")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_remove_access_policy() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/policies/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::AccessPolicyEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .policies_api()
            .remove_access_policy("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_access_policy() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/policies/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::AccessPolicyEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.policies_api().get_access_policy("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_access_policy() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/policies/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::AccessPolicyEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::AccessPolicyEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .policies_api()
            .update_access_policy("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod processgroups_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_delete_replace_process_group_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/process-groups/replace-requests/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ProcessGroupReplaceRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .delete_replace_process_group_request("test-id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_replace_process_group_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/replace-requests/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ProcessGroupReplaceRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .get_replace_process_group_request("test-id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_remove_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/process-groups/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ProcessGroupEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .remove_process_group("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ProcessGroupEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .get_process_group("test-id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/process-groups/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ProcessGroupEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ProcessGroupEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .update_process_group("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_connections() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/test-id/connections"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ConnectionsEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .connections("test-id")
            .get_connections()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_connection() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/process-groups/test-id/connections"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ConnectionEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ConnectionEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .connections("test-id")
            .create_connection(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_controller_service_1() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/process-groups/test-id/controller-services"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ControllerServiceEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ControllerServiceEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .controller_services("test-id")
            .create_controller_service_1(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_copy() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/process-groups/test-id/copy"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::CopyResponseEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::CopyRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.processgroups_api().copy("test-id").copy(&body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_export_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/test-id/download"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .download("test-id")
            .export_process_group(None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_empty_all_connections_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/process-groups/test-id/empty-all-connections-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::DropRequestEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .empty_all_connections_requests("test-id")
            .create_empty_all_connections_request()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_remove_drop_request_1() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(
                path(
                    "/nifi-api/process-groups/test-id/empty-all-connections-requests/test-drop_request_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::DropRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .empty_all_connections_requests("test-id")
            .remove_drop_request_1("test-drop_request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_drop_all_flowfiles_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(
                path(
                    "/nifi-api/process-groups/test-id/empty-all-connections-requests/test-drop_request_id",
                ),
            )
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(
                        serde_json::to_value(
                                nifi_rust_client::types::DropRequestEntity::default(),
                            )
                            .unwrap(),
                    ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .empty_all_connections_requests("test-id")
            .get_drop_all_flowfiles_request("test-drop_request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_replace_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/process-groups/test-id/flow-contents"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ProcessGroupImportEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ProcessGroupImportEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .flow_contents("test-id")
            .replace_process_group(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_funnels() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/test-id/funnels"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::FunnelsEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .funnels("test-id")
            .get_funnels()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_funnel() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/process-groups/test-id/funnels"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::FunnelEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::FunnelEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .funnels("test-id")
            .create_funnel(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_input_ports() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/test-id/input-ports"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::InputPortsEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .input_ports("test-id")
            .get_input_ports()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_input_port() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/process-groups/test-id/input-ports"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::PortEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::PortEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .input_ports("test-id")
            .create_input_port(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_labels() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/test-id/labels"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::LabelsEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .labels("test-id")
            .get_labels()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_label() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/process-groups/test-id/labels"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::LabelEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::LabelEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .labels("test-id")
            .create_label(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_local_modifications() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/test-id/local-modifications"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::FlowComparisonEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .local_modifications("test-id")
            .get_local_modifications()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_output_ports() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/test-id/output-ports"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::OutputPortsEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .output_ports("test-id")
            .get_output_ports()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_output_port() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/process-groups/test-id/output-ports"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::PortEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::PortEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .output_ports("test-id")
            .create_output_port(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_paste() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/process-groups/test-id/paste"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::PasteResponseEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::PasteRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .paste("test-id")
            .paste(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_process_groups() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/test-id/process-groups"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ProcessGroupsEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .process_groups("test-id")
            .get_process_groups()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/process-groups/test-id/process-groups"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ProcessGroupEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ProcessGroupEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .process_groups("test-id")
            .create_process_group(None, &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_import_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/process-groups/test-id/process-groups/import",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ProcessGroupEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ProcessGroupUploadEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .process_groups("test-id")
            .import_process_group(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_upload_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/process-groups/test-id/process-groups/upload",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ProcessGroupEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .process_groups("test-id")
            .upload_process_group()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_processors() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/test-id/processors"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProcessorsEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .processors("test-id")
            .get_processors(None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_processor() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/process-groups/test-id/processors"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProcessorEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ProcessorEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .processors("test-id")
            .create_processor(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_remote_process_groups() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/process-groups/test-id/remote-process-groups",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::RemoteProcessGroupsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .remote_process_groups("test-id")
            .get_remote_process_groups()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_remote_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/process-groups/test-id/remote-process-groups",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::RemoteProcessGroupEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::RemoteProcessGroupEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .remote_process_groups("test-id")
            .create_remote_process_group(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_initiate_replace_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/process-groups/test-id/replace-requests"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ProcessGroupReplaceRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ProcessGroupImportEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .replace_requests("test-id")
            .initiate_replace_process_group(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_copy_snippet() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/process-groups/test-id/snippet-instance"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::FlowEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::CopySnippetRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processgroups_api()
            .snippet_instance("test-id")
            .copy_snippet(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod processors_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_get_processor_run_status_details() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/processors/run-status-details/queries"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ProcessorsRunStatusDetailsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::RunStatusDetailsRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .get_processor_run_status_details(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_processor() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/processors/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProcessorEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .delete_processor("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_processor() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/processors/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProcessorEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.processors_api().get_processor("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_processor() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/processors/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProcessorEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ProcessorEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .update_processor("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_bulletins_5() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/processors/test-id/bulletins/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ClearBulletinsResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ClearBulletinsRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .bulletins("test-id")
            .clear_bulletins_5(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_analyze_configuration_2() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/processors/test-id/config/analysis"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ConfigurationAnalysisEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ConfigurationAnalysisEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .config("test-id")
            .analyze_configuration_2(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_submit_processor_verification_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/processors/test-id/config/verification-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::VerifyConfigRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .config("test-id")
            .submit_processor_verification_request(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_verification_request_2() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/nifi-api/processors/test-id/config/verification-requests/test-request_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .config("test-id")
            .delete_verification_request_2("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_verification_request_2() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/processors/test-id/config/verification-requests/test-request_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .config("test-id")
            .get_verification_request_2("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_property_descriptor_3() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/processors/test-id/descriptors"))
            .and(query_param("propertyName", "test-value"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::PropertyDescriptorEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .descriptors("test-id")
            .get_property_descriptor_3(None, "test-value", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_processor_diagnostics() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/processors/test-id/diagnostics"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProcessorEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .diagnostics("test-id")
            .get_processor_diagnostics()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_run_status_4() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/processors/test-id/run-status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProcessorEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ProcessorRunStatusEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .run_status("test-id")
            .update_run_status_4(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_state_2() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/processors/test-id/state"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ComponentStateEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.processors_api().state("test-id").get_state_2().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_state_3() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/processors/test-id/state/clear-requests"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ComponentStateEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ComponentStateEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .state("test-id")
            .clear_state_3(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_terminate_processor() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/processors/test-id/threads"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProcessorEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .processors_api()
            .threads("test-id")
            .terminate_processor()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod provenance_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_submit_provenance_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/provenance"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProvenanceEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ProvenanceEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .provenance_api()
            .submit_provenance_request(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_submit_lineage_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/provenance/lineage"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::LineageEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::LineageEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.provenance_api().submit_lineage_request(&body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_lineage() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/provenance/lineage/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::LineageEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .provenance_api()
            .delete_lineage("test-id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_lineage() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/provenance/lineage/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::LineageEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.provenance_api().get_lineage("test-id", None).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_search_options() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/provenance/search-options"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ProvenanceOptionsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.provenance_api().get_search_options().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_provenance() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/provenance/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProvenanceEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .provenance_api()
            .delete_provenance("test-id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_provenance() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/provenance/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ProvenanceEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .provenance_api()
            .get_provenance("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod provenanceevents_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_submit_replay_latest_event() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/provenance-events/latest/replays"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ReplayLastEventResponseEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ReplayLastEventRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .provenanceevents_api()
            .submit_replay_latest_event(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_latest_provenance_events() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/provenance-events/latest/test-component_id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::LatestProvenanceEventsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .provenanceevents_api()
            .get_latest_provenance_events("test-component_id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_submit_replay() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/provenance-events/replays"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ProvenanceEventEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::SubmitReplayRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.provenanceevents_api().submit_replay(&body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_provenance_event() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/provenance-events/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ProvenanceEventEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .provenanceevents_api()
            .get_provenance_event("test-id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_input_content() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/provenance-events/test-id/content/input"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .provenanceevents_api()
            .content("test-id")
            .get_input_content(None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_output_content() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/provenance-events/test-id/content/output"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .provenanceevents_api()
            .content("test-id")
            .get_output_content(None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod remoteprocessgroups_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_remove_remote_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/remote-process-groups/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::RemoteProcessGroupEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .remoteprocessgroups_api()
            .remove_remote_process_group("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_remote_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/remote-process-groups/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::RemoteProcessGroupEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .remoteprocessgroups_api()
            .get_remote_process_group("test-id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_remote_process_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/remote-process-groups/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::RemoteProcessGroupEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::RemoteProcessGroupEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .remoteprocessgroups_api()
            .update_remote_process_group("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_bulletins_6() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/remote-process-groups/test-id/bulletins/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ClearBulletinsResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ClearBulletinsRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .remoteprocessgroups_api()
            .bulletins("test-id")
            .clear_bulletins_6(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_remote_process_group_input_port() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/nifi-api/remote-process-groups/test-id/input-ports/test-port_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::RemoteProcessGroupPortEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::RemoteProcessGroupPortEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .remoteprocessgroups_api()
            .input_ports("test-id")
            .update_remote_process_group_input_port("test-port_id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_remote_process_group_input_port_run_status() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/nifi-api/remote-process-groups/test-id/input-ports/test-port_id/run-status",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::RemoteProcessGroupPortEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::RemotePortRunStatusEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .remoteprocessgroups_api()
            .input_ports("test-id")
            .update_remote_process_group_input_port_run_status("test-port_id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_remote_process_group_output_port() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/nifi-api/remote-process-groups/test-id/output-ports/test-port_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::RemoteProcessGroupPortEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::RemoteProcessGroupPortEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .remoteprocessgroups_api()
            .output_ports("test-id")
            .update_remote_process_group_output_port("test-port_id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_remote_process_group_output_port_run_status() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/nifi-api/remote-process-groups/test-id/output-ports/test-port_id/run-status",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::RemoteProcessGroupPortEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::RemotePortRunStatusEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .remoteprocessgroups_api()
            .output_ports("test-id")
            .update_remote_process_group_output_port_run_status("test-port_id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_remote_process_group_run_statuses() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/nifi-api/remote-process-groups/process-group/test-id/run-status",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::RemoteProcessGroupEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::RemotePortRunStatusEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .remoteprocessgroups_api()
            .run_status("test-id")
            .update_remote_process_group_run_statuses(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_remote_process_group_run_status() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/remote-process-groups/test-id/run-status"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::RemoteProcessGroupEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::RemotePortRunStatusEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .remoteprocessgroups_api()
            .run_status("test-id")
            .update_remote_process_group_run_status(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_state_3() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/remote-process-groups/test-id/state"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ComponentStateEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .remoteprocessgroups_api()
            .state("test-id")
            .get_state_3()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod reportingtasks_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_remove_reporting_task() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/reporting-tasks/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ReportingTaskEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .reportingtasks_api()
            .remove_reporting_task("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_reporting_task() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/reporting-tasks/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ReportingTaskEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .reportingtasks_api()
            .get_reporting_task("test-id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_reporting_task() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/reporting-tasks/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ReportingTaskEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ReportingTaskEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .reportingtasks_api()
            .update_reporting_task("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_bulletins_7() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/reporting-tasks/test-id/bulletins/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ClearBulletinsResultEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ClearBulletinsRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .reportingtasks_api()
            .bulletins("test-id")
            .clear_bulletins_7(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_analyze_configuration_3() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/reporting-tasks/test-id/config/analysis"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::ConfigurationAnalysisEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ConfigurationAnalysisEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .reportingtasks_api()
            .config("test-id")
            .analyze_configuration_3(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_submit_config_verification_request_2() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/reporting-tasks/test-id/config/verification-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::VerifyConfigRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .reportingtasks_api()
            .config("test-id")
            .submit_config_verification_request_2(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_verification_request_3() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/nifi-api/reporting-tasks/test-id/config/verification-requests/test-request_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .reportingtasks_api()
            .config("test-id")
            .delete_verification_request_3("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_verification_request_3() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/reporting-tasks/test-id/config/verification-requests/test-request_id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VerifyConfigRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .reportingtasks_api()
            .config("test-id")
            .get_verification_request_3("test-request_id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_property_descriptor_4() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/reporting-tasks/test-id/descriptors"))
            .and(query_param("propertyName", "test-value"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::PropertyDescriptorEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .reportingtasks_api()
            .descriptors("test-id")
            .get_property_descriptor_4("test-value", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_run_status_5() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/reporting-tasks/test-id/run-status"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ReportingTaskEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ReportingTaskRunStatusEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .reportingtasks_api()
            .run_status("test-id")
            .update_run_status_5(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_state_4() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/reporting-tasks/test-id/state"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ComponentStateEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .reportingtasks_api()
            .state("test-id")
            .get_state_4()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_clear_state_4() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/reporting-tasks/test-id/state/clear-requests",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(nifi_rust_client::types::ComponentStateEntity::default())
                        .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::ComponentStateEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .reportingtasks_api()
            .state("test-id")
            .clear_state_4(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod resources_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_get_resources() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/resources"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ResourcesEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.resources_api().get_resources().await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod sitetosite_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_get_site_to_site_details() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/site-to-site"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::ControllerEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.sitetosite_api().get_site_to_site_details().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_peers() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/site-to-site/peers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::PeersEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.sitetosite_api().get_peers().await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod snippets_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_create_snippet() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/snippets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::SnippetEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::SnippetEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.snippets_api().create_snippet(&body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_snippet() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/snippets/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::SnippetEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.snippets_api().delete_snippet("test-id", None).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_snippet() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/snippets/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::SnippetEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::SnippetEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.snippets_api().update_snippet("test-id", &body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod systemdiagnostics_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_get_system_diagnostics() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/system-diagnostics"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::SystemDiagnosticsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .systemdiagnostics_api()
            .get_system_diagnostics(None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_jmx_metrics() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/system-diagnostics/jmx-metrics"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::JmxMetricsResultsEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.systemdiagnostics_api().get_jmx_metrics(None).await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod tenants_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_search_tenants() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/tenants/search-results"))
            .and(query_param("q", "test-value"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::TenantsEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.tenants_api().search_tenants("test-value").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_user_groups() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/tenants/user-groups"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::UserGroupsEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.tenants_api().get_user_groups().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_user_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/tenants/user-groups"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::UserGroupEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::UserGroupEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.tenants_api().create_user_group(&body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_remove_user_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/tenants/user-groups/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::UserGroupEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .tenants_api()
            .remove_user_group("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_user_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/tenants/user-groups/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::UserGroupEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.tenants_api().get_user_group("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_user_group() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/tenants/user-groups/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::UserGroupEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::UserGroupEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .tenants_api()
            .update_user_group("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_users() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/tenants/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::UsersEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.tenants_api().get_users().await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_create_user() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/tenants/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::UserEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::UserEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.tenants_api().create_user(&body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_remove_user() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/tenants/users/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::UserEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .tenants_api()
            .remove_user("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_user() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/tenants/users/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::UserEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.tenants_api().get_user("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_user() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/tenants/users/test-id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::to_value(nifi_rust_client::types::UserEntity::default()).unwrap(),
            ))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::UserEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.tenants_api().update_user("test-id", &body).await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
#[cfg(test)]
mod versions_generated_tests {
    use super::*;
    #[tokio::test]
    async fn test_create_version_control_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/versions/active-requests"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::CreateActiveRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .versions_api()
            .create_version_control_request(&body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_version_control_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/versions/active-requests/test-id"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .versions_api()
            .delete_version_control_request("test-id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_version_control_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/versions/active-requests/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VersionControlInformationEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::VersionControlComponentMappingEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .versions_api()
            .update_version_control_request("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_stop_version_control() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/versions/process-groups/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VersionControlInformationEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .versions_api()
            .stop_version_control("test-id", None, None, None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_version_information() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/versions/process-groups/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VersionControlInformationEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .versions_api()
            .get_version_information("test-id")
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_save_to_flow_registry() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/versions/process-groups/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VersionControlInformationEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::StartVersionControlRequestEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .versions_api()
            .save_to_flow_registry("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_update_flow_version() {
        let mock_server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/versions/process-groups/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VersionControlInformationEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::VersionedFlowSnapshotEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .versions_api()
            .update_flow_version("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_initiate_revert_flow_version() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/versions/revert-requests/process-groups/test-id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VersionedFlowUpdateRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::VersionControlInformationEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .versions_api()
            .initiate_revert_flow_version("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_revert_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/versions/revert-requests/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VersionedFlowUpdateRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .versions_api()
            .delete_revert_request("test-id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_revert_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/versions/revert-requests/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VersionedFlowUpdateRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.versions_api().get_revert_request("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_initiate_version_control_update() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/versions/update-requests/process-groups/test-id",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VersionedFlowUpdateRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let body = nifi_rust_client::types::VersionControlInformationEntity {
            ..Default::default()
        };
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .versions_api()
            .initiate_version_control_update("test-id", &body)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_delete_update_request_1() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/versions/update-requests/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VersionedFlowUpdateRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .versions_api()
            .delete_update_request_1("test-id", None)
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_get_update_request() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/versions/update-requests/test-id"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::to_value(
                        nifi_rust_client::types::VersionedFlowUpdateRequestEntity::default(),
                    )
                    .unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client.versions_api().get_update_request("test-id").await;
        assert!(result.is_ok(), "{:?}", result);
    }
    #[tokio::test]
    async fn test_export_flow_version() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/versions/process-groups/test-id/download"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let client = NifiClientBuilder::new(&mock_server.uri())
            .unwrap()
            .build()
            .unwrap();
        let result = client
            .versions_api()
            .download("test-id")
            .export_flow_version()
            .await;
        assert!(result.is_ok(), "{:?}", result);
    }
}
