#![deny(missing_docs)]
//! Polling helpers for NiFi state transitions and async queries.
//!
//! Each helper polls a single resource until a condition is met, or until
//! the configured `WaitConfig::timeout` elapses (returning [`NifiError::Timeout`]).
//!
//! Pattern mirrors [`crate::pagination`]: free functions with a `_dynamic`
//! sibling for use with `crate::dynamic::DynamicClient` (gated on the
//! `dynamic` feature).

use std::time::Duration;

use reqwest::StatusCode;

use crate::NifiError;

/// Synthetic status code used when a NiFi async query reports a
/// `failureReason` — there is no real HTTP error to forward, so we
/// surface the failure as `Api { status: 500, .. }`.
const STATUS_OPERATION_FAILED: u16 = StatusCode::INTERNAL_SERVER_ERROR.as_u16();

// ── Configuration ──────────────────────────────────────────────────────────

/// Configuration for polling helpers.
///
/// `WaitConfig::default()` yields a 30-second timeout, 500ms poll interval,
/// no initial delay, and cleanup enabled (for async-query helpers).
#[derive(Debug, Clone)]
pub struct WaitConfig {
    /// Maximum total time before returning [`NifiError::Timeout`].
    pub timeout: Duration,
    /// Delay between polls once the initial delay has elapsed.
    pub poll_interval: Duration,
    /// Delay before the first poll. Useful when a state transition is
    /// known to take a minimum amount of time.
    pub initial_delay: Duration,
    /// Controls the trailing DELETE on async-query helpers. Honored by
    /// `parameter_context_update` and `provenance_query` only; ignored by
    /// `processor_state` and `controller_service_state` (which have no
    /// server-side state to clean up). When `true`, the helper issues a
    /// `DELETE` after the query resolves, regardless of success or failure,
    /// and swallows any error from that DELETE so it cannot mask the poll
    /// result.
    pub cleanup: bool,
}

impl Default for WaitConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            poll_interval: Duration::from_millis(500),
            initial_delay: Duration::ZERO,
            cleanup: true,
        }
    }
}

// ── Target-state enums ─────────────────────────────────────────────────────

/// The subset of processor states users can legitimately wait for.
///
/// Transient / command states (e.g. `RUN_ONCE`) are intentionally omitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorTargetState {
    /// Processor is scheduled and running.
    Running,
    /// Processor is stopped.
    Stopped,
    /// Processor is disabled.
    Disabled,
}

impl ProcessorTargetState {
    /// The server wire value (e.g. `"RUNNING"`). Sourced from the generated
    /// `ProcessorDtoState` enum so the wire vocabulary lives in exactly one
    /// place.
    pub(crate) fn wire_value(&self) -> &'static str {
        #[cfg(feature = "dynamic")]
        use crate::dynamic::types::ProcessorDtoState;
        #[cfg(not(feature = "dynamic"))]
        use crate::types::ProcessorDtoState;

        match self {
            Self::Running => ProcessorDtoState::Running.as_str(),
            Self::Stopped => ProcessorDtoState::Stopped.as_str(),
            Self::Disabled => ProcessorDtoState::Disabled.as_str(),
        }
    }
}

/// The subset of controller-service states users can legitimately wait for.
///
/// `Enabling` and `Disabling` are transient states the client polls
/// through and are not valid targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerServiceTargetState {
    /// Service is enabled (steady state).
    Enabled,
    /// Service is disabled (steady state).
    Disabled,
}

impl ControllerServiceTargetState {
    /// The server wire value (e.g. `"ENABLED"`). Sourced from the generated
    /// `ControllerServiceDtoState` enum so the wire vocabulary lives in
    /// exactly one place.
    pub(crate) fn wire_value(&self) -> &'static str {
        #[cfg(feature = "dynamic")]
        use crate::dynamic::types::ControllerServiceDtoState;
        #[cfg(not(feature = "dynamic"))]
        use crate::types::ControllerServiceDtoState;

        match self {
            Self::Enabled => ControllerServiceDtoState::Enabled.as_str(),
            Self::Disabled => ControllerServiceDtoState::Disabled.as_str(),
        }
    }
}

// ── Polling primitive ──────────────────────────────────────────────────────

/// Internal outcome of a single poll check. The fetched value itself flows
/// back to the caller through [`poll_until`]'s `Result<T, _>` return — this
/// enum just signals the control flow.
#[derive(Debug)]
enum PollOutcome {
    /// State not satisfied yet — keep polling.
    Pending,
    /// Terminal success; return the current `fetch` value with `Ok`.
    Ready,
    /// Terminal failure; return this error without further polling.
    Failed(NifiError),
}

