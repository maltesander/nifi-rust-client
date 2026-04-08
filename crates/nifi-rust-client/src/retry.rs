/// Configuration for retrying transient errors.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub initial_backoff: std::time::Duration,
    pub max_backoff: std::time::Duration,
}
