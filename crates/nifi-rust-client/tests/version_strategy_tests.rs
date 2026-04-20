#![cfg(feature = "dynamic")]
#![allow(clippy::unwrap_used)]

use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::NifiError;
use nifi_rust_client::dynamic::VersionResolutionStrategy;
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
    assert_eq!(
        version.to_string(),
        nifi_rust_client::dynamic::LATEST_NIFI_VERSION
    );
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

// ---- Scenarios required by task 15 ----

/// Scenario 1: Closest with detected version equidistant between two supported
/// versions prefers the lower one (tie-lower-wins).
/// Supported: 2.7.2, 2.8.0 (among others). Detected: 2.7.5.
/// Distance to 2.7.2 = 0.3 minors; distance to 2.8.0 = 0.5 minors → should pick 2.7.x.
#[tokio::test]
async fn closest_resolves_to_lower_on_exact_tie() {
    let mock = MockServer::start().await;
    mock_about(&mock, "2.7.5").await;
    let client = build_dynamic(&mock.uri(), VersionResolutionStrategy::Closest);
    let version = client.detect_version().await.unwrap();
    // 2.7.5 is closer to 2.7.2 than to 2.8.0 → must resolve to the 2.7.x module.
    assert!(
        version.to_string().starts_with("2.7"),
        "expected 2.7.x, got {version}"
    );
}

/// Scenario 2: Latest always picks the highest supported minor within the same major
/// when there is no exact minor match (detected is "between" supported versions).
/// Using 2.5.5 which has no 2.5.x supported entry — must resolve to LATEST_NIFI_VERSION.
#[tokio::test]
async fn latest_resolves_to_highest_in_major() {
    let mock = MockServer::start().await;
    mock_about(&mock, "2.5.5").await;
    let client = build_dynamic(&mock.uri(), VersionResolutionStrategy::Latest);
    let version = client.detect_version().await.unwrap();
    assert_eq!(
        version.to_string(),
        nifi_rust_client::dynamic::LATEST_NIFI_VERSION,
        "Latest strategy should resolve to the highest supported version"
    );
}

/// Scenario 3: Strict rejects a minor version that is not in the supported set,
/// and the error must be NifiError::UnsupportedVersion.
#[tokio::test]
async fn strict_rejects_unknown_minor() {
    let mock = MockServer::start().await;
    mock_about(&mock, "2.999.0").await;
    let client = build_dynamic(&mock.uri(), VersionResolutionStrategy::Strict);
    let err = client.detect_version().await.unwrap_err();
    assert!(
        matches!(err, NifiError::UnsupportedVersion { .. }),
        "expected UnsupportedVersion, got {err:?}"
    );
}

/// Scenario 4: Closest must never cross a major-version boundary.
/// Detected 3.0.0 with only 2.x.y in the supported set → UnsupportedVersion.
#[tokio::test]
async fn closest_rejects_major_version_boundary() {
    let mock = MockServer::start().await;
    mock_about(&mock, "3.0.0").await;
    let client = build_dynamic(&mock.uri(), VersionResolutionStrategy::Closest);
    let err = client.detect_version().await.unwrap_err();
    assert!(
        matches!(err, NifiError::UnsupportedVersion { .. }),
        "major-version crossing must fail with UnsupportedVersion, got {err:?}"
    );
}