/// Private polling primitive.
///
/// `fetch` returns the current resource state. `done` inspects it and
/// returns one of `PollOutcome::{Pending, Ready, Failed(err)}`.
///
/// Deadline is computed **after** the `initial_delay` sleep, so the full
/// `config.timeout` window is available for actual polling. The final sleep
/// is clamped to the remaining time so we don't overshoot.
///
/// Audit follow-up A6: previously the deadline snapshot ran before the
/// initial-delay sleep. With `initial_delay >= timeout` the deadline was
/// already in the past on the first iteration, so a single `fetch` fired
/// and the loop returned `Timeout` immediately.
async fn poll_until<T, FetchFn, FetchFut>(
    config: &WaitConfig,
    operation: &str,
    fetch: FetchFn,
    done: impl Fn(&T) -> PollOutcome,
) -> Result<T, NifiError>
where
    FetchFn: Fn() -> FetchFut,
    FetchFut: core::future::Future<Output = Result<T, NifiError>>,
{
    if !config.initial_delay.is_zero() {
        tokio::time::sleep(config.initial_delay).await;
    }

    let deadline = tokio::time::Instant::now() + config.timeout;

    loop {
        let value = fetch().await?;
        match done(&value) {
            PollOutcome::Ready => return Ok(value),
            PollOutcome::Failed(e) => return Err(e),
            PollOutcome::Pending => {}
        }
        if tokio::time::Instant::now() >= deadline {
            return Err(NifiError::Timeout {
                operation: operation.to_string(),
            });
        }
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        let next = core::cmp::min(config.poll_interval, remaining);
        tokio::time::sleep(next).await;
    }
}

/// Log a cleanup DELETE failure at warn level and discard the error.
///
/// Cleanup is best-effort: the primary operation has already completed
/// (or timed out), and the user should see that result, not the
/// cleanup DELETE's failure. But silently ignoring it hides real stuck
/// state — warn so operators can see it in logs.
fn warn_cleanup_failure<E: std::fmt::Display>(
    operation: &str,
    target: &str,
    result: Result<(), E>,
) {
    if let Err(err) = result {
        tracing::warn!(
            operation,
            target,
            error = %err,
            "cleanup request failed (best-effort; ignored)"
        );
    }
}

// ── wait::processor_state ──────────────────────────────────────────────────

#[cfg(not(feature = "dynamic"))]
use crate::types::ProcessorEntity;

/// Poll a processor until its state matches `target`.
///
/// Fetches `GET /processors/{id}` on each tick. Returns the final
/// `ProcessorEntity` on success, [`NifiError::Timeout`] on timeout, or
/// the underlying fetch error if NiFi returns a non-2xx response.
///
/// # Example
///
/// ```no_run
/// use std::time::Duration;
/// use nifi_rust_client::wait::{self, ProcessorTargetState, WaitConfig};
///
/// # async fn example(client: nifi_rust_client::NifiClient) -> Result<(), nifi_rust_client::NifiError> {
/// let config = WaitConfig {
///     timeout: Duration::from_secs(60),
///     poll_interval: Duration::from_millis(250),
///     ..Default::default()
/// };
/// let processor = wait::processor_state(
///     &client,
///     "processor-id",
///     ProcessorTargetState::Running,
///     config,
/// ).await?;
/// # let _ = processor;
/// # Ok(()) }
/// ```
#[cfg(not(feature = "dynamic"))]
pub async fn processor_state(
    client: &crate::NifiClient,
    processor_id: &str,
    target: ProcessorTargetState,
    config: WaitConfig,
) -> Result<ProcessorEntity, NifiError> {
    use crate::types::ProcessorDtoState;
    let op = format!(
        "wait_for_processor_state({processor_id}, {})",
        target.wire_value()
    );
    let fetch = || async { client.processors().get_processor(processor_id).await };
    let done = move |entity: &ProcessorEntity| {
        let matches = entity
            .component
            .as_ref()
            .and_then(|c| c.state.as_ref())
            .is_some_and(|s| {
                matches!(
                    (target, s),
                    (ProcessorTargetState::Running, ProcessorDtoState::Running)
                        | (ProcessorTargetState::Stopped, ProcessorDtoState::Stopped)
                        | (ProcessorTargetState::Disabled, ProcessorDtoState::Disabled)
                )
            });
        if matches {
            PollOutcome::Ready
        } else {
            PollOutcome::Pending
        }
    };
    poll_until(&config, &op, fetch, done).await
}

// ── wait::controller_service_state ─────────────────────────────────────────

#[cfg(not(feature = "dynamic"))]
use crate::types::ControllerServiceEntity;

/// Poll a controller service until its state matches `target`.
///
/// Fetches `GET /controller-services/{id}` on each tick. Returns the final
/// `ControllerServiceEntity` on success. The transient `ENABLING` and
/// `DISABLING` server states are polled through (they are not valid targets).
#[cfg(not(feature = "dynamic"))]
pub async fn controller_service_state(
    client: &crate::NifiClient,
    service_id: &str,
    target: ControllerServiceTargetState,
    config: WaitConfig,
) -> Result<ControllerServiceEntity, NifiError> {
    use crate::types::ControllerServiceDtoState;
    let op = format!(
        "wait_for_controller_service_state({service_id}, {})",
        target.wire_value()
    );
    let fetch = || async {
        client
            .controller_services()
            .get_controller_service(service_id, None)
            .await
    };
    let done = move |entity: &ControllerServiceEntity| {
        let matches = entity
            .component
            .as_ref()
            .and_then(|c| c.state.as_ref())
            .is_some_and(|s| {
                matches!(
                    (target, s),
                    (
                        ControllerServiceTargetState::Enabled,
                        ControllerServiceDtoState::Enabled
                    ) | (
                        ControllerServiceTargetState::Disabled,
                        ControllerServiceDtoState::Disabled
                    )
                )
            });
        if matches {
            PollOutcome::Ready
        } else {
            PollOutcome::Pending
        }
    };
    poll_until(&config, &op, fetch, done).await
}

