#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::NifiError;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── get_parameter_provider ────────────────────────────────────────────────────

#[tokio::test]
async fn get_parameter_provider_returns_id_and_name() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/parameter-providers/pp-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "pp-1",
            "component": {
                "id": "pp-1",
                "name": "MyProvider",
                "type": "org.apache.nifi.parameter.FileBasedParameterProvider"
            },
            "revision": { "version": 0 }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let pp = client
        .parameterproviders_api()
        .get_parameter_provider("pp-1")
        .await
        .unwrap();

    assert_eq!(pp.id.as_deref(), Some("pp-1"));
    assert_eq!(
        pp.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("MyProvider")
    );
}

// ── update_parameter_provider ─────────────────────────────────────────────────

#[tokio::test]
async fn update_parameter_provider_sends_body_and_returns_entity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/nifi-api/parameter-providers/pp-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "pp-1",
            "component": { "id": "pp-1", "name": "UpdatedProvider" },
            "revision": { "version": 1 }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ParameterProviderEntity::default();
    let updated = client
        .parameterproviders_api()
        .update_parameter_provider("pp-1", &body)
        .await
        .unwrap();

    assert_eq!(updated.id.as_deref(), Some("pp-1"));
    assert_eq!(
        updated.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("UpdatedProvider")
    );
}

// ── remove_parameter_provider ─────────────────────────────────────────────────

#[tokio::test]
async fn remove_parameter_provider_returns_entity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/parameter-providers/pp-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "pp-1",
            "component": { "id": "pp-1", "name": "RemovedProvider" },
            "revision": { "version": 1 }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .parameterproviders_api()
        .remove_parameter_provider("pp-1", Some("1"), None, None)
        .await
        .unwrap();

    assert_eq!(result.id.as_deref(), Some("pp-1"));
}

// ── behavior: 409 on conflicting delete ──────────────────────────────────────

#[tokio::test]
async fn remove_parameter_provider_returns_409_when_conflict() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/parameter-providers/pp-1"))
        .respond_with(
            ResponseTemplate::new(409)
                .set_body_json(serde_json::json!({"message": "Cannot delete while in use"})),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client
        .parameterproviders_api()
        .remove_parameter_provider("pp-1", Some("1"), None, None)
        .await
        .unwrap_err();

    assert!(matches!(err, NifiError::Conflict { .. }));
}

// ── get_parameter_provider_references ────────────────────────────────────────

#[tokio::test]
async fn get_parameter_provider_references_returns_entity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/parameter-providers/pp-1/references"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "parameterProviderReferencingComponents": []
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .parameterproviders_api()
        .references("pp-1")
        .get_parameter_provider_references()
        .await
        .unwrap();

    assert!(
        result
            .parameter_provider_referencing_components
            .as_ref()
            .map(|v| v.is_empty())
            .unwrap_or(true)
    );
}

// ── get_state ─────────────────────────────────────────────────────────────────

#[tokio::test]
async fn get_state_returns_component_state() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/parameter-providers/pp-1/state"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "componentState": {
                "componentId": "pp-1",
                "localState": { "state": [] },
                "clusterState": null
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let state = client
        .parameterproviders_api()
        .state("pp-1")
        .get_state_1()
        .await
        .unwrap();

    assert_eq!(state.component_id.as_deref(), Some("pp-1"));
}

// ── clear_state ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn clear_state_returns_component_state() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/parameter-providers/pp-1/state/clear-requests"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "componentState": {
                "componentId": "pp-1",
                "localState": { "state": [] }
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ComponentStateEntity::default();
    let state = client
        .parameterproviders_api()
        .state("pp-1")
        .clear_state_2(&body)
        .await
        .unwrap();

    assert_eq!(state.component_id.as_deref(), Some("pp-1"));
}

// ── get_property_descriptor ───────────────────────────────────────────────────

