use std::collections::HashSet;

use crate::diff::VersionDiff;
use crate::layout::RepoLayout;
use crate::parser::{ApiSpec, HttpMethod};
use crate::plan::FileEdit;
use crate::util::wire_to_pascal;

fn method_str(m: &HttpMethod) -> &'static str {
    match m {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Put => "PUT",
        HttpMethod::Delete => "DELETE",
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => {
            let mut out = c.to_uppercase().to_string();
            out.extend(chars);
            out
        }
    }
}

/// Generates the integration coverage section content (between the markers in README.md).
///
/// `all_specs` must be sorted semver-ascending (oldest first).
/// `diffs` must be the consecutive diffs between adjacent versions.
///
/// The `tested_*` parameters contain keys for entries that have actual generated tests:
/// - `tested_types`: type names with field-presence tests (e.g. `"ProcessorEntity"`)
/// - `tested_endpoints`: `"{METHOD} {path}"` keys (e.g. `"POST /processors/{id}/bulletins/clear-requests"`)
/// - `tested_enum_values`: `"{TypeName}::{Variant}"` keys (e.g. `"IncludedRegistries::VersionInfo"`)
/// - `tested_query_params`: `"{METHOD} {path} {param}"` keys (e.g. `"GET /flow/metrics/{producer} flowMetricsReportingStrategy"`)
pub fn generate_integration_coverage_content(
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
    tested_types: &[&str],
    tested_endpoints: &HashSet<String>,
    tested_enum_values: &HashSet<String>,
    tested_query_params: &HashSet<String>,
) -> String {
    if all_specs.len() < 2 || diffs.is_empty() {
        return String::new();
    }

    // Count totals across all diffs
    let version_count = all_specs.len();

    let endpoint_checks: usize = diffs.iter().map(|d| d.endpoints.added.len()).sum();

    let enum_checks: usize = diffs
        .iter()
        .flat_map(|d| &d.endpoints.changed)
        .flat_map(|ec| &ec.changed_params)
        .map(|pc| pc.added_enum_values.len())
        .sum();

    let field_checks: usize = diffs
        .iter()
        .flat_map(|d| &d.types.changed)
        .map(|tc| tc.added_fields.len())
        .sum();

    let param_checks: usize = diffs
        .iter()
        .flat_map(|d| &d.endpoints.changed)
        .map(|ec| ec.added_params.len())
        .sum();

    let mut out = String::new();

    // Count tested items per category
    let tested_endpoint_count = tested_endpoints.len();
    let tested_enum_count = tested_enum_values.len();
    let tested_field_count: usize = diffs
        .iter()
        .flat_map(|d| &d.types.changed)
        .filter(|tc| tested_types.contains(&tc.name.as_str()))
        .map(|tc| tc.added_fields.len())
        .sum();
    let tested_param_count = tested_query_params.len();

    fn summary_part(total: usize, tested: usize, label: &str) -> String {
        if tested > 0 {
            format!("**{total}** {label} ({tested} tested)")
        } else {
            format!("**{total}** {label}")
        }
    }

    // Summary line
    out.push_str(&format!(
        "**{version_count}** NiFi versions tested · {} · {} · {} · {}\n",
        summary_part(
            endpoint_checks,
            tested_endpoint_count,
            "added-endpoint checks"
        ),
        summary_part(enum_checks, tested_enum_count, "enum param checks"),
        summary_part(field_checks, tested_field_count, "field presence checks"),
        summary_part(param_checks, tested_param_count, "query param checks"),
    ));

    // Per-version details blocks
    for diff in diffs {
        out.push_str(&format!(
            "\n<details><summary>{} (vs {})</summary>\n\n",
            diff.to, diff.from
        ));

        // rows: (category, what, tested)
        let mut rows: Vec<(String, String, bool)> = Vec::new();

        // Added endpoints
        for ep in &diff.endpoints.added {
            let key = format!("{} {}", method_str(&ep.method), ep.path);
            let tested = tested_endpoints.contains(&key);
            rows.push(("Added endpoint".to_string(), format!("`{key}`"), tested));
        }

        // Added query params on changed endpoints
        for ec in &diff.endpoints.changed {
            for param in &ec.added_params {
                let key = format!("{} {} {}", method_str(&ec.method), ec.path, param.name);
                let tested = tested_query_params.contains(&key);
                rows.push((
                    "Query param".to_string(),
                    format!("`{} {}` +`{}`", method_str(&ec.method), ec.path, param.name,),
                    tested,
                ));
            }
        }

        // New enum values on changed params
        for ec in &diff.endpoints.changed {
            for pc in &ec.changed_params {
                for wire_value in &pc.added_enum_values {
                    let variant = wire_to_pascal(wire_value);
                    let type_name = capitalize_first(&pc.name);
                    let key = format!("{type_name}::{variant}");
                    let tested = tested_enum_values.contains(&key);
                    rows.push((
                        "Enum value".to_string(),
                        format!("`{key}` accepted"),
                        tested,
                    ));
                }
            }
        }

        // Added fields on changed types
        for tc in &diff.types.changed {
            let tested = tested_types.contains(&tc.name.as_str());
            for field in &tc.added_fields {
                rows.push((
                    "Field presence".to_string(),
                    format!("`{}.{field}`", tc.name),
                    tested,
                ));
            }
        }

        if rows.is_empty() {
            out.push_str("_No tracked integration checks for this version bump._\n");
        } else {
            out.push_str("| Category | What | Tested |\n");
            out.push_str("|----------|------|--------|\n");
            for (category, what, tested) in &rows {
                let check = if *tested { "✓" } else { "" };
                out.push_str(&format!("| {category} | {what} | {check} |\n"));
            }
        }

        out.push_str("\n</details>\n");
    }

    out
}

pub fn emit_integration_coverage(
    layout: &RepoLayout,
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
    tested_types: &[&str],
    tested_endpoints: &HashSet<String>,
    tested_enum_values: &HashSet<String>,
    tested_query_params: &HashSet<String>,
) -> Vec<FileEdit> {
    let content = generate_integration_coverage_content(
        all_specs,
        diffs,
        tested_types,
        tested_endpoints,
        tested_enum_values,
        tested_query_params,
    );
    vec![FileEdit::ReplaceBlock {
        path: layout.client_readme.clone(),
        start_marker: "<!-- INTEGRATION_COVERAGE_START -->".into(),
        end_marker: "<!-- INTEGRATION_COVERAGE_END -->".into(),
        content,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::RepoLayout;
    use crate::plan::FileEdit;
    use std::path::Path;

    #[test]
    fn emit_integration_coverage_returns_one_replace_block() {
        let layout = RepoLayout::from_workspace_root(Path::new("/fake"));
        let edits = emit_integration_coverage(
            &layout,
            &[],
            &[],
            &[],
            &HashSet::new(),
            &HashSet::new(),
            &HashSet::new(),
        );
        assert_eq!(edits.len(), 1);
        assert!(matches!(&edits[0], FileEdit::ReplaceBlock { path, start_marker, .. }
            if *path == Path::new("/fake/crates/nifi-rust-client/README.md")
            && start_marker.contains("INTEGRATION_COVERAGE_START")
        ));
    }
}
