//! Wait-flag plumbing for nifictl.
//!
//! Holds helpers for parsing `--wait-timeout` and, in later tasks, the
//! pre-/post-dispatch logic for attaching `--wait` semantics to the four
//! supported generated commands.

use std::time::Duration;

use crate::error::CliError;

/// Parse a `--wait-timeout` argument like `"30s"`, `"2m"`, `"1500ms"`.
pub fn parse_wait_timeout(raw: &str) -> Result<Duration, CliError> {
    humantime::parse_duration(raw).map_err(|e| {
        CliError::User(format!("invalid --wait-timeout '{raw}': {e}"))
    })
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
            CliError::User(msg) => assert!(msg.contains("invalid --wait-timeout")),
            other => panic!("expected User error, got {other:?}"),
        }
    }
}
