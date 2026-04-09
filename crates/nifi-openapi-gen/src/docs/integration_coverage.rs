use crate::diff::VersionDiff;
use crate::parser::{ApiSpec, HttpMethod};
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
pub fn generate_integration_coverage_content(
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
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

    // Summary line
    out.push_str(&format!(
        "**{version_count}** NiFi versions tested · **{endpoint_checks}** added-endpoint checks · **{enum_checks}** enum param checks · **{field_checks}** field presence checks · **{param_checks}** query param checks\n"
    ));

    // Per-version details blocks
    for diff in diffs {
        out.push_str(&format!(
            "\n<details><summary>{} (vs {})</summary>\n\n",
            diff.to, diff.from
        ));

        let mut rows: Vec<(String, String)> = Vec::new();

        // Added endpoints
        for ep in &diff.endpoints.added {
            rows.push((
                "Added endpoint".to_string(),
                format!("`{} {}`", method_str(&ep.method), ep.path),
            ));
        }

        // Added query params on changed endpoints
        for ec in &diff.endpoints.changed {
            for param in &ec.added_params {
                rows.push((
                    "Query param".to_string(),
                    format!(
                        "`{} {}` +`{param}`",
                        method_str(&ec.method),
                        ec.path
                    ),
                ));
            }
        }

        // New enum values on changed params
        for ec in &diff.endpoints.changed {
            for pc in &ec.changed_params {
                for wire_value in &pc.added_enum_values {
                    let variant = wire_to_pascal(wire_value);
                    let type_name = capitalize_first(&pc.name);
                    rows.push((
                        "Enum value".to_string(),
                        format!("`{type_name}::{variant}` accepted"),
                    ));
                }
            }
        }

        // Added fields on changed types
        for tc in &diff.types.changed {
            for field in &tc.added_fields {
                rows.push((
                    "Field presence".to_string(),
                    format!("`{}.{field}`", tc.name),
                ));
            }
        }

        if rows.is_empty() {
            out.push_str("_No tracked integration checks for this version bump._\n");
        } else {
            out.push_str("| Category | What |\n");
            out.push_str("|----------|------|\n");
            for (category, what) in &rows {
                out.push_str(&format!("| {category} | {what} |\n"));
            }
        }

        out.push_str("\n</details>\n");
    }

    out
}
