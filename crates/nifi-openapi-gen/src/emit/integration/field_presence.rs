use super::overrides::lookup_field_presence_override;
use crate::diff::VersionDiff;
use crate::parser::ApiSpec;
use crate::util::version_to_feature;

/// Mapping of type names to integration test fetch expressions.
/// Each entry: (type_name, fetch_expr, field_access_prefix)
///
/// The fetch_expression must return the type or a container holding it.
/// field_access_prefix navigates from the fetch result to the type instance.
///
/// Only types that can be obtained from the test flow are listed here.
/// The test flow is a GenerateFlowFile → LogAttribute pipeline set up by
/// `helpers::ensure_test_flow`.
const TESTABLE_TYPES: &[(&str, &str, &str)] = &[
    (
        "ProcessorEntity",
        "helpers::get_test_processor_entity(&client).await",
        "item.",
    ),
    (
        "ProvenanceEventDto",
        "helpers::get_test_provenance_event(&client).await",
        "item.",
    ),
];

/// Returns the type names that have entries in TESTABLE_TYPES.
/// Used by the integration coverage doc generator to annotate which
/// field-presence checks have actual test coverage.
pub fn tested_type_names() -> Vec<&'static str> {
    TESTABLE_TYPES.iter().map(|(name, _, _)| *name).collect()
}

