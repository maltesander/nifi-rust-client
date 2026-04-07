/// Integration tests for the Provenance and ProvenanceEvents APIs.
/// Requires a running NiFi instance — use tests/run.sh to start one.
mod helpers;

use nifi_rust_client::types::{
    PositionDto, ProcessorConfigDto, ProcessorDto, ProcessorEntity, ProvenanceDto,
    ProvenanceEntity, ProvenanceRequestDto,
};
use std::time::Duration;

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn provenance_search_and_events() {
    let client = helpers::logged_in_client().await;
    let (pg_id, pg_version) =
        helpers::create_temp_process_group(&client, "test-provenance-pg").await;

    // ── create GenerateFlowFile and start it ──────────────────────────────────
    let proc_body = ProcessorEntity {
        component: Some(ProcessorDto {
            name: Some("test-provenance-generate".to_string()),
            r#type: Some("org.apache.nifi.processors.standard.GenerateFlowFile".to_string()),
            position: Some(PositionDto {
                x: Some(0.0),
                y: Some(0.0),
            }),
            config: Some(ProcessorConfigDto {
                scheduling_period: Some("1 sec".to_string()),
                auto_terminated_relationships: Some(vec!["success".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let created = client
        .processgroups_api()
        .processors(&pg_id)
        .create_processor(&proc_body)
        .await
        .expect("failed to create processor");
    let proc_id = created.id.clone().expect("processor has no id");
    let proc_version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("processor has no revision");

    let running_version = helpers::start_processor(&client, &proc_id, proc_version).await;

    // ── let a few flowfiles generate so provenance events exist ───────────────
    tokio::time::sleep(Duration::from_secs(3)).await;

    // ── get_search_options — no data required ─────────────────────────────────
    client
        .provenance_api()
        .get_search_options()
        .await
        .expect("failed to get provenance search options");

    // ── submit a provenance query ─────────────────────────────────────────────
    let query_body = ProvenanceEntity {
        provenance: Some(ProvenanceDto {
            request: Some(ProvenanceRequestDto {
                max_results: Some(10),
                incremental_results: Some(false),
                ..Default::default()
            }),
            ..Default::default()
        }),
    };
    let submitted = client
        .provenance_api()
        .submit_provenance_request(&query_body)
        .await
        .expect("failed to submit provenance query");
    let query_id = submitted.id.clone().expect("provenance query has no id");

    // ── poll until the query finishes ─────────────────────────────────────────
    let provenance = loop {
        let p = client
            .provenance_api()
            .get_provenance(&query_id, None, None, Some(false))
            .await
            .expect("failed to poll provenance query");
        if p.finished.unwrap_or(false) || p.percent_completed.unwrap_or(0) >= 100 {
            break p;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    };

    // ── fetch a specific event by id if the query returned any ────────────────
    if let Some(event) = provenance
        .results
        .as_ref()
        .and_then(|r| r.provenance_events.as_deref())
        .and_then(|e| e.first())
    {
        if let Some(event_id) = event.event_id {
            client
                .provenanceevents_api()
                .get_provenance_event(&event_id.to_string(), None)
                .await
                .expect("failed to get provenance event by id");
        }
    }

    // ── delete the provenance query ───────────────────────────────────────────
    client
        .provenance_api()
        .delete_provenance(&query_id, None)
        .await
        .expect("failed to delete provenance query");

    // ── cleanup ───────────────────────────────────────────────────────────────
    let stopped_version = helpers::stop_processor(&client, &proc_id, running_version).await;
    client
        .processors_api()
        .delete_processor(&proc_id, Some(&stopped_version.to_string()), None, None)
        .await
        .expect("failed to delete processor");
    helpers::delete_temp_process_group(&client, &pg_id, pg_version).await;
}
