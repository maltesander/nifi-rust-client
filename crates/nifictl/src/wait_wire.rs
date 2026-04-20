//! Wait-flag plumbing for nifictl.
//!
//! Holds helpers for parsing `--wait-timeout` and the pre-/post-dispatch
//! logic for attaching `--wait` semantics to supported generated commands.

use std::path::Path;
use std::time::Duration;

use nifi_rust_client::dynamic::DynamicClient;
use nifi_rust_client::wait::{self, ControllerServiceTargetState, ProcessorTargetState, WaitConfig};
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
    /// Wait for a controller service to reach a target state.
    ControllerServiceState {
        service_id: String,
        target: ControllerServiceTargetState,
    },
    /// Wait on the async request started by `submit-parameter-context-update`.
    /// `context_id` is known pre-dispatch; `request_id` is extracted from the
    /// response body at `/request/requestId`.
    ParameterContextUpdate { context_id: String },
}

/// Inspect a `processors update-run-status` body and extract the target state.
///
/// Returns `Ok(ProcessorTargetState)` for steady states: `RUNNING`, `STOPPED`, `DISABLED`.
/// Rejects `RUN_ONCE` with a descriptive error since it has no steady state to converge on.
pub fn processor_target_from_body(
    inline: Option<&str>,
    body_file: Option<&Path>,
) -> Result<ProcessorTargetState, CliError> {
    let value = body::resolve_body(inline, body_file)?;
    let Some(state) = value.get("state").and_then(Value::as_str) else {
        return Err(CliError::User(
            "processors update-run-status --wait requires a body with a 'state' field".to_string(),
        ));
    };
    match state {
        "RUNNING" => Ok(ProcessorTargetState::Running),
        "STOPPED" => Ok(ProcessorTargetState::Stopped),
        "DISABLED" => Ok(ProcessorTargetState::Disabled),
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

/// Inspect a `controller-services update-run-status` body and extract the target state.
///
/// Returns `Ok(ControllerServiceTargetState)` for steady states: `ENABLED`, `DISABLED`.
/// Rejects `ENABLING` and `DISABLING` — those are transient server states, not valid targets.
pub fn controller_service_target_from_body(
    inline: Option<&str>,
    body_file: Option<&Path>,
) -> Result<ControllerServiceTargetState, CliError> {
    let value = body::resolve_body(inline, body_file)?;
    let Some(state) = value.get("state").and_then(Value::as_str) else {
        return Err(CliError::User(
            "controller-services update-run-status --wait requires a body with a 'state' field"
                .to_string(),
        ));
    };
    match state {
        "ENABLED" => Ok(ControllerServiceTargetState::Enabled),
        "DISABLED" => Ok(ControllerServiceTargetState::Disabled),
        "ENABLING" | "DISABLING" => Err(CliError::User(format!(
            "--wait on '{state}' has no effect — '{state}' is a transient state; \
             use ENABLED or DISABLED"
        ))),
        other => Err(CliError::User(format!(
            "unsupported controller-service state for --wait: '{other}' \
             (expected ENABLED or DISABLED)"
        ))),
    }
}

/// Inspect a `GeneratedResource` and produce a `WaitPlan` if the specific
/// subcommand is one of the ones we support with `--wait`. Returns
/// `Ok(None)` for any other subcommand.
pub fn peek_wait_plan(
    resource: &crate::generated::GeneratedResource,
) -> Result<Option<WaitPlan>, CliError> {
    use crate::generated::{
        ControllerServicesCommand, GeneratedResource, ParameterContextsCommand, ProcessorsCommand,
    };

    if let GeneratedResource::Processors { command } = resource
        && let ProcessorsCommand::UpdateRunStatus(args) = command
    {
        return Ok(Some(WaitPlan::ProcessorState {
            processor_id: args.id.clone(),
            target: processor_target_from_body(
                args.body.as_deref(),
                args.body_file.as_deref(),
            )?,
        }));
    }

    if let GeneratedResource::ControllerServices { command } = resource
        && let ControllerServicesCommand::UpdateRunStatus(args) = command
    {
        return Ok(Some(WaitPlan::ControllerServiceState {
            service_id: args.id.clone(),
            target: controller_service_target_from_body(
                args.body.as_deref(),
                args.body_file.as_deref(),
            )?,
        }));
    }

    if let GeneratedResource::ParameterContexts { command } = resource
        && let ParameterContextsCommand::SubmitParameterContextUpdate(args) = command
    {
        return Ok(Some(WaitPlan::ParameterContextUpdate {
            context_id: args.context_id.clone(),
        }));
    }

    Ok(None)
}

/// Run a prepared wait plan. Returns the final `CliOutput` produced by the
/// wait helper, replacing the initial dispatch response.
pub async fn run_wait_plan(
    plan: WaitPlan,
    dispatch_result: Value,
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
        WaitPlan::ControllerServiceState { service_id, target } => {
            let entity = wait::controller_service_state_dynamic(client, &service_id, target, config).await?;
            let value = serde_json::to_value(&entity)
                .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
            Ok(CliOutput::Single(value))
        }
        WaitPlan::ParameterContextUpdate { context_id } => {
            let request_id = dispatch_result
                .pointer("/request/requestId")
                .and_then(Value::as_str)
                .ok_or_else(|| CliError::User(
                    "submit response missing /request/requestId — cannot wait".to_string(),
                ))?
                .to_string();
            let entity = wait::parameter_context_update_dynamic(
                client, &context_id, &request_id, config,
            ).await?;
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
        assert_eq!(out, super::ProcessorTargetState::Running);
    }

    #[test]
    fn processor_target_from_body_parses_stopped() {
        let out = super::processor_target_from_body(
            Some(r#"{"state":"STOPPED"}"#), None,
        ).unwrap();
        assert_eq!(out, super::ProcessorTargetState::Stopped);
    }

    #[test]
    fn processor_target_from_body_parses_disabled() {
        let out = super::processor_target_from_body(
            Some(r#"{"state":"DISABLED"}"#), None,
        ).unwrap();
        assert_eq!(out, super::ProcessorTargetState::Disabled);
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

    // --- controller_service_target_from_body ---

    #[test]
    fn controller_service_target_from_body_parses_enabled() {
        let out = super::controller_service_target_from_body(
            Some(r#"{"state":"ENABLED"}"#), None,
        ).unwrap();
        assert_eq!(out, super::ControllerServiceTargetState::Enabled);
    }

    #[test]
    fn controller_service_target_from_body_parses_disabled() {
        let out = super::controller_service_target_from_body(
            Some(r#"{"state":"DISABLED"}"#), None,
        ).unwrap();
        assert_eq!(out, super::ControllerServiceTargetState::Disabled);
    }

    #[test]
    fn controller_service_target_from_body_rejects_enabling() {
        let err = super::controller_service_target_from_body(
            Some(r#"{"state":"ENABLING"}"#), None,
        ).unwrap_err();
        match err {
            crate::error::CliError::User(msg) => {
                assert!(msg.contains("ENABLING"), "message should mention ENABLING: {msg}");
                assert!(msg.contains("transient"), "message should mention transient: {msg}");
            }
            other => panic!("expected User, got {other:?}"),
        }
    }

    #[test]
    fn controller_service_target_from_body_rejects_disabling() {
        let err = super::controller_service_target_from_body(
            Some(r#"{"state":"DISABLING"}"#), None,
        ).unwrap_err();
        assert!(matches!(err, crate::error::CliError::User(_)));
    }

    #[test]
    fn controller_service_target_from_body_rejects_missing_state() {
        let err = super::controller_service_target_from_body(Some("{}"), None).unwrap_err();
        match err {
            crate::error::CliError::User(msg) => assert!(msg.contains("'state'")),
            other => panic!("expected User, got {other:?}"),
        }
    }

    #[test]
    fn controller_service_target_from_body_rejects_unknown_state() {
        let err = super::controller_service_target_from_body(
            Some(r#"{"state":"WAT"}"#), None,
        ).unwrap_err();
        match err {
            crate::error::CliError::User(msg) => assert!(msg.contains("WAT")),
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
        let out = super::run_wait_plan(plan, json!({}), &dyn_client, Duration::from_secs(2))
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

    #[tokio::test]
    async fn run_wait_plan_controller_service_state_returns_final_entity() {
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
            .and(path("/nifi-api/controller-services/svc-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "component": { "state": "ENABLED" }
            })))
            .mount(&mock)
            .await;

        let client = NifiClientBuilder::new(&mock.uri()).unwrap().build().unwrap();
        let dyn_client = DynamicClient::from_client(client).await.unwrap();

        let plan = super::WaitPlan::ControllerServiceState {
            service_id: "svc-1".to_string(),
            target: super::ControllerServiceTargetState::Enabled,
        };
        let out = super::run_wait_plan(plan, json!({}), &dyn_client, Duration::from_secs(2))
            .await
            .unwrap();
        match out {
            crate::output::CliOutput::Single(v) => {
                assert_eq!(
                    v.pointer("/component/state").and_then(|s| s.as_str()),
                    Some("ENABLED")
                );
            }
            _ => panic!("expected Single"),
        }
    }

    // --- run_wait_plan parameter-context-update arm ---

    #[tokio::test]
    async fn run_wait_plan_parameter_context_update_polls_and_cleans_up() {
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
            .and(path("/nifi-api/parameter-contexts/ctx-1/update-requests/req-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "request": { "requestId": "req-1", "complete": true }
            })))
            .mount(&mock)
            .await;
        Mock::given(method("DELETE"))
            .and(path("/nifi-api/parameter-contexts/ctx-1/update-requests/req-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .expect(1)
            .mount(&mock)
            .await;

        let client = NifiClientBuilder::new(&mock.uri()).unwrap().build().unwrap();
        let dyn_client = DynamicClient::from_client(client).await.unwrap();

        let plan = super::WaitPlan::ParameterContextUpdate {
            context_id: "ctx-1".to_string(),
        };
        let dispatch = json!({ "request": { "requestId": "req-1" } });
        let out = super::run_wait_plan(plan, dispatch, &dyn_client, Duration::from_secs(2))
            .await
            .unwrap();
        match out {
            crate::output::CliOutput::Single(v) => {
                assert_eq!(
                    v.pointer("/request/complete").and_then(|b| b.as_bool()),
                    Some(true)
                );
            }
            _ => panic!("expected Single"),
        }
    }

    #[tokio::test]
    async fn run_wait_plan_parameter_context_missing_request_id_errors() {
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

        let client = NifiClientBuilder::new(&mock.uri()).unwrap().build().unwrap();
        let dyn_client = DynamicClient::from_client(client).await.unwrap();

        let plan = super::WaitPlan::ParameterContextUpdate {
            context_id: "ctx-1".to_string(),
        };
        let dispatch = json!({});
        let result = super::run_wait_plan(plan, dispatch, &dyn_client, Duration::from_secs(2))
            .await;
        match result {
            Err(crate::error::CliError::User(msg)) => {
                assert!(msg.contains("/request/requestId"), "message should reference the missing pointer: {msg}");
            }
            Err(other) => panic!("expected User, got {other:?}"),
            Ok(_) => panic!("expected Err, got Ok"),
        }
    }
}
