//! Operator porcelain — bulk state changes on a process group.
//!
//! Each function is a thin wrapper over a `bulk::*_dynamic` helper,
//! chosen for its value to daily operator workflow. Phase 1 does not
//! support `--wait` on these commands (requires wait helpers that do
//! not yet exist in the client library).

use nifi_rust_client::bulk;
use nifi_rust_client::dynamic::DynamicClient;
use serde_json::json;

use crate::confirm;
use crate::dry_run::{self, CliCtx};
use crate::error::CliError;
use crate::output::CliOutput;

/// Start (schedule) all authorized processors in a process group.
pub async fn start_pg(
    client: &DynamicClient,
    pg_id: &str,
    ctx: &CliCtx<'_>,
) -> Result<CliOutput, CliError> {
    if ctx.dry_run {
        let path = format!("/flow/process-groups/{pg_id}");
        let url = dry_run::format_url(ctx.base_url, &path, &[]);
        let body = json!({ "id": pg_id, "state": "RUNNING" });
        dry_run::print(&mut std::io::stdout(), "PUT", &url, Some(&body))
            .map_err(CliError::Io)?;
        return Ok(CliOutput::Empty);
    }
    let entity = bulk::start_process_group_dynamic(client, pg_id).await?;
    let value = serde_json::to_value(&entity)
        .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
    Ok(CliOutput::Single(value))
}

/// Stop (unschedule) all processors in a process group.
pub async fn stop_pg(
    client: &DynamicClient,
    pg_id: &str,
    ctx: &CliCtx<'_>,
) -> Result<CliOutput, CliError> {
    if ctx.dry_run {
        let path = format!("/flow/process-groups/{pg_id}");
        let url = dry_run::format_url(ctx.base_url, &path, &[]);
        let body = json!({ "id": pg_id, "state": "STOPPED" });
        dry_run::print(&mut std::io::stdout(), "PUT", &url, Some(&body))
            .map_err(CliError::Io)?;
        return Ok(CliOutput::Empty);
    }
    confirm::confirm_destructive(
        &format!("stop all processors in process group '{pg_id}'"),
        ctx,
    )?;
    let entity = bulk::stop_process_group_dynamic(client, pg_id).await?;
    let value = serde_json::to_value(&entity)
        .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
    Ok(CliOutput::Single(value))
}

/// Enable all authorized controller services in a process group.
pub async fn enable_services(
    client: &DynamicClient,
    pg_id: &str,
    ctx: &CliCtx<'_>,
) -> Result<CliOutput, CliError> {
    if ctx.dry_run {
        let path = format!("/flow/process-groups/{pg_id}/controller-services");
        let url = dry_run::format_url(ctx.base_url, &path, &[]);
        let body = json!({ "id": pg_id, "state": "ENABLED" });
        dry_run::print(&mut std::io::stdout(), "PUT", &url, Some(&body))
            .map_err(CliError::Io)?;
        return Ok(CliOutput::Empty);
    }
    let entity = bulk::enable_all_controller_services_dynamic(client, pg_id).await?;
    let value = serde_json::to_value(&entity)
        .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
    Ok(CliOutput::Single(value))
}

/// Disable all controller services in a process group.
pub async fn disable_services(
    client: &DynamicClient,
    pg_id: &str,
    ctx: &CliCtx<'_>,
) -> Result<CliOutput, CliError> {
    if ctx.dry_run {
        let path = format!("/flow/process-groups/{pg_id}/controller-services");
        let url = dry_run::format_url(ctx.base_url, &path, &[]);
        let body = json!({ "id": pg_id, "state": "DISABLED" });
        dry_run::print(&mut std::io::stdout(), "PUT", &url, Some(&body))
            .map_err(CliError::Io)?;
        return Ok(CliOutput::Empty);
    }
    confirm::confirm_destructive(
        &format!("disable all controller services in process group '{pg_id}'"),
        ctx,
    )?;
    let entity = bulk::disable_all_controller_services_dynamic(client, pg_id).await?;
    let value = serde_json::to_value(&entity)
        .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
    Ok(CliOutput::Single(value))
}