#[cfg(feature = "dynamic")]
use crate::dynamic::types::ControllerServiceEntity;

/// Dynamic-mode counterpart of `controller_service_state`.
#[cfg(feature = "dynamic")]
pub async fn controller_service_state_dynamic(
    client: &crate::dynamic::DynamicClient,
    service_id: &str,
    target: ControllerServiceTargetState,
    config: WaitConfig,
) -> Result<ControllerServiceEntity, NifiError> {
    let target_wire = target.wire_value();
    let op = format!("wait_for_controller_service_state({service_id}, {target_wire})");
    let fetch = || async {
        client
            .controller_services()
            .get_controller_service(service_id, None)
            .await
    };
    let done = move |entity: &ControllerServiceEntity| {
        let state = entity.component.as_ref().and_then(|c| c.state.as_deref());
        if state == Some(target_wire) {
            PollOutcome::Ready
        } else {
            PollOutcome::Pending
        }
    };
    poll_until(&config, &op, fetch, done).await
}

#[cfg(feature = "dynamic")]
use crate::dynamic::types::ProcessorEntity;

/// Dynamic-mode counterpart of `processor_state`.
#[cfg(feature = "dynamic")]
pub async fn processor_state_dynamic(
    client: &crate::dynamic::DynamicClient,
    processor_id: &str,
    target: ProcessorTargetState,
    config: WaitConfig,
) -> Result<ProcessorEntity, NifiError> {
    let target_wire = target.wire_value();
    let op = format!("wait_for_processor_state({processor_id}, {target_wire})");
    let fetch = || async { client.processors().get_processor(processor_id).await };
    let done = move |entity: &ProcessorEntity| {
        let state = entity.component.as_ref().and_then(|c| c.state.as_deref());
        if state == Some(target_wire) {
            PollOutcome::Ready
        } else {
            PollOutcome::Pending
        }
    };
    poll_until(&config, &op, fetch, done).await
}

// ── Shared outcome predicate ───────────────────────────────────────────────

/// Shared `(terminal, failure_reason) -> PollOutcome` decision used by every
/// async-request helper that polls a `(complete | finished, failureReason)`
/// pair. `failure_reason` is terminal regardless of the boolean; an absent
/// boolean (server has not yet populated the field) is treated as Pending.
///
/// `op_kind` is a short noun phrase interpolated into the failure message
/// (e.g. `"parameter context update"`, `"drop request"`, `"verification"`).
fn terminal_outcome(
    terminal: Option<bool>,
    failure_reason: Option<&str>,
    op_kind: &str,
) -> PollOutcome {
    if let Some(reason) = failure_reason {
        return PollOutcome::Failed(NifiError::Api {
            status: STATUS_OPERATION_FAILED,
            message: format!("{op_kind} failed: {reason}"),
        });
    }
    if terminal.unwrap_or(false) {
        PollOutcome::Ready
    } else {
        PollOutcome::Pending
    }
}

// ── wait::parameter_context_update ─────────────────────────────────────────

#[cfg(not(feature = "dynamic"))]
use crate::types::ParameterContextUpdateRequestEntity;

/// Poll a parameter-context update request until it reaches a terminal state.
///
/// Fetches `GET /parameter-contexts/{context_id}/update-requests/{request_id}`.
/// Returns the final entity on success. If the request completes with a
/// `failureReason`, returns [`NifiError::Api`] with status 500 and the
/// failure reason in the message. On timeout, returns [`NifiError::Timeout`].
///
/// If [`WaitConfig::cleanup`] is `true` (default), issues a trailing
/// `DELETE /parameter-contexts/{context_id}/update-requests/{request_id}`
/// to free server-side state regardless of success or failure. The DELETE
/// is best-effort — its errors are swallowed.
#[cfg(not(feature = "dynamic"))]
pub async fn parameter_context_update(
    client: &crate::NifiClient,
    context_id: &str,
    request_id: &str,
    config: WaitConfig,
) -> Result<ParameterContextUpdateRequestEntity, NifiError> {
    let op = format!("wait_for_parameter_context_update({context_id}/{request_id})");
    let fetch = || async {
        client
            .parametercontexts()
            .get_parameter_context_update(context_id, request_id)
            .await
    };
    let done = |entity: &ParameterContextUpdateRequestEntity| {
        let req = entity.request.as_ref();
        terminal_outcome(
            req.and_then(|r| r.complete),
            req.and_then(|r| r.failure_reason.as_deref()),
            "parameter context update",
        )
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .parametercontexts()
            .delete_update_request(context_id, request_id, None)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, &format!("{context_id}/{request_id}"), res);
    }
    result
}

#[cfg(feature = "dynamic")]
use crate::dynamic::types::ParameterContextUpdateRequestEntity;

