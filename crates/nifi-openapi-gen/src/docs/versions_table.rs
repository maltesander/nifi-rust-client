use crate::diff::SemverBump;
use crate::layout::RepoLayout;
use crate::parser::ApiSpec;
use crate::plan::FileEdit;
use crate::util::version_to_feature;

pub(crate) fn count_spec_endpoints(spec: &ApiSpec) -> usize {
    spec.tags
        .iter()
        .map(|t| {
            t.root_endpoints.len()
                + t.sub_groups
                    .iter()
                    .map(|sg| sg.endpoints.len())
                    .sum::<usize>()
        })
        .sum()
}

pub(crate) fn count_spec_types(spec: &ApiSpec) -> usize {
    spec.all_types.len()
}

/// Generates the Markdown rows for the supported-versions table.
/// `all_specs` must be sorted semver-ascending (oldest first).
/// The table is rendered newest-first so users see the current default at a glance.
pub fn generate_versions_table_content(all_specs: &[(String, ApiSpec)]) -> String {
    let mut rows: Vec<String> = Vec::new();
    rows.push("| NiFi Version | Feature flag | Endpoints | Types | Changes |".to_string());
    rows.push("|---|---|---|---|---|".to_string());

    for (i, (version, spec)) in all_specs.iter().enumerate().rev() {
        let endpoints = count_spec_endpoints(spec);
        let types = count_spec_types(spec);
        let feature = version_to_feature(version);

        let changes = if i == 0 {
            "\u{2014}".to_string()
        } else {
            let (prev_version, prev_spec) = &all_specs[i - 1];
            let diff = crate::compute_diff(prev_spec, spec, prev_version, version);
            let summary = diff.summary();
            if diff.semver_bump() == SemverBump::Major {
                format!("⚠️ breaking · {summary}")
            } else {
                summary
            }
        };

        rows.push(format!(
            "| {version} | `{feature}` | {endpoints} | {types} | {changes} |"
        ));
    }

    rows.join("\n")
}

