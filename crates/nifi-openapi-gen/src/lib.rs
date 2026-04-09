pub mod diff;
pub mod docs;
pub mod emit;
pub mod parser;
pub mod repo;
pub mod util;

// ── Backward-compatible flat re-exports ──
// Used by generate.rs binary and external test files.

// Emit
pub use emit::{
    collect_merged_field_names, collect_universal_fields, emit_api, emit_api_with_prefix,
    emit_dynamic, emit_dynamic_conversions, emit_dynamic_dispatch, emit_dynamic_impls,
    emit_dynamic_tests, emit_dynamic_traits, emit_dynamic_types, emit_endpoint_availability_tests,
    emit_enum_coverage_tests, emit_field_presence_tests, emit_query_param_coverage_tests,
    emit_static_traits, emit_tests, emit_types, tested_type_names,
};

// Diff
pub use diff::{
    EndpointChanges, EndpointDiff, EndpointSummary, FieldChange, ParamChange, TypeChanges,
    TypesDiff, VersionDiff, compute_diff,
};

// Parser
pub use parser::{
    ApiSpec, Endpoint, Field, FieldType, HttpMethod, QueryParam, QueryParamType, SubGroup,
    TagGroup, TypeDef, TypeKind, load,
};

/// Absolute path to the nifi-openapi-gen crate directory.
pub const SPECS_DIR: &str = env!("CARGO_MANIFEST_DIR");
