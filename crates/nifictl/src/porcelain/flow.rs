//! Flow portability porcelain — export, import, replace.
//!
//! Each fn is a thin wrapper over the generated `processgroups()` method
//! plus the existing `bulk::*_dynamic` helpers. Dry-run branches print
//! the would-be request via `dry_run::print`; destructive commands run
//! `confirm::confirm_destructive` at the point they would send real
//! bytes.

use std::io::Write;
use std::path::Path;

use nifi_rust_client::dynamic::DynamicClient;
use serde_json::Value;

use crate::confirm;
use crate::dry_run::{self, CliCtx};
use crate::error::CliError;
use crate::output::CliOutput;

/// Export a process group's flow snapshot to `output` (stdout if `None`).
///
/// `--dry-run` prints the GET URL and exits with `CliOutput::Empty`.
pub async fn export(
    client: &DynamicClient,
    pg_id: &str,
    output: Option<&Path>,
    include_referenced_services: bool,
    ctx: &CliCtx<'_>,
) -> Result<CliOutput, CliError> {
    if ctx.dry_run {
        let path = format!("/process-groups/{pg_id}/download");
        let query: Vec<(&str, String)> = if include_referenced_services {
            vec![("includeReferencedServices", "true".to_string())]
        } else {
            Vec::new()
        };
        let url = dry_run::format_url(ctx.base_url, &path, &query);
        dry_run::print(&mut std::io::stdout(), "GET", &url, None).map_err(CliError::Io)?;
        return Ok(CliOutput::Empty);
    }

    let snapshot: Value = client
        .processgroups()
        .export_process_group(pg_id, Some(include_referenced_services))
        .await?;

    match output {
        None => Ok(CliOutput::Single(snapshot)),
        Some(path) => {
            let pretty = serde_json::to_string_pretty(&snapshot)
                .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
            let mut file = std::fs::File::create(path).map_err(CliError::Io)?;
            file.write_all(pretty.as_bytes()).map_err(CliError::Io)?;
            file.write_all(b"\n").map_err(CliError::Io)?;
            Ok(CliOutput::Empty)
        }
    }
}