/// Dynamic-mode counterpart of `parameter_context_update`.
#[cfg(feature = "dynamic")]
pub async fn parameter_context_update_dynamic(
    client: &crate::dynamic::DynamicClient,
    context_id: &str,
    request_id: &str,
    config: WaitConfig,
) -> Result<ParameterContextUpdateRequestEntity, NifiError> {
    let op = format!("wait_for_parameter_context_update({context_id}/{request_id})");
    let fetch = || async {
        client
            .parametercontexts()
            .get_parameter_context_update(context_id, request_id)
            .await
    };
    let done = |entity: &ParameterContextUpdateRequestEntity| {
        let req = entity.request.as_ref();
        terminal_outcome(
            req.and_then(|r| r.complete),
            req.and_then(|r| r.failure_reason.as_deref()),
            "parameter context update",
        )
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .parametercontexts()
            .delete_update_request(context_id, request_id, None)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, &format!("{context_id}/{request_id}"), res);
    }
    result
}

// ── wait::provenance_query ─────────────────────────────────────────────────

#[cfg(not(feature = "dynamic"))]
use crate::types::ProvenanceDto;

/// Poll a provenance query until it reports `finished == true`.
///
/// Fetches `GET /provenance/{id}` on each tick, passing no query
/// parameters (the server defaults are used). Returns the final
/// `ProvenanceDto` on success.
///
/// If [`WaitConfig::cleanup`] is `true` (default), issues a trailing
/// `DELETE /provenance/{id}` to free server-side state. The DELETE is
/// best-effort — its errors are swallowed.
#[cfg(not(feature = "dynamic"))]
pub async fn provenance_query(
    client: &crate::NifiClient,
    query_id: &str,
    config: WaitConfig,
) -> Result<ProvenanceDto, NifiError> {
    let op = format!("wait_for_provenance_query({query_id})");
    let fetch = || async {
        client
            .provenance()
            .get_provenance(query_id, None, None, None)
            .await
    };
    let done = |dto: &ProvenanceDto| {
        if dto.finished.unwrap_or(false) {
            PollOutcome::Ready
        } else {
            PollOutcome::Pending
        }
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .provenance()
            .delete_provenance(query_id, None)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, query_id, res);
    }
    result
}

#[cfg(feature = "dynamic")]
use crate::dynamic::types::ProvenanceDto;

/// Dynamic-mode counterpart of `provenance_query`.
#[cfg(feature = "dynamic")]
pub async fn provenance_query_dynamic(
    client: &crate::dynamic::DynamicClient,
    query_id: &str,
    config: WaitConfig,
) -> Result<ProvenanceDto, NifiError> {
    let op = format!("wait_for_provenance_query({query_id})");
    let fetch = || async {
        client
            .provenance()
            .get_provenance(query_id, None, None, None)
            .await
    };
    let done = |dto: &ProvenanceDto| {
        if dto.finished.unwrap_or(false) {
            PollOutcome::Ready
        } else {
            PollOutcome::Pending
        }
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .provenance()
            .delete_provenance(query_id, None)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, query_id, res);
    }
    result
}

// ── wait::flowfile_drop ────────────────────────────────────────────────────

#[cfg(not(feature = "dynamic"))]
use crate::types::DropRequestDto;

/// Poll a FlowFile-queue drop request until it reports `finished == true`.
///
/// Fetches `GET /flowfile-queues/{id}/drop-requests/{drop-request-id}`.
/// Returns the final `DropRequestDto` on success. If the request reports
/// a `failureReason`, returns [`NifiError::Api`] with status 500. On
/// timeout, returns [`NifiError::Timeout`].
///
/// If [`WaitConfig::cleanup`] is `true` (default), issues a trailing
/// `DELETE` to free server-side state. Best-effort — its errors are swallowed.
#[cfg(not(feature = "dynamic"))]
pub async fn flowfile_drop(
    client: &crate::NifiClient,
    queue_id: &str,
    drop_request_id: &str,
    config: WaitConfig,
) -> Result<DropRequestDto, NifiError> {
    let op = format!("wait_for_flowfile_drop({queue_id}/{drop_request_id})");
    let fetch = || async {
        client
            .flowfilequeues()
            .get_drop_request(queue_id, drop_request_id)
            .await
    };
    let done = |dto: &DropRequestDto| {
        terminal_outcome(
            dto.finished,
            dto.failure_reason.as_deref(),
            "drop request",
        )
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .flowfilequeues()
            .remove_drop_request(queue_id, drop_request_id)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, &format!("{queue_id}/{drop_request_id}"), res);
    }
    result
}

#[cfg(feature = "dynamic")]
use crate::dynamic::types::DropRequestDto;

/// Dynamic-mode counterpart of `flowfile_drop`.
#[cfg(feature = "dynamic")]
pub async fn flowfile_drop_dynamic(
    client: &crate::dynamic::DynamicClient,
    queue_id: &str,
    drop_request_id: &str,
    config: WaitConfig,
) -> Result<DropRequestDto, NifiError> {
    let op = format!("wait_for_flowfile_drop({queue_id}/{drop_request_id})");
    let fetch = || async {
        client
            .flowfilequeues()
            .get_drop_request(queue_id, drop_request_id)
            .await
    };
    let done = |dto: &DropRequestDto| {
        terminal_outcome(
            dto.finished,
            dto.failure_reason.as_deref(),
            "drop request",
        )
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .flowfilequeues()
            .remove_drop_request(queue_id, drop_request_id)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, &format!("{queue_id}/{drop_request_id}"), res);
    }
    result
}

