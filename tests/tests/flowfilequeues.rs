/// Integration tests for FlowFileQueues API — listing and drop requests.
/// Requires a running NiFi instance — use tests/run.sh to start one.
///
/// The test creates a GenerateFlowFile → LogAttribute flow, starts only the
/// source processor so flowfiles accumulate in the queue, then exercises the
/// listing-request and drop-request lifecycle.
mod helpers;

use nifi_rust_client::types::{
    ConnectableDto, ConnectableDtoType, ConnectionDto, ConnectionEntity, PositionDto,
    ProcessorConfigDto, ProcessorDto, ProcessorEntity,
};
use std::time::Duration;

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn flowfile_queue_listing_and_drop() {
    let client = helpers::logged_in_client().await;
    let (pg_id, pg_version) = helpers::create_temp_process_group(&client, "test-queue-pg").await;

    // ── source: GenerateFlowFile, 1 sec schedule ──────────────────────────────
    let src_body = ProcessorEntity {
        component: Some(ProcessorDto {
            name: Some("test-queue-src".to_string()),
            r#type: Some("org.apache.nifi.processors.standard.GenerateFlowFile".to_string()),
            position: Some(PositionDto {
                x: Some(0.0),
                y: Some(0.0),
            }),
            config: Some(ProcessorConfigDto {
                scheduling_period: Some("1 sec".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let src = client
        .processgroups_api()
        .processors(&pg_id)
        .create_processor(&src_body)
        .await
        .expect("failed to create source processor");
    let src_id = src.id.clone().expect("src has no id");
    let src_version = src
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("src has no revision");

    // ── destination: LogAttribute, never scheduled (stays stopped) ────────────
    let dst_body = ProcessorEntity {
        component: Some(ProcessorDto {
            name: Some("test-queue-dst".to_string()),
            r#type: Some("org.apache.nifi.processors.standard.LogAttribute".to_string()),
            position: Some(PositionDto {
                x: Some(400.0),
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
    let dst = client
        .processgroups_api()
        .processors(&pg_id)
        .create_processor(&dst_body)
        .await
        .expect("failed to create destination processor");
    let dst_id = dst.id.clone().expect("dst has no id");
    let dst_version = dst
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("dst has no revision");

    // ── connect src.success → dst ─────────────────────────────────────────────
    let conn_body = ConnectionEntity {
        component: Some(ConnectionDto {
            source: Some(ConnectableDto {
                id: src_id.clone(),
                group_id: pg_id.clone(),
                r#type: ConnectableDtoType::Processor,
                ..Default::default()
            }),
            destination: Some(ConnectableDto {
                id: dst_id.clone(),
                group_id: pg_id.clone(),
                r#type: ConnectableDtoType::Processor,
                ..Default::default()
            }),
            selected_relationships: Some(vec!["success".to_string()]),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let conn = client
        .processgroups_api()
        .connections(&pg_id)
        .create_connection(&conn_body)
        .await
        .expect("failed to create connection");
    let conn_id = conn.id.clone().expect("connection has no id");
    let conn_version = conn
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("connection has no revision");

    // ── start only the source so flowfiles queue up ───────────────────────────
    let src_running_version = helpers::start_processor(&client, &src_id, src_version).await;

    tokio::time::sleep(Duration::from_secs(3)).await;

    // ── stop source before operating on the queue so no new flowfiles arrive ──
    let src_final_version = helpers::stop_processor(&client, &src_id, src_running_version).await;

    // ── listing request ───────────────────────────────────────────────────────
    let listing = client
        .flowfilequeues_api()
        .listing_requests(&conn_id)
        .create_flow_file_listing()
        .await
        .expect("failed to create listing request");
    let listing_id = listing.id.clone().expect("listing request has no id");

    // poll until finished
    loop {
        let l = client
            .flowfilequeues_api()
            .listing_requests(&conn_id)
            .get_listing_request(&listing_id)
            .await
            .expect("failed to poll listing request");
        if l.finished.unwrap_or(false) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(300)).await;
    }

    // clean up the listing request
    client
        .flowfilequeues_api()
        .listing_requests(&conn_id)
        .delete_listing_request(&listing_id)
        .await
        .expect("failed to delete listing request");

    // ── drop request — empties the queue ─────────────────────────────────────
    let drop_req = client
        .flowfilequeues_api()
        .drop_requests(&conn_id)
        .create_drop_request()
        .await
        .expect("failed to create drop request");
    let drop_id = drop_req.id.clone().expect("drop request has no id");

    // poll until finished
    loop {
        let d = client
            .flowfilequeues_api()
            .drop_requests(&conn_id)
            .get_drop_request(&drop_id)
            .await
            .expect("failed to poll drop request");
        if d.finished.unwrap_or(false) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(300)).await;
    }

    // clean up the drop request
    client
        .flowfilequeues_api()
        .drop_requests(&conn_id)
        .remove_drop_request(&drop_id)
        .await
        .expect("failed to delete drop request");

    client
        .connections_api()
        .delete_connection(&conn_id, Some(&conn_version.to_string()), None, None)
        .await
        .expect("failed to delete connection");
    client
        .processors_api()
        .delete_processor(&src_id, Some(&src_final_version.to_string()), None, None)
        .await
        .expect("failed to delete source processor");
    client
        .processors_api()
        .delete_processor(&dst_id, Some(&dst_version.to_string()), None, None)
        .await
        .expect("failed to delete destination processor");
    helpers::delete_temp_process_group(&client, &pg_id, pg_version).await;
}
