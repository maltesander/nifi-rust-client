#![cfg(feature = "dynamic")]

use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::dynamic::traits::{
    ControllerServicesApi, ControllerServicesBulletinsApi, FlowApi, ResourcesApi,
};
use nifi_rust_client::dynamic::types::{FlowMetricsReportingStrategy, IncludedRegistries};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Helper: mount the about mock and create a DynamicClient
#[allow(clippy::unwrap_used)]
async fn dynamic_client_on(
    mock: &MockServer,
    version: &str,
) -> nifi_rust_client::dynamic::DynamicClient {
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": { "title": "NiFi", "version": version }
        })))
        .mount(mock)
        .await;

    let client = NifiClientBuilder::new(&mock.uri())
        .unwrap()
        .build()
        .unwrap();
    nifi_rust_client::dynamic::DynamicClient::from_client(client)
        .await
        .unwrap()
}

#[tokio::test]
async fn dynamic_about_returns_fields() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": {
                "title": "NiFi",
                "version": "2.8.0",
                "uri": "http://localhost/nifi-api/",
                "contentViewerUrl": "/nifi-content-viewer/",
                "timezone": "UTC",
                "buildTag": "nifi-2.8.0",
                "buildTimestamp": "01/01/2026 00:00:00 UTC"
            }
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

    let about = dynamic.flow_api().get_about_info().await.unwrap();

    assert_eq!(about.version.as_deref(), Some("2.8.0"));
    assert_eq!(about.title.as_deref(), Some("NiFi"));
    assert_eq!(about.timezone.as_deref(), Some("UTC"));
}

#[tokio::test]
async fn dynamic_current_user_returns_identity() {
    let mock = MockServer::start().await;
    let dynamic = dynamic_client_on(&mock, "2.8.0").await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/current-user"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "identity": "admin",
            "anonymous": false,
            "canVersionFlows": false
        })))
        .mount(&mock)
        .await;

    let user = dynamic.flow_api().get_current_user().await.unwrap();

    assert_eq!(user.identity.as_deref(), Some("admin"));
    assert!(!user.anonymous.unwrap_or(true));
}

#[tokio::test]
async fn dynamic_get_resources() {
    let mock = MockServer::start().await;
    let dynamic = dynamic_client_on(&mock, "2.8.0").await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/resources"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "resources": [
                { "identifier": "/flow", "name": "Flow" },
                { "identifier": "/system", "name": "System" }
            ]
        })))
        .mount(&mock)
        .await;

    let result = dynamic.resources_api().get_resources().await;

    assert!(result.is_ok());
    let entity = result.unwrap();
    assert!(entity.resources.as_deref().unwrap_or_default().len() >= 2);
}

#[tokio::test]
async fn dynamic_patch_version_detection() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "about": { "title": "NiFi", "version": "2.8.1" }
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

    // patch version 2.8.1 should map to v2.8.0 (major.minor match)
    assert_eq!(
        dynamic.detected_version(),
        Some(nifi_rust_client::dynamic::DetectedVersion::V2_8_0)
    );
    assert_eq!(dynamic.detected_version().unwrap().to_string(), "2.8.0");
}

// ── Query param dispatch tests ───────────────────────────────────────────────
// get_flow_metrics has 5 query params in v2.6.0 but 6 in v2.7.2/v2.8.0
// (extra flow_metrics_reporting_strategy). These tests verify the dynamic
// dispatch passes the correct params to each version.

