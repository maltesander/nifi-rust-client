//! Dry-run support for nifictl.
//!
//! `CliCtx` is the per-invocation flag context shared by porcelain and
//! generated handlers. `print_dry_run` formats a "would send" block to
//! stdout in the format documented in the Phase 2 spec.

/// Flags resolved once per CLI invocation and threaded through dispatch.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct CliCtx<'a> {
    pub dry_run: bool,
    pub yes: bool,
    /// Base URL, e.g. `"https://nifi:8443"` (no trailing `/nifi-api`).
    pub base_url: &'a str,
}
