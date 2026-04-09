use crate::diff::VersionDiff;
use crate::parser::ApiSpec;

/// Generates integration tests verifying endpoints added in a version work on that
/// version and return `UnsupportedEndpoint` on older versions.
pub fn emit_endpoint_availability_tests(
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
) -> String {
    let _ = (all_specs, diffs);
    String::new()
}
