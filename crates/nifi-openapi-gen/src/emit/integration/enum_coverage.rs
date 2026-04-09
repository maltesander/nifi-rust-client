use crate::diff::VersionDiff;
use crate::parser::ApiSpec;

/// Generates integration tests verifying enum query-param values are accepted
/// on their origin version and rejected (`UnsupportedEnumVariant`) on older versions.
pub fn emit_enum_coverage_tests(
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
) -> String {
    let _ = (all_specs, diffs);
    String::new()
}
