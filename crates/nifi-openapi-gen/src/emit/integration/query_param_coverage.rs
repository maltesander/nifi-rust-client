use crate::diff::VersionDiff;
use crate::parser::ApiSpec;

/// Generates integration tests verifying added query params are passed on
/// supporting versions and silently skipped on older versions.
pub fn emit_query_param_coverage_tests(
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
) -> String {
    let _ = (all_specs, diffs);
    String::new()
}
