mod helpers;

use nifi_rust_client::types::{ReportingTaskDto, ReportingTaskEntity};

/// Discover the first available reporting task type from the NiFi instance.
async fn first_reporting_task_type(client: &nifi_rust_client::NifiClient) -> String {
    let types = client
        .flow_api()
        .get_reporting_task_types(None, None, None)
        .await
        .expect("failed to list reporting task types");
    types
        .reporting_task_types
        .as_deref()
        .and_then(|v| v.first())
        .and_then(|t| t.r#type.clone())
        .expect("no reporting task types available on this NiFi instance")
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn reporting_task_crud_lifecycle() {
    let client = helpers::logged_in_client().await;
    let task_type = first_reporting_task_type(&client).await;

    // Create via controller
    let body = ReportingTaskEntity {
        component: Some(ReportingTaskDto {
            name: Some("test-reporting-task".to_string()),
            r#type: Some(task_type),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let created = client
        .controller_api()
        .create_reporting_task(&body)
        .await
        .expect("failed to create reporting task");
    let task_id = created
        .id
        .clone()
        .expect("created reporting task has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created reporting task has no revision version");

    // Get — verify name
    let fetched = client
        .reportingtasks_api()
        .get_reporting_task(&task_id)
        .await
        .expect("failed to get reporting task");
    assert_eq!(
        fetched.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("test-reporting-task")
    );

    // Update — rename
    let update_body = ReportingTaskEntity {
        id: Some(task_id.clone()),
        component: Some(ReportingTaskDto {
            id: Some(task_id.clone()),
            name: Some("test-reporting-task-renamed".to_string()),
            ..Default::default()
        }),
        revision: Some(helpers::revision(version)),
        ..Default::default()
    };
    let updated = client
        .reportingtasks_api()
        .update_reporting_task(&task_id, &update_body)
        .await
        .expect("failed to update reporting task");
    assert_eq!(
        updated.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("test-reporting-task-renamed")
    );
    let version_after_update = updated
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("updated reporting task has no revision version");

    // Delete
    client
        .reportingtasks_api()
        .remove_reporting_task(
            &task_id,
            Some(&version_after_update.to_string()),
            None,
            None,
        )
        .await
        .expect("failed to delete reporting task");

    // Verify gone
    assert!(
        client
            .reportingtasks_api()
            .get_reporting_task(&task_id)
            .await
            .is_err(),
        "expected error fetching deleted reporting task"
    );
}