// ── wait::flowfile_listing ─────────────────────────────────────────────────

#[cfg(not(feature = "dynamic"))]
use crate::types::ListingRequestDto;

/// Poll a FlowFile-queue listing request until it reports `finished == true`.
///
/// Fetches `GET /flowfile-queues/{id}/listing-requests/{listing-request-id}`.
/// Returns the final `ListingRequestDto` on success. If the request reports
/// a `failureReason`, returns [`NifiError::Api`] with status 500. On
/// timeout, returns [`NifiError::Timeout`].
///
/// If [`WaitConfig::cleanup`] is `true` (default), issues a trailing
/// `DELETE /flowfile-queues/{id}/listing-requests/{listing-request-id}`
/// to free server-side state. Best-effort — its errors are swallowed.
#[cfg(not(feature = "dynamic"))]
pub async fn flowfile_listing(
    client: &crate::NifiClient,
    queue_id: &str,
    listing_request_id: &str,
    config: WaitConfig,
) -> Result<ListingRequestDto, NifiError> {
    let op = format!("wait_for_flowfile_listing({queue_id}/{listing_request_id})");
    let fetch = || async {
        client
            .flowfilequeues()
            .get_listing_request(queue_id, listing_request_id)
            .await
    };
    let done = |dto: &ListingRequestDto| {
        terminal_outcome(
            dto.finished,
            dto.failure_reason.as_deref(),
            "listing request",
        )
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .flowfilequeues()
            .delete_listing_request(queue_id, listing_request_id)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, &format!("{queue_id}/{listing_request_id}"), res);
    }
    result
}

#[cfg(feature = "dynamic")]
use crate::dynamic::types::ListingRequestDto;

/// Dynamic-mode counterpart of `flowfile_listing`.
#[cfg(feature = "dynamic")]
pub async fn flowfile_listing_dynamic(
    client: &crate::dynamic::DynamicClient,
    queue_id: &str,
    listing_request_id: &str,
    config: WaitConfig,
) -> Result<ListingRequestDto, NifiError> {
    let op = format!("wait_for_flowfile_listing({queue_id}/{listing_request_id})");
    let fetch = || async {
        client
            .flowfilequeues()
            .get_listing_request(queue_id, listing_request_id)
            .await
    };
    let done = |dto: &ListingRequestDto| {
        terminal_outcome(
            dto.finished,
            dto.failure_reason.as_deref(),
            "listing request",
        )
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .flowfilequeues()
            .delete_listing_request(queue_id, listing_request_id)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, &format!("{queue_id}/{listing_request_id}"), res);
    }
    result
}

// ── wait::empty_all_connections ────────────────────────────────────────────

/// Poll a process-group "empty all connections" drop request until finished.
///
/// Fetches `GET /process-groups/{id}/empty-all-connections-requests/{drop-request-id}`.
/// Returns the final `DropRequestDto` on success. If the request reports a
/// `failureReason`, returns [`NifiError::Api`] with status 500. On timeout,
/// returns [`NifiError::Timeout`].
///
/// If [`WaitConfig::cleanup`] is `true` (default), issues a trailing
/// `DELETE /process-groups/{id}/empty-all-connections-requests/{drop-request-id}`
/// to free server-side state. Best-effort — its errors are swallowed.
#[cfg(not(feature = "dynamic"))]
pub async fn empty_all_connections(
    client: &crate::NifiClient,
    process_group_id: &str,
    drop_request_id: &str,
    config: WaitConfig,
) -> Result<DropRequestDto, NifiError> {
    let op = format!("wait_for_empty_all_connections({process_group_id}/{drop_request_id})");
    let fetch = || async {
        client
            .processgroups()
            .get_drop_all_flowfiles_request(process_group_id, drop_request_id)
            .await
    };
    let done = |dto: &DropRequestDto| {
        terminal_outcome(
            dto.finished,
            dto.failure_reason.as_deref(),
            "empty-all-connections request",
        )
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .processgroups()
            .remove_drop_request(process_group_id, drop_request_id)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, &format!("{process_group_id}/{drop_request_id}"), res);
    }
    result
}

/// Dynamic-mode counterpart of `empty_all_connections`.
#[cfg(feature = "dynamic")]
pub async fn empty_all_connections_dynamic(
    client: &crate::dynamic::DynamicClient,
    process_group_id: &str,
    drop_request_id: &str,
    config: WaitConfig,
) -> Result<DropRequestDto, NifiError> {
    let op = format!("wait_for_empty_all_connections({process_group_id}/{drop_request_id})");
    let fetch = || async {
        client
            .processgroups()
            .get_drop_all_flowfiles_request(process_group_id, drop_request_id)
            .await
    };
    let done = |dto: &DropRequestDto| {
        terminal_outcome(
            dto.finished,
            dto.failure_reason.as_deref(),
            "empty-all-connections request",
        )
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .processgroups()
            .remove_drop_request(process_group_id, drop_request_id)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, &format!("{process_group_id}/{drop_request_id}"), res);
    }
    result
}

// ── wait::provenance_lineage ───────────────────────────────────────────────

#[cfg(not(feature = "dynamic"))]
use crate::types::LineageDto;

