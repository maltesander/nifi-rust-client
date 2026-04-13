// Shared helpers for integration tests.
// This file is auto-discovered by Cargo as a test binary with no tests of its own;
// functions here are used via `mod helpers;` in sibling test files.
#![allow(dead_code)]

use nifi_rust_client::NifiClientBuilder;

pub fn nifi_url() -> String {
    std::env::var("NIFI_URL").unwrap_or_else(|_| "https://localhost:8443".to_string())
}

pub fn nifi_username() -> String {
    std::env::var("NIFI_USERNAME").unwrap_or_else(|_| "admin".to_string())
}

pub fn nifi_password() -> String {
    std::env::var("NIFI_PASSWORD").unwrap_or_else(|_| "adminpassword123".to_string())
}

// ── Static-mode helpers (not available when `dynamic` feature is active) ─────

#[cfg(not(feature = "dynamic"))]
use nifi_rust_client::{
    NifiClient,
    types::{
        PositionDto, ProcessGroupDto, ProcessGroupEntity, ProcessorRunStatusEntity,
        ProcessorRunStatusEntityState, RevisionDto,
    },
};

#[cfg(not(feature = "dynamic"))]
pub async fn logged_in_client() -> NifiClient {
    let client = NifiClientBuilder::new(&nifi_url())
        .expect("failed to parse NiFi URL")
        .danger_accept_invalid_certs(true)
        .build()
        .expect("failed to build NiFi client");

    // Reuse a cached token when multiple tests run concurrently to avoid hitting
    // NiFi's rate limit on the login endpoint (HTTP 429 Too Many Requests).
    // Token path: /tmp/nifi_test_token_<url_hash>
    let token_path = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        nifi_url().hash(&mut h);
        std::path::PathBuf::from(format!("/tmp/nifi_test_token_{:x}", h.finish()))
    };

    // Try reading a cached token first.
    if let Ok(token) = std::fs::read_to_string(&token_path) {
        let token = token.trim().to_string();
        if !token.is_empty() {
            client.set_token(token).await;
            // Quick sanity-check: if the token is still valid, use it.
            if client.flow().get_about_info().await.is_ok() {
                return client;
            }
            // Token expired — fall through to re-login below.
        }
    }

    client
        .login(&nifi_username(), &nifi_password())
        .await
        .expect("failed to log in");

    // Cache the fresh token for sibling tests in this run.
    if let Some(token) = client.token().await {
        let _ = std::fs::write(&token_path, token);
    }

    client
}

