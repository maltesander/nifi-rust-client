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

use crate::dry_run::{self, CliCtx};
use crate::error::CliError;
use crate::output::CliOutput;

/// Export a process group's flow snapshot to `output` (stdout if `None`).
///
/// `--dry-run` prints the GET URL and exits with `CliOutput::Empty`.
// Wired in by Task 10 (FlowCommand → main.rs); suppress dead_code until then.
#[allow(dead_code)]
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
}
