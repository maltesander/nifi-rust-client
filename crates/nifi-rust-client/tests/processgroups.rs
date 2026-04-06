#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::types::ParameterContextHandlingStrategy;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn pg_entity() -> serde_json::Value {
    serde_json::to_value(nifi_rust_client::types::ProcessGroupEntity::default())
        .expect("default ProcessGroupEntity is serializable")
}

fn processor_entity() -> serde_json::Value {
    serde_json::to_value(nifi_rust_client::types::ProcessorEntity::default())
        .expect("default ProcessorEntity is serializable")
}

fn processors_entity() -> serde_json::Value {
    serde_json::to_value(nifi_rust_client::types::ProcessorsEntity::default())
        .expect("default ProcessorsEntity is serializable")
}

// ── create_process_group ──────────────────────────────────────────────────────

#[tokio::test]
async fn create_process_group_sends_parameter_context_handling_strategy() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/process-groups/parent-id/process-groups"))
        .and(query_param(
            "parameterContextHandlingStrategy",
            "KEEP_EXISTING",
        ))
        .respond_with(ResponseTemplate::new(201).set_body_json(pg_entity()))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ProcessGroupEntity::default();
    let result = client
        .processgroups_api()
        .process_groups("parent-id")
        .create_process_group(Some(ParameterContextHandlingStrategy::KeepExisting), &body)
        .await;
    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn create_process_group_omits_query_when_strategy_is_none() {
    let mock_server = MockServer::start().await;
    // No query_param matcher — wiremock will reject if the param is present
    Mock::given(method("POST"))
        .and(path("/nifi-api/process-groups/parent-id/process-groups"))
        .respond_with(ResponseTemplate::new(201).set_body_json(pg_entity()))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ProcessGroupEntity::default();
    let result = client
        .processgroups_api()
        .process_groups("parent-id")
        .create_process_group(None, &body)
        .await;
    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn create_process_group_returns_entity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/process-groups/parent-id/process-groups"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "new-pg-id",
            "component": { "id": "new-pg-id", "name": "My Group" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ProcessGroupEntity::default();
    let entity = client
        .processgroups_api()
        .process_groups("parent-id")
        .create_process_group(None, &body)
        .await
        .unwrap();
    assert_eq!(entity.id.as_deref(), Some("new-pg-id"));
}

// ── remove_process_group ─────────────────────────────────────────────────────

#[tokio::test]
async fn remove_process_group_sends_version_query_param() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/process-groups/pg-id"))
        .and(query_param("version", "7"))
        .respond_with(ResponseTemplate::new(200).set_body_json(pg_entity()))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .processgroups_api()
        .remove_process_group("pg-id", Some("7"), None, None)
        .await;
    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn remove_process_group_sends_all_query_params() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/process-groups/pg-id"))
        .and(query_param("version", "3"))
        .and(query_param("clientId", "my-client"))
        .and(query_param("disconnectedNodeAcknowledged", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(pg_entity()))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .processgroups_api()
        .remove_process_group("pg-id", Some("3"), Some("my-client"), Some(true))
        .await;
    assert!(result.is_ok(), "{:?}", result);
}

// ── create_processor ─────────────────────────────────────────────────────────

#[tokio::test]
async fn create_processor_returns_entity() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/process-groups/pg-id/processors"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "proc-id",
            "component": { "id": "proc-id", "name": "GenerateFlowFile" }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ProcessorEntity::default();
    let entity = client
        .processgroups_api()
        .processors("pg-id")
        .create_processor(&body)
        .await
        .unwrap();
    assert_eq!(entity.id.as_deref(), Some("proc-id"));
}

// ── get_processors (includeDescendantGroups) ──────────────────────────────────

#[tokio::test]
async fn get_processors_sends_include_descendant_groups_when_true() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/process-groups/pg-id/processors"))
        .and(query_param("includeDescendantGroups", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(processors_entity()))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .processgroups_api()
        .processors("pg-id")
        .get_processors(Some(true))
        .await;
    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn get_processors_sends_include_descendant_groups_when_false() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/process-groups/pg-id/processors"))
        .and(query_param("includeDescendantGroups", "false"))
        .respond_with(ResponseTemplate::new(200).set_body_json(processors_entity()))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .processgroups_api()
        .processors("pg-id")
        .get_processors(Some(false))
        .await;
    assert!(result.is_ok(), "{:?}", result);
}

// ── delete_processor ──────────────────────────────────────────────────────────

#[tokio::test]
async fn delete_processor_sends_version_query_param() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/processors/proc-id"))
        .and(query_param("version", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(processor_entity()))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .processors_api()
        .delete_processor("proc-id", Some("5"), None, None)
        .await;
    assert!(result.is_ok(), "{:?}", result);
}
