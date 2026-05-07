//! Cached-token persistence for nifictl.
//!
//! `~/.nifictl/tokens/<context>` holds the JWT obtained by the most
//! recent `nifictl login --context <context>`. Subsequent commands
//! read it via [`read_cached_token`], skip the password round-trip
//! when the JWT's `exp` claim is comfortably in the future, and fall
//! back to a fresh `client.login(...)` otherwise.
//!
//! Token files are written by `porcelain::login::write_token_file`
//! with mode `0o600`. This module reads only — writes happen in the
//! `login` module to keep the write path's permission-handling and
//! diagnostics in one place.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use crate::jwt;

/// Treat tokens as stale if `exp` is within this window of `now`.
/// Re-authentication takes a sub-second round-trip; a 60s skew avoids
/// the rare case where the cached token expires while the request is
/// in flight.
const FRESHNESS_SKEW: Duration = Duration::from_secs(60);

/// Resolve the on-disk cache path for a given context name.
pub(crate) fn cache_path(context_name: &str) -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".nifictl")
        .join("tokens")
        .join(context_name)
}

/// Read a JWT from `path`. Returns `None` if the file is missing,
/// unreadable, or empty. Used both by [`read_cached_token`] (which
/// resolves the path from `HOME`) and by tests that pass a tempdir
/// path directly.
pub(crate) fn read_cached_token_at(path: &Path) -> Option<String> {
    let raw = std::fs::read_to_string(path).ok()?;
    let token = raw.trim();
    if token.is_empty() {
        return None;
    }
    Some(token.to_string())
}

/// Read the cached token for `context_name`. Returns `None` if the
/// file is missing, unreadable, or empty. Callers should follow up
/// with [`is_token_fresh`] before installing the token.
pub(crate) fn read_cached_token(context_name: &str) -> Option<String> {
    read_cached_token_at(&cache_path(context_name))
}

/// `true` if the JWT's `exp` claim is at least [`FRESHNESS_SKEW`]
/// seconds in the future relative to `now`. Returns `false` for
/// malformed tokens, missing `exp`, or already-expired tokens.
pub(crate) fn is_token_fresh(token: &str, now: SystemTime) -> bool {
    match jwt::expiry_remaining(token, now) {
        Some(remaining) => remaining > FRESHNESS_SKEW,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use std::time::UNIX_EPOCH;

    fn jwt_with_exp(exp_secs: i64) -> String {
        let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256"}"#);
        let payload = URL_SAFE_NO_PAD.encode(format!(r#"{{"exp":{exp_secs}}}"#));
        let sig = URL_SAFE_NO_PAD.encode(b"sig");
        format!("{header}.{payload}.{sig}")
    }

    #[test]
    fn fresh_token_is_fresh() {
        let now = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let in_one_hour = 1_700_000_000 + 3600;
        let token = jwt_with_exp(in_one_hour);
        assert!(is_token_fresh(&token, now));
    }

    #[test]
    fn token_within_skew_is_not_fresh() {
        let now = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let in_thirty_seconds = 1_700_000_000 + 30; // < 60s skew
        let token = jwt_with_exp(in_thirty_seconds);
        assert!(!is_token_fresh(&token, now));
    }

    #[test]
    fn expired_token_is_not_fresh() {
        let now = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let past = 1_700_000_000 - 60;
        let token = jwt_with_exp(past);
        assert!(!is_token_fresh(&token, now));
    }

    #[test]
    fn malformed_token_is_not_fresh() {
        let now = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        assert!(!is_token_fresh("garbage", now));
    }
}
