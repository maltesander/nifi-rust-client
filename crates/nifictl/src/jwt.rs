//! JWT helpers for nifictl.
//!
//! A JWT is `header.payload.signature`, each segment base64url-encoded
//! without padding. We only care about the `exp` (expiry) claim in the
//! payload — everything else is ignored. Decoding failures return
//! `None` (the caller — `porcelain::login` — silently skips its warning
//! when we can't compute a remaining window).

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

/// Decode the `exp` claim of a JWT and return the remaining time
/// until expiry, measured from `now`. Returns `None` if the token is
/// malformed, if `exp` is missing / wrong type, or if the token has
/// already expired (i.e. `exp < now`).
#[allow(dead_code)]
pub fn expiry_remaining(token: &str, now: SystemTime) -> Option<Duration> {
    let payload = token.split('.').nth(1)?;
    let bytes = URL_SAFE_NO_PAD.decode(payload).ok()?;
    let json: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    let exp = json.get("exp")?.as_i64()?;
    if exp < 0 {
        return None;
    }
    let exp_time = UNIX_EPOCH + Duration::from_secs(exp as u64);
    exp_time.duration_since(now).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_token(payload_json: &str) -> String {
        let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256","typ":"JWT"}"#);
        let payload = URL_SAFE_NO_PAD.encode(payload_json);
        let sig = URL_SAFE_NO_PAD.encode(b"dummy");
        format!("{header}.{payload}.{sig}")
    }

    #[test]
    fn decodes_exp_claim() {
        let now = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let in_one_hour = 1_700_000_000 + 3600;
        let token = make_token(&format!(r#"{{"exp":{in_one_hour},"sub":"admin"}}"#));
        let remaining = expiry_remaining(&token, now).unwrap();
        assert_eq!(remaining, Duration::from_secs(3600));
    }

    #[test]
    fn already_expired_returns_none() {
        let now = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let past = 1_700_000_000 - 60;
        let token = make_token(&format!(r#"{{"exp":{past}}}"#));
        assert!(expiry_remaining(&token, now).is_none());
    }

    #[test]
    fn missing_exp_returns_none() {
        let now = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let token = make_token(r#"{"sub":"admin"}"#);
        assert!(expiry_remaining(&token, now).is_none());
    }

    #[test]
    fn malformed_token_returns_none() {
        let now = UNIX_EPOCH;
        assert!(expiry_remaining("not-a-jwt", now).is_none());
        assert!(expiry_remaining("only.two", now).is_none());
        assert!(expiry_remaining("a.!!!not-base64!!!.c", now).is_none());
    }

    #[test]
    fn non_json_payload_returns_none() {
        let now = UNIX_EPOCH;
        let payload = URL_SAFE_NO_PAD.encode(b"not json");
        let token = format!("a.{payload}.c");
        assert!(expiry_remaining(&token, now).is_none());
    }

    #[test]
    fn exp_wrong_type_returns_none() {
        let now = UNIX_EPOCH;
        let token = make_token(r#"{"exp":"not-a-number"}"#);
        assert!(expiry_remaining(&token, now).is_none());
    }
}
