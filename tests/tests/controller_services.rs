mod helpers;

use nifi_rust_client::types::{ControllerServiceDto, ControllerServiceEntity};

/// Discover the first available controller service type from the NiFi instance.
async fn first_controller_service_type(client: &nifi_rust_client::NifiClient) -> String {
    let types = client
        .flow_api()
        .get_controller_service_types(None, None, None, None, None, None, None)
        .await
        .expect("failed to list controller service types");
    types
        .controller_service_types
        .as_deref()
        .and_then(|v| v.first())
        .and_then(|t| t.r#type.clone())
        .expect("no controller service types available on this NiFi instance")
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn controller_service_crud_lifecycle() {
    let client = helpers::logged_in_client().await;
    let svc_type = first_controller_service_type(&client).await;

    // Create via controller_api.
    let body = ControllerServiceEntity {
        component: Some(ControllerServiceDto {
            name: Some("test-controller-service".to_string()),
            r#type: Some(svc_type),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let created = client
        .controller_api()
        .create_controller_service(&body)
        .await
        .expect("failed to create controller service");
    let svc_id = created.id.clone().expect("created service has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created service has no revision version");

    // Get — verify name.
    let fetched = client
        .controller_services_api()
        .get_controller_service(&svc_id, None)
        .await
        .expect("failed to get controller service");
    assert_eq!(
        fetched.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("test-controller-service")
    );

    // Update — rename.
    let update_body = ControllerServiceEntity {
        id: Some(svc_id.clone()),
        component: Some(ControllerServiceDto {
            id: Some(svc_id.clone()),
            name: Some("test-controller-service-renamed".to_string()),
            ..Default::default()
        }),
        revision: Some(helpers::revision(version)),
        ..Default::default()
    };
    let updated = client
        .controller_services_api()
        .update_controller_service(&svc_id, &update_body)
        .await
        .expect("failed to update controller service");
    assert_eq!(
        updated.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("test-controller-service-renamed")
    );
    let version_after_update = updated
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("updated service has no revision version");

    // Delete.
    client
        .controller_services_api()
        .remove_controller_service(&svc_id, Some(&version_after_update.to_string()), None, None)
        .await
        .expect("failed to delete controller service");

    // Verify gone.
    assert!(
        client
            .controller_services_api()
            .get_controller_service(&svc_id, None)
            .await
            .is_err(),
        "expected error fetching deleted controller service"
    );
}
