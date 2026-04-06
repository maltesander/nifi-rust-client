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
