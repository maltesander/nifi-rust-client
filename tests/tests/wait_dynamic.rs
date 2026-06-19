#![cfg(feature = "dynamic")]
//! Integration tests for the dynamic process-group wait helpers against a real
//! NiFi. Run via `./tests/run.sh` (these are `#[ignore]`d without a live server).
mod helpers;

use std::time::Duration;

use nifi_rust_client::bulk;
use nifi_rust_client::dynamic::DynamicClient;
use nifi_rust_client::dynamic::types::{
    ControllerServiceDto, ControllerServiceEntity, PositionDto, ProcessGroupDto,
    ProcessGroupEntity, ProcessorConfigDto, ProcessorDto, ProcessorEntity, RevisionDto,
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

/// Origin position (dynamic DTOs are `#[non_exhaustive]`, so build by mutation).
fn origin() -> PositionDto {
    let mut p = PositionDto::default();
    p.x = Some(0.0);
    p.y = Some(0.0);
    p
}

/// Create a throwaway child PG under root and return its id.
async fn create_temp_pg(client: &DynamicClient, name: &str) -> String {
    let mut component = ProcessGroupDto::default();
    component.name = Some(name.to_string());
    component.position = Some(origin());
    let mut revision = RevisionDto::default();
    revision.version = Some(0);
    let mut body = ProcessGroupEntity::default();
    body.component = Some(component);
    body.revision = Some(revision);
    let created = client
        .processgroups()
        .create_process_group("root", None, &body)
        .await
        .expect("create_process_group failed");
    created.id.clone().expect("created PG has no id")
}

/// Create a valid, runnable GenerateFlowFile (auto-terminated relationship).
async fn create_runnable_processor(client: &DynamicClient, pg_id: &str, name: &str) {
    let mut config = ProcessorConfigDto::default();
    config.scheduling_period = Some("1 hour".to_string());
    config.auto_terminated_relationships = Some(vec!["success".to_string()]);
    let mut component = ProcessorDto::default();
    component.name = Some(name.to_string());
    component.r#type = Some("org.apache.nifi.processors.standard.GenerateFlowFile".to_string());
    component.position = Some(origin());
    component.config = Some(config);
    let mut revision = RevisionDto::default();
    revision.version = Some(0);
    let mut body = ProcessorEntity::default();
    body.component = Some(component);
    body.revision = Some(revision);
    client
        .processgroups()
        .create_processor(pg_id, &body)
        .await
        .expect("failed to create processor");
}

/// Re-fetch the current PG revision and delete it (and its children).
async fn cleanup_pg(client: &DynamicClient, pg_id: &str) {
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
    client
        .processgroups()
        .remove_process_group(pg_id, Some(&version.to_string()), None, None)
        .await
        .expect("failed to delete temp PG");
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn process_group_state_dynamic_waits_for_running_then_stopped() {
    let client = helpers::dynamic_logged_in_client().await;
    let pg_id = create_temp_pg(&client, "test-wait-state-dyn-pg").await;
    create_runnable_processor(&client, &pg_id, "wait-src").await;

    bulk::start_process_group_dynamic(&client, &pg_id)
        .await
        .expect("bulk start failed");
    let running = wait::process_group_state_dynamic(
        &client,
        &pg_id,
        ProcessGroupTargetState::Running,
        wait_config(),
    )
    .await
    .expect("PG did not reach Running");
    assert_eq!(running.stopped_count, Some(0));

    bulk::stop_process_group_dynamic(&client, &pg_id)
        .await
        .expect("bulk stop failed");
    let stopped = wait::process_group_state_dynamic(
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
async fn process_group_controller_services_state_dynamic_settles() {
    let client = helpers::dynamic_logged_in_client().await;
    let pg_id = create_temp_pg(&client, "test-wait-cs-dyn-pg").await;

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

    let mut component = ControllerServiceDto::default();
    component.name = Some("test-wait-cs".to_string());
    component.r#type = Some(svc_type);
    let mut revision = RevisionDto::default();
    revision.version = Some(0);
    let mut body = ControllerServiceEntity::default();
    body.component = Some(component);
    body.revision = Some(revision);
    client
        .processgroups()
        .create_controller_service(&pg_id, &body)
        .await
        .expect("failed to create controller service");

    bulk::enable_all_controller_services_dynamic(&client, &pg_id)
        .await
        .expect("bulk enable failed");
    let enabled = wait::process_group_controller_services_state_dynamic(
        &client,
        &pg_id,
        ControllerServiceTargetState::Enabled,
        wait_config(),
    )
    .await
    .expect("controller services did not settle on enable");
    assert!(enabled.controller_services.is_some());

    bulk::disable_all_controller_services_dynamic(&client, &pg_id)
        .await
        .expect("bulk disable failed");
    let disabled = wait::process_group_controller_services_state_dynamic(
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
