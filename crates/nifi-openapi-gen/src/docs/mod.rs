mod api_changes;
pub mod integration_coverage;
mod readme_examples;
mod resource_accessors;
mod versions_table;

pub use api_changes::{emit_api_changes, format_diff_body, generate_api_changes_content};
pub use integration_coverage::{emit_integration_coverage, generate_integration_coverage_content};
pub use readme_examples::{emit_client_readme_examples, update_client_readme_examples};
pub use resource_accessors::{emit_resource_accessors, generate_resource_accessors_content};
pub use versions_table::{emit_versions_table, generate_versions_table_content};
