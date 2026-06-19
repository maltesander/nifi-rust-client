#![cfg(not(feature = "dynamic"))]
//! Integration tests for the process-group wait helpers against a real NiFi.
//!
//! Run via `./tests/run.sh` (these are `#[ignore]`d without a live server).
mod helpers;

use std::time::Duration;

use nifi_rust_client::NifiClient;
use nifi_rust_client::types::{
    ControllerServiceDto, ControllerServiceEntity, PositionDto, ProcessorConfigDto, ProcessorDto,
    ProcessorEntity,
};
use nifi_rust_client::wait::{
    self, ControllerServiceTargetState, ProcessGroupTargetState, WaitConfig,
};

fn wait_config() -> WaitConfig {
    WaitConfig {
        timeout: Duration::from_secs(30),
        poll_interval: Duration::from_millis(500),
        ..Default::default()
    }
}

/// Create a valid, runnable GenerateFlowFile. Its only relationship is
/// auto-terminated, so the processor is valid with no downstream connection
/// and the parent PG can transition to RUNNING.
async fn create_runnable_processor(client: &NifiClient, pg_id: &str, name: &str) {
    let body = ProcessorEntity {
        component: Some(ProcessorDto {
            name: Some(name.to_string()),
            r#type: Some("org.apache.nifi.processors.standard.GenerateFlowFile".to_string()),
            position: Some(PositionDto {
                x: Some(0.0),
                y: Some(0.0),
            }),
            config: Some(ProcessorConfigDto {
                scheduling_period: Some("1 hour".to_string()),
                auto_terminated_relationships: Some(vec!["success".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    client
        .processgroups()
        .create_processor(pg_id, &body)
        .await
        .expect("failed to create processor");
}

/// Re-fetch the current PG revision and delete it (and its children).
async fn cleanup_pg(client: &NifiClient, pg_id: &str) {
    let pg = client
        .processgroups()
        .get_process_group(pg_id)
        .await
        .expect("failed to fetch PG for cleanup");
    let version = pg
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("PG has no revision version");
    helpers::delete_temp_process_group(client, pg_id, version).await;
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn process_group_state_waits_for_running_then_stopped() {
    let client = helpers::logged_in_client().await;
    let (pg_id, _v) = helpers::create_temp_process_group(&client, "test-wait-state-pg").await;
    create_runnable_processor(&client, &pg_id, "wait-src").await;

    nifi_rust_client::bulk::start_process_group(&client, &pg_id)
        .await
        .expect("bulk start failed");
    let running = wait::process_group_state(
        &client,
        &pg_id,
        ProcessGroupTargetState::Running,
        wait_config(),
    )
    .await
    .expect("PG did not reach Running");
    assert_eq!(running.stopped_count, Some(0));

    nifi_rust_client::bulk::stop_process_group(&client, &pg_id)
        .await
        .expect("bulk stop failed");
    let stopped = wait::process_group_state(
        &client,
        &pg_id,
        ProcessGroupTargetState::Stopped,
        wait_config(),
    )
    .await
    .expect("PG did not reach Stopped");
    assert_eq!(stopped.running_count, Some(0));

    cleanup_pg(&client, &pg_id).await;
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn process_group_controller_services_state_settles_enable_then_disable() {
    let client = helpers::logged_in_client().await;
    let (pg_id, _v) = helpers::create_temp_process_group(&client, "test-wait-cs-pg").await;

    let svc_type = client
        .flow()
        .get_controller_service_types(None, None, None, None, None, None, None)
        .await
        .expect("failed to list controller service types")
        .controller_service_types
        .as_deref()
        .and_then(|v| v.first())
        .and_then(|t| t.r#type.clone())
        .expect("no controller service types available");

    let body = ControllerServiceEntity {
        component: Some(ControllerServiceDto {
            name: Some("test-wait-cs".to_string()),
            r#type: Some(svc_type),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    client
        .processgroups()
        .create_controller_service(&pg_id, &body)
        .await
        .expect("failed to create controller service");

    // Enable: a service that can't enable (invalid by default) settles as
    // DISABLED+INVALID; a valid one settles ENABLED. Either way the wait
    // resolves — the point is exercising the real wire shapes (state +
    // validationStatus) against a live server.
    nifi_rust_client::bulk::enable_all_controller_services(&client, &pg_id)
        .await
        .expect("bulk enable failed");
    let enabled = wait::process_group_controller_services_state(
        &client,
        &pg_id,
        ControllerServiceTargetState::Enabled,
        wait_config(),
    )
    .await
    .expect("controller services did not settle on enable");
    assert!(enabled.controller_services.is_some());

    nifi_rust_client::bulk::disable_all_controller_services(&client, &pg_id)
        .await
        .expect("bulk disable failed");
    let disabled = wait::process_group_controller_services_state(
        &client,
        &pg_id,
        ControllerServiceTargetState::Disabled,
        wait_config(),
    )
    .await
    .expect("controller services did not settle on disable");
    assert!(disabled.controller_services.is_some());

    cleanup_pg(&client, &pg_id).await;
}
