use crate::diff::{FieldChangeKind, VersionDiff};
use crate::parser::ApiSpec;

pub fn format_diff_body(diff: &VersionDiff) -> String {
    let mut out = String::new();

    if !diff.endpoints.added.is_empty() {
        out.push_str("### Added endpoints\n\n");
        for ep in &diff.endpoints.added {
            let doc = ep
                .doc
                .as_deref()
                .map(|d| format!(" \u{2014} {d}"))
                .unwrap_or_default();
            out.push_str(&format!(
                "- `{} {}`{} ({})\n",
                ep.method.as_str(),
                ep.path,
                doc,
                ep.tag,
            ));
        }
        out.push('\n');
    }

    if !diff.endpoints.removed.is_empty() {
        out.push_str("### Removed endpoints\n\n");
        for ep in &diff.endpoints.removed {
            out.push_str(&format!(
                "- `{} {}` ({})\n",
                ep.method.as_str(),
                ep.path,
                ep.tag,
            ));
        }
        out.push('\n');
    }

    let meaningful_changes: Vec<&crate::diff::EndpointChanges> = diff
        .endpoints
        .changed
        .iter()
        .filter(|ec| {
            !ec.added_params.is_empty()
                || !ec.removed_params.is_empty()
                || !ec.changed_params.is_empty()
                || !ec.added_path_params.is_empty()
                || !ec.removed_path_params.is_empty()
        })
        .collect();

    if !meaningful_changes.is_empty() {
        out.push_str("### Changed endpoints\n\n");
        for ec in meaningful_changes {
            let mut parts = Vec::new();
            if !ec.added_params.is_empty() {
                parts.push(format!("added params: `{}`", ec.added_params.join("`, `")));
            }
            if !ec.removed_params.is_empty() {
                parts.push(format!(
                    "removed params: `{}`",
                    ec.removed_params.join("`, `")
                ));
            }
            for pc in &ec.changed_params {
                if !pc.added_enum_values.is_empty() {
                    parts.push(format!(
                        "param `{}`: +enum values (`{}`)",
                        pc.name,
                        pc.added_enum_values.join("`, `")
                    ));
                }
                if !pc.removed_enum_values.is_empty() {
                    parts.push(format!(
                        "param `{}`: -enum values (`{}`)",
                        pc.name,
                        pc.removed_enum_values.join("`, `")
                    ));
                }
            }
            if !ec.added_path_params.is_empty() {
                parts.push(format!(
                    "added path params: `{}`",
                    ec.added_path_params.join("`, `")
                ));
            }
            if !ec.removed_path_params.is_empty() {
                parts.push(format!(
                    "removed path params: `{}`",
                    ec.removed_path_params.join("`, `")
                ));
            }
            out.push_str(&format!(
                "- `{} {}` \u{2014} {}\n",
                ec.method.as_str(),
                ec.path,
                parts.join("; ")
            ));
        }
        out.push('\n');
    }

    if !diff.types.added.is_empty() {
        out.push_str("### Added types\n\n");
        for t in &diff.types.added {
            out.push_str(&format!("- `{t}`\n"));
        }
        out.push('\n');
    }

    if !diff.types.removed.is_empty() {
        out.push_str("### Removed types\n\n");
        for t in &diff.types.removed {
            out.push_str(&format!("- `{t}`\n"));
        }
        out.push('\n');
    }

    let meaningful_types: Vec<&crate::diff::TypeChanges> = diff
        .types
        .changed
        .iter()
        .filter(|tc| {
            !tc.added_fields.is_empty()
                || !tc.removed_fields.is_empty()
                || !tc.changed_fields.is_empty()
                || !tc.added_variants.is_empty()
                || !tc.removed_variants.is_empty()
        })
        .collect();

    if !meaningful_types.is_empty() {
        out.push_str("### Changed types\n\n");
        for tc in meaningful_types {
            let mut parts = Vec::new();
            if !tc.added_fields.is_empty() {
                parts.push(format!("added fields: `{}`", tc.added_fields.join("`, `")));
            }
            if !tc.removed_fields.is_empty() {
                parts.push(format!(
                    "removed fields: `{}`",
                    tc.removed_fields.join("`, `")
                ));
            }
            for fc in &tc.changed_fields {
                let desc = match &fc.kind {
                    FieldChangeKind::BecameOptional => "became optional".to_string(),
                    FieldChangeKind::BecameRequired => "became required".to_string(),
                };
                parts.push(format!("`{}` {}", fc.name, desc));
            }
            if !tc.added_variants.is_empty() {
                parts.push(format!(
                    "added variants: `{}`",
                    tc.added_variants.join("`, `")
                ));
            }
            if !tc.removed_variants.is_empty() {
                parts.push(format!(
                    "removed variants: `{}`",
                    tc.removed_variants.join("`, `")
                ));
            }
            out.push_str(&format!("- `{}` \u{2014} {}\n", tc.name, parts.join("; ")));
        }
        out.push('\n');
    }

    if out.is_empty() {
        out.push_str("_No API changes._\n\n");
    }

    out
}

/// Generates the full body of NIFI_API_CHANGES.md (content between the markers).
/// `all_specs` must be sorted semver-ascending (oldest first).
pub fn generate_api_changes_content(all_specs: &[(String, ApiSpec)]) -> String {
    if all_specs.len() < 2 {
        return String::new();
    }

    let (baseline_version, baseline_spec) = &all_specs[0];
    let mut out = String::new();

    // Iterate newest to oldest, skipping baseline
    for i in (1..all_specs.len()).rev() {
        let (version, spec) = &all_specs[i];
        let (prev_version, prev_spec) = &all_specs[i - 1];

        out.push_str(&format!("## {version}\n\n"));

        // Consecutive diff
        let consec = crate::compute_diff(prev_spec, spec, prev_version, version);
        out.push_str(&format!(
            "<details><summary>vs {prev_version} (consecutive)</summary>\n\n"
        ));
        out.push_str(&format_diff_body(&consec));
        out.push_str("</details>\n\n");

        // Cumulative diff — only when there is more than one step from baseline
        if i > 1 {
            let cumul = crate::compute_diff(baseline_spec, spec, baseline_version, version);
            out.push_str(&format!(
                "<details><summary>vs {baseline_version} (cumulative)</summary>\n\n"
            ));
            out.push_str(&format_diff_body(&cumul));
            out.push_str("</details>\n\n");
        }
    }

    out.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_api_changes_content_shape() {
        use std::path::Path;
        let codegen_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let specs_dir = codegen_dir.join("specs");
        let versions = crate::util::discover_spec_versions(&specs_dir);
        if versions.len() < 2 {
            return;
        }
        let all_specs: Vec<(String, ApiSpec)> = versions
            .iter()
            .map(|v| {
                let path = specs_dir.join(v).join("nifi-api.json");
                let spec = crate::load(path.to_str().unwrap());
                (v.clone(), spec)
            })
            .collect();

        let content = generate_api_changes_content(&all_specs);

        // Newest version heading present
        let newest = versions.last().unwrap();
        assert!(
            content.contains(&format!("## {newest}")),
            "missing heading for {newest}"
        );

        // Baseline version has no heading
        let oldest = versions.first().unwrap();
        assert!(
            !content.contains(&format!("## {oldest}")),
            "baseline should have no heading"
        );

        // Contains collapsible details blocks
        assert!(content.contains("<details>"));
        assert!(content.contains("</details>"));
        assert!(content.contains("(consecutive)"));

        // Newest version with 3+ versions also has cumulative block
        if versions.len() >= 3 {
            assert!(content.contains("(cumulative)"));
        }
    }
}
