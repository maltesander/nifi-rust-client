#![cfg(not(feature = "dynamic"))]

use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{header_regex, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn post_multipart_sends_file_part() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/process-groups/root/process-groups/upload"))
        .and(header_regex(
            "content-type",
            r"^multipart/form-data; boundary=",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .expect(1)
        .mount(&server)
        .await;

    let client = NifiClientBuilder::new(&server.uri())
        .unwrap()
        .build()
        .unwrap();

    // Signature: (id, client_id, disconnected_node_acknowledged, group_name,
    // position_x, position_y, filename, data) — alphabetic-by-wire-name order
    // for the non-file schema properties.
    let _entity = client
        .processgroups()
        .upload_process_group(
            "root",
            "nifictl-probe",
            None,
            "imported",
            0.0,
            0.0,
            "flow.json",
            b"{}".to_vec(),
        )
        .await
        .unwrap();
}
