//! Operator porcelain — bulk state changes on a process group.
//!
//! Each function is a thin wrapper over a `bulk::*_dynamic` helper,
//! chosen for its value to daily operator workflow. With `--wait`, the
//! dispatch layer follows the action with [`wait_for_ops`], which polls
//! the process group until the requested state is reached.

use nifi_rust_client::bulk;
use nifi_rust_client::dynamic::DynamicClient;
use nifi_rust_client::wait::{
    self, ControllerServiceTargetState, ProcessGroupTargetState, WaitConfig,
};
use serde_json::json;

use crate::cli::OpsCommand;
use crate::dry_run::{self, CliCtx};
use crate::error::CliError;
use crate::output::CliOutput;

/// The confirmation prompt description for `stop_pg`. Shared between
/// `main.rs`'s pre-client confirm and the porcelain's internal confirm
/// so the text stays in sync.
pub fn stop_pg_what(pg_id: &str) -> String {
    format!("stop all processors in process group '{pg_id}'")
}

/// The confirmation prompt description for `disable_services`.
pub fn disable_services_what(pg_id: &str) -> String {
    format!("disable all controller services in process group '{pg_id}'")
}

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
        dry_run::print(&mut std::io::stdout(), "PUT", &url, Some(&body)).map_err(CliError::Io)?;
        return Ok(CliOutput::Empty);
    }
    let entity = bulk::start_process_group_dynamic(client, pg_id).await?;
    let value = serde_json::to_value(&entity)
        .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
    Ok(CliOutput::Single(value))
}

/// Stop (unschedule) all processors in a process group.
///
/// **Confirm contract:** the `Commands::Ops` arm in `dispatch.rs` runs
/// [`crate::confirm::run_for_ops`] before reaching this function. There
/// is no internal gate — direct callers (e.g. tests, future library use)
/// are responsible for gating themselves; the unit tests in this module
/// pass `ctx.yes = true` to make this explicit.
pub async fn stop_pg(
    client: &DynamicClient,
    pg_id: &str,
    ctx: &CliCtx<'_>,
) -> Result<CliOutput, CliError> {
    if ctx.dry_run {
        let path = format!("/flow/process-groups/{pg_id}");
        let url = dry_run::format_url(ctx.base_url, &path, &[]);
        let body = json!({ "id": pg_id, "state": "STOPPED" });
        dry_run::print(&mut std::io::stdout(), "PUT", &url, Some(&body)).map_err(CliError::Io)?;
        return Ok(CliOutput::Empty);
    }
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
        dry_run::print(&mut std::io::stdout(), "PUT", &url, Some(&body)).map_err(CliError::Io)?;
        return Ok(CliOutput::Empty);
    }
    let entity = bulk::enable_all_controller_services_dynamic(client, pg_id).await?;
    let value = serde_json::to_value(&entity)
        .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
    Ok(CliOutput::Single(value))
}

/// Disable all controller services in a process group.
///
/// **Confirm contract:** the `Commands::Ops` arm in `dispatch.rs` runs
/// [`crate::confirm::run_for_ops`] before reaching this function — see
/// [`stop_pg`] for the full contract.
pub async fn disable_services(
    client: &DynamicClient,
    pg_id: &str,
    ctx: &CliCtx<'_>,
) -> Result<CliOutput, CliError> {
    if ctx.dry_run {
        let path = format!("/flow/process-groups/{pg_id}/controller-services");
        let url = dry_run::format_url(ctx.base_url, &path, &[]);
        let body = json!({ "id": pg_id, "state": "DISABLED" });
        dry_run::print(&mut std::io::stdout(), "PUT", &url, Some(&body)).map_err(CliError::Io)?;
        return Ok(CliOutput::Empty);
    }
    let entity = bulk::disable_all_controller_services_dynamic(client, pg_id).await?;
    let value = serde_json::to_value(&entity)
        .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
    Ok(CliOutput::Single(value))
}

