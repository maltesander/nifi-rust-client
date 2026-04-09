#![cfg(feature = "dynamic")]

use nifi_rust_client::dynamic::VersionResolutionStrategy;
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Helper: mount a mock that returns the given version from /flow/about.
async fn mock_about(mock: &MockServer, version: &str) {
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": { "title": "NiFi", "version": version }
        })))
        .mount(mock)
        .await;
}

/// Helper: build a DynamicClient with the given strategy against a mock server.
fn build_dynamic(
    uri: &str,
    strategy: VersionResolutionStrategy,
) -> nifi_rust_client::dynamic::DynamicClient {
    NifiClientBuilder::new(uri)
        .unwrap()
        .version_strategy(strategy)
        .build_dynamic()
        .unwrap()
}

#[tokio::test]
async fn strict_exact_match_succeeds() {
    let mock = MockServer::start().await;
    mock_about(&mock, "2.8.0").await;
    let client = build_dynamic(&mock.uri(), VersionResolutionStrategy::Strict);
    let version = client.detect_version().await.unwrap();
    assert_eq!(version.to_string(), "2.8.0");
}

#[tokio::test]
async fn strict_unknown_version_errors() {
    let mock = MockServer::start().await;
    mock_about(&mock, "2.5.0").await;
    let client = build_dynamic(&mock.uri(), VersionResolutionStrategy::Strict);
    let result = client.detect_version().await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("2.5.0"));
}

#[tokio::test]
async fn closest_picks_nearest() {
    let mock = MockServer::start().await;
    mock_about(&mock, "2.5.0").await;
    let client = build_dynamic(&mock.uri(), VersionResolutionStrategy::Closest);
    let version = client.detect_version().await.unwrap();
    assert_eq!(version.to_string(), "2.6.0");
}

#[tokio::test]
async fn closest_cross_major_errors() {
    let mock = MockServer::start().await;
    mock_about(&mock, "1.25.0").await;
    let client = build_dynamic(&mock.uri(), VersionResolutionStrategy::Closest);
    let result = client.detect_version().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn latest_picks_highest() {
    let mock = MockServer::start().await;
    mock_about(&mock, "2.5.0").await;
    let client = build_dynamic(&mock.uri(), VersionResolutionStrategy::Latest);
    let version = client.detect_version().await.unwrap();
    assert_eq!(version.to_string(), "2.8.0");
}

#[tokio::test]
async fn latest_cross_major_errors() {
    let mock = MockServer::start().await;
    mock_about(&mock, "1.25.0").await;
    let client = build_dynamic(&mock.uri(), VersionResolutionStrategy::Latest);
    let result = client.detect_version().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn default_strategy_is_strict() {
    // build_dynamic without version_strategy should default to Strict
    let mock = MockServer::start().await;
    mock_about(&mock, "2.5.0").await;
    let client = NifiClientBuilder::new(&mock.uri())
        .unwrap()
        .build_dynamic()
        .unwrap();
    let result = client.detect_version().await;
    assert!(result.is_err());
}
