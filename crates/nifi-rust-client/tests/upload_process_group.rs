#![cfg(feature = "dynamic")]

//! Wiremock coverage for the dynamic upload_process_group endpoint:
//! asserts that the generated method issues a multipart POST and
//! deserializes the JSON response into `ProcessGroupEntity`. Body-level
//! multipart part assertions are awkward in wiremock — end-to-end
//! correctness against a real NiFi is covered by the live integration
//! smoke pass.

use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::dynamic::DynamicClient;
use serde_json::json;
use wiremock::matchers::{header_regex, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn dynamic_client_on(mock: &MockServer, version: &str) -> DynamicClient {
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "about": { "title": "NiFi", "version": version }
        })))
        .mount(mock)
        .await;
    let client = NifiClientBuilder::new(&mock.uri())
        .unwrap()
        .build()
        .unwrap();
    DynamicClient::from_client(client).await.unwrap()
}

#[tokio::test]
async fn upload_process_group_posts_multipart_with_all_required_fields() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/nifi-api/process-groups/parent-1/process-groups/upload",
        ))
        .and(header_regex(
            "content-type",
            r"^multipart/form-data; boundary=",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "new-child-pg",
            "component": { "name": "imported" }
        })))
        .expect(1)
        .mount(&mock)
        .await;

    let client = dynamic_client_on(&mock, "2.9.0").await;
    let snapshot = serde_json::to_vec(&json!({ "flowContents": {} })).unwrap();
    // Signature (post-fix, alphabetic-by-wire-name order):
    // (id, client_id, disconnected_node_acknowledged, group_name,
    //  position_x, position_y, filename, data)
    let entity = client
        .processgroups()
        .upload_process_group(
            "parent-1",
            "nifictl-probe",
            None,
            "imported",
            0.0,
            0.0,
            "snapshot.json",
            snapshot,
        )
        .await
        .unwrap();
    assert_eq!(entity.id.as_deref(), Some("new-child-pg"));
}
