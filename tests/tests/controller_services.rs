#![cfg(not(feature = "dynamic"))]
mod helpers;

use nifi_rust_client::types::{
    ComponentStateEntity, ControllerServiceDto, ControllerServiceEntity,
    ControllerServiceRunStatusEntity, ControllerServiceRunStatusEntityState,
};

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

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn controller_service_run_status_and_state() {
    let client = helpers::logged_in_client().await;
    let svc_type = first_controller_service_type(&client).await;

    // ── create ────────────────────────────────────────────────────────────────
    let body = ControllerServiceEntity {
        component: Some(ControllerServiceDto {
            name: Some("test-cs-run-status".to_string()),
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

    // ── enable / disable ─────────────────────────────────────────────────────
    #[cfg(feature = "nifi-2-9-0")]
    client
        .controller_services_api()
        .run_status(&svc_id)
        .update_run_status_2(&ControllerServiceRunStatusEntity {
            state: Some(ControllerServiceRunStatusEntityState::Enabled),
            revision: Some(helpers::revision(version)),
            ..Default::default()
        })
        .await
        .expect("failed to enable controller service");
    #[cfg(not(feature = "nifi-2-9-0"))]
    client
        .controller_services_api()
        .run_status(&svc_id)
        .update_run_status_1(&ControllerServiceRunStatusEntity {
            state: Some(ControllerServiceRunStatusEntityState::Enabled),
            revision: Some(helpers::revision(version)),
            ..Default::default()
        })
        .await
        .expect("failed to enable controller service");

    // Re-fetch to get the current revision — the service may still be in ENABLING transition.
    let after_enable = client
        .controller_services_api()
        .get_controller_service(&svc_id, None)
        .await
        .expect("failed to re-fetch controller service after enable");
    let version_after_enable = after_enable
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("re-fetched service has no revision version");
    assert!(
        matches!(
            after_enable
                .component
                .as_ref()
                .and_then(|c| c.state.as_ref()),
            Some(
                nifi_rust_client::types::ControllerServiceDtoState::Enabled
                    | nifi_rust_client::types::ControllerServiceDtoState::Enabling
            )
        ),
        "expected controller service state ENABLED or ENABLING after enable"
    );

    #[cfg(feature = "nifi-2-9-0")]
    client
        .controller_services_api()
        .run_status(&svc_id)
        .update_run_status_2(&ControllerServiceRunStatusEntity {
            state: Some(ControllerServiceRunStatusEntityState::Disabled),
            revision: Some(helpers::revision(version_after_enable)),
            ..Default::default()
        })
        .await
        .expect("failed to disable controller service");
    #[cfg(not(feature = "nifi-2-9-0"))]
    client
        .controller_services_api()
        .run_status(&svc_id)
        .update_run_status_1(&ControllerServiceRunStatusEntity {
            state: Some(ControllerServiceRunStatusEntityState::Disabled),
            revision: Some(helpers::revision(version_after_enable)),
            ..Default::default()
        })
        .await
        .expect("failed to disable controller service");

    // Re-fetch to get the final revision after disable.
    let after_disable = client
        .controller_services_api()
        .get_controller_service(&svc_id, None)
        .await
        .expect("failed to re-fetch controller service after disable");
    let version_after_disable = after_disable
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("re-fetched service has no revision version");
    assert!(
        matches!(
            after_disable
                .component
                .as_ref()
                .and_then(|c| c.state.as_ref()),
            Some(
                nifi_rust_client::types::ControllerServiceDtoState::Disabled
                    | nifi_rust_client::types::ControllerServiceDtoState::Disabling
            )
        ),
        "expected controller service state DISABLED or DISABLING after disable"
    );

    // ── wait for DISABLED state (NiFi transitions asynchronously) ──────────
    for _ in 0..30 {
        let svc = client
            .controller_services_api()
            .get_controller_service(&svc_id, None)
            .await
            .expect("failed to re-fetch service while waiting for DISABLED");
        let state = svc.component.as_ref().and_then(|c| c.state.as_ref());
        if matches!(
            state,
            Some(nifi_rust_client::types::ControllerServiceDtoState::Disabled)
        ) {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    // ── state operations ─────────────────────────────────────────────────────
    client
        .controller_services_api()
        .state(&svc_id)
        .get_state()
        .await
        .expect("failed to get controller service state");

    client
        .controller_services_api()
        .state(&svc_id)
        .clear_state_1(&ComponentStateEntity {
            ..Default::default()
        })
        .await
        .expect("failed to clear controller service state");

    // ── delete ────────────────────────────────────────────────────────────────
    client
        .controller_services_api()
        .remove_controller_service(
            &svc_id,
            Some(&version_after_disable.to_string()),
            None,
            None,
        )
        .await
        .expect("failed to delete controller service");
}