/// Import a snapshot file as a new child process group under `parent_pg_id`.
///
/// The new child's name is `name` if provided, else the file's stem.
/// Non-destructive — no confirmation prompt.
pub async fn import(
    client: &DynamicClient,
    parent_pg_id: &str,
    file: &Path,
    name: Option<&str>,
    ctx: &CliCtx<'_>,
) -> Result<CliOutput, CliError> {
    let group_name = name
        .map(str::to_string)
        .or_else(|| {
            file.file_stem()
                .and_then(|s| s.to_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "imported".to_string());

    if ctx.dry_run {
        let path = format!("/process-groups/{parent_pg_id}/process-groups/upload");
        let url = dry_run::format_url(ctx.base_url, &path, &[]);
        // A full snapshot can be megabytes; summarise the multipart parts
        // rather than dumping the whole body.
        let summary = serde_json::json!({
            "multipart": {
                "file": format!("<contents of {}>", file.display()),
                "clientId": "nifictl",
                "groupName": group_name,
                "positionX": 0.0,
                "positionY": 0.0,
            }
        });
        dry_run::print(&mut std::io::stdout(), "POST", &url, Some(&summary))
            .map_err(CliError::Io)?;
        return Ok(CliOutput::Empty);
    }

    let data = std::fs::read(file)
        .map_err(|e| CliError::User(format!("failed to read {}: {e}", file.display())))?;
    let filename = file
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("snapshot.json");

    let entity = client
        .processgroups()
        .upload_process_group(
            parent_pg_id,
            /* client_id */ "nifictl",
            /* disconnected_node_acknowledged */ None,
            /* group_name */ &group_name,
            /* position_x */ 0.0,
            /* position_y */ 0.0,
            filename,
            data,
        )
        .await?;

    let value = serde_json::to_value(&entity)
        .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
    Ok(CliOutput::Single(value))
}

/// Short prompt description, shared with main.rs's pre-client confirm gate.
pub fn replace_what(pg_id: &str) -> String {
    format!("replace contents of process group '{pg_id}'")
}

/// Overwrite a process group's contents with a snapshot file.
///
/// **Destructive.** Runs `confirm::confirm_destructive` internally as
/// defense-in-depth; production callers in `main.rs` run the confirm
/// gate before touching the client so the internal call is a no-op when
/// `ctx.yes = true`. `stop_first` handled in Task 9.
pub async fn replace(
    client: &DynamicClient,
    pg_id: &str,
    file: &Path,
    stop_first: bool,
    ctx: &CliCtx<'_>,
) -> Result<CliOutput, CliError> {
    if ctx.dry_run {
        if stop_first {
            // 0. stop
            let stop_url =
                dry_run::format_url(ctx.base_url, &format!("/flow/process-groups/{pg_id}"), &[]);
            let stop_body = serde_json::json!({ "id": pg_id, "state": "STOPPED" });
            dry_run::print(&mut std::io::stdout(), "PUT", &stop_url, Some(&stop_body))
                .map_err(CliError::Io)?;
        }
        // 1. GET the target to show where the revision comes from.
        let get_url = dry_run::format_url(ctx.base_url, &format!("/process-groups/{pg_id}"), &[]);
        dry_run::print(&mut std::io::stdout(), "GET", &get_url, None).map_err(CliError::Io)?;
        // 2. PUT the replace — summarise rather than inlining the snapshot.
        let put_url = dry_run::format_url(
            ctx.base_url,
            &format!("/process-groups/{pg_id}/flow-contents"),
            &[],
        );
        let put_body = serde_json::json!({
            "disconnectedNodeAcknowledged": false,
            "processGroupRevision": { "version": "<fetched from GET above>" },
            "versionedFlowSnapshot": format!("<contents of {}>", file.display()),
        });
        dry_run::print(&mut std::io::stdout(), "PUT", &put_url, Some(&put_body))
            .map_err(CliError::Io)?;
        if stop_first {
            // 3. start
            let start_url =
                dry_run::format_url(ctx.base_url, &format!("/flow/process-groups/{pg_id}"), &[]);
            let start_body = serde_json::json!({ "id": pg_id, "state": "RUNNING" });
            dry_run::print(&mut std::io::stdout(), "PUT", &start_url, Some(&start_body))
                .map_err(CliError::Io)?;
        }
        return Ok(CliOutput::Empty);
    }

    confirm::confirm_destructive(&replace_what(pg_id), ctx)?;

    // 0. Stop (if --stop-first).
    if stop_first {
        nifi_rust_client::bulk::stop_process_group_dynamic(client, pg_id).await?;
    }

    // 1. Fetch current revision.
    let pg = client.processgroups().get_process_group(pg_id).await?;
    let revision = pg.revision.ok_or_else(|| {
        CliError::User(format!(
            "process group '{pg_id}' response had no revision field"
        ))
    })?;

    // 2. Read and parse the snapshot.
    let content = std::fs::read_to_string(file)
        .map_err(|e| CliError::User(format!("failed to read {}: {e}", file.display())))?;
    let snapshot: nifi_rust_client::dynamic::types::RegisteredFlowSnapshot =
        serde_json::from_str(&content).map_err(|e| {
            CliError::User(format!("invalid snapshot JSON in {}: {e}", file.display()))
        })?;

    // 3. Assemble the ProcessGroupImportEntity and PUT.
    // `ProcessGroupImportEntity` is `#[non_exhaustive]`; construct via
    // `Default::default()` + field assignments.
    let mut body = nifi_rust_client::dynamic::types::ProcessGroupImportEntity::default();
    body.disconnected_node_acknowledged = Some(false);
    body.process_group_revision = Some(revision);
    body.versioned_flow_snapshot = Some(snapshot);
    let entity = client
        .processgroups()
        .replace_process_group(pg_id, &body)
        .await?;

    // 4. Restart (if --stop-first). A failure here does NOT undo the replace;
    //    surface a distinct error so the operator sees the partial state.
    if stop_first
        && let Err(e) = nifi_rust_client::bulk::start_process_group_dynamic(client, pg_id).await
    {
        return Err(CliError::User(format!(
            "flow replaced, but restart failed: {e}"
        )));
    }

    let value = serde_json::to_value(&entity)
        .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
    Ok(CliOutput::Single(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nifi_rust_client::NifiClientBuilder;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn dynamic_client_on(mock: &MockServer, version: &str) -> DynamicClient {
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/about"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "about": { "title": "NiFi", "version": version }
            })))
            .mount(mock)
            .await;
        let client = NifiClientBuilder::new(&mock.uri())
            .unwrap()
            .build()
            .unwrap();
        DynamicClient::from_client(client).await.unwrap()
    }

    #[tokio::test]
    async fn export_to_stdout_returns_single_value() {
        let mock = MockServer::start().await;
        let snapshot = json!({ "flowContents": { "name": "root" } });
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/pg-1/download"))
            .respond_with(ResponseTemplate::new(200).set_body_json(snapshot.clone()))
            .expect(1)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = CliCtx {
            dry_run: false,
            yes: false,
            base_url: &base_url,
        };
        let result = export(&client, "pg-1", None, false, &ctx).await.unwrap();
        match result {
            CliOutput::Single(v) => assert_eq!(v, snapshot),
            _ => panic!("expected Single"),
        }
    }

    #[tokio::test]
    async fn export_to_file_writes_pretty_json() {
        let mock = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/pg-2/download"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "flowContents": { "name": "x" }
            })))
            .mount(&mock)
            .await;
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("export.json");
        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = CliCtx {
            dry_run: false,
            yes: false,
            base_url: &base_url,
        };
        let result = export(&client, "pg-2", Some(&out), false, &ctx)
            .await
            .unwrap();
        assert!(matches!(result, CliOutput::Empty));
        let body = std::fs::read_to_string(&out).unwrap();
        assert!(body.contains("\"flowContents\""));
        assert!(body.ends_with('\n'));
    }

    #[tokio::test]
    async fn export_dry_run_does_not_hit_server() {
        let mock = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/pg-3/download"))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(&mock)
            .await;
        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = CliCtx {
            dry_run: true,
            yes: false,
            base_url: &base_url,
        };
        let result = export(&client, "pg-3", None, false, &ctx).await.unwrap();
        assert!(matches!(result, CliOutput::Empty));
    }

    #[tokio::test]
    async fn import_reads_file_and_posts_upload() {
        let mock = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/process-groups/parent-1/process-groups/upload",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "new-child",
                "component": { "name": "imported" }
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let snap = dir.path().join("snap.json");
        std::fs::write(&snap, r#"{"flowContents": {"name": "x"}}"#).unwrap();

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = CliCtx {
            dry_run: false,
            yes: false,
            base_url: &base_url,
        };
        let result = import(&client, "parent-1", &snap, Some("imported"), &ctx)
            .await
            .unwrap();
        match result {
            CliOutput::Single(v) => {
                assert_eq!(v.get("id").and_then(|v| v.as_str()), Some("new-child"));
            }
            _ => panic!("expected Single"),
        }
    }

    #[tokio::test]
    async fn import_dry_run_does_not_read_file_or_hit_server() {
        let mock = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/nifi-api/process-groups/parent-2/process-groups/upload",
            ))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = CliCtx {
            dry_run: true,
            yes: false,
            base_url: &base_url,
        };
        // Pass a path that does NOT exist — dry-run must NOT try to read it.
        let bogus = std::path::Path::new("/tmp/definitely-does-not-exist-task7.json");
        let result = import(&client, "parent-2", bogus, Some("imported"), &ctx)
            .await
            .unwrap();
        assert!(matches!(result, CliOutput::Empty));
    }

    #[tokio::test]
    async fn replace_gets_revision_then_puts_import_entity() {
        let mock = MockServer::start().await;
        // GET current revision
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/pg-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-1",
                "revision": { "version": 42 },
                "component": {}
            })))
            .expect(1)
            .mount(&mock)
            .await;
        // PUT replace
        Mock::given(method("PUT"))
            .and(path("/nifi-api/process-groups/pg-1/flow-contents"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "processGroupRevision": { "version": 43 },
                "versionedFlowSnapshot": { "flowContents": {} }
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let snap = dir.path().join("snap.json");
        std::fs::write(&snap, r#"{"flowContents": {"name": "x"}}"#).unwrap();

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = CliCtx {
            dry_run: false,
            yes: true,
            base_url: &base_url,
        };
        let result = replace(&client, "pg-1", &snap, false, &ctx).await.unwrap();
        match result {
            CliOutput::Single(v) => {
                assert_eq!(
                    v.pointer("/processGroupRevision/version")
                        .and_then(|v| v.as_i64()),
                    Some(43)
                );
            }
            _ => panic!("expected Single"),
        }
    }

    #[tokio::test]
    async fn replace_dry_run_prints_get_and_put_without_hitting_server() {
        let mock = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/pg-2"))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(&mock)
            .await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/process-groups/pg-2/flow-contents"))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(&mock)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let snap = dir.path().join("snap.json");
        std::fs::write(&snap, r#"{}"#).unwrap();

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = CliCtx {
            dry_run: true,
            yes: true,
            base_url: &base_url,
        };
        let result = replace(&client, "pg-2", &snap, false, &ctx).await.unwrap();
        assert!(matches!(result, CliOutput::Empty));
    }

    #[tokio::test]
    async fn replace_without_yes_in_non_tty_refuses() {
        let mock = MockServer::start().await;
        // No mocks registered — any HTTP call would fail.
        let dir = tempfile::tempdir().unwrap();
        let snap = dir.path().join("snap.json");
        std::fs::write(&snap, r#"{}"#).unwrap();

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = CliCtx {
            dry_run: false,
            yes: false,
            base_url: &base_url,
        };
        let err = replace(&client, "pg-3", &snap, false, &ctx)
            .await
            .unwrap_err();
        match err {
            CliError::User(msg) => {
                assert!(msg.contains("--yes"), "msg should mention --yes: {msg}")
            }
            other => panic!("expected User, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn replace_stop_first_happy_path_sends_four_requests() {
        let mock = MockServer::start().await;
        // 1. stop
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-1"))
            .and(wiremock::matchers::body_partial_json(
                json!({ "id": "pg-1", "state": "STOPPED" }),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-1", "state": "STOPPED"
            })))
            .expect(1)
            .mount(&mock)
            .await;
        // 2. GET revision
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/pg-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-1",
                "revision": { "version": 7 },
                "component": {}
            })))
            .expect(1)
            .mount(&mock)
            .await;
        // 3. PUT replace
        Mock::given(method("PUT"))
            .and(path("/nifi-api/process-groups/pg-1/flow-contents"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "processGroupRevision": { "version": 8 }
            })))
            .expect(1)
            .mount(&mock)
            .await;
        // 4. start
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-1"))
            .and(wiremock::matchers::body_partial_json(
                json!({ "id": "pg-1", "state": "RUNNING" }),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-1", "state": "RUNNING"
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let snap = dir.path().join("snap.json");
        std::fs::write(&snap, r#"{"flowContents": {}}"#).unwrap();

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = CliCtx {
            dry_run: false,
            yes: true,
            base_url: &base_url,
        };
        let result = replace(&client, "pg-1", &snap, true, &ctx).await.unwrap();
        assert!(matches!(result, CliOutput::Single(_)));
    }

    #[tokio::test]
    async fn replace_stop_first_restart_failure_surfaces_clear_error() {
        let mock = MockServer::start().await;
        // stop OK
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-2"))
            .and(wiremock::matchers::body_partial_json(
                json!({ "state": "STOPPED" }),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-2", "state": "STOPPED"
            })))
            .expect(1)
            .mount(&mock)
            .await;
        // GET revision OK
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/pg-2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-2",
                "revision": { "version": 1 }
            })))
            .expect(1)
            .mount(&mock)
            .await;
        // PUT replace OK
        Mock::given(method("PUT"))
            .and(path("/nifi-api/process-groups/pg-2/flow-contents"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "processGroupRevision": { "version": 2 }
            })))
            .expect(1)
            .mount(&mock)
            .await;
        // start FAILS with 409
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-2"))
            .and(wiremock::matchers::body_partial_json(
                json!({ "state": "RUNNING" }),
            ))
            .respond_with(ResponseTemplate::new(409).set_body_string("some component invalid"))
            .expect(1)
            .mount(&mock)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let snap = dir.path().join("snap.json");
        std::fs::write(&snap, r#"{"flowContents": {}}"#).unwrap();

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = CliCtx {
            dry_run: false,
            yes: true,
            base_url: &base_url,
        };
        let err = replace(&client, "pg-2", &snap, true, &ctx)
            .await
            .unwrap_err();
        match err {
            CliError::User(msg) => {
                assert!(
                    msg.starts_with("flow replaced, but restart failed:"),
                    "message should surface partial success: {msg}"
                );
            }
            other => panic!("expected User, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn replace_stop_first_dry_run_does_not_hit_server() {
        let mock = MockServer::start().await;
        Mock::given(method("PUT"))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(&mock)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let snap = dir.path().join("snap.json");
        std::fs::write(&snap, r#"{}"#).unwrap();

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = CliCtx {
            dry_run: true,
            yes: true,
            base_url: &base_url,
        };
        let result = replace(&client, "pg-3", &snap, true, &ctx).await.unwrap();
        assert!(matches!(result, CliOutput::Empty));
    }
}
