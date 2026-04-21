#![cfg(not(feature = "dynamic"))]

use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{header_regex, method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

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

/// Pins the full `post_multipart_with_fields` wire shape: every non-`None`
/// schema property must appear as its own `form-data` part with the
/// camelCase wire name, the file part must carry the supplied filename,
/// and a `None` optional field (`disconnected_node_acknowledged`) must
/// not be sent at all.
#[tokio::test]
async fn post_multipart_with_fields_sends_each_extra_field_as_own_part() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/nifi-api/process-groups/root/process-groups/upload"))
        .and(header_regex(
            "content-type",
            r"^multipart/form-data; boundary=",
        ))
        .respond_with(|req: &Request| {
            let body = std::str::from_utf8(&req.body).expect("body is utf-8 text");

            for (wire_name, expected_value) in [
                ("clientId", "nifictl-probe"),
                ("groupName", "imported"),
                ("positionX", "1.5"),
                ("positionY", "2.5"),
            ] {
                let part_header = format!("name=\"{wire_name}\"");
                assert!(
                    body.contains(&part_header),
                    "missing multipart part for {wire_name}: body={body}"
                );
                assert!(
                    body.contains(expected_value),
                    "missing value {expected_value} for {wire_name}: body={body}"
                );
            }

            // Optional field set to None must NOT appear on the wire.
            assert!(
                !body.contains("disconnectedNodeAcknowledged"),
                "None-valued optional field leaked onto wire: body={body}"
            );

            // File part: filename preserved.
            assert!(
                body.contains("name=\"file\"") && body.contains("filename=\"flow.json\""),
                "file part missing or misnamed: body={body}"
            );

            ResponseTemplate::new(200).set_body_json(serde_json::json!({}))
        })
        .expect(1)
        .mount(&server)
        .await;

    let client = NifiClientBuilder::new(&server.uri())
        .unwrap()
        .build()
        .unwrap();

    client
        .processgroups()
        .upload_process_group(
            "root",
            "nifictl-probe",
            None,
            "imported",
            1.5,
            2.5,
            "flow.json",
            b"{}".to_vec(),
        )
        .await
        .unwrap();
}
