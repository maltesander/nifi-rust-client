mod helpers;

use nifi_rust_client::types::{LabelDto, LabelEntity, PositionDto};

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn label_crud_lifecycle() {
    let client = helpers::logged_in_client().await;
    let (pg_id, pg_version) = helpers::create_temp_process_group(&client, "test-label-pg").await;

    // Create
    let body = LabelEntity {
        component: Some(LabelDto {
            label: Some("test label".to_string()),
            position: Some(PositionDto {
                x: Some(0.0),
                y: Some(0.0),
            }),
            width: Some(150.0),
            height: Some(50.0),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let created = client
        .processgroups_api()
        .labels(&pg_id)
        .create_label(&body)
        .await
        .expect("failed to create label");
    let label_id = created.id.clone().expect("created label has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created label has no revision version");

    // Get — verify text
    let fetched = client
        .labels_api()
        .get_label(&label_id)
        .await
        .expect("failed to get label");
    assert_eq!(
        fetched.component.as_ref().and_then(|c| c.label.as_deref()),
        Some("test label")
    );

    // Update — change text
    let update_body = LabelEntity {
        id: Some(label_id.clone()),
        component: Some(LabelDto {
            id: Some(label_id.clone()),
            label: Some("updated label".to_string()),
            ..Default::default()
        }),
        revision: Some(helpers::revision(version)),
        ..Default::default()
    };
    let updated = client
        .labels_api()
        .update_label(&label_id, &update_body)
        .await
        .expect("failed to update label");
    assert_eq!(
        updated.component.as_ref().and_then(|c| c.label.as_deref()),
        Some("updated label")
    );
    let version_after_update = updated
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("updated label has no revision version");

    // Delete
    client
        .labels_api()
        .remove_label(
            &label_id,
            Some(&version_after_update.to_string()),
            None,
            None,
        )
        .await
        .expect("failed to delete label");

    // Verify gone
    assert!(
        client.labels_api().get_label(&label_id).await.is_err(),
        "expected error fetching deleted label"
    );

    helpers::delete_temp_process_group(&client, &pg_id, pg_version).await;
}
