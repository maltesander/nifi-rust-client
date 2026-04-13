#![cfg(not(feature = "dynamic"))]
#![allow(clippy::panic)]
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::NifiError;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── Unit tests for extract_error_message ──────────────────────────────────────

#[cfg(test)]
mod extract_error_message_tests {
    use nifi_rust_client::client::extract_error_message;

    #[test]
    fn extracts_message_field_from_json() {
        let body = r#"{"message": "Processor not found"}"#;
        assert_eq!(extract_error_message(body), "Processor not found");
    }

    #[test]
    fn falls_back_to_raw_text_for_plain_text() {
        let body = "Unauthorized";
        assert_eq!(extract_error_message(body), "Unauthorized");
    }

    #[test]
    fn falls_back_to_raw_text_when_no_message_field() {
        let body = r#"{"error": "something", "code": 403}"#;
        assert_eq!(extract_error_message(body), body);
    }

    #[test]
    fn falls_back_to_raw_text_for_invalid_json() {
        let body = "not json at all";
        assert_eq!(extract_error_message(body), "not json at all");
    }

    #[test]
    fn handles_empty_body() {
        assert_eq!(extract_error_message(""), "");
    }
}

// ── Wiremock tests for structured error responses ─────────────────────────────

fn assert_api_error(err: &NifiError, expected_status: u16, expected_message: &str) {
    match (expected_status, err) {
        (401, NifiError::Unauthorized { message }) => {
            assert_eq!(message, expected_message, "message mismatch");
        }
        (403, NifiError::Forbidden { message }) => {
            assert_eq!(message, expected_message, "message mismatch");
        }
        (404, NifiError::NotFound { message }) => {
            assert_eq!(message, expected_message, "message mismatch");
        }
        (409, NifiError::Conflict { message }) => {
            assert_eq!(message, expected_message, "message mismatch");
        }
        (_, NifiError::Api { status, message }) => {
            assert_eq!(*status, expected_status, "status mismatch");
            assert_eq!(message, expected_message, "message mismatch");
        }
        (_, other) => panic!("expected API error with status {expected_status}, got: {other:?}"),
    }
}

#[tokio::test]
async fn returns_json_message_on_401() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_json(serde_json::json!({"message": "Authentication required"})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.flow().get_about_info().await.unwrap_err();
    assert_api_error(&err, 401, "Authentication required");
}

#[tokio::test]
async fn returns_json_message_on_403() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(
            ResponseTemplate::new(403).set_body_json(serde_json::json!({"message": "Forbidden"})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.flow().get_about_info().await.unwrap_err();
    assert_api_error(&err, 403, "Forbidden");
}

#[tokio::test]
async fn returns_json_message_on_404() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(
            ResponseTemplate::new(404)
                .set_body_json(serde_json::json!({"message": "Resource not found"})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.flow().get_about_info().await.unwrap_err();
    assert_api_error(&err, 404, "Resource not found");
}

#[tokio::test]
async fn returns_json_message_on_409() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(
            ResponseTemplate::new(409)
                .set_body_json(serde_json::json!({"message": "Conflict: component is running"})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.flow().get_about_info().await.unwrap_err();
    assert_api_error(&err, 409, "Conflict: component is running");
}

#[tokio::test]
async fn falls_back_to_plain_text_body_on_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(503).set_body_string("Service Unavailable"))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.flow().get_about_info().await.unwrap_err();
    assert_api_error(&err, 503, "Service Unavailable");
}

#[tokio::test]
async fn caller_can_match_on_status_code() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(serde_json::json!({"message": "Not found"})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.flow().get_about_info().await.unwrap_err();

    assert!(matches!(err, NifiError::NotFound { .. }));
}

#[tokio::test]
async fn returns_json_message_on_delete_403() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/nifi-api/access/logout"))
        .respond_with(
            ResponseTemplate::new(403)
                .set_body_json(serde_json::json!({"message": "Not logged in"})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.access().log_out().await.unwrap_err();
    assert_api_error(&err, 403, "Not logged in");
}
