mod emit_api;
mod emit_dynamic;
mod emit_dynamic_conversions;
mod emit_dynamic_tests;
mod emit_dynamic_types;
mod emit_tests;
mod emit_types;
pub mod parser;
pub(crate) mod util;

pub use emit_api::{emit_api, emit_api_with_prefix};
pub use emit_dynamic::emit_dynamic;
pub use emit_dynamic_conversions::emit_dynamic_conversions;
pub use emit_dynamic_tests::emit_dynamic_tests;
pub use emit_dynamic_types::{
    collect_merged_field_names, collect_universal_fields, emit_dynamic_types,
};
pub use emit_tests::emit_tests;
pub use emit_types::emit_types;
pub use parser::{
    ApiSpec, Endpoint, Field, FieldType, HttpMethod, QueryParam, QueryParamType, SubGroup,
    TagGroup, TypeDef, TypeKind, load,
};

/// Absolute path to the nifi-openapi-gen crate directory.
/// Use this in build.rs to locate spec files:
/// `format!("{}/specs/nifi-api.json", nifi_openapi_gen::SPECS_DIR)`
pub const SPECS_DIR: &str = env!("CARGO_MANIFEST_DIR");