#[tokio::test]
async fn get_property_descriptor_returns_display_name() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/parameter-providers/pp-1/descriptors"))
        .and(query_param("propertyName", "Directory"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "propertyDescriptor": {
                "name": "Directory",
                "displayName": "Parameter Directory",
                "description": "The directory to read parameters from"
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let descriptor = client
        .parameterproviders_api()
        .descriptors("pp-1")
        .get_property_descriptor_2("Directory")
        .await
        .unwrap();

    assert_eq!(descriptor.name.as_deref(), Some("Directory"));
    assert_eq!(
        descriptor.display_name.as_deref(),
        Some("Parameter Directory")
    );
}

// ── analyze_configuration ─────────────────────────────────────────────────────

#[tokio::test]
async fn analyze_configuration_returns_dto() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/parameter-providers/pp-1/config/analysis"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "configurationAnalysis": {
                "componentId": "pp-1",
                "properties": {}
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ConfigurationAnalysisEntity::default();
    let analysis = client
        .parameterproviders_api()
        .config("pp-1")
        .analyze_configuration_1(&body)
        .await
        .unwrap();

    assert_eq!(analysis.component_id.as_deref(), Some("pp-1"));
}

// ── submit_config_verification_request ───────────────────────────────────────

#[tokio::test]
async fn submit_config_verification_request_returns_dto() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/nifi-api/parameter-providers/pp-1/config/verification-requests",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "request": {
                "requestId": "vr-1",
                "complete": false,
                "percentCompleted": 0
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::VerifyConfigRequestEntity::default();
    let req = client
        .parameterproviders_api()
        .config("pp-1")
        .submit_config_verification_request_1(&body)
        .await
        .unwrap();

    assert_eq!(req.request_id.as_deref(), Some("vr-1"));
}

// ── get_verification_request ──────────────────────────────────────────────────

#[tokio::test]
async fn get_verification_request_returns_dto() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/nifi-api/parameter-providers/pp-1/config/verification-requests/vr-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "request": {
                "requestId": "vr-1",
                "complete": true,
                "percentCompleted": 100
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let req = client
        .parameterproviders_api()
        .config("pp-1")
        .get_verification_request_1("vr-1")
        .await
        .unwrap();

    assert_eq!(req.request_id.as_deref(), Some("vr-1"));
    assert_eq!(req.percent_completed, Some(100));
}

// ── delete_verification_request ───────────────────────────────────────────────

#[tokio::test]
async fn delete_verification_request_returns_dto() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(
            "/nifi-api/parameter-providers/pp-1/config/verification-requests/vr-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "request": {
                "requestId": "vr-1",
                "complete": true,
                "percentCompleted": 100
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let req = client
        .parameterproviders_api()
        .config("pp-1")
        .delete_verification_request_1("vr-1")
        .await
        .unwrap();

    assert_eq!(req.request_id.as_deref(), Some("vr-1"));
    assert!(req.complete.unwrap_or(false));
}

// ── fetch_parameters ──────────────────────────────────────────────────────────

#[tokio::test]
async fn fetch_parameters_returns_entity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/nifi-api/parameter-providers/pp-1/parameters/fetch-requests",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "pp-1",
            "component": {
                "id": "pp-1",
                "name": "MyProvider",
                "fetchedParameterGroups": []
            },
            "revision": { "version": 1 }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ParameterProviderParameterFetchEntity::default();
    let result = client
        .parameterproviders_api()
        .parameters("pp-1")
        .fetch_parameters(&body)
        .await
        .unwrap();

    assert_eq!(result.id.as_deref(), Some("pp-1"));
}

// ── submit_apply_parameters ───────────────────────────────────────────────────

#[tokio::test]
async fn submit_apply_parameters_returns_dto() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/nifi-api/parameter-providers/pp-1/apply-parameters-requests",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "request": {
                "requestId": "apr-1",
                "complete": false,
                "percentCompleted": 0,
                "state": "Pending"
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body =
        nifi_rust_client::types::ParameterProviderParameterApplicationEntity::default();
    let req = client
        .parameterproviders_api()
        .apply_parameters_requests("pp-1")
        .submit_apply_parameters(&body)
        .await
        .unwrap();

    assert_eq!(req.request_id.as_deref(), Some("apr-1"));
    assert_eq!(req.state.as_deref(), Some("Pending"));
}

