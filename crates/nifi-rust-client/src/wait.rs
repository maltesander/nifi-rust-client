#![deny(missing_docs)]
//! Polling helpers for NiFi state transitions and async queries.
//!
//! Each helper polls a single resource until a condition is met, or until
//! [`WaitConfig::timeout`] elapses (returning [`NifiError::Timeout`]).
//!
//! Pattern mirrors [`crate::pagination`]: free functions with a `_dynamic`
//! sibling for use with [`crate::dynamic::DynamicClient`] (gated on the
//! `dynamic` feature).

use std::time::Duration;

use crate::NifiError;

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
    /// For async-query helpers only: if `true`, issue the trailing
    /// `DELETE` to free NiFi-side state after the query resolves,
    /// regardless of success or failure. Ignored by helpers where it
    /// doesn't apply.
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
    /// The server wire value (e.g. `"RUNNING"`). Used by dynamic-mode
    /// helpers for string comparison; not part of the public API shape.
    #[allow(dead_code)]
    pub(crate) fn wire_value(&self) -> &'static str {
        match self {
            Self::Running => "RUNNING",
            Self::Stopped => "STOPPED",
            Self::Disabled => "DISABLED",
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
    #[allow(dead_code)]
    pub(crate) fn wire_value(&self) -> &'static str {
        match self {
            Self::Enabled => "ENABLED",
            Self::Disabled => "DISABLED",
        }
    }
}

// ── Polling primitive ──────────────────────────────────────────────────────

/// Internal outcome of a single poll check.
#[allow(dead_code)]
enum PollOutcome<T> {
    /// State not satisfied yet — keep polling.
    Pending,
    /// Terminal success; return the current value with `Ok`.
    Ready(T),
    /// Terminal failure; return this error without further polling.
    Failed(NifiError),
}

/// Private polling primitive.
///
/// `fetch` returns the current resource state. `done` inspects it and
/// returns one of `PollOutcome::{Pending, Ready(()), Failed(err)}`.
///
/// Deadline is `Instant::now() + config.timeout`. The final sleep is
/// clamped to the remaining time so we don't overshoot.
#[allow(dead_code)]
async fn poll_until<T, FetchFn, FetchFut>(
    config: &WaitConfig,
    operation: &str,
    fetch: FetchFn,
    done: impl Fn(&T) -> PollOutcome<()>,
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
            PollOutcome::Ready(()) => return Ok(value),
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
