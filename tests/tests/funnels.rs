#![cfg(not(feature = "dynamic"))]
mod helpers;

use nifi_rust_client::types::{FunnelDto, FunnelEntity, PositionDto};

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn funnel_crud_lifecycle() {
    let client = helpers::logged_in_client().await;
    let (pg_id, pg_version) = helpers::create_temp_process_group(&client, "test-funnel-pg").await;

    // Create
    let body = FunnelEntity {
        component: Some(FunnelDto {
            position: Some(PositionDto {
                x: Some(0.0),
                y: Some(0.0),
            }),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let created = client
        .processgroups_api()
        .funnels(&pg_id)
        .create_funnel(&body)
        .await
        .expect("failed to create funnel");
    let funnel_id = created.id.clone().expect("created funnel has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created funnel has no revision version");

    // Get — verify it exists
    let fetched = client
        .funnels_api()
        .get_funnel(&funnel_id)
        .await
        .expect("failed to get funnel");
    assert_eq!(fetched.id.as_deref(), Some(funnel_id.as_str()));

    // Update — move position
    let update_body = FunnelEntity {
        id: Some(funnel_id.clone()),
        component: Some(FunnelDto {
            id: Some(funnel_id.clone()),
            position: Some(PositionDto {
                x: Some(100.0),
                y: Some(100.0),
            }),
            ..Default::default()
        }),
        revision: Some(helpers::revision(version)),
        ..Default::default()
    };
    let updated = client
        .funnels_api()
        .update_funnel(&funnel_id, &update_body)
        .await
        .expect("failed to update funnel");
    let version_after_update = updated
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("updated funnel has no revision version");

    // Delete
    client
        .funnels_api()
        .remove_funnel(
            &funnel_id,
            Some(&version_after_update.to_string()),
            None,
            None,
        )
        .await
        .expect("failed to delete funnel");

    // Verify gone
    assert!(
        client.funnels_api().get_funnel(&funnel_id).await.is_err(),
        "expected error fetching deleted funnel"
    );

    helpers::delete_temp_process_group(&client, &pg_id, pg_version).await;
}
