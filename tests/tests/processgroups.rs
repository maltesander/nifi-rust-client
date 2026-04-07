/// Integration tests for process group and processor CRUD lifecycle.
/// Requires a running NiFi instance — use tests/run.sh to start one.
///
///   NIFI_URL          (default: https://localhost:8443)
///   NIFI_USERNAME     (default: admin)
///   NIFI_PASSWORD     (default: adminpassword123)
///   NIFI_CA_CERT_PATH (optional: path to PEM CA cert)
use nifi_rust_client::{
    NifiClient, NifiClientBuilder,
    types::{
        ComponentStateEntity, PositionDto, ProcessGroupDto, ProcessGroupEntity, ProcessorConfigDto,
        ProcessorDto, ProcessorEntity, ProcessorRunStatusEntity, ProcessorRunStatusEntityState,
        RevisionDto,
    },
};

mod helpers;

fn nifi_url() -> String {
    std::env::var("NIFI_URL").unwrap_or_else(|_| "https://localhost:8443".to_string())
}

fn nifi_username() -> String {
    std::env::var("NIFI_USERNAME").unwrap_or_else(|_| "admin".to_string())
}

fn nifi_password() -> String {
    std::env::var("NIFI_PASSWORD").unwrap_or_else(|_| "adminpassword123".to_string())
}

async fn logged_in_client() -> NifiClient {
    let mut client = NifiClientBuilder::new(&nifi_url())
        .expect("failed to parse NiFi URL")
        .danger_accept_invalid_certs(true)
        .build()
        .expect("failed to build NiFi client");
    client
        .login(&nifi_username(), &nifi_password())
        .await
        .expect("failed to log in");
    client
}

// ── Process Group CRUD ────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn process_group_crud_lifecycle() {
    let client = logged_in_client().await;
    let root_id = "root";

    // Create
    let body = ProcessGroupEntity {
        component: Some(ProcessGroupDto {
            name: Some("test-pg-lifecycle".to_string()),
            position: Some(PositionDto {
                x: Some(0.0),
                y: Some(0.0),
            }),
            ..Default::default()
        }),
        revision: Some(RevisionDto {
            version: Some(0),
            ..Default::default()
        }),
        ..Default::default()
    };
    let created = client
        .processgroups_api()
        .process_groups(root_id)
        .create_process_group(None, &body)
        .await
        .expect("failed to create process group");
    let pg_id = created.id.clone().expect("created entity has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created entity has no revision version");

    // Get — verify it exists with the right name
    let fetched = client
        .processgroups_api()
        .get_process_group(&pg_id)
        .await
        .expect("failed to get process group");
    assert_eq!(
        fetched.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("test-pg-lifecycle")
    );

    // Update — rename it
    let update_body = ProcessGroupEntity {
        id: Some(pg_id.clone()),
        component: Some(ProcessGroupDto {
            id: Some(pg_id.clone()),
            name: Some("test-pg-lifecycle-renamed".to_string()),
            ..Default::default()
        }),
        revision: Some(RevisionDto {
            version: Some(version),
            ..Default::default()
        }),
        ..Default::default()
    };
    let updated = client
        .processgroups_api()
        .update_process_group(&pg_id, &update_body)
        .await
        .expect("failed to update process group");
    assert_eq!(
        updated.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("test-pg-lifecycle-renamed")
    );
    let version_after_update = updated
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("updated entity has no revision version");

    // Delete
    client
        .processgroups_api()
        .remove_process_group(&pg_id, Some(&version_after_update.to_string()), None, None)
        .await
        .expect("failed to delete process group");

    // Verify gone — expect an error (404)
    let fetch_after_delete = client.processgroups_api().get_process_group(&pg_id).await;
    assert!(
        fetch_after_delete.is_err(),
        "expected error fetching deleted process group, got {:?}",
        fetch_after_delete
    );
}