#[cfg(test)]
mod tests {
    use nifi_rust_client::NifiClientBuilder;
    use nifi_rust_client::dynamic::DynamicClient;
    use serde_json::json;
    use wiremock::matchers::{body_partial_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn dynamic_client_on(mock: &MockServer, version: &str) -> DynamicClient {
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/about"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "about": { "title": "NiFi", "version": version }
            })))
            .mount(mock)
            .await;
        let client = NifiClientBuilder::new(&mock.uri()).unwrap().build().unwrap();
        DynamicClient::from_client(client).await.unwrap()
    }

    #[tokio::test]
    async fn start_pg_sends_running_body() {
        let mock = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-1"))
            .and(body_partial_json(json!({ "id": "pg-1", "state": "RUNNING" })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-1", "state": "RUNNING"
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx { dry_run: false, yes: true, base_url: &base_url };
        let result = super::start_pg(&client, "pg-1", &ctx).await.unwrap();
        match result {
            crate::output::CliOutput::Single(v) => {
                assert_eq!(v.get("state").and_then(|s| s.as_str()), Some("RUNNING"));
            }
            _ => panic!("expected Single"),
        }
    }

    #[tokio::test]
    async fn stop_pg_sends_stopped_body() {
        let mock = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-2"))
            .and(body_partial_json(json!({ "id": "pg-2", "state": "STOPPED" })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-2", "state": "STOPPED"
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx { dry_run: false, yes: true, base_url: &base_url };
        let result = super::stop_pg(&client, "pg-2", &ctx).await.unwrap();
        match result {
            crate::output::CliOutput::Single(v) => {
                assert_eq!(v.get("state").and_then(|s| s.as_str()), Some("STOPPED"));
            }
            _ => panic!("expected Single"),
        }
    }

    #[tokio::test]
    async fn enable_services_sends_enabled_body() {
        let mock = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-3/controller-services"))
            .and(body_partial_json(json!({ "id": "pg-3", "state": "ENABLED" })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-3", "state": "ENABLED"
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx { dry_run: false, yes: true, base_url: &base_url };
        let result = super::enable_services(&client, "pg-3", &ctx).await.unwrap();
        match result {
            crate::output::CliOutput::Single(v) => {
                assert_eq!(v.get("state").and_then(|s| s.as_str()), Some("ENABLED"));
            }
            _ => panic!("expected Single"),
        }
    }

    #[tokio::test]
    async fn disable_services_sends_disabled_body() {
        let mock = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-4/controller-services"))
            .and(body_partial_json(json!({ "id": "pg-4", "state": "DISABLED" })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-4", "state": "DISABLED"
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx { dry_run: false, yes: true, base_url: &base_url };
        let result = super::disable_services(&client, "pg-4", &ctx).await.unwrap();
        match result {
            crate::output::CliOutput::Single(v) => {
                assert_eq!(v.get("state").and_then(|s| s.as_str()), Some("DISABLED"));
            }
            _ => panic!("expected Single"),
        }
    }

    #[tokio::test]
    async fn start_pg_dry_run_does_not_hit_server() {
        let mock = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-1"))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx { dry_run: true, yes: false, base_url: &base_url };

        let result = super::start_pg(&client, "pg-1", &ctx).await.unwrap();
        assert!(matches!(result, crate::output::CliOutput::Empty));
    }

    #[tokio::test]
    async fn stop_pg_without_yes_in_non_tty_refuses() {
        let mock = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-2"))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx { dry_run: false, yes: false, base_url: &base_url };

        let err = super::stop_pg(&client, "pg-2", &ctx).await.unwrap_err();
        match err {
            crate::error::CliError::User(msg) => {
                assert!(msg.contains("--yes"), "message should mention --yes: {msg}");
                assert!(msg.contains("non-interactive"), "should mention non-interactive: {msg}");
            }
            other => panic!("expected User, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn stop_pg_with_yes_bypasses_prompt_and_hits_server() {
        let mock = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-3"))
            .and(body_partial_json(json!({ "id": "pg-3", "state": "STOPPED" })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-3", "state": "STOPPED"
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx { dry_run: false, yes: true, base_url: &base_url };

        super::stop_pg(&client, "pg-3", &ctx).await.unwrap();
    }

    #[tokio::test]
    async fn disable_services_without_yes_in_non_tty_refuses() {
        let mock = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-4/controller-services"))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx { dry_run: false, yes: false, base_url: &base_url };

        let err = super::disable_services(&client, "pg-4", &ctx).await.unwrap_err();
        match err {
            crate::error::CliError::User(msg) => {
                assert!(msg.contains("--yes"));
            }
            other => panic!("expected User, got {other:?}"),
        }
    }
}
