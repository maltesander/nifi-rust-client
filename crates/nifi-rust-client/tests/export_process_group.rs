#![cfg(feature = "dynamic")]

use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::dynamic::DynamicClient;
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn dynamic_client_on(mock: &MockServer, version: &str) -> DynamicClient {
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "about": { "title": "NiFi", "version": version }
        })))
        .mount(mock)
        .await;
    let client = NifiClientBuilder::new(&mock.uri()).unwrap().build().unwrap();
    DynamicClient::from_client(client).await.unwrap()
}

#[tokio::test]
async fn export_process_group_returns_raw_json_value() {
    let mock = MockServer::start().await;
    let snapshot = json!({
        "flowContents": { "identifier": "root", "name": "NiFi Flow" },
        "flowEncodingVersion": "1.0",
        "parameterContexts": {}
    });
    Mock::given(method("GET"))
        .and(path("/nifi-api/process-groups/pg-1/download"))
        .respond_with(ResponseTemplate::new(200).set_body_json(snapshot.clone()))
        .expect(1)
        .mount(&mock)
        .await;

    let client = dynamic_client_on(&mock, "2.9.0").await;
    let result = client
        .processgroups()
        .export_process_group("pg-1", None)
        .await
        .unwrap();
    assert_eq!(result, snapshot);
}

#[tokio::test]
async fn export_process_group_forwards_include_referenced_services() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/process-groups/pg-2/download"))
        .and(query_param("includeReferencedServices", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "flowContents": {} })))
        .expect(1)
        .mount(&mock)
        .await;

    let client = dynamic_client_on(&mock, "2.9.0").await;
    let _ = client
        .processgroups()
        .export_process_group("pg-2", Some(true))
        .await
        .unwrap();
}
