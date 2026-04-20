#![cfg(not(feature = "dynamic"))]

use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

fn about_json() -> serde_json::Value {
    serde_json::json!({
        "about": {
            "title": "NiFi",
            "version": "2.8.0",
            "uri": "https://localhost:8443/nifi-api",
            "contentViewerUrl": "../nifi-content-viewer/",
            "timezone": "UTC",
            "buildTag": "nifi-2.8.0",
            "buildTimestamp": "2024-01-01T00:00:00Z"
        }
    })
}

fn uuid_v4_regex() -> regex::Regex {
    regex::Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$")
        .unwrap()
}

#[tokio::test]
async fn header_present_when_configured() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(about_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .request_id_header(Some("X-Request-Id"))
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;
    client.flow().get_about_info().await.unwrap();

    let received = mock_server.received_requests().await.unwrap();
    let req: &Request = received.first().expect("one request");
    let id = req
        .headers
        .get("x-request-id")
        .expect("X-Request-Id header present");
    let id_str = id.to_str().expect("header is ascii");
    assert!(
        uuid_v4_regex().is_match(id_str),
        "expected UUIDv4, got: {id_str}"
    );
}

#[tokio::test]
async fn header_absent_when_not_configured() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(about_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;
    client.flow().get_about_info().await.unwrap();

    let received = mock_server.received_requests().await.unwrap();
    let req: &Request = received.first().expect("one request");
    assert!(
        req.headers.get("x-request-id").is_none(),
        "no X-Request-Id expected by default"
    );
    assert!(
        req.headers.get("x-correlation-id").is_none(),
        "no X-Correlation-Id expected by default"
    );
}

#[tokio::test]
async fn fresh_id_per_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(about_json()))
        .expect(2)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .request_id_header(Some("X-Request-Id"))
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;
    client.flow().get_about_info().await.unwrap();
    client.flow().get_about_info().await.unwrap();

    let received = mock_server.received_requests().await.unwrap();
    assert_eq!(received.len(), 2);
    let id0 = received[0].headers.get("x-request-id").unwrap();
    let id1 = received[1].headers.get("x-request-id").unwrap();
    assert_ne!(id0, id1, "each request must get a fresh UUIDv4");
}

#[tokio::test]
async fn custom_header_name() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(about_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .request_id_header(Some("X-Correlation-Id"))
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;
    client.flow().get_about_info().await.unwrap();

    let received = mock_server.received_requests().await.unwrap();
    assert!(
        received[0].headers.get("x-correlation-id").is_some(),
        "custom header name should appear on the wire"
    );
    assert!(
        received[0].headers.get("x-request-id").is_none(),
        "default name should NOT appear when a custom one is set"
    );
}

#[tokio::test]
async fn login_sends_request_id_header_when_configured() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/nifi-api/access/token"))
        .respond_with(ResponseTemplate::new(201).set_body_string("jwt-token"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .request_id_header(Some("X-Request-Id"))
        .build()
        .unwrap();
    client.login("admin", "pw").await.unwrap();

    let received = mock_server.received_requests().await.unwrap();
    let req: &Request = received.first().expect("one request");
    assert!(
        req.headers.get("x-request-id").is_some(),
        "login must attach X-Request-Id when configured"
    );
}

#[tokio::test]
async fn logout_sends_request_id_header_when_configured() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/nifi-api/access/logout"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .request_id_header(Some("X-Request-Id"))
        .build()
        .unwrap();
    client.set_token("jwt".to_string()).await;
    client.logout().await.unwrap();

    let received = mock_server.received_requests().await.unwrap();
    let req: &Request = received.first().expect("one request");
    assert!(
        req.headers.get("x-request-id").is_some(),
        "logout must attach X-Request-Id when configured"
    );
}
