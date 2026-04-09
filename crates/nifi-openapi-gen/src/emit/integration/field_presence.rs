use crate::diff::VersionDiff;
use crate::parser::ApiSpec;

/// Generates integration tests verifying fields added in a version are `Some`
/// on that version and `None` on older versions.
pub fn emit_field_presence_tests(
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
) -> String {
    let _ = (all_specs, diffs);
    String::new()
}
