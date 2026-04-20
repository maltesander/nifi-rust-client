//! Dry-run support for nifictl.
//!
//! `CliCtx` is the per-invocation flag context shared by porcelain and
//! generated handlers. `print_dry_run` formats a "would send" block to
//! stdout in the format documented in the Phase 2 spec.

use std::io::{self, Write};

use serde_json::Value;

/// Flags resolved once per CLI invocation and threaded through dispatch.
#[derive(Debug, Clone, Copy)]
pub struct CliCtx<'a> {
    pub dry_run: bool,
    pub yes: bool,
    /// Base URL, e.g. `"https://nifi:8443"` (no trailing `/nifi-api`).
    pub base_url: &'a str,
}

/// Build the full URL that the real request would hit.
///
/// `path` is the endpoint path with path params already substituted
/// (e.g. `"/flow/process-groups/abc-123"`). Appends `/nifi-api` and
/// joins any non-empty query.
pub fn format_url(base_url: &str, path: &str, query: &[(&str, String)]) -> String {
    let base = base_url.trim_end_matches('/');
    let mut url = format!("{base}/nifi-api{path}");
    if !query.is_empty() {
        url.push('?');
        let encoded: Vec<String> = query
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect();
        url.push_str(&encoded.join("&"));
    }
    url
}

/// Print a "DRY RUN — would send:" block to `writer`.
///
/// * `method` is one of `"GET" | "POST" | "PUT" | "PATCH" | "DELETE"`.
/// * `body` is `None` for DELETE / empty-body requests, `Some(Value)` otherwise.
pub fn print(
    writer: &mut dyn Write,
    method: &str,
    url: &str,
    body: Option<&Value>,
) -> io::Result<()> {
    writeln!(writer, "DRY RUN — would send:")?;
    writeln!(writer, "  {method} {url}")?;
    if let Some(body) = body {
        writeln!(writer, "  Body:")?;
        let pretty = serde_json::to_string_pretty(body).unwrap_or_else(|_| body.to_string());
        for line in pretty.lines() {
            writeln!(writer, "  {line}")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_url_no_query() {
        let url = format_url("https://nifi:8443", "/flow/process-groups/abc-123", &[]);
        assert_eq!(
            url,
            "https://nifi:8443/nifi-api/flow/process-groups/abc-123"
        );
    }

    #[test]
    fn format_url_strips_trailing_slash() {
        let url = format_url("https://nifi:8443/", "/flow/about", &[]);
        assert_eq!(url, "https://nifi:8443/nifi-api/flow/about");
    }

    #[test]
    fn format_url_with_query() {
        let url = format_url(
            "https://nifi:8443",
            "/flow/history",
            &[("offset", "0".to_string()), ("count", "50".to_string())],
        );
        assert_eq!(
            url,
            "https://nifi:8443/nifi-api/flow/history?offset=0&count=50"
        );
    }

    #[test]
    fn format_url_url_encodes_query_values() {
        let url = format_url("https://nifi:8443", "/foo", &[("q", "a b&c".to_string())]);
        assert_eq!(url, "https://nifi:8443/nifi-api/foo?q=a%20b%26c");
    }

    #[test]
    fn print_delete_no_body() {
        let mut buf = Vec::new();
        print(
            &mut buf,
            "DELETE",
            "https://nifi:8443/nifi-api/processors/p1",
            None,
        )
        .unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert_eq!(
            s,
            "DRY RUN — would send:\n  DELETE https://nifi:8443/nifi-api/processors/p1\n"
        );
    }

    #[test]
    fn print_put_with_body_pretty_prints() {
        let mut buf = Vec::new();
        let body = json!({ "id": "abc-123", "state": "RUNNING" });
        print(
            &mut buf,
            "PUT",
            "https://nifi:8443/nifi-api/flow/process-groups/abc-123",
            Some(&body),
        )
        .unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.starts_with("DRY RUN — would send:\n  PUT https://nifi:8443/nifi-api/flow/process-groups/abc-123\n  Body:\n"));
        assert!(s.contains("  \"id\": \"abc-123\""));
        assert!(s.contains("  \"state\": \"RUNNING\""));
    }
}
