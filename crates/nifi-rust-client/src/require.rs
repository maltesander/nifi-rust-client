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

/// Walk a chain of `Option<T>` fields, returning a borrow of the leaf value.
///
/// Expands into nested [`RequireField::require`] calls, stamping each hop's
/// identifier into a dotted path that is carried in the
/// [`NifiError::MissingField`] error on failure.
///
/// # Example
///
/// ```
/// use nifi_rust_client::{NifiError, RequireField, require};
///
/// struct Outer { inner: Option<Inner> }
/// struct Inner { leaf: Option<String> }
///
/// fn example() -> Result<(), NifiError> {
///     let o = Outer { inner: Some(Inner { leaf: Some("v".to_string()) }) };
///     let leaf: &String = require!(o.inner.leaf);
///     assert_eq!(leaf, "v");
///
///     let missing = Outer { inner: Some(Inner { leaf: None }) };
///     let err = (|| -> Result<(), NifiError> {
///         let _leaf: &String = require!(missing.inner.leaf);
///         Ok(())
///     })().unwrap_err();
///     assert!(matches!(err, NifiError::MissingField { path } if path == "inner.leaf"));
///     Ok(())
/// }
/// # example().unwrap();
/// ```
///
/// # Notes
///
/// The macro expands into an expression block that uses the `?` operator,
/// so the surrounding function must return
/// `Result<_, impl From<NifiError>>`. To use it inside a function that
/// returns something else, wrap the call in a closure that returns
/// `Result<_, NifiError>`, as shown above.
#[macro_export]
macro_rules! require {
    ($root:ident $(. $field:ident)+) => {{
        let mut __path = ::std::string::String::new();
        let __v = &$root;
        $(
            if !__path.is_empty() {
                __path.push('.');
            }
            __path.push_str(::core::stringify!($field));
            let __v = $crate::RequireField::require(&__v.$field, &__path)?;
        )+
        __v
    }};
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

    // ── Macro tests ──────────────────────────────────────────────────────
    //
    // Macro tests use locally-defined structs to avoid depending on any
    // generated dynamic DTO. This keeps the suite stable across NiFi spec
    // bumps.

    #[derive(Debug)]
    struct Outer {
        inner: Option<Inner>,
    }
    #[derive(Debug)]
    struct Inner {
        leaf: Option<String>,
    }

    #[test]
    fn macro_single_hop_ok() {
        let o = Outer {
            inner: Some(Inner {
                leaf: Some("v".to_string()),
            }),
        };
        let result: Result<String, NifiError> = (|| {
            let got: &String = crate::require!(o.inner.leaf);
            Ok(got.clone())
        })();
        assert_eq!(result.unwrap(), "v");
    }

    #[test]
    fn macro_missing_outer_reports_outer_path() {
        let o = Outer { inner: None };
        let result: Result<(), NifiError> = (|| {
            let _leaf: &String = crate::require!(o.inner.leaf);
            Ok(())
        })();
        let err = result.unwrap_err();
        match err {
            NifiError::MissingField { path } => assert_eq!(path, "inner"),
            other => panic!("expected MissingField, got {other:?}"),
        }
    }

    #[test]
    fn macro_missing_leaf_reports_full_dotted_path() {
        let o = Outer {
            inner: Some(Inner { leaf: None }),
        };
        let result: Result<(), NifiError> = (|| {
            let _leaf: &String = crate::require!(o.inner.leaf);
            Ok(())
        })();
        let err = result.unwrap_err();
        match err {
            NifiError::MissingField { path } => assert_eq!(path, "inner.leaf"),
            other => panic!("expected MissingField, got {other:?}"),
        }
    }
}