pub fn emit_versions_table(
    layout: &RepoLayout,
    all_specs: &[(String, ApiSpec)],
) -> Vec<FileEdit> {
    let content = generate_versions_table_content(all_specs);
    vec![
        FileEdit::ReplaceBlock {
            path: layout.workspace_readme.clone(),
            start_marker: "<!-- SUPPORTED_VERSIONS_START -->".into(),
            end_marker: "<!-- SUPPORTED_VERSIONS_END -->".into(),
            content: content.clone(),
        },
        FileEdit::ReplaceBlock {
            path: layout.client_readme.clone(),
            start_marker: "<!-- SUPPORTED_VERSIONS_START -->".into(),
            end_marker: "<!-- SUPPORTED_VERSIONS_END -->".into(),
            content,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emit_versions_table_returns_two_replace_blocks() {
        use crate::layout::RepoLayout;
        use crate::plan::FileEdit;
        use std::path::Path;

        let layout = RepoLayout::from_workspace_root(Path::new("/fake"));
        let edits = emit_versions_table(&layout, &[]);
        assert_eq!(edits.len(), 2);
        assert!(matches!(&edits[0], FileEdit::ReplaceBlock { path, start_marker, .. }
            if *path == Path::new("/fake/README.md")
            && start_marker.contains("SUPPORTED_VERSIONS_START")
        ));
        assert!(matches!(&edits[1], FileEdit::ReplaceBlock { path, .. }
            if *path == Path::new("/fake/crates/nifi-rust-client/README.md")
        ));
    }

    #[test]
    fn test_generate_versions_table_latest_first_and_marked() {
        let specs = vec![
            (
                "2.7.2".to_string(),
                ApiSpec {
                    tags: vec![],
                    all_types: vec![],
                },
            ),
            (
                "2.8.0".to_string(),
                ApiSpec {
                    tags: vec![],
                    all_types: vec![],
                },
            ),
        ];
        let table = generate_versions_table_content(&specs);
        // Latest version appears first in the rendered table
        let pos_new = table.find("2.8.0").unwrap();
        let pos_old = table.find("2.7.2").unwrap();
        assert!(pos_new < pos_old, "latest version should appear first");
        // Both versions have equal counts -> no API changes message
        assert!(
            table.contains("no API changes vs 2.7.2"),
            "expected no-changes message"
        );
        // Oldest shows em dash
        assert!(
            table.contains("| \u{2014} |"),
            "oldest version should show \u{2014}"
        );
        // No Default column
        assert!(
            !table.contains("Default"),
            "table should not have Default column"
        );
    }

    #[test]
    fn test_generate_versions_table_shows_delta() {
        use crate::parser::{Endpoint, HttpMethod, TagGroup};
        fn make_spec(endpoint_count: usize) -> ApiSpec {
            ApiSpec {
                tags: vec![TagGroup {
                    tag: "flow".to_string(),
                    struct_name: "FlowApi".to_string(),
                    module_name: "flow".to_string(),
                    accessor_fn: "flow_api".to_string(),
                    types: vec![],
                    root_endpoints: (0..endpoint_count)
                        .map(|i| Endpoint {
                            method: HttpMethod::Get,
                            path: format!("/nifi-api/flow/ep{i}"),
                            fn_name: format!("get_ep{i}"),
                            doc: None,
                            description: None,
                            path_params: vec![],
                            request_type: None,
                            body_kind: None,
                            body_doc: None,
                            response_type: None,
                            response_inner: None,
                            response_field: None,
                            response_kind: crate::content_type::ResponseBodyKind::Empty,
                            query_params: vec![],
                            success_responses: vec![],
                            error_responses: vec![],
                            security: None,
                        })
                        .collect(),
                    sub_groups: vec![],
                }],
                all_types: vec![],
            }
        }

        let specs = vec![
            ("2.6.0".to_string(), make_spec(10)),
            ("2.7.2".to_string(), make_spec(12)),
        ];
        let table = generate_versions_table_content(&specs);
        assert!(
            table.contains("+2 endpoints vs 2.6.0"),
            "expected +2 endpoints delta"
        );
    }

    #[test]
    fn test_generate_versions_table_uses_diff_summary() {
        use std::path::Path;
        let codegen_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let specs_dir = codegen_dir.join("specs");
        let versions = crate::util::discover_spec_versions(&specs_dir);
        if versions.len() < 2 {
            return; // skip if only one version present
        }
        let all_specs: Vec<(String, ApiSpec)> = versions
            .iter()
            .map(|v| {
                let path = specs_dir.join(v).join("nifi-api.json");
                let spec = crate::load(path.to_str().unwrap());
                (v.clone(), spec)
            })
            .collect();
        let _latest = versions.last().unwrap().as_str();
        let content = generate_versions_table_content(&all_specs);
        // Table has the header and separator rows plus one row per version
        assert!(content.contains("| NiFi Version |"));
        // Baseline row has em dash
        assert!(content.contains("| \u{2014} |"));
        // Non-baseline row has "vs" in changes column
        assert!(content.contains(" vs ") || content.contains("no API changes"));
    }

    #[test]
    fn test_generate_versions_table_breaking_marker() {
        use crate::parser::{Endpoint, HttpMethod, TagGroup};

        fn make_spec_with_endpoints(paths: &[&str]) -> ApiSpec {
            ApiSpec {
                tags: vec![TagGroup {
                    tag: "flow".to_string(),
                    struct_name: "FlowApi".to_string(),
                    module_name: "flow".to_string(),
                    accessor_fn: "flow_api".to_string(),
                    types: vec![],
                    root_endpoints: paths
                        .iter()
                        .map(|p| Endpoint {
                            method: HttpMethod::Get,
                            path: p.to_string(),
                            fn_name: "get_ep".to_string(),
                            doc: None,
                            description: None,
                            path_params: vec![],
                            request_type: None,
                            body_kind: None,
                            body_doc: None,
                            response_type: None,
                            response_inner: None,
                            response_field: None,
                            response_kind: crate::content_type::ResponseBodyKind::Empty,
                            query_params: vec![],
                            success_responses: vec![],
                            error_responses: vec![],
                            security: None,
                        })
                        .collect(),
                    sub_groups: vec![],
                }],
                all_types: vec![],
            }
        }

        // old has an endpoint that new removes → breaking
        let old = make_spec_with_endpoints(&["/nifi-api/flow/a", "/nifi-api/flow/b"]);
        let new = make_spec_with_endpoints(&["/nifi-api/flow/a"]);
        let specs = vec![("2.7.2".to_string(), old), ("2.8.0".to_string(), new)];
        let table = generate_versions_table_content(&specs);
        assert!(
            table.contains("⚠️ breaking"),
            "removed endpoint should produce breaking marker; got:\n{table}"
        );

        // additive only → no breaking marker
        let old2 = make_spec_with_endpoints(&["/nifi-api/flow/a"]);
        let new2 = make_spec_with_endpoints(&["/nifi-api/flow/a", "/nifi-api/flow/b"]);
        let specs2 = vec![("2.7.2".to_string(), old2), ("2.8.0".to_string(), new2)];
        let table2 = generate_versions_table_content(&specs2);
        assert!(
            !table2.contains("⚠️ breaking"),
            "additive-only diff should not have breaking marker; got:\n{table2}"
        );
    }
}