#[tokio::test]
async fn dynamic_flow_metrics_v2_6_0_skips_extra_param() {
    let mock = MockServer::start().await;
    let dynamic = dynamic_client_on(&mock, "2.6.0").await;

    // v2.6.0 should pass includedRegistries but NOT flowMetricsReportingStrategy
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/metrics/prometheus"))
        .and(query_param("includedRegistries", "JVM"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock)
        .await;

    dynamic
        .flow_api()
        .get_flow_metrics(
            "prometheus",
            Some(IncludedRegistries::Jvm), // includedRegistries
            None,                          // sampleName
            None,                          // sampleLabelValue
            None,                          // rootFieldName
            Some(FlowMetricsReportingStrategy::AllProcessGroups), // should be IGNORED for 2.6.0
        )
        .await
        .unwrap();

    // wiremock verifies the mock was called exactly once with the expected params
}

#[tokio::test]
async fn dynamic_flow_metrics_v2_8_0_passes_all_params() {
    let mock = MockServer::start().await;
    let dynamic = dynamic_client_on(&mock, "2.8.0").await;

    // v2.8.0 should pass both includedRegistries AND flowMetricsReportingStrategy
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/metrics/prometheus"))
        .and(query_param("includedRegistries", "JVM"))
        .and(query_param(
            "flowMetricsReportingStrategy",
            "ALL_PROCESS_GROUPS",
        ))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock)
        .await;

    dynamic
        .flow_api()
        .get_flow_metrics(
            "prometheus",
            Some(IncludedRegistries::Jvm),
            None,
            None,
            None,
            Some(FlowMetricsReportingStrategy::AllProcessGroups),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn dynamic_flow_metrics_v2_7_2_passes_all_params() {
    let mock = MockServer::start().await;
    let dynamic = dynamic_client_on(&mock, "2.7.2").await;

    // v2.7.2 also has flowMetricsReportingStrategy
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/metrics/prometheus"))
        .and(query_param("includedRegistries", "NIFI"))
        .and(query_param("sampleName", "my_sample"))
        .and(query_param(
            "flowMetricsReportingStrategy",
            "ALL_COMPONENTS",
        ))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock)
        .await;

    dynamic
        .flow_api()
        .get_flow_metrics(
            "prometheus",
            Some(IncludedRegistries::Nifi),
            Some("my_sample"),
            None,
            None,
            Some(FlowMetricsReportingStrategy::AllComponents),
        )
        .await
        .unwrap();
}

// ── Narrowing conversion error tests ────────────────────────────────────────

#[tokio::test]
async fn dynamic_unsupported_enum_variant() {
    // IncludedRegistries::VersionInfo only exists in v2.8.0.
    // Using it against v2.6.0 should return UnsupportedEnumVariant.
    let mock = MockServer::start().await;
    let dynamic = dynamic_client_on(&mock, "2.6.0").await;

    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/metrics/prometheus"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock)
        .await;

    let err = dynamic
        .flow_api()
        .get_flow_metrics(
            "prometheus",
            Some(IncludedRegistries::VersionInfo), // only in 2.8.0
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap_err();

    assert!(
        matches!(
            err,
            nifi_rust_client::NifiError::UnsupportedEnumVariant { .. }
        ),
        "expected UnsupportedEnumVariant, got: {err:?}"
    );
}

#[tokio::test]
async fn dynamic_universal_fields_not_optional() {
    // ConnectableDto has `id`, `group_id`, and `type` as required in all versions.
    // After the universal field optimization, these should be non-optional Strings.
    let dto = nifi_rust_client::dynamic::types::ConnectableDto::default();
    // Universal fields default to empty string (String::default), not None
    assert_eq!(dto.id, "");
    assert_eq!(dto.group_id, "");
    assert_eq!(dto.r#type, "");
    // Non-universal fields remain Option and default to None
    assert!(dto.name.is_none());
    assert!(dto.comments.is_none());
}

#[tokio::test]
async fn dynamic_missing_required_field_error() {
    // clear_bulletins on controller_services requires a body with from_timestamp
    // (required in v2.7.2+). Sending from_timestamp = None should fail at the
    // narrowing TryFrom conversion before the HTTP call.
    let mock = MockServer::start().await;
    let dynamic = dynamic_client_on(&mock, "2.8.0").await;

    // ClearBulletinsRequestEntity is #[non_exhaustive]; use Default::default() to construct.
    // from_timestamp is None by default, which is required in v2.8.0 and should trigger an error.
    let body = nifi_rust_client::dynamic::types::ClearBulletinsRequestEntity::default();

    let err = dynamic
        .controller_services_api()
        .bulletins("some-id")
        .clear_bulletins(&body)
        .await
        .unwrap_err();

    assert!(
        matches!(
            err,
            nifi_rust_client::NifiError::MissingRequiredField { .. }
        ),
        "expected MissingRequiredField, got: {err:?}"
    );
}