/// Create a throwaway process group under root for use as a test parent.
/// Returns `(id, revision_version)`.
#[cfg(not(feature = "dynamic"))]
pub async fn create_temp_process_group(client: &NifiClient, name: &str) -> (String, i64) {
    let body = ProcessGroupEntity {
        component: Some(ProcessGroupDto {
            name: Some(name.to_string()),
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
        .processgroups()
        .create_process_group("root", None, &body)
        .await
        .expect("failed to create temp process group");
    let id = created.id.clone().expect("created PG has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created PG has no revision version");
    (id, version)
}

#[cfg(not(feature = "dynamic"))]
pub async fn delete_temp_process_group(client: &NifiClient, id: &str, version: i64) {
    client
        .processgroups()
        .remove_process_group(id, Some(&version.to_string()), None, None)
        .await
        .expect("failed to delete temp process group");
}

#[cfg(not(feature = "dynamic"))]
pub fn revision(version: i64) -> RevisionDto {
    RevisionDto {
        version: Some(version),
        ..Default::default()
    }
}

/// Start a processor and return the new revision version.
#[cfg(not(feature = "dynamic"))]
pub async fn start_processor(client: &NifiClient, proc_id: &str, version: i64) -> i64 {
    let result = client
        .processors()
        .update_run_status(
            proc_id,
            &ProcessorRunStatusEntity {
                state: Some(ProcessorRunStatusEntityState::Running),
                revision: Some(revision(version)),
                ..Default::default()
            },
        )
        .await
        .expect("failed to start processor");
    result
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("started entity has no revision version")
}

/// Stop a processor and return the new revision version.
#[cfg(not(feature = "dynamic"))]
pub async fn stop_processor(client: &NifiClient, proc_id: &str, version: i64) -> i64 {
    let result = client
        .processors()
        .update_run_status(
            proc_id,
            &ProcessorRunStatusEntity {
                state: Some(ProcessorRunStatusEntityState::Stopped),
                revision: Some(revision(version)),
                ..Default::default()
            },
        )
        .await
        .expect("failed to stop processor");
    result
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("stopped entity has no revision version")
}

// ── Dynamic-mode helpers ─────────────────────────────────────────────────────

#[cfg(feature = "dynamic")]
pub async fn dynamic_logged_in_client() -> nifi_rust_client::dynamic::DynamicClient {
    let client = NifiClientBuilder::new(&nifi_url())
        .expect("failed to parse NiFi URL")
        .danger_accept_invalid_certs(true)
        .build_dynamic()
        .expect("failed to build dynamic client");

    // Reuse a cached token when multiple tests run concurrently to avoid hitting
    // NiFi's rate limit on the login endpoint (HTTP 429 Too Many Requests).
    let token_path = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        nifi_url().hash(&mut h);
        std::path::PathBuf::from(format!("/tmp/nifi_test_token_dynamic_{:x}", h.finish()))
    };

    // Try reading a cached token first.
    if let Ok(token) = std::fs::read_to_string(&token_path) {
        let token = token.trim().to_string();
        if !token.is_empty() {
            client.set_token(token).await;
            // Detect version using the cached token.
            if client.detect_version().await.is_ok() && client.flow().get_about_info().await.is_ok()
            {
                return client;
            }
            // Token expired — fall through to re-login below.
        }
    }

    // login() authenticates AND detects the NiFi version automatically.
    client
        .login(&nifi_username(), &nifi_password())
        .await
        .expect("failed to log in");

    // Cache the fresh token for sibling tests in this run.
    if let Some(token) = client.token().await {
        let _ = std::fs::write(&token_path, token);
    }

    client
}

// ── Test flow setup (dynamic mode) ──────────────────────────────────────────
//
// A minimal GenerateFlowFile → LogAttribute flow used by field-presence tests.
// The flow is created once per test binary via `OnceCell` and left running.
// Each call to `ensure_test_flow` verifies the processors are still running.

#[cfg(feature = "dynamic")]
pub const TEST_FLOW_PG_NAME: &str = "nifi_rust_client_test__field_presence";
#[cfg(feature = "dynamic")]
pub const TEST_FLOW_GFF_NAME: &str = "nifi_rust_client_test__generate_flow_file";
#[cfg(feature = "dynamic")]
pub const TEST_FLOW_LA_NAME: &str = "nifi_rust_client_test__log_attribute";

#[cfg(feature = "dynamic")]
pub struct TestFlowIds {
    pub pg_id: String,
    pub gff_id: String,
    pub la_id: String,
    pub gff_revision: i64,
    pub la_revision: i64,
}

#[cfg(feature = "dynamic")]
static TEST_FLOW: tokio::sync::OnceCell<TestFlowIds> = tokio::sync::OnceCell::const_new();

/// Ensures the test flow exists and is running. Creates it on first call;
/// verifies processors are running on every call.
#[cfg(feature = "dynamic")]
pub async fn ensure_test_flow(
    client: &nifi_rust_client::dynamic::DynamicClient,
) -> &'static TestFlowIds {
    use nifi_rust_client::dynamic::types::{
        PositionDto, ProcessGroupDto, ProcessGroupEntity, RevisionDto,
    };

    let ids = TEST_FLOW
        .get_or_init(|| async {
            // ── Find or create the dedicated child PG ───────────────────────
            // Processors live in a child PG (not root) so that the cascading
            // /data/process-groups/{root} policy grants provenance visibility
            // on all NiFi versions.
            let flow = client
                .flow()
                .get_flow("root", None)
                .await
                .expect("failed to get root flow");

            let child_pgs = flow
                .process_group_flow
                .as_ref()
                .and_then(|pgf| pgf.flow.as_ref())
                .and_then(|f| f.process_groups.as_ref())
                .cloned()
                .unwrap_or_default();

            let existing_pg = child_pgs.iter().find(|pg| {
                pg.component.as_ref().and_then(|c| c.name.as_deref()) == Some(TEST_FLOW_PG_NAME)
            });

            let pg_id = if let Some(pg) = existing_pg {
                pg.id.clone().expect("test PG has no id")
            } else {
                let mut pos = PositionDto::default();
                pos.x = Some(0.0);
                pos.y = Some(0.0);
                let mut pg_dto = ProcessGroupDto::default();
                pg_dto.name = Some(TEST_FLOW_PG_NAME.to_string());
                pg_dto.position = Some(pos);
                let mut revision = RevisionDto::default();
                revision.version = Some(0);
                let mut body = ProcessGroupEntity::default();
                body.component = Some(pg_dto);
                body.revision = Some(revision);

                let created = client
                    .processgroups()
                    .create_process_group("root", None, &body)
                    .await
                    .expect("failed to create test PG");
                created.id.clone().expect("created PG has no id")
            };

            // ── Find or create processors inside the child PG ───────────────
            let pg_flow = client
                .flow()
                .get_flow(&pg_id, None)
                .await
                .expect("failed to get test PG flow");
            let processors = pg_flow
                .process_group_flow
                .as_ref()
                .and_then(|pgf| pgf.flow.as_ref())
                .and_then(|f| f.processors.as_ref())
                .cloned()
                .unwrap_or_default();

            let existing_gff = processors.iter().find(|p| {
                p.component.as_ref().and_then(|c| c.name.as_deref()) == Some(TEST_FLOW_GFF_NAME)
            });
            let existing_la = processors.iter().find(|p| {
                p.component.as_ref().and_then(|c| c.name.as_deref()) == Some(TEST_FLOW_LA_NAME)
            });

            let (gff_id, gff_rev) = if let Some(p) = existing_gff {
                (
                    p.id.clone().expect("GFF processor has no id"),
                    p.revision.as_ref().and_then(|r| r.version).unwrap_or(0),
                )
            } else {
                let body = make_processor_entity(
                    TEST_FLOW_GFF_NAME,
                    "org.apache.nifi.processors.standard.GenerateFlowFile",
                    100.0,
                    100.0,
                    Some("5 sec"),
                    None,
                );
                let created = client
                    .processgroups()
                    .create_processor(&pg_id, &body)
                    .await
                    .expect("failed to create GFF processor");
                (
                    created.id.clone().expect("created GFF has no id"),
                    created
                        .revision
                        .as_ref()
                        .and_then(|r| r.version)
                        .unwrap_or(0),
                )
            };

            let (la_id, la_rev) = if let Some(p) = existing_la {
                (
                    p.id.clone().expect("LA processor has no id"),
                    p.revision.as_ref().and_then(|r| r.version).unwrap_or(0),
                )
            } else {
                let body = make_processor_entity(
                    TEST_FLOW_LA_NAME,
                    "org.apache.nifi.processors.standard.LogAttribute",
                    500.0,
                    100.0,
                    None,
                    Some(vec!["success".to_string()]),
                );
                let created = client
                    .processgroups()
                    .create_processor(&pg_id, &body)
                    .await
                    .expect("failed to create LA processor");
                (
                    created.id.clone().expect("created LA has no id"),
                    created
                        .revision
                        .as_ref()
                        .and_then(|r| r.version)
                        .unwrap_or(0),
                )
            };

            // ── Find or create connection GFF → LA ──────────────────────────
            let connections = pg_flow
                .process_group_flow
                .as_ref()
                .and_then(|pgf| pgf.flow.as_ref())
                .and_then(|f| f.connections.as_ref())
                .cloned()
                .unwrap_or_default();

            let conn_exists = connections.iter().any(|c| {
                c.component
                    .as_ref()
                    .map(|cd| {
                        cd.source.as_ref().map(|s| s.id.as_str()) == Some(&gff_id)
                            && cd.destination.as_ref().map(|d| d.id.as_str()) == Some(&la_id)
                    })
                    .unwrap_or(false)
            });

            if !conn_exists {
                let conn_body = make_connection_entity(&gff_id, &la_id, &pg_id);
                client
                    .processgroups()
                    .create_connection(&pg_id, &conn_body)
                    .await
                    .expect("failed to create GFF→LA connection");
            }

            // ── Start processors if not running ─────────────────────────────
            for (proc_id, rev) in [(&gff_id, gff_rev), (&la_id, la_rev)] {
                start_if_not_running(client, proc_id, rev).await;
            }

            TestFlowIds {
                pg_id,
                gff_id,
                la_id,
                gff_revision: gff_rev,
                la_revision: la_rev,
            }
        })
        .await;

    // ── Per-call verify: ensure processors are still running ─────────────
    verify_test_flow_running(client, ids).await;
    ids
}

/// Quick check that the test flow processors are running; restarts if stopped.
#[cfg(feature = "dynamic")]
async fn verify_test_flow_running(
    client: &nifi_rust_client::dynamic::DynamicClient,
    ids: &TestFlowIds,
) {
    for proc_id in [&ids.gff_id, &ids.la_id] {
        start_if_not_running(client, proc_id, 0).await;
    }
}

/// Fetches a processor's state and starts it if not running.
#[cfg(feature = "dynamic")]
async fn start_if_not_running(
    client: &nifi_rust_client::dynamic::DynamicClient,
    proc_id: &str,
    fallback_rev: i64,
) {
    let proc = client
        .processors()
        .get_processor(proc_id)
        .await
        .expect("failed to get processor");
    let state = proc
        .component
        .as_ref()
        .and_then(|c| c.state.as_deref())
        .unwrap_or("UNKNOWN");
    if state != "RUNNING" {
        let rev = proc
            .revision
            .as_ref()
            .and_then(|r| r.version)
            .unwrap_or(fallback_rev);
        let mut body = nifi_rust_client::dynamic::types::ProcessorRunStatusEntity::default();
        body.state = Some("RUNNING".to_string());
        let mut revision = nifi_rust_client::dynamic::types::RevisionDto::default();
        revision.version = Some(rev);
        body.revision = Some(revision);
        client
            .processors()
            .update_run_status(proc_id, &body)
            .await
            .expect("failed to start processor");
    }
}

/// Build a ProcessorEntity for creating a processor (non-exhaustive safe).
#[cfg(feature = "dynamic")]
fn make_processor_entity(
    name: &str,
    processor_type: &str,
    x: f64,
    y: f64,
    scheduling_period: Option<&str>,
    auto_terminate: Option<Vec<String>>,
) -> nifi_rust_client::dynamic::types::ProcessorEntity {
    use nifi_rust_client::dynamic::types::{
        PositionDto, ProcessorConfigDto, ProcessorDto, ProcessorEntity, RevisionDto,
    };

    let mut pos = PositionDto::default();
    pos.x = Some(x);
    pos.y = Some(y);

    let mut config = ProcessorConfigDto::default();
    if let Some(period) = scheduling_period {
        config.scheduling_period = Some(period.to_string());
    }
    if let Some(rels) = auto_terminate {
        config.auto_terminated_relationships = Some(rels);
    }

    let mut dto = ProcessorDto::default();
    dto.name = Some(name.to_string());
    dto.r#type = Some(processor_type.to_string());
    dto.position = Some(pos);
    dto.config = Some(config);

    let mut revision = RevisionDto::default();
    revision.version = Some(0);

    let mut entity = ProcessorEntity::default();
    entity.component = Some(dto);
    entity.revision = Some(revision);
    entity
}

/// Build a ConnectionEntity for connecting two processors (non-exhaustive safe).
#[cfg(feature = "dynamic")]
fn make_connection_entity(
    source_id: &str,
    dest_id: &str,
    group_id: &str,
) -> nifi_rust_client::dynamic::types::ConnectionEntity {
    use nifi_rust_client::dynamic::types::{
        ConnectableDto, ConnectionDto, ConnectionEntity, RevisionDto,
    };

    let mut src = ConnectableDto::default();
    src.id = source_id.to_string();
    src.group_id = group_id.to_string();
    src.r#type = "PROCESSOR".to_string();

    let mut dst = ConnectableDto::default();
    dst.id = dest_id.to_string();
    dst.group_id = group_id.to_string();
    dst.r#type = "PROCESSOR".to_string();

    let mut conn_dto = ConnectionDto::default();
    conn_dto.source = Some(src);
    conn_dto.destination = Some(dst);
    conn_dto.selected_relationships = Some(vec!["success".to_string()]);

    let mut revision = RevisionDto::default();
    revision.version = Some(0);

    let mut entity = ConnectionEntity::default();
    entity.source_type = "PROCESSOR".to_string();
    entity.destination_type = "PROCESSOR".to_string();
    entity.component = Some(conn_dto);
    entity.revision = Some(revision);
    entity
}

/// Returns a `ProcessorEntity` from the test flow (the GenerateFlowFile processor).
#[cfg(feature = "dynamic")]
pub async fn get_test_processor_entity(
    client: &nifi_rust_client::dynamic::DynamicClient,
) -> nifi_rust_client::dynamic::types::ProcessorEntity {
    let ids = ensure_test_flow(client).await;
    client
        .processors()
        .get_processor(&ids.gff_id)
        .await
        .expect("failed to get test processor entity")
}

/// Returns a `ProvenanceEventDto` from the test flow. Submits a provenance query,
/// polls until finished, and retries up to 10 times (with 2s sleeps) to allow
/// the GenerateFlowFile processor to produce events.
#[cfg(feature = "dynamic")]
pub async fn get_test_provenance_event(
    client: &nifi_rust_client::dynamic::DynamicClient,
) -> nifi_rust_client::dynamic::types::ProvenanceEventDto {
    use nifi_rust_client::dynamic::types::{ProvenanceDto, ProvenanceEntity, ProvenanceRequestDto};

    let _ids = ensure_test_flow(client).await;

    // Give the flow time to generate events.
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    for attempt in 0..10 {
        let mut request = ProvenanceRequestDto::default();
        request.max_results = Some(10);
        request.incremental_results = Some(false);
        let mut prov_dto = ProvenanceDto::default();
        prov_dto.request = Some(request);
        let query_body = ProvenanceEntity {
            provenance: Some(prov_dto),
        };
        let submitted = client
            .provenance()
            .submit_provenance_request(&query_body)
            .await
            .expect("failed to submit provenance query");
        let query_id = submitted.id.clone().expect("provenance query has no id");

        // Poll until finished.
        let provenance = loop {
            let p = client
                .provenance()
                .get_provenance(&query_id, None, None, Some(false))
                .await
                .expect("failed to poll provenance query");
            if p.finished.unwrap_or(false) || p.percent_completed.unwrap_or(0) >= 100 {
                break p;
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        };

        // Clean up the query.
        let _ = client.provenance().delete_provenance(&query_id, None).await;

        if let Some(event) = provenance
            .results
            .and_then(|r| r.provenance_events)
            .and_then(|mut events| {
                if events.is_empty() {
                    None
                } else {
                    Some(events.remove(0))
                }
            })
        {
            return event;
        }

        if attempt < 9 {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }

    panic!("no provenance events found after 10 attempts — is the test flow running?");
}
