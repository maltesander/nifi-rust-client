#![cfg(not(feature = "dynamic"))]
#![allow(clippy::panic)]
use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::NifiError;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn returns_unauthorized_on_401() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_json(serde_json::json!({"message": "Token expired"})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.flow().get_about_info().await.unwrap_err();
    assert!(
        matches!(err, NifiError::Unauthorized { .. }),
        "expected Unauthorized, got: {err:?}"
    );
}

#[tokio::test]
async fn returns_forbidden_on_403() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(
            ResponseTemplate::new(403)
                .set_body_json(serde_json::json!({"message": "Access denied"})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.flow().get_about_info().await.unwrap_err();
    assert!(
        matches!(err, NifiError::Forbidden { .. }),
        "expected Forbidden, got: {err:?}"
    );
}

#[tokio::test]
async fn returns_not_found_on_404() {
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
    assert!(
        matches!(err, NifiError::NotFound { .. }),
        "expected NotFound, got: {err:?}"
    );
}

#[tokio::test]
async fn returns_conflict_on_409() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(
            ResponseTemplate::new(409)
                .set_body_json(serde_json::json!({"message": "Component is running"})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.flow().get_about_info().await.unwrap_err();
    assert!(
        matches!(err, NifiError::Conflict { .. }),
        "expected Conflict, got: {err:?}"
    );
}

#[tokio::test]
async fn returns_api_for_other_status_codes() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(
            ResponseTemplate::new(503)
                .set_body_json(serde_json::json!({"message": "Service Unavailable"})),
        )
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let err = client.flow().get_about_info().await.unwrap_err();
    assert!(
        matches!(err, NifiError::Api { status: 503, .. }),
        "expected Api with 503, got: {err:?}"
    );
}

#[tokio::test]
async fn status_code_returns_correct_values() {
    let unauthorized = NifiError::Unauthorized {
        message: String::new(),
    };
    assert_eq!(unauthorized.status_code(), Some(401));

    let forbidden = NifiError::Forbidden {
        message: String::new(),
    };
    assert_eq!(forbidden.status_code(), Some(403));

    let not_found = NifiError::NotFound {
        message: String::new(),
    };
    assert_eq!(not_found.status_code(), Some(404));

    let conflict = NifiError::Conflict {
        message: String::new(),
    };
    assert_eq!(conflict.status_code(), Some(409));

    let api = NifiError::Api {
        status: 500,
        message: String::new(),
    };
    assert_eq!(api.status_code(), Some(500));

    let http_err = NifiError::InvalidBaseUrl {
        source: url::Url::parse("://bad").unwrap_err(),
    };
    assert_eq!(http_err.status_code(), None);
}

#[tokio::test]
async fn is_retryable_returns_true_for_transient_errors() {
    let api_503 = NifiError::Api {
        status: 503,
        message: String::new(),
    };
    assert!(api_503.is_retryable());

    let api_429 = NifiError::Api {
        status: 429,
        message: String::new(),
    };
    assert!(api_429.is_retryable());
}

#[tokio::test]
async fn is_retryable_returns_false_for_permanent_errors() {
    let not_found = NifiError::NotFound {
        message: String::new(),
    };
    assert!(!not_found.is_retryable());

    let unauthorized = NifiError::Unauthorized {
        message: String::new(),
    };
    assert!(!unauthorized.is_retryable());
}
