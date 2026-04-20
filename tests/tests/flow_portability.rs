#![cfg(feature = "dynamic")]
//! End-to-end integration tests for the Phase 3 flow portability
//! endpoints (`export_process_group`, `upload_process_group`,
//! `replace_process_group`) against a live NiFi instance. Mirrors the
//! Task 13 manual smoke pass so the happy path is re-runnable from CI.
//!
//! Each test is `#[ignore]`d — run with `./tests/run.sh` which sets up a
//! NiFi container and invokes `cargo test -- --ignored`.

mod helpers;

use helpers::dynamic_logged_in_client;
use nifi_rust_client::bulk;
use nifi_rust_client::dynamic::types::{
    PositionDto, ProcessGroupDto, ProcessGroupEntity, ProcessGroupImportEntity, RevisionDto,
};

/// Create a throwaway child PG under root and return `(id, revision_version)`.
async fn create_temp_pg(
    client: &nifi_rust_client::dynamic::DynamicClient,
    name: &str,
) -> (String, i64) {
    let mut position = PositionDto::default();
    position.x = Some(0.0);
    position.y = Some(0.0);
    let mut component = ProcessGroupDto::default();
    component.name = Some(name.to_string());
    component.position = Some(position);
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
    let id = created.id.clone().expect("created PG has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created PG has no revision version");
    (id, version)
}

async fn delete_temp_pg(
    client: &nifi_rust_client::dynamic::DynamicClient,
    id: &str,
    version: i64,
) {
    client
        .processgroups()
        .remove_process_group(id, Some(&version.to_string()), None, None)
        .await
        .expect("remove_process_group failed");
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn flow_export_returns_registered_flow_snapshot_shape() {
    let client = dynamic_logged_in_client().await;
    let snapshot = client
        .processgroups()
        .export_process_group("root", None)
        .await
        .expect("export_process_group failed");

    // The response is `serde_json::Value` (inline-schema fix from Task 2).
    // Every NiFi 2.x version observed in Task 1 returns these top-level keys.
    let obj = snapshot
        .as_object()
        .expect("export body is not a JSON object");
    for key in ["flowContents", "flowEncodingVersion", "parameterContexts"] {
        assert!(
            obj.contains_key(key),
            "snapshot missing expected top-level key '{key}': keys={:?}",
            obj.keys().collect::<Vec<_>>()
        );
    }
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn flow_import_then_replace_round_trip() {
    let client = dynamic_logged_in_client().await;

    // 1. Export root to get a realistic snapshot to feed back.
    let snapshot = client
        .processgroups()
        .export_process_group("root", None)
        .await
        .expect("export_process_group failed");
    let snapshot_bytes = serde_json::to_vec(&snapshot).expect("snapshot not serializable");

    // 2. Upload as a new child PG. Exercises the multipart emitter fix
    //    from Task 5 — file-only uploads return 400.
    let entity = client
        .processgroups()
        .upload_process_group(
            "root",
            "nifictl-int-test",
            None,
            "flow-portability-test",
            0.0,
            0.0,
            "snapshot.json",
            snapshot_bytes,
        )
        .await
        .expect("upload_process_group failed");
    let child_id = entity
        .id
        .as_deref()
        .expect("uploaded PG has no id")
        .to_string();

    // 3. Re-fetch the revision, then replace in place.
    let pg = client
        .processgroups()
        .get_process_group(&child_id)
        .await
        .expect("get_process_group failed");
    let rev = pg.revision.clone().expect("child PG has no revision");

    let mut body = ProcessGroupImportEntity::default();
    body.disconnected_node_acknowledged = Some(false);
    body.process_group_revision = Some(rev);
    body.versioned_flow_snapshot = Some(
        serde_json::from_value(snapshot.clone()).expect("snapshot not deserializable"),
    );
    let replaced = client
        .processgroups()
        .replace_process_group(&child_id, &body)
        .await
        .expect("replace_process_group failed");
    let replace_rev = replaced
        .process_group_revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("replace response has no revision");
    assert!(
        replace_rev > 0,
        "replace should bump revision; got {replace_rev}"
    );

    // 4. Exercise the stop-first orchestration via the bulk helpers.
    bulk::stop_process_group_dynamic(&client, &child_id)
        .await
        .expect("bulk stop failed");
    let pg2 = client
        .processgroups()
        .get_process_group(&child_id)
        .await
        .expect("get_process_group failed");
    let rev2 = pg2.revision.clone().expect("child PG has no revision");
    let mut body2 = ProcessGroupImportEntity::default();
    body2.disconnected_node_acknowledged = Some(false);
    body2.process_group_revision = Some(rev2);
    body2.versioned_flow_snapshot = Some(
        serde_json::from_value(snapshot).expect("snapshot not deserializable"),
    );
    client
        .processgroups()
        .replace_process_group(&child_id, &body2)
        .await
        .expect("second replace_process_group failed");
    bulk::start_process_group_dynamic(&client, &child_id)
        .await
        .expect("bulk start failed");

    // 5. Cleanup — NiFi refuses to delete a PG with running components,
    //    and the start above kicked off the imported processors.
    bulk::stop_process_group_dynamic(&client, &child_id)
        .await
        .expect("bulk stop before delete failed");
    let final_pg = client
        .processgroups()
        .get_process_group(&child_id)
        .await
        .expect("final get_process_group failed");
    let final_rev = final_pg
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("final PG has no revision");
    delete_temp_pg(&client, &child_id, final_rev).await;
}

/// Proves the upload is rejected without `groupName` — regression test
/// that the Task 5 multipart fields remain required by NiFi.
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn create_temp_pg_then_cleanup() {
    let client = dynamic_logged_in_client().await;
    let (id, _) = create_temp_pg(&client, "flow-portability-cleanup").await;
    let pg = client
        .processgroups()
        .get_process_group(&id)
        .await
        .expect("get_process_group failed");
    let rev = pg
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("missing revision");
    delete_temp_pg(&client, &id, rev).await;
}
