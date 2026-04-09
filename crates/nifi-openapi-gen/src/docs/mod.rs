mod api_changes;
pub mod integration_coverage;
mod readme_examples;
mod resource_accessors;
mod versions_table;

pub use api_changes::{format_diff_body, generate_api_changes_content};
pub use integration_coverage::generate_integration_coverage_content;
pub use readme_examples::update_client_readme_examples;
pub use resource_accessors::generate_resource_accessors_content;
pub use versions_table::generate_versions_table_content;
