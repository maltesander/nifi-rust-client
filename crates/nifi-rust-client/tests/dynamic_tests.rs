#![cfg(feature = "dynamic")]

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn auto_detects_v2_7_2() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": { "title": "NiFi", "version": "2.7.2" }
        })))
        .mount(&mock)
        .await;

    let client = NifiClientBuilder::new(&mock.uri())
        .unwrap()
        .build()
        .unwrap();
    let dynamic = nifi_rust_client::dynamic::DynamicClient::from_client(client)
        .await
        .unwrap();
    assert_eq!(dynamic.detected_version().to_string(), "2.7.2");
}

#[tokio::test]
async fn auto_detects_v2_8_0() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": { "title": "NiFi", "version": "2.8.0" }
        })))
        .mount(&mock)
        .await;

    let client = NifiClientBuilder::new(&mock.uri())
        .unwrap()
        .build()
        .unwrap();
    let dynamic = nifi_rust_client::dynamic::DynamicClient::from_client(client)
        .await
        .unwrap();
    assert_eq!(dynamic.detected_version().to_string(), "2.8.0");
}

#[tokio::test]
async fn unknown_version_returns_error() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": { "title": "NiFi", "version": "1.15.0" }
        })))
        .mount(&mock)
        .await;

    let client = NifiClientBuilder::new(&mock.uri())
        .unwrap()
        .build()
        .unwrap();
    let result = nifi_rust_client::dynamic::DynamicClient::from_client(client).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("1.15.0"));
}