/// Generates integration tests verifying fields added in a version are `Some`
/// on that version and `None` on older versions.
pub fn emit_field_presence_tests(all_specs: &[(String, ApiSpec)], diffs: &[VersionDiff]) -> String {
    if all_specs.is_empty() || diffs.is_empty() {
        return String::new();
    }

    let mut tests: Vec<String> = Vec::new();

    // Collect all version features for cumulative gating.
    let all_features: Vec<String> = all_specs
        .iter()
        .map(|(v, _)| version_to_feature(v))
        .collect();

    for diff in diffs {
        let to_feature = version_to_feature(&diff.to);

        // Features for the "to" version and all later versions — the field
        // exists in all of these, so negative tests must exclude them all.
        let supporting_features: Vec<&str> = all_features
            .iter()
            .skip_while(|f| f.as_str() != to_feature)
            .map(|f| f.as_str())
            .collect();
        let negative_cfg = if supporting_features.len() == 1 {
            format!("not(feature = \"{}\")", supporting_features[0])
        } else {
            let any_list: Vec<String> = supporting_features
                .iter()
                .map(|f| format!("feature = \"{f}\""))
                .collect();
            format!("not(any({}))", any_list.join(", "))
        };

        for tc in &diff.types.changed {
            if tc.added_fields.is_empty() {
                continue;
            }

            let Some((_type_name, fetch_expr, field_prefix)) =
                TESTABLE_TYPES.iter().find(|(name, _, _)| *name == tc.name)
            else {
                continue;
            };

            for field in &tc.added_fields {
                let snake_field = camel_to_snake(field);
                let test_base = format!("field_{}_{}", tc.name.to_lowercase(), snake_field);

                // Check whether this field is overridden as conditional-by-design.
                let override_entry = lookup_field_presence_override(&tc.name, &snake_field);

                // Positive: field is Some on the version that added it — unless overridden.
                if let Some(ov) = override_entry {
                    tests.push(format!(
                        "// positive test for `{type_name}.{field_name}` skipped by overrides\n\
                         // reason: {reason}\n",
                        type_name = tc.name,
                        field_name = snake_field,
                        reason = ov.reason,
                    ));
                } else {
                    tests.push(format!(
                        r#"#[cfg(feature = "{to_feature}")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {test_base}_present() {{
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
                }

                // Negative: field is None on older versions
                tests.push(format!(
                    r#"#[cfg({negative_cfg})]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {test_base}_absent() {{
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

    let mut out = String::from(
        "// @generated — do not edit; run `cargo run -p nifi-openapi-gen`\n\n#![cfg(feature = \"dynamic\")]\n\nmod helpers;\n\n",
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff::{EndpointDiff, TypeChanges, TypesDiff, VersionDiff};
    use crate::parser::ApiSpec;

    fn empty_spec() -> ApiSpec {
        ApiSpec {
            tags: vec![],
            all_types: vec![],
        }
    }

    #[test]
    fn empty_inputs_returns_empty_string() {
        let result = emit_field_presence_tests(&[], &[]);
        assert_eq!(result, "");
    }

    #[test]
    fn no_matching_types_returns_empty_string() {
        let specs = vec![
            ("2.7.2".to_string(), empty_spec()),
            ("2.8.0".to_string(), empty_spec()),
        ];
        let diffs = vec![VersionDiff {
            from: "2.7.2".to_string(),
            to: "2.8.0".to_string(),
            endpoints: EndpointDiff {
                added: vec![],
                removed: vec![],
                changed: vec![],
            },
            types: TypesDiff {
                added: vec![],
                removed: vec![],
                changed: vec![TypeChanges {
                    name: "UnknownType".to_string(),
                    added_fields: vec!["some_field".to_string()],
                    removed_fields: vec![],
                    changed_fields: vec![],
                    added_variants: vec![],
                    removed_variants: vec![],
                }],
            },
        }];
        let result = emit_field_presence_tests(&specs, &diffs);
        assert_eq!(result, "");
    }

    #[test]
    fn generates_tests_for_testable_type() {
        let specs = vec![
            ("2.6.0".to_string(), empty_spec()),
            ("2.7.2".to_string(), empty_spec()),
            ("2.8.0".to_string(), empty_spec()),
        ];
        let diffs = vec![VersionDiff {
            from: "2.6.0".to_string(),
            to: "2.7.2".to_string(),
            endpoints: EndpointDiff {
                added: vec![],
                removed: vec![],
                changed: vec![],
            },
            types: TypesDiff {
                added: vec![],
                removed: vec![],
                changed: vec![TypeChanges {
                    name: "ProcessorEntity".to_string(),
                    added_fields: vec!["physicalState".to_string()],
                    removed_fields: vec![],
                    changed_fields: vec![],
                    added_variants: vec![],
                    removed_variants: vec![],
                }],
            },
        }];
        let result = emit_field_presence_tests(&specs, &diffs);
        assert!(result.contains("field_processorentity_physical_state_present"));
        assert!(result.contains("field_processorentity_physical_state_absent"));
        assert!(result.contains("#[cfg(feature = \"nifi-2-7-2\")]"));
    }

    #[test]
    fn cumulative_gating_excludes_later_versions() {
        let specs = vec![
            ("2.6.0".to_string(), empty_spec()),
            ("2.7.2".to_string(), empty_spec()),
            ("2.8.0".to_string(), empty_spec()),
        ];
        let diffs = vec![VersionDiff {
            from: "2.6.0".to_string(),
            to: "2.7.2".to_string(),
            endpoints: EndpointDiff {
                added: vec![],
                removed: vec![],
                changed: vec![],
            },
            types: TypesDiff {
                added: vec![],
                removed: vec![],
                changed: vec![TypeChanges {
                    name: "ProcessorEntity".to_string(),
                    added_fields: vec!["physicalState".to_string()],
                    removed_fields: vec![],
                    changed_fields: vec![],
                    added_variants: vec![],
                    removed_variants: vec![],
                }],
            },
        }];
        let result = emit_field_presence_tests(&specs, &diffs);
        assert!(
            result.contains("not(any(feature = \"nifi-2-7-2\", feature = \"nifi-2-8-0\"))"),
            "negative test should exclude 2.7.2 AND 2.8.0, got:\n{result}"
        );
    }

    #[test]
    fn tested_type_names_returns_all() {
        let names = tested_type_names();
        assert!(names.contains(&"ProcessorEntity"));
        assert!(names.contains(&"ProvenanceEventDto"));
    }

    #[test]
    fn overridden_field_omits_positive_test_but_keeps_negative() {
        let specs = vec![
            ("2.8.0".to_string(), empty_spec()),
            ("2.9.0".to_string(), empty_spec()),
        ];
        let diffs = vec![VersionDiff {
            from: "2.8.0".to_string(),
            to: "2.9.0".to_string(),
            endpoints: EndpointDiff {
                added: vec![],
                removed: vec![],
                changed: vec![],
            },
            types: TypesDiff {
                added: vec![],
                removed: vec![],
                changed: vec![TypeChanges {
                    name: "ProvenanceEventDto".to_string(),
                    added_fields: vec!["connector_id".to_string()],
                    removed_fields: vec![],
                    changed_fields: vec![],
                    added_variants: vec![],
                    removed_variants: vec![],
                }],
            },
        }];
        let result = emit_field_presence_tests(&specs, &diffs);
        assert!(
            !result.contains("field_provenanceeventdto_connector_id_present"),
            "positive test must be skipped for overridden field, got:\n{result}"
        );
        assert!(
            result.contains("field_provenanceeventdto_connector_id_absent"),
            "negative test must still be emitted, got:\n{result}"
        );
        assert!(
            result.contains("skipped by overrides"),
            "generated file should explain why, got:\n{result}"
        );
    }

    #[test]
    fn camel_to_snake_works() {
        assert_eq!(camel_to_snake("eventTimestamp"), "event_timestamp");
        assert_eq!(camel_to_snake("physicalState"), "physical_state");
        assert_eq!(camel_to_snake("id"), "id");
    }
}
