//! Wait-flag plumbing for nifictl.
//!
//! Holds helpers for parsing `--wait-timeout` and the pre-/post-dispatch
//! logic for attaching `--wait` semantics to supported generated commands.

use std::path::Path;
use std::time::Duration;

use nifi_rust_client::dynamic::DynamicClient;
use nifi_rust_client::wait::{self, ProcessorTargetState, WaitConfig};
use serde_json::Value;

use crate::body;
use crate::error::CliError;
use crate::output::CliOutput;

/// Parse a `--wait-timeout` argument like `"30s"`, `"2m"`, `"1500ms"`.
pub fn parse_wait_timeout(raw: &str) -> Result<Duration, CliError> {
    humantime::parse_duration(raw).map_err(|e| {
        CliError::User(format!("invalid --wait-timeout '{raw}': {e}"))
    })
}

/// A post-dispatch wait plan, produced by peeking at a generated resource
/// before `dispatch_generated` takes ownership.
pub enum WaitPlan {
    /// Wait for a processor to reach a target state.
    ProcessorState {
        processor_id: String,
        target: ProcessorTargetState,
    },
}

/// Inspect a `processors update-run-status` body and extract the target state.
///
/// Returns `Ok(Some(...))` for steady states: `RUNNING`, `STOPPED`, `DISABLED`.
/// Rejects `RUN_ONCE` with a descriptive error since it has no steady state to converge on.
pub fn processor_target_from_body(
    inline: Option<&str>,
    body_file: Option<&Path>,
) -> Result<Option<ProcessorTargetState>, CliError> {
    let value = body::resolve_body(inline, body_file)?;
    let Some(state) = value.get("state").and_then(Value::as_str) else {
        return Err(CliError::User(
            "processors update-run-status --wait requires a body with a 'state' field".to_string(),
        ));
    };
    match state {
        "RUNNING" => Ok(Some(ProcessorTargetState::Running)),
        "STOPPED" => Ok(Some(ProcessorTargetState::Stopped)),
        "DISABLED" => Ok(Some(ProcessorTargetState::Disabled)),
        "RUN_ONCE" => Err(CliError::User(
            "--wait on 'RUN_ONCE' has no effect — RUN_ONCE is a transient state with no steady state to converge on; remove --wait"
                .to_string(),
        )),
        other => Err(CliError::User(format!(
            "unsupported processor state for --wait: '{other}' \
             (expected RUNNING, STOPPED, or DISABLED)"
        ))),
    }
}

/// Inspect a `GeneratedResource` and produce a `WaitPlan` if the specific
/// subcommand is one of the ones we support with `--wait`. Returns
/// `Ok(None)` for any other subcommand.
pub fn peek_wait_plan(
    resource: &crate::generated::GeneratedResource,
) -> Result<Option<WaitPlan>, CliError> {
    use crate::generated::{GeneratedResource, ProcessorsCommand};

    if let GeneratedResource::Processors { command } = resource
        && let ProcessorsCommand::UpdateRunStatus(args) = command
    {
        let target = processor_target_from_body(
            args.body.as_deref(),
            args.body_file.as_deref(),
        )?;
        return Ok(target.map(|t| WaitPlan::ProcessorState {
            processor_id: args.id.clone(),
            target: t,
        }));
    }
    Ok(None)
}

