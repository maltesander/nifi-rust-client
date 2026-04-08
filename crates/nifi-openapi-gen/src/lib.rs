pub mod diff;
pub mod docs;
pub mod emit;
pub mod parser;
pub(crate) mod util;

pub use diff::{
    EndpointChanges, EndpointDiff, EndpointSummary, FieldChange, ParamChange, TypeChanges,
    TypesDiff, VersionDiff, compute_diff,
};
pub use emit::{
    collect_merged_field_names, collect_universal_fields, emit_api, emit_api_with_prefix,
    emit_dynamic, emit_dynamic_conversions, emit_dynamic_dispatch, emit_dynamic_impls,
    emit_dynamic_tests, emit_dynamic_traits, emit_dynamic_types, emit_tests, emit_types,
};
pub use parser::{
    ApiSpec, Endpoint, Field, FieldType, HttpMethod, QueryParam, QueryParamType, SubGroup,
    TagGroup, TypeDef, TypeKind, load,
};

/// Absolute path to the nifi-openapi-gen crate directory.
/// Use this in build.rs to locate spec files:
/// `format!("{}/specs/nifi-api.json", nifi_openapi_gen::SPECS_DIR)`
pub const SPECS_DIR: &str = env!("CARGO_MANIFEST_DIR");