// ── Processor CRUD inside a process group ────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn processor_crud_lifecycle() {
    let client = logged_in_client().await;
    let root_id = "root";

    // Create a parent process group to hold the processor
    let pg_body = ProcessGroupEntity {
        component: Some(ProcessGroupDto {
            name: Some("test-processor-lifecycle-pg".to_string()),
            position: Some(PositionDto {
                x: Some(100.0),
                y: Some(100.0),
            }),
            ..Default::default()
        }),
        revision: Some(RevisionDto {
            version: Some(0),
            ..Default::default()
        }),
        ..Default::default()
    };
    let pg = client
        .processgroups_api()
        .process_groups(root_id)
        .create_process_group(None, &pg_body)
        .await
        .expect("failed to create parent process group");
    let pg_id = pg.id.clone().expect("process group has no id");
    let pg_version = pg
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("process group has no revision version");

    // Create processor (GenerateFlowFile — always available in NiFi)
    let proc_body = ProcessorEntity {
        component: Some(ProcessorDto {
            name: Some("test-generate-flowfile".to_string()),
            r#type: Some("org.apache.nifi.processors.standard.GenerateFlowFile".to_string()),
            position: Some(PositionDto {
                x: Some(0.0),
                y: Some(0.0),
            }),
            config: Some(ProcessorConfigDto {
                scheduling_period: Some("10 sec".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        revision: Some(RevisionDto {
            version: Some(0),
            ..Default::default()
        }),
        ..Default::default()
    };
    let created_proc = client
        .processgroups_api()
        .processors(&pg_id)
        .create_processor(&proc_body)
        .await
        .expect("failed to create processor");
    let proc_id = created_proc.id.clone().expect("processor has no id");
    let proc_version = created_proc
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("processor has no revision version");

    // Get — verify it exists
    let fetched_proc = client
        .processors_api()
        .get_processor(&proc_id)
        .await
        .expect("failed to get processor");
    assert_eq!(
        fetched_proc
            .component
            .as_ref()
            .and_then(|c| c.name.as_deref()),
        Some("test-generate-flowfile")
    );

    // List with includeDescendantGroups=true — processor should appear under root
    let all_processors = client
        .processgroups_api()
        .processors(root_id)
        .get_processors(Some(true))
        .await
        .expect("failed to list processors with includeDescendantGroups");
    let found = all_processors
        .processors
        .as_deref()
        .unwrap_or_default()
        .iter()
        .any(|p| p.id.as_deref() == Some(&proc_id));
    assert!(found, "created processor not found in descendant listing");

    // Delete processor
    client
        .processors_api()
        .delete_processor(&proc_id, Some(&proc_version.to_string()), None, None)
        .await
        .expect("failed to delete processor");

    // Delete parent process group
    client
        .processgroups_api()
        .remove_process_group(&pg_id, Some(&pg_version.to_string()), None, None)
        .await
        .expect("failed to delete parent process group");

    // Verify processor is gone
    let fetch_after_delete = client.processors_api().get_processor(&proc_id).await;
    assert!(
        fetch_after_delete.is_err(),
        "expected error fetching deleted processor, got {:?}",
        fetch_after_delete
    );
}

// ── Processor run-status lifecycle ───────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn processor_run_status_lifecycle() {
    let client = logged_in_client().await;

    // ── setup ──────────────────────────────────────────────────────────────────
    let (pg_id, pg_version) =
        helpers::create_temp_process_group(&client, "test-proc-run-status-pg").await;

    let proc_body = ProcessorEntity {
        component: Some(ProcessorDto {
            name: Some("test-run-status-generate".to_string()),
            r#type: Some("org.apache.nifi.processors.standard.GenerateFlowFile".to_string()),
            position: Some(PositionDto {
                x: Some(0.0),
                y: Some(0.0),
            }),
            config: Some(ProcessorConfigDto {
                scheduling_period: Some("10 sec".to_string()),
                // Auto-terminate the success relationship so the processor is valid and startable.
                auto_terminated_relationships: Some(vec!["success".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        }),
        revision: Some(RevisionDto {
            version: Some(0),
            ..Default::default()
        }),
        ..Default::default()
    };
    let created_proc = client
        .processgroups_api()
        .processors(&pg_id)
        .create_processor(&proc_body)
        .await
        .expect("failed to create processor");
    let proc_id = created_proc.id.clone().expect("processor has no id");
    let proc_version = created_proc
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("processor has no revision version");

    // ── property descriptor (works on a stopped processor) ───────────────────
    let descriptor = client
        .processors_api()
        .descriptors(&proc_id)
        .get_property_descriptor_3(None, "File Size", None)
        .await
        .expect("failed to get property descriptor");
    assert!(
        descriptor.name.is_some(),
        "expected property descriptor to have a name"
    );

    // ── processor state read + clear (processor is freshly created = stopped) ─
    client
        .processors_api()
        .state(&proc_id)
        .get_state_2()
        .await
        .expect("failed to get processor state");

    client
        .processors_api()
        .state(&proc_id)
        .clear_state_3(&ComponentStateEntity {
            ..Default::default()
        })
        .await
        .expect("failed to clear processor state");

    // ── start / stop ──────────────────────────────────────────────────────────
    let started = client
        .processors_api()
        .run_status(&proc_id)
        .update_run_status_4(&ProcessorRunStatusEntity {
            state: Some(ProcessorRunStatusEntityState::Running),
            revision: Some(helpers::revision(proc_version)),
            ..Default::default()
        })
        .await
        .expect("failed to start processor");
    assert!(
        matches!(
            started.component.as_ref().and_then(|c| c.state.as_ref()),
            Some(nifi_rust_client::types::ProcessorDtoState::Running)
        ),
        "expected processor state RUNNING after start"
    );
    let running_version = started
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("started entity has no revision version");

    // ── diagnostics (requires running processor) ──────────────────────────────
    client
        .processors_api()
        .diagnostics(&proc_id)
        .get_processor_diagnostics()
        .await
        .expect("failed to get processor diagnostics");

    let stopped = client
        .processors_api()
        .run_status(&proc_id)
        .update_run_status_4(&ProcessorRunStatusEntity {
            state: Some(ProcessorRunStatusEntityState::Stopped),
            revision: Some(helpers::revision(running_version)),
            ..Default::default()
        })
        .await
        .expect("failed to stop processor");
    assert!(
        matches!(
            stopped.component.as_ref().and_then(|c| c.state.as_ref()),
            Some(nifi_rust_client::types::ProcessorDtoState::Stopped)
        ),
        "expected processor state STOPPED after stop"
    );

    // ── cleanup ───────────────────────────────────────────────────────────────
    let final_version = stopped
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("stopped entity has no revision version");

    client
        .processors_api()
        .delete_processor(&proc_id, Some(&final_version.to_string()), None, None)
        .await
        .expect("failed to delete processor");

    helpers::delete_temp_process_group(&client, &pg_id, pg_version).await;
}

// ── Processor terminate ───────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn processor_terminate_clears_threads() {
    // terminate_processor (DELETE /processors/{id}/threads) clears any residual threads
    // on a stopped processor — it is most useful for processors stuck in STOPPING state,
    // but also succeeds on a freshly-stopped processor with no active threads.
    let client = logged_in_client().await;

    let (pg_id, pg_version) =
        helpers::create_temp_process_group(&client, "test-proc-terminate-pg").await;

    let proc_body = ProcessorEntity {
        component: Some(ProcessorDto {
            name: Some("test-terminate-generate".to_string()),
            r#type: Some("org.apache.nifi.processors.standard.GenerateFlowFile".to_string()),
            position: Some(PositionDto {
                x: Some(0.0),
                y: Some(0.0),
            }),
            config: Some(ProcessorConfigDto {
                scheduling_period: Some("10 sec".to_string()),
                auto_terminated_relationships: Some(vec!["success".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        }),
        revision: Some(RevisionDto {
            version: Some(0),
            ..Default::default()
        }),
        ..Default::default()
    };
    let created_proc = client
        .processgroups_api()
        .processors(&pg_id)
        .create_processor(&proc_body)
        .await
        .expect("failed to create processor");
    let proc_id = created_proc.id.clone().expect("processor has no id");
    let proc_version = created_proc
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("processor has no revision version");

    // Terminate on a stopped processor clears any residual threads and returns the entity.
    client
        .processors_api()
        .threads(&proc_id)
        .terminate_processor()
        .await
        .expect("failed to terminate processor threads");

    client
        .processors_api()
        .delete_processor(&proc_id, Some(&proc_version.to_string()), None, None)
        .await
        .expect("failed to delete processor");

    helpers::delete_temp_process_group(&client, &pg_id, pg_version).await;
}