/// Poll a provenance lineage query until it reports `finished == true`.
///
/// Fetches `GET /provenance/lineage/{id}` with no `clusterNodeId`. Returns
/// the final `LineageDto` on success. `LineageDto` carries no `failureReason`
/// — only `finished` and `percentCompleted` — so this helper never resolves
/// to a server-side failure (only fetch errors and timeouts can fail).
///
/// If [`WaitConfig::cleanup`] is `true` (default), issues a trailing
/// `DELETE /provenance/lineage/{id}` to free server-side state. Best-effort —
/// its errors are swallowed.
#[cfg(not(feature = "dynamic"))]
pub async fn provenance_lineage(
    client: &crate::NifiClient,
    lineage_id: &str,
    config: WaitConfig,
) -> Result<LineageDto, NifiError> {
    let op = format!("wait_for_provenance_lineage({lineage_id})");
    let fetch = || async {
        client
            .provenance()
            .get_lineage(lineage_id, None)
            .await
    };
    let done = |dto: &LineageDto| {
        if dto.finished.unwrap_or(false) {
            PollOutcome::Ready
        } else {
            PollOutcome::Pending
        }
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .provenance()
            .delete_lineage(lineage_id, None)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, lineage_id, res);
    }
    result
}

#[cfg(feature = "dynamic")]
use crate::dynamic::types::LineageDto;

/// Dynamic-mode counterpart of `provenance_lineage`.
#[cfg(feature = "dynamic")]
pub async fn provenance_lineage_dynamic(
    client: &crate::dynamic::DynamicClient,
    lineage_id: &str,
    config: WaitConfig,
) -> Result<LineageDto, NifiError> {
    let op = format!("wait_for_provenance_lineage({lineage_id})");
    let fetch = || async {
        client
            .provenance()
            .get_lineage(lineage_id, None)
            .await
    };
    let done = |dto: &LineageDto| {
        if dto.finished.unwrap_or(false) {
            PollOutcome::Ready
        } else {
            PollOutcome::Pending
        }
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .provenance()
            .delete_lineage(lineage_id, None)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, lineage_id, res);
    }
    result
}

// ── wait::processor_verify_config ──────────────────────────────────────────

#[cfg(not(feature = "dynamic"))]
use crate::types::VerifyConfigRequestDto;

/// Poll a processor config-verification request until `complete == true`.
///
/// Fetches `GET /processors/{id}/config/verification-requests/{requestId}`.
/// Returns the final `VerifyConfigRequestDto` on success. If the request
/// reports a `failureReason`, returns [`NifiError::Api`] with status 500.
/// On timeout, returns [`NifiError::Timeout`].
///
/// If [`WaitConfig::cleanup`] is `true` (default), issues a trailing
/// `DELETE /processors/{id}/config/verification-requests/{requestId}` to
/// free server-side state. Best-effort — its errors are swallowed.
#[cfg(not(feature = "dynamic"))]
pub async fn processor_verify_config(
    client: &crate::NifiClient,
    processor_id: &str,
    request_id: &str,
    config: WaitConfig,
) -> Result<VerifyConfigRequestDto, NifiError> {
    let op = format!("wait_for_processor_verify_config({processor_id}/{request_id})");
    let fetch = || async {
        client
            .processors()
            .get_verification_request(processor_id, request_id)
            .await
    };
    let done = |dto: &VerifyConfigRequestDto| {
        terminal_outcome(dto.complete, dto.failure_reason.as_deref(), "verification")
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .processors()
            .delete_verification_request(processor_id, request_id)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, &format!("{processor_id}/{request_id}"), res);
    }
    result
}

#[cfg(feature = "dynamic")]
use crate::dynamic::types::VerifyConfigRequestDto;

/// Dynamic-mode counterpart of `processor_verify_config`.
#[cfg(feature = "dynamic")]
pub async fn processor_verify_config_dynamic(
    client: &crate::dynamic::DynamicClient,
    processor_id: &str,
    request_id: &str,
    config: WaitConfig,
) -> Result<VerifyConfigRequestDto, NifiError> {
    let op = format!("wait_for_processor_verify_config({processor_id}/{request_id})");
    let fetch = || async {
        client
            .processors()
            .get_verification_request(processor_id, request_id)
            .await
    };
    let done = |dto: &VerifyConfigRequestDto| {
        terminal_outcome(dto.complete, dto.failure_reason.as_deref(), "verification")
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .processors()
            .delete_verification_request(processor_id, request_id)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, &format!("{processor_id}/{request_id}"), res);
    }
    result
}

// ── wait::controller_service_verify_config ─────────────────────────────────

/// Poll a controller-service config-verification request until `complete == true`.
///
/// Fetches `GET /controller-services/{id}/config/verification-requests/{requestId}`.
/// Returns the final `VerifyConfigRequestDto` on success. If the request reports
/// a `failureReason`, returns [`NifiError::Api`] with status 500. On timeout,
/// returns [`NifiError::Timeout`].
///
/// If [`WaitConfig::cleanup`] is `true` (default), issues a trailing
/// `DELETE /controller-services/{id}/config/verification-requests/{requestId}`
/// to free server-side state. Best-effort — its errors are swallowed.
#[cfg(not(feature = "dynamic"))]
pub async fn controller_service_verify_config(
    client: &crate::NifiClient,
    service_id: &str,
    request_id: &str,
    config: WaitConfig,
) -> Result<VerifyConfigRequestDto, NifiError> {
    let op = format!("wait_for_controller_service_verify_config({service_id}/{request_id})");
    let fetch = || async {
        client
            .controller_services()
            .get_verification_request(service_id, request_id)
            .await
    };
    let done = |dto: &VerifyConfigRequestDto| {
        terminal_outcome(dto.complete, dto.failure_reason.as_deref(), "verification")
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .controller_services()
            .delete_verification_request(service_id, request_id)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, &format!("{service_id}/{request_id}"), res);
    }
    result
}