/// Poll the process group until the state requested by `cmd` is reached.
///
/// Called by the dispatch layer after the bulk action when `--wait` is set
/// (never on `--dry-run`). Each `ops` subcommand maps to the matching
/// process-group wait helper:
///
/// - `start-pg` / `stop-pg` → [`wait::process_group_state_dynamic`]
///   (`Running` / `Stopped`).
/// - `enable-services` / `disable-services` →
///   [`wait::process_group_controller_services_state_dynamic`]
///   (`Enabled` / `Disabled`).
///
/// Returns `NifiError::Timeout` (mapped to [`CliError`]) if the state is not
/// reached within `config.timeout`.
pub async fn wait_for_ops(
    client: &DynamicClient,
    cmd: &OpsCommand,
    config: WaitConfig,
) -> Result<(), CliError> {
    match cmd {
        OpsCommand::StartPg { pg_id } => {
            wait::process_group_state_dynamic(
                client,
                pg_id,
                ProcessGroupTargetState::Running,
                config,
            )
            .await?;
        }
        OpsCommand::StopPg { pg_id } => {
            wait::process_group_state_dynamic(
                client,
                pg_id,
                ProcessGroupTargetState::Stopped,
                config,
            )
            .await?;
        }
        OpsCommand::EnableServices { pg_id } => {
            wait::process_group_controller_services_state_dynamic(
                client,
                pg_id,
                ControllerServiceTargetState::Enabled,
                config,
            )
            .await?;
        }
        OpsCommand::DisableServices { pg_id } => {
            wait::process_group_controller_services_state_dynamic(
                client,
                pg_id,
                ControllerServiceTargetState::Disabled,
                config,
            )
            .await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use nifi_rust_client::NifiClientBuilder;
    use nifi_rust_client::dynamic::DynamicClient;
    use nifi_rust_client::wait::WaitConfig;
    use serde_json::json;
    use wiremock::matchers::{body_partial_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::cli::OpsCommand;

    fn fast_config(timeout_ms: u64) -> WaitConfig {
        WaitConfig {
            timeout: Duration::from_millis(timeout_ms),
            poll_interval: Duration::from_millis(10),
            ..Default::default()
        }
    }

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
    async fn start_pg_sends_running_body() {
        let mock = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-1"))
            .and(body_partial_json(
                json!({ "id": "pg-1", "state": "RUNNING" }),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-1", "state": "RUNNING"
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx {
            dry_run: false,
            yes: true,
            base_url: &base_url,
        };
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
            .and(body_partial_json(
                json!({ "id": "pg-2", "state": "STOPPED" }),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-2", "state": "STOPPED"
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx {
            dry_run: false,
            yes: true,
            base_url: &base_url,
        };
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
            .and(path(
                "/nifi-api/flow/process-groups/pg-3/controller-services",
            ))
            .and(body_partial_json(
                json!({ "id": "pg-3", "state": "ENABLED" }),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-3", "state": "ENABLED"
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx {
            dry_run: false,
            yes: true,
            base_url: &base_url,
        };
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
            .and(path(
                "/nifi-api/flow/process-groups/pg-4/controller-services",
            ))
            .and(body_partial_json(
                json!({ "id": "pg-4", "state": "DISABLED" }),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-4", "state": "DISABLED"
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx {
            dry_run: false,
            yes: true,
            base_url: &base_url,
        };
        let result = super::disable_services(&client, "pg-4", &ctx)
            .await
            .unwrap();
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
        let ctx = crate::dry_run::CliCtx {
            dry_run: true,
            yes: false,
            base_url: &base_url,
        };

        let result = super::start_pg(&client, "pg-1", &ctx).await.unwrap();
        assert!(matches!(result, crate::output::CliOutput::Empty));
    }

    #[tokio::test]
    async fn stop_pg_with_yes_bypasses_prompt_and_hits_server() {
        let mock = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-3"))
            .and(body_partial_json(
                json!({ "id": "pg-3", "state": "STOPPED" }),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-3", "state": "STOPPED"
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let base_url = mock.uri();
        let ctx = crate::dry_run::CliCtx {
            dry_run: false,
            yes: true,
            base_url: &base_url,
        };

        super::stop_pg(&client, "pg-3", &ctx).await.unwrap();
    }

    #[tokio::test]
    async fn wait_for_ops_start_pg_resolves_when_running() {
        let mock = MockServer::start().await;
        // process_group_state_dynamic(Running) is satisfied by stopped_count == 0.
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/pg-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-1", "runningCount": 2, "stoppedCount": 0
            })))
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let cmd = OpsCommand::StartPg {
            pg_id: "pg-1".to_string(),
        };
        super::wait_for_ops(&client, &cmd, fast_config(1000))
            .await
            .expect("start-pg wait should resolve");
    }

    #[tokio::test]
    async fn wait_for_ops_disable_services_resolves_when_disabled() {
        let mock = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/nifi-api/flow/process-groups/pg-2/controller-services",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "controllerServices": [
                    { "component": { "state": "DISABLED", "validationStatus": "VALID" } }
                ]
            })))
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let cmd = OpsCommand::DisableServices {
            pg_id: "pg-2".to_string(),
        };
        super::wait_for_ops(&client, &cmd, fast_config(1000))
            .await
            .expect("disable-services wait should resolve");
    }

    #[tokio::test]
    async fn wait_for_ops_times_out_when_state_never_reached() {
        let mock = MockServer::start().await;
        // stopped_count stays non-zero → Running target never satisfied.
        Mock::given(method("GET"))
            .and(path("/nifi-api/process-groups/pg-9"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-9", "runningCount": 1, "stoppedCount": 3
            })))
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let cmd = OpsCommand::StartPg {
            pg_id: "pg-9".to_string(),
        };
        let err = super::wait_for_ops(&client, &cmd, fast_config(50))
            .await
            .expect_err("should time out");
        let rendered = format!("{err}");
        assert!(
            rendered.to_lowercase().contains("timed out")
                || rendered.to_lowercase().contains("timeout"),
            "expected a timeout error, got: {rendered}"
        );
    }
}
