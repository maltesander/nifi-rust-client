//! Code generator for [`nifi-rust-client`](https://docs.rs/nifi-rust-client).
//!
//! Published to crates.io solely so that `nifi-rust-client` can consume it
//! as a build-dependency. Not intended for general use.
//!
//! # Stability
//!
//! Only the [`build_api`] module and the top-level [`specs_dir`] function
//! are covered by a stability guarantee. See [`build_api`] for details.
//! All other modules are implementation details, marked `#[doc(hidden)]`,
//! and may change without a semver major bump.

pub mod build_api;

#[doc(hidden)]
pub mod canonical;
#[doc(hidden)]
pub mod content_type;
#[doc(hidden)]
pub mod diff;
#[doc(hidden)]
pub mod docs;
#[doc(hidden)]
pub mod emit;
#[doc(hidden)]
pub mod layout;
#[doc(hidden)]
pub mod naming;
#[doc(hidden)]
pub mod naming_overrides;
#[doc(hidden)]
pub mod non_additive_detector;
#[doc(hidden)]
pub mod non_additive_overrides;
#[doc(hidden)]
pub mod parser;
#[doc(hidden)]
pub mod parser_strict;
#[doc(hidden)]
pub mod plan;
#[doc(hidden)]
pub mod repo;
#[doc(hidden)]
pub mod util;

// ── Backward-compatible flat re-exports ──
// Used by generate.rs binary and the generator's own integration tests.
// These are `#[doc(hidden)]` and not part of the build-dep stability
// contract. Downstream `build.rs` callers must only use `build_api::*`
// and `specs_dir()`.

#[doc(hidden)]
pub use emit::dynamic::{EndpointIndex, emit_dynamic};
#[doc(hidden)]
pub use emit::{
    collect_endpoint_metadata, collect_enum_metadata, collect_query_param_metadata, emit_api,
    emit_api_with_prefix, emit_cli, emit_endpoint_availability_tests, emit_enum_coverage_tests,
    emit_field_presence_tests, emit_query_param_coverage_tests, emit_static_traits, emit_tests,
    emit_types, tested_type_names,
};

#[doc(hidden)]
pub use diff::{
    EndpointChanges, EndpointDiff, EndpointSummary, FieldChange, ParamChange, TypeChanges,
    TypesDiff, VersionDiff, compute_diff,
};

#[doc(hidden)]
pub use parser::{
    ApiSpec, Endpoint, Field, FieldType, HttpMethod, QueryParam, QueryParamType, SubGroup,
    TagGroup, TypeDef, TypeKind, load,
};

/// Absolute path to the bundled OpenAPI specs directory inside this crate.
///
/// `env!("CARGO_MANIFEST_DIR")` is resolved at the time `nifi-openapi-gen` itself
/// is compiled, so this points to the workspace source tree during dev builds and
/// to the registry-cache copy when consumed as a build-dependency from crates.io.
///
/// This function is part of the stable [`build_api`]-tier surface.
pub fn specs_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs")
}
