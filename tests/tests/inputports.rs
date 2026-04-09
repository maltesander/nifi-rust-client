#![cfg(not(feature = "dynamic"))]
mod helpers;

use nifi_rust_client::types::{PortDto, PortEntity, PositionDto};

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn input_port_crud_lifecycle() {
    let client = helpers::logged_in_client().await;
    let (pg_id, pg_version) =
        helpers::create_temp_process_group(&client, "test-inputport-pg").await;

    // Create
    let body = PortEntity {
        component: Some(PortDto {
            name: Some("test-input-port".to_string()),
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
        .input_ports(&pg_id)
        .create_input_port(&body)
        .await
        .expect("failed to create input port");
    let port_id = created.id.clone().expect("created input port has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created input port has no revision version");

    // Get — verify name
    let fetched = client
        .inputports_api()
        .get_input_port(&port_id)
        .await
        .expect("failed to get input port");
    assert_eq!(
        fetched.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("test-input-port")
    );

    // Update — rename
    let update_body = PortEntity {
        id: Some(port_id.clone()),
        component: Some(PortDto {
            id: Some(port_id.clone()),
            name: Some("test-input-port-renamed".to_string()),
            ..Default::default()
        }),
        revision: Some(helpers::revision(version)),
        ..Default::default()
    };
    let updated = client
        .inputports_api()
        .update_input_port(&port_id, &update_body)
        .await
        .expect("failed to update input port");
    assert_eq!(
        updated.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("test-input-port-renamed")
    );
    let version_after_update = updated
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("updated input port has no revision version");

    // Delete
    client
        .inputports_api()
        .remove_input_port(
            &port_id,
            Some(&version_after_update.to_string()),
            None,
            None,
        )
        .await
        .expect("failed to delete input port");

    // Verify gone
    assert!(
        client
            .inputports_api()
            .get_input_port(&port_id)
            .await
            .is_err(),
        "expected error fetching deleted input port"
    );

    helpers::delete_temp_process_group(&client, &pg_id, pg_version).await;
}
