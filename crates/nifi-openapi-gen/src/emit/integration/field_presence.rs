use crate::diff::VersionDiff;
use crate::parser::ApiSpec;
use crate::util::version_to_feature;

/// Mapping of type names to integration test fetch expressions.
/// Each entry: (type_name, trait_import, fetch_expr, field_access_prefix)
///
/// The fetch_expression must return the type or a container holding it.
/// field_access_prefix navigates from the fetch result to the type instance.
///
/// Currently empty — field presence tests require knowing how to fetch each type
/// from a fresh NiFi, which depends on integration test setup capabilities.
/// Add entries here as test infrastructure grows.
const TESTABLE_TYPES: &[(&str, &str, &str, &str)] = &[];

/// Generates integration tests verifying fields added in a version are `Some`
/// on that version and `None` on older versions.
pub fn emit_field_presence_tests(
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
) -> String {
    if all_specs.is_empty() || diffs.is_empty() {
        return String::new();
    }

    let mut tests: Vec<String> = Vec::new();

    for diff in diffs {
        let to_feature = version_to_feature(&diff.to);

        for tc in &diff.types.changed {
            if tc.added_fields.is_empty() {
                continue;
            }

            let Some((_type_name, trait_import, fetch_expr, field_prefix)) =
                TESTABLE_TYPES.iter().find(|(name, _, _, _)| *name == tc.name)
            else {
                continue;
            };

            for field in &tc.added_fields {
                let snake_field = camel_to_snake(field);
                let test_base = format!(
                    "field_{}_{}",
                    tc.name.to_lowercase(),
                    snake_field
                );

                // Positive: field is Some on the version that added it
                tests.push(format!(
                    r#"#[cfg(feature = "{to_feature}")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {test_base}_present() {{
    use nifi_rust_client::dynamic::traits::{{{trait_import}}};

    let client = helpers::dynamic_logged_in_client().await;
    let item = {fetch_expr};
    assert!(
        {field_prefix}{snake_field}.is_some(),
        "expected {snake_field} to be Some on version {to_ver}"
    );
}}
"#,
                    to_ver = diff.to,
                ));

                // Negative: field is None on older versions
                tests.push(format!(
                    r#"#[cfg(not(feature = "{to_feature}"))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {test_base}_absent() {{
    use nifi_rust_client::dynamic::traits::{{{trait_import}}};

    let client = helpers::dynamic_logged_in_client().await;
    let item = {fetch_expr};
    assert!(
        {field_prefix}{snake_field}.is_none(),
        "expected {snake_field} to be None on version older than {to_ver}"
    );
}}
"#,
                    to_ver = diff.to,
                ));
            }
        }
    }

    if tests.is_empty() {
        return String::new();
    }

    let mut out = String::from("#![cfg(feature = \"dynamic\")]\n\nmod helpers;\n\n");
    out.push_str(&tests.join("\n"));
    out
}

fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}
