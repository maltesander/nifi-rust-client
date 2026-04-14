//! Shared method-body emitter used by both static (`emit::api`) and
//! dynamic (`emit::dynamic::api`) emitters. Consolidates doc-comment
//! emission, path-building, query-param serialization, header-arg
//! threading, HTTP helper dispatch, and response-type conversion.
//!
//! Divergence between static and dynamic lives in `EmitMode` and is
//! gated on `match mode` inside the helpers.

use crate::parser::Endpoint;

/// Selects whether the emitted method body targets a static
/// per-version resource struct or the canonical-superset dynamic
/// resource struct.
pub enum EmitMode<'a> {
    /// Static per-version emission. `version` is the semver string
    /// (e.g. `"2.9.0"`); `api_type_path` is the Rust path prefix used
    /// when referencing types from this version's type module
    /// (e.g. `"crate::v2_9_0"`).
    Static {
        version: &'a str,
        api_type_path: &'a str,
    },
    /// Canonical-superset dynamic emission.
    Dynamic,
}

impl<'a> EmitMode<'a> {
    /// Expression used to reach the underlying `NifiClient` from the
    /// current `self`.
    pub fn client_expr(&self) -> &'static str {
        match self {
            EmitMode::Static { .. } => "self.client",
            EmitMode::Dynamic => "self.client.inner()",
        }
    }

    /// Rust path prefix for referencing a type (DTO or enum) from
    /// generated code.
    pub fn type_path(&self, ty_name: &str) -> String {
        match self {
            EmitMode::Static { api_type_path, .. } => {
                format!("{api_type_path}::types::{ty_name}")
            }
            EmitMode::Dynamic => format!("crate::dynamic::types::{ty_name}"),
        }
    }
}

/// Emit the body of a method for the given endpoint into `out`.
///
/// The caller is responsible for emitting the enclosing `impl`
/// block and the struct declaration; this function only appends
/// the method itself (doc comment + signature + body).
///
/// **Currently unimplemented** — Tasks C.7 and C.8 migrate the
/// existing static and dynamic emitters into this function.
pub fn emit_method(_endpoint: &Endpoint, _mode: &EmitMode<'_>, _out: &mut String) {
    unimplemented!("emit_method is filled in by Task C.7 / C.8")
}
