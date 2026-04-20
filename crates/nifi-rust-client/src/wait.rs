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
/// Deadline is `Instant::now() + config.timeout`. The final sleep is
/// clamped to the remaining time so we don't overshoot.
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
    let deadline = tokio::time::Instant::now() + config.timeout;

    if !config.initial_delay.is_zero() {
        tokio::time::sleep(config.initial_delay).await;
    }

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
        let complete = req.and_then(|r| r.complete).unwrap_or(false);
        let failure = req.and_then(|r| r.failure_reason.as_ref());
        match (complete, failure) {
            (true, Some(reason)) => PollOutcome::Failed(NifiError::Api {
                status: STATUS_OPERATION_FAILED,
                message: format!("parameter context update failed: {reason}"),
            }),
            (true, None) => PollOutcome::Ready,
            (false, _) => PollOutcome::Pending,
        }
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let _ = client
            .parametercontexts()
            .delete_update_request(context_id, request_id, None)
            .await;
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
        let complete = req.and_then(|r| r.complete).unwrap_or(false);
        let failure = req.and_then(|r| r.failure_reason.as_ref());
        match (complete, failure) {
            (true, Some(reason)) => PollOutcome::Failed(NifiError::Api {
                status: STATUS_OPERATION_FAILED,
                message: format!("parameter context update failed: {reason}"),
            }),
            (true, None) => PollOutcome::Ready,
            (false, _) => PollOutcome::Pending,
        }
    };
    let result = poll_until(&config, &op, fetch, done).await;

    if config.cleanup {
        let _ = client
            .parametercontexts()
            .delete_update_request(context_id, request_id, None)
            .await;
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
        let _ = client.provenance().delete_provenance(query_id, None).await;
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
        let _ = client.provenance().delete_provenance(query_id, None).await;
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
