mod api;
pub(crate) mod common;
pub mod dynamic;
pub mod integration;
mod tests;
mod traits;
mod types;

pub use api::{emit_api, emit_api_with_prefix};
pub use dynamic::{
    collect_merged_field_names, collect_universal_fields, emit_dynamic, emit_dynamic_conversions,
    emit_dynamic_dispatch, emit_dynamic_impls, emit_dynamic_tests, emit_dynamic_traits,
    emit_dynamic_types,
};
pub use integration::{
    emit_endpoint_availability_tests, emit_enum_coverage_tests, emit_field_presence_tests,
    emit_query_param_coverage_tests, tested_type_names,
};
pub use tests::emit_tests;
pub use traits::emit_static_traits;
pub use types::emit_types;