// ── get_apply_parameters_request ─────────────────────────────────────────────

#[tokio::test]
async fn get_apply_parameters_request_returns_dto() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/nifi-api/parameter-providers/pp-1/apply-parameters-requests/apr-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "request": {
                "requestId": "apr-1",
                "complete": true,
                "percentCompleted": 100,
                "state": "Complete"
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let req = client
        .parameterproviders_api()
        .apply_parameters_requests("pp-1")
        .get_parameter_provider_apply_parameters_request("apr-1")
        .await
        .unwrap();

    assert_eq!(req.request_id.as_deref(), Some("apr-1"));
    assert_eq!(req.percent_completed, Some(100));
}

// ── delete_apply_parameters_request ──────────────────────────────────────────

#[tokio::test]
async fn delete_apply_parameters_request_returns_dto() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(
            "/nifi-api/parameter-providers/pp-1/apply-parameters-requests/apr-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "request": {
                "requestId": "apr-1",
                "complete": true,
                "percentCompleted": 100
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let req = client
        .parameterproviders_api()
        .apply_parameters_requests("pp-1")
        .delete_apply_parameters_request("apr-1", None)
        .await
        .unwrap();

    assert_eq!(req.request_id.as_deref(), Some("apr-1"));
    assert!(req.complete.unwrap_or(false));
}

// ── clear_bulletins (parameter-providers sub-resource) ───────────────────────

#[tokio::test]
async fn clear_bulletins_returns_result_entity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/nifi-api/parameter-providers/pp-1/bulletins/clear-requests",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bulletinsCleared": 3,
            "componentId": "pp-1"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ClearBulletinsRequestEntity::default();
    let result = client
        .parameterproviders_api()
        .bulletins("pp-1")
        .clear_bulletins_4(&body)
        .await
        .unwrap();

    assert_eq!(result.bulletins_cleared, Some(3));
    assert_eq!(result.component_id.as_deref(), Some("pp-1"));
}

// ── create_parameter_provider (via controller_api) ───────────────────────────

#[tokio::test]
async fn create_parameter_provider_returns_entity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/controller/parameter-providers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "pp-new",
            "component": {
                "id": "pp-new",
                "name": "NewProvider",
                "type": "org.apache.nifi.parameter.FileBasedParameterProvider"
            },
            "revision": { "version": 0 }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ParameterProviderEntity::default();
    let created = client
        .controller_api()
        .create_parameter_provider(&body)
        .await
        .unwrap();

    assert_eq!(created.id.as_deref(), Some("pp-new"));
    assert_eq!(
        created.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("NewProvider")
    );
}

// ── clear_parameter_provider_bulletins (via controller_api bulletins sub-resource)

#[tokio::test]
async fn controller_clear_parameter_provider_bulletins_returns_result() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/nifi-api/controller/parameter-providers/pp-1/bulletins/clear-requests",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bulletinsCleared": 2,
            "componentId": "pp-1"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ClearBulletinsRequestEntity::default();
    let result = client
        .controller_api()
        .bulletins("pp-1")
        .clear_parameter_provider_bulletins(&body)
        .await
        .unwrap();

    assert_eq!(result.bulletins_cleared, Some(2));
    assert_eq!(result.component_id.as_deref(), Some("pp-1"));
}

// ── get_parameter_providers (via flow_api) ────────────────────────────────────

#[tokio::test]
async fn flow_get_parameter_providers_returns_list() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/parameter-providers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "parameterProviders": [
                {
                    "id": "pp-1",
                    "component": { "id": "pp-1", "name": "MyProvider" }
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
    let result = client
        .flow_api()
        .get_parameter_providers()
        .await
        .unwrap();

    let providers = result.parameter_providers.unwrap_or_default();
    assert_eq!(providers.len(), 1);
    assert_eq!(providers[0].id.as_deref(), Some("pp-1"));
}
