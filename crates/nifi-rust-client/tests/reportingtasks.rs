#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_reporting_task_returns_name_and_type() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/reporting-tasks/task-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "task-id",
            "component": {
                "id": "task-id",
                "name": "my-task",
                "type": "org.apache.nifi.reporting.SiteToSiteStatusReportingTask"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let task = client
        .reportingtasks_api()
        .get_reporting_task("task-id")
        .await
        .unwrap();

    assert_eq!(task.id.as_deref(), Some("task-id"));
    assert_eq!(
        task.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("my-task")
    );
}

// ── clear_bulletins ───────────────────────────────────────────────────────────
// Added in NiFi 2.7.2 — gate so nifi-2-6-0 builds stay green.

#[cfg(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0"))]
#[tokio::test]
async fn clear_reporting_task_bulletins_returns_cleared_count() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/nifi-api/reporting-tasks/some-id/bulletins/clear-requests",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bulletinsCleared": 4,
            "componentId": "some-id"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let body = nifi_rust_client::types::ClearBulletinsRequestEntity::default();
    let result = client
        .reportingtasks_api()
        .bulletins("some-id")
        .clear_bulletins(&body)
        .await;

    assert!(
        result.is_ok(),
        "clear_reporting_task_bulletins failed: {:?}",
        result.err()
    );
}