/// Dynamic-mode counterpart of `controller_service_verify_config`.
#[cfg(feature = "dynamic")]
pub async fn controller_service_verify_config_dynamic(
    client: &crate::dynamic::DynamicClient,
    service_id: &str,
    request_id: &str,
    config: WaitConfig,
) -> Result<VerifyConfigRequestDto, NifiError> {
    let op = format!("wait_for_controller_service_verify_config({service_id}/{request_id})");
    let fetch = || async {
        client
            .controller_services()
            .get_verification_request(service_id, request_id)
            .await
    };
    let done = |dto: &VerifyConfigRequestDto| {
        terminal_outcome(dto.complete, dto.failure_reason.as_deref(), "verification")
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let res = client
            .controller_services()
            .delete_verification_request(service_id, request_id)
            .await
            .map(|_| ());
        warn_cleanup_failure(&op, &format!("{service_id}/{request_id}"), res);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test]
    async fn poll_until_returns_ok_on_first_ready() {
        let config = WaitConfig {
            timeout: Duration::from_secs(1),
            poll_interval: Duration::from_millis(10),
            initial_delay: Duration::ZERO,
            cleanup: true,
        };
        let fetch = || async { Ok::<i32, NifiError>(42) };
        let done = |v: &i32| {
            if *v == 42 {
                PollOutcome::Ready
            } else {
                PollOutcome::Pending
            }
        };
        let result = poll_until(&config, "op", fetch, done).await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn poll_until_polls_until_ready() {
        let counter = Arc::new(AtomicUsize::new(0));
        let config = WaitConfig {
            timeout: Duration::from_secs(1),
            poll_interval: Duration::from_millis(5),
            initial_delay: Duration::ZERO,
            cleanup: true,
        };
        let c = Arc::clone(&counter);
        let fetch = move || {
            let c = Arc::clone(&c);
            async move {
                let n = c.fetch_add(1, Ordering::SeqCst);
                Ok::<usize, NifiError>(n)
            }
        };
        let done = |v: &usize| {
            if *v >= 3 {
                PollOutcome::Ready
            } else {
                PollOutcome::Pending
            }
        };
        let result = poll_until(&config, "op", fetch, done).await.unwrap();
        assert_eq!(result, 3);
        assert_eq!(counter.load(Ordering::SeqCst), 4);
    }

    #[tokio::test]
    async fn poll_until_times_out() {
        let config = WaitConfig {
            timeout: Duration::from_millis(50),
            poll_interval: Duration::from_millis(10),
            initial_delay: Duration::ZERO,
            cleanup: true,
        };
        let fetch = || async { Ok::<i32, NifiError>(0) };
        let done = |_: &i32| PollOutcome::Pending;
        let err = poll_until(&config, "test_op", fetch, done)
            .await
            .unwrap_err();
        match err {
            NifiError::Timeout { operation } => assert_eq!(operation, "test_op"),
            other => panic!("expected Timeout, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn poll_until_propagates_failed_outcome() {
        let config = WaitConfig::default();
        let fetch = || async { Ok::<i32, NifiError>(1) };
        let done = |_: &i32| {
            PollOutcome::Failed(NifiError::Api {
                status: 500,
                message: "boom".to_string(),
            })
        };
        let err = poll_until(&config, "op", fetch, done).await.unwrap_err();
        match err {
            NifiError::Api { status, message } => {
                assert_eq!(status, 500);
                assert_eq!(message, "boom");
            }
            other => panic!("expected Api, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn poll_until_propagates_fetch_error() {
        let config = WaitConfig::default();
        let fetch = || async {
            Err::<i32, NifiError>(NifiError::Api {
                status: 404,
                message: "not found".to_string(),
            })
        };
        let done = |_: &i32| PollOutcome::Pending;
        let err = poll_until(&config, "op", fetch, done).await.unwrap_err();
        assert!(matches!(err, NifiError::Api { status: 404, .. }));
    }

    /// Audit follow-up A6: a `WaitConfig` whose `initial_delay >= timeout`
    /// previously fired exactly one fetch and immediately returned `Timeout`,
    /// because the deadline snapshot ran before the initial-delay sleep. With
    /// the fix, the full `timeout` budget is available **after** the initial
    /// delay elapses, so several fetches happen.
    ///
    /// Uses small real-time durations to keep the test fast — the absolute
    /// numbers don't matter; what matters is that `initial_delay > timeout`
    /// no longer causes a 1-fetch-then-Timeout sequence.
    #[tokio::test]
    async fn poll_until_initial_delay_does_not_consume_timeout_window() {
        let config = WaitConfig {
            timeout: Duration::from_millis(80),
            poll_interval: Duration::from_millis(10),
            initial_delay: Duration::from_millis(120),
            cleanup: true,
        };
        let counter = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&counter);
        let fetch = move || {
            let c = Arc::clone(&c);
            async move {
                c.fetch_add(1, Ordering::SeqCst);
                Ok::<i32, NifiError>(0)
            }
        };
        let done = |_: &i32| PollOutcome::Pending;
        let err = poll_until(&config, "delayed_op", fetch, done)
            .await
            .unwrap_err();
        assert!(matches!(err, NifiError::Timeout { .. }));
        // 80ms timeout / 10ms tick ≈ 8 polls. Pre-fix would be exactly 1.
        let n = counter.load(Ordering::SeqCst);
        assert!(
            n >= 3,
            "expected several polls within the timeout window after initial_delay, got {n}"
        );
    }

    /// A7: `Some(failure_reason)` is terminal regardless of `complete`.
    /// Previously, a NiFi response with `complete: null` and a populated
    /// `failureReason` was treated as still-pending and the helper timed out.
    #[test]
    fn parameter_context_update_outcome_failure_reason_is_terminal() {
        // complete: None, failureReason: Some
        if let PollOutcome::Failed(NifiError::Api { status, message }) =
            terminal_outcome(None, Some("boom"), "parameter context update")
        {
            assert_eq!(status, STATUS_OPERATION_FAILED);
            assert!(message.contains("boom"));
        } else {
            panic!("expected Failed(Api) for (complete=None, failure=Some)");
        }

        // complete: Some(false), failureReason: Some — also terminal
        if let PollOutcome::Failed(NifiError::Api { message, .. }) =
            terminal_outcome(Some(false), Some("nope"), "parameter context update")
        {
            assert!(message.contains("nope"));
        } else {
            panic!("expected Failed(Api) for (complete=Some(false), failure=Some)");
        }

        // complete: Some(true), failureReason: Some — failure still wins
        if let PollOutcome::Failed(NifiError::Api { message, .. }) =
            terminal_outcome(Some(true), Some("failed-after-complete"), "parameter context update")
        {
            assert!(message.contains("failed-after-complete"));
        } else {
            panic!("expected Failed(Api) for (complete=Some(true), failure=Some)");
        }
    }

    /// A7: `complete: Some(true)` with no failure reason → Ready.
    /// `complete: None | Some(false)` with no failure reason → Pending.
    #[test]
    fn parameter_context_update_outcome_no_failure_reason_paths() {
        assert!(matches!(
            terminal_outcome(Some(true), None, "parameter context update"),
            PollOutcome::Ready
        ));
        assert!(matches!(
            terminal_outcome(Some(false), None, "parameter context update"),
            PollOutcome::Pending
        ));
        assert!(matches!(
            terminal_outcome(None, None, "parameter context update"),
            PollOutcome::Pending
        ));
    }

    #[tokio::test]
    async fn poll_until_honors_initial_delay() {
        let config = WaitConfig {
            timeout: Duration::from_secs(1),
            poll_interval: Duration::from_millis(10),
            initial_delay: Duration::from_millis(30),
            cleanup: true,
        };
        let counter = Arc::new(AtomicUsize::new(0));
        let start = tokio::time::Instant::now();
        let c = Arc::clone(&counter);
        let fetch = move || {
            let c = Arc::clone(&c);
            async move {
                c.fetch_add(1, Ordering::SeqCst);
                Ok::<i32, NifiError>(1)
            }
        };
        let done = |_: &i32| PollOutcome::Ready;
        let _ = poll_until(&config, "op", fetch, done).await.unwrap();
        let elapsed = start.elapsed();
        assert!(
            elapsed >= Duration::from_millis(25),
            "initial_delay not honored, elapsed = {elapsed:?}"
        );
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}

#[cfg(test)]
mod outcome_tests {
    use super::*;

    #[test]
    fn terminal_outcome_pending_when_not_terminal() {
        match terminal_outcome(Some(false), None, "drop request") {
            PollOutcome::Pending => {}
            other => panic!("expected Pending, got {other:?}"),
        }
    }

    #[test]
    fn terminal_outcome_pending_when_none() {
        match terminal_outcome(None, None, "drop request") {
            PollOutcome::Pending => {}
            other => panic!("expected Pending, got {other:?}"),
        }
    }

    #[test]
    fn terminal_outcome_ready_when_terminal() {
        match terminal_outcome(Some(true), None, "drop request") {
            PollOutcome::Ready => {}
            other => panic!("expected Ready, got {other:?}"),
        }
    }

    #[test]
    fn terminal_outcome_failed_when_failure_reason_set() {
        match terminal_outcome(Some(true), Some("queue empty"), "drop request") {
            PollOutcome::Failed(NifiError::Api { status, message }) => {
                assert_eq!(status, 500);
                assert!(message.contains("drop request failed:"));
                assert!(message.contains("queue empty"));
            }
            other => panic!("expected Failed(Api), got {other:?}"),
        }
    }

    #[test]
    fn terminal_outcome_failure_overrides_pending_terminal() {
        match terminal_outcome(Some(false), Some("oops"), "verification") {
            PollOutcome::Failed(_) => {}
            other => panic!("expected Failed, got {other:?}"),
        }
    }
}
