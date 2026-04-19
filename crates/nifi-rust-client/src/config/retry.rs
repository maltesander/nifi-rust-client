#![deny(missing_docs)]
//! Retry policy for transient HTTP errors with exponential backoff.

use std::time::Duration;

/// Configuration for retrying transient errors with exponential backoff.
///
/// When attached to an [`NifiClient`](crate::NifiClient) via
/// [`NifiClientBuilder::retry_policy`](crate::NifiClientBuilder::retry_policy),
/// HTTP helpers will automatically retry requests that fail with
/// [retryable](crate::NifiError::is_retryable) errors.
///
/// # Example
///
/// ```
/// use std::time::Duration;
/// use nifi_rust_client::config::retry::RetryPolicy;
///
/// let policy = RetryPolicy::default();
/// assert_eq!(policy.max_retries, 3);
///
/// // Exponential backoff: 500ms, 1000ms, 2000ms, …
/// assert_eq!(policy.backoff_for(0), Duration::from_millis(500));
/// assert_eq!(policy.backoff_for(1), Duration::from_millis(1000));
/// assert_eq!(policy.backoff_for(2), Duration::from_millis(2000));
///
/// // Capped at max_backoff
/// let policy = RetryPolicy {
///     max_retries: 10,
///     initial_backoff: Duration::from_secs(1),
///     max_backoff: Duration::from_secs(5),
/// };
/// assert_eq!(policy.backoff_for(5), Duration::from_secs(5));
/// ```
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts after the initial request.
    pub max_retries: u32,
    /// Backoff duration for the first retry (attempt 0).
    pub initial_backoff: Duration,
    /// Upper bound on the backoff duration.
    pub max_backoff: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(10),
        }
    }
}

impl RetryPolicy {
    /// Compute the backoff duration for the given retry `attempt` (0-indexed).
    ///
    /// Uses exponential backoff: `initial_backoff * 2^attempt`, capped at
    /// [`max_backoff`](Self::max_backoff).
    pub fn backoff_for(&self, attempt: u32) -> Duration {
        let backoff = self
            .initial_backoff
            .saturating_mul(2u32.saturating_pow(attempt));
        std::cmp::min(backoff, self.max_backoff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_for_handles_overflow() {
        // attempt=1000 would overflow 2^attempt; saturating_pow + saturating_mul
        // must yield max_backoff, not panic.
        let policy = RetryPolicy::default();
        assert_eq!(policy.backoff_for(1000), policy.max_backoff);
    }

    #[test]
    fn default_policy_matches_documented_values() {
        // Pins the documented defaults so changes require a deliberate update.
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_retries, 3);
        assert_eq!(policy.initial_backoff, Duration::from_millis(500));
        assert_eq!(policy.max_backoff, Duration::from_secs(10));
    }

    #[test]
    fn backoff_for_capped_at_max() {
        let policy = RetryPolicy {
            max_retries: 10,
            initial_backoff: Duration::from_secs(1),
            max_backoff: Duration::from_secs(5),
        };
        // 2^5 = 32s but capped to 5s.
        assert_eq!(policy.backoff_for(5), Duration::from_secs(5));
    }
}