/// Run a prepared wait plan. Returns the final `CliOutput` produced by the
/// wait helper, replacing the initial dispatch response.
pub async fn run_wait_plan(
    plan: WaitPlan,
    client: &DynamicClient,
    timeout: Duration,
) -> Result<CliOutput, CliError> {
    let config = WaitConfig {
        timeout,
        ..WaitConfig::default()
    };
    match plan {
        WaitPlan::ProcessorState { processor_id, target } => {
            let entity = wait::processor_state_dynamic(client, &processor_id, target, config).await?;
            let value = serde_json::to_value(&entity)
                .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
            Ok(CliOutput::Single(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_seconds() {
        assert_eq!(parse_wait_timeout("30s").unwrap(), Duration::from_secs(30));
    }

    #[test]
    fn parses_minutes() {
        assert_eq!(parse_wait_timeout("2m").unwrap(), Duration::from_secs(120));
    }

    #[test]
    fn parses_milliseconds() {
        assert_eq!(
            parse_wait_timeout("1500ms").unwrap(),
            Duration::from_millis(1500)
        );
    }

    #[test]
    fn rejects_bare_integer() {
        let err = parse_wait_timeout("30").unwrap_err();
        match err {
            CliError::User(msg) => {
                assert!(msg.contains("invalid --wait-timeout"));
                assert!(msg.contains("'30'"), "message should include the rejected value: {msg}");
            }
            other => panic!("expected User error, got {other:?}"),
        }
    }

    #[test]
    fn rejects_empty_string() {
        let err = parse_wait_timeout("").unwrap_err();
        match err {
            CliError::User(msg) => assert!(msg.contains("invalid --wait-timeout")),
            other => panic!("expected User error, got {other:?}"),
        }
    }

    // --- processor_target_from_body ---

    #[test]
    fn processor_target_from_body_parses_running() {
        let out = super::processor_target_from_body(
            Some(r#"{"state":"RUNNING"}"#), None,
        ).unwrap();
        assert_eq!(out, Some(super::ProcessorTargetState::Running));
    }

    #[test]
    fn processor_target_from_body_parses_stopped() {
        let out = super::processor_target_from_body(
            Some(r#"{"state":"STOPPED"}"#), None,
        ).unwrap();
        assert_eq!(out, Some(super::ProcessorTargetState::Stopped));
    }

    #[test]
    fn processor_target_from_body_parses_disabled() {
        let out = super::processor_target_from_body(
            Some(r#"{"state":"DISABLED"}"#), None,
        ).unwrap();
        assert_eq!(out, Some(super::ProcessorTargetState::Disabled));
    }

    #[test]
    fn processor_target_from_body_run_once_is_rejected() {
        let err = super::processor_target_from_body(
            Some(r#"{"state":"RUN_ONCE"}"#), None,
        ).unwrap_err();
        match err {
            crate::error::CliError::User(msg) => {
                assert!(msg.contains("RUN_ONCE"), "message should mention RUN_ONCE: {msg}");
                assert!(msg.contains("remove --wait"), "message should hint at the fix: {msg}");
            }
            other => panic!("expected User, got {other:?}"),
        }
    }

    #[test]
    fn processor_target_from_body_rejects_missing_state() {
        let err = super::processor_target_from_body(Some("{}"), None).unwrap_err();
        match err {
            crate::error::CliError::User(msg) => assert!(msg.contains("'state'")),
            other => panic!("expected User, got {other:?}"),
        }
    }

    #[test]
    fn processor_target_from_body_rejects_unknown_state() {
        let err = super::processor_target_from_body(
            Some(r#"{"state":"SOMETHING_ELSE"}"#), None,
        ).unwrap_err();
        match err {
            crate::error::CliError::User(msg) => assert!(msg.contains("SOMETHING_ELSE")),
            other => panic!("expected User, got {other:?}"),
        }
    }

    // --- run_wait_plan ---

    #[tokio::test]
    async fn run_wait_plan_processor_state_returns_final_entity() {
        use nifi_rust_client::NifiClientBuilder;
        use nifi_rust_client::dynamic::DynamicClient;
        use serde_json::json;
        use std::time::Duration;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/about"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "about": { "title": "NiFi", "version": "2.9.0" }
            })))
            .mount(&mock)
            .await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/processors/proc-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "component": { "state": "RUNNING" }
            })))
            .mount(&mock)
            .await;

        let client = NifiClientBuilder::new(&mock.uri()).unwrap().build().unwrap();
        let dyn_client = DynamicClient::from_client(client).await.unwrap();

        let plan = super::WaitPlan::ProcessorState {
            processor_id: "proc-1".to_string(),
            target: super::ProcessorTargetState::Running,
        };
        let out = super::run_wait_plan(plan, &dyn_client, Duration::from_secs(2))
            .await
            .unwrap();
        match out {
            crate::output::CliOutput::Single(v) => {
                assert_eq!(
                    v.pointer("/component/state").and_then(|s| s.as_str()),
                    Some("RUNNING")
                );
            }
            _ => panic!("expected Single"),
        }
    }
}
