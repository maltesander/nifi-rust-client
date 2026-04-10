#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── create_port_transaction ───────────────────────────────────────────────────

#[tokio::test]
async fn create_port_transaction_returns_response_code() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/nifi-api/data-transfer/input-ports/port-id/transactions",
        ))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "responseCode": 3,
            "message": "handshake"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .datatransfer_api()
        .transactions("port-id")
        .create_port_transaction("input-ports")
        .await
        .unwrap();

    assert_eq!(result.response_code, Some(3));
}

// ── extend_input_port_transaction_t_t_l ───────────────────────────────────────

#[tokio::test]
async fn extend_input_port_transaction_ttl_returns_response_code() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path(
            "/nifi-api/data-transfer/input-ports/port-id/transactions/tx-id",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "responseCode": 13,
            "message": "extended"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .datatransfer_api()
        .transactions("port-id")
        .extend_input_port_transaction_t_t_l("tx-id")
        .await
        .unwrap();

    assert_eq!(result.response_code, Some(13));
    assert_eq!(result.message.as_deref(), Some("extended"));
}

// ── commit_input_port_transaction ─────────────────────────────────────────────

#[tokio::test]
async fn commit_input_port_transaction_returns_result() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(
            "/nifi-api/data-transfer/input-ports/port-id/transactions/tx-id",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "responseCode": 12,
            "flowFileSent": 5
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .datatransfer_api()
        .transactions("port-id")
        .commit_input_port_transaction("tx-id", 12)
        .await
        .unwrap();

    assert_eq!(result.response_code, Some(12));
    assert_eq!(result.flow_file_sent, Some(5));
}

// ── receive_flow_files ────────────────────────────────────────────────────────

#[tokio::test]
async fn receive_flow_files_sends_octet_stream_and_succeeds() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/nifi-api/data-transfer/input-ports/port-id/transactions/tx-id/flow-files",
        ))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .datatransfer_api()
        .transactions("port-id")
        .receive_flow_files("tx-id", Some("flowfile.bin"), b"hello".to_vec())
        .await;

    assert!(result.is_ok());
}

// ── extend_output_port_transaction_t_t_l ──────────────────────────────────────

#[tokio::test]
async fn extend_output_port_transaction_ttl_returns_response_code() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path(
            "/nifi-api/data-transfer/output-ports/port-id/transactions/tx-id",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "responseCode": 13,
            "message": "extended"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .datatransfer_api()
        .transactions("port-id")
        .extend_output_port_transaction_t_t_l("tx-id")
        .await
        .unwrap();

    assert_eq!(result.response_code, Some(13));
    assert_eq!(result.message.as_deref(), Some("extended"));
}

// ── commit_output_port_transaction ────────────────────────────────────────────

#[tokio::test]
async fn commit_output_port_transaction_returns_result() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(
            "/nifi-api/data-transfer/output-ports/port-id/transactions/tx-id",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "responseCode": 12,
            "flowFileSent": 3
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .datatransfer_api()
        .transactions("port-id")
        .commit_output_port_transaction("tx-id", 12, "abc123checksum")
        .await
        .unwrap();

    assert_eq!(result.response_code, Some(12));
    assert_eq!(result.flow_file_sent, Some(3));
}

// ── transfer_flow_files ───────────────────────────────────────────────────────

#[tokio::test]
async fn transfer_flow_files_succeeds_on_200() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/nifi-api/data-transfer/output-ports/port-id/transactions/tx-id/flow-files",
        ))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = client
        .datatransfer_api()
        .transactions("port-id")
        .transfer_flow_files("tx-id")
        .await;

    assert!(result.is_ok());
}
