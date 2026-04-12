//! Ergonomic extraction of required fields from `Option<T>` values.
//!
//! Dynamic-mode NiFi DTOs carry every field as `Option<T>` because the
//! union of fields across supported versions includes values that may or
//! may not be populated by any given NiFi server. Writing
//! `.ok_or_else(|| ...)` at every hop is tedious and loses the "which
//! field was absent" signal callers need for graceful fallback.
//!
//! This module provides two helpers:
//!
//! - [`RequireField::require`] — a blanket trait method on any `Option<T>`
//!   that returns `Result<&T, NifiError>`.
//! - The [`require!`](crate::require) macro — sugar for walking a chain
//!   of fields, stamping a dotted path into the error automatically.
//!
//! Both work in static and dynamic mode; the helper is not mode-gated.

use crate::error::NifiError;

/// Extension trait for `Option<T>` that returns
/// [`NifiError::MissingField`] instead of panicking or forcing an
/// `.ok_or_else` closure.
///
/// # Example
///
/// ```
/// use nifi_rust_client::{NifiError, RequireField};
///
/// fn example() -> Result<(), NifiError> {
///     let version: Option<String> = Some("2.9.0".to_string());
///     let v: &String = version.require("version")?;
///     assert_eq!(v, "2.9.0");
///     Ok(())
/// }
/// # example().unwrap();
/// ```
pub trait RequireField<T> {
    /// Borrow the inner value, returning [`NifiError::MissingField`] if absent.
    ///
    /// `path` identifies the field in the returned error. Prefer the
    /// [`require!`](crate::require) macro when walking a chain of fields —
    /// it fills in the path automatically.
    fn require(&self, path: &str) -> Result<&T, NifiError>;
}

impl<T> RequireField<T> for Option<T> {
    fn require(&self, path: &str) -> Result<&T, NifiError> {
        self.as_ref().ok_or_else(|| NifiError::MissingField {
            path: path.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn require_some_returns_borrow() {
        let opt: Option<String> = Some("v1".to_string());
        let got: &String = opt.require("version").unwrap();
        assert_eq!(got, "v1");
    }

    #[test]
    fn require_none_returns_missing_field_with_verbatim_path() {
        let opt: Option<String> = None;
        let err = opt.require("about.version").unwrap_err();
        match err {
            NifiError::MissingField { path } => assert_eq!(path, "about.version"),
            other => panic!("expected MissingField, got {other:?}"),
        }
    }

    #[test]
    fn require_does_not_consume_the_option() {
        let opt: Option<String> = Some("kept".to_string());
        let _first = opt.require("x").unwrap();
        let second = opt.require("x").unwrap();
        assert_eq!(second, "kept");
    }
}
