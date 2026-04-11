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

    let _entity = client
        .processgroups_api()
        .process_groups("root")
        .upload_process_group("flow.json", b"{}".to_vec())
        .await
        .unwrap();
}
