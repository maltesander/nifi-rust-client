#![cfg(not(feature = "dynamic"))]
mod helpers;

use nifi_rust_client::types::{PortDto, PortEntity, PositionDto};

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn output_port_crud_lifecycle() {
    let client = helpers::logged_in_client().await;
    let (pg_id, pg_version) =
        helpers::create_temp_process_group(&client, "test-outputport-pg").await;

    // Create
    let body = PortEntity {
        component: Some(PortDto {
            name: Some("test-output-port".to_string()),
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
        .output_ports(&pg_id)
        .create_output_port(&body)
        .await
        .expect("failed to create output port");
    let port_id = created.id.clone().expect("created output port has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created output port has no revision version");

    // Get — verify name
    let fetched = client
        .outputports_api()
        .get_output_port(&port_id)
        .await
        .expect("failed to get output port");
    assert_eq!(
        fetched.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("test-output-port")
    );

    // Update — rename
    let update_body = PortEntity {
        id: Some(port_id.clone()),
        component: Some(PortDto {
            id: Some(port_id.clone()),
            name: Some("test-output-port-renamed".to_string()),
            ..Default::default()
        }),
        revision: Some(helpers::revision(version)),
        ..Default::default()
    };
    let updated = client
        .outputports_api()
        .update_output_port(&port_id, &update_body)
        .await
        .expect("failed to update output port");
    assert_eq!(
        updated.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("test-output-port-renamed")
    );
    let version_after_update = updated
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("updated output port has no revision version");

    // Delete
    client
        .outputports_api()
        .remove_output_port(
            &port_id,
            Some(&version_after_update.to_string()),
            None,
            None,
        )
        .await
        .expect("failed to delete output port");

    // Verify gone
    assert!(
        client
            .outputports_api()
            .get_output_port(&port_id)
            .await
            .is_err(),
        "expected error fetching deleted output port"
    );

    helpers::delete_temp_process_group(&client, &pg_id, pg_version).await;
}
