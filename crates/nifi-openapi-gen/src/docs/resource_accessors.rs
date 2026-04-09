use std::collections::BTreeMap;

use crate::parser::{ApiSpec, TagGroup};

/// Counts the total number of endpoints in a tag group (root + sub-group endpoints).
fn tag_endpoint_count(tag: &TagGroup) -> usize {
    tag.root_endpoints.len()
        + tag
            .sub_groups
            .iter()
            .map(|sg| sg.endpoints.len())
            .sum::<usize>()
}

/// Splits a PascalCase tag name into a human-readable description.
/// E.g. "ControllerServices" → "Controller services"
fn tag_to_description(tag: &str) -> String {
    // Split on existing spaces first, then split each PascalCase token
    let mut words = Vec::new();
    for space_token in tag.split_whitespace() {
        let mut current = String::new();
        for ch in space_token.chars() {
            if ch.is_uppercase() && !current.is_empty() {
                words.push(current);
                current = String::new();
            }
            current.push(ch);
        }
        if !current.is_empty() {
            words.push(current);
        }
    }
    if words.is_empty() {
        return String::new();
    }
    let first = words[0].clone();
    let rest: Vec<String> = words[1..].iter().map(|w| w.to_lowercase()).collect();
    if rest.is_empty() {
        first
    } else {
        format!("{} {}", first, rest.join(" "))
    }
}

/// Maximum number of version columns to display.
const MAX_VERSION_COLUMNS: usize = 10;

/// Generates a markdown table of resource accessors with per-version endpoint counts.
/// `all_specs` must be sorted semver-ascending (oldest first).
pub fn generate_resource_accessors_content(all_specs: &[(String, ApiSpec)]) -> String {
    // Cap at 10 most recent versions
    let start = all_specs.len().saturating_sub(MAX_VERSION_COLUMNS);
    let display_specs = &all_specs[start..];

    // Collect the union of all accessor_fn values across all specs, keyed by accessor_fn.
    // Value: (tag name, BTreeMap<version, endpoint_count>)
    let mut accessors: BTreeMap<String, (String, BTreeMap<String, usize>)> = BTreeMap::new();

    for (version, spec) in display_specs.iter() {
        for tag in &spec.tags {
            let entry = accessors
                .entry(tag.accessor_fn.clone())
                .or_insert_with(|| (tag.tag.clone(), BTreeMap::new()));
            entry.1.insert(version.clone(), tag_endpoint_count(tag));
        }
    }

    let versions: Vec<&str> = display_specs.iter().map(|(v, _)| v.as_str()).collect();

    let mut rows = Vec::new();

    // Header
    let mut header = "| Accessor | Resource |".to_string();
    for v in &versions {
        header.push_str(&format!(" {v} |"));
    }
    rows.push(header);

    // Separator
    let mut sep = "|----------|----------|".to_string();
    for _ in &versions {
        sep.push_str("-------|");
    }
    rows.push(sep);

    // Data rows
    for (accessor_fn, (tag, version_counts)) in &accessors {
        let description = tag_to_description(tag);
        let mut row = format!("| `client.{accessor_fn}()` | {description} |");
        for v in &versions {
            match version_counts.get(*v) {
                Some(count) => row.push_str(&format!(" {count} |")),
                None => row.push_str(" \u{2014} |"),
            }
        }
        rows.push(row);
    }

    // Legend
    rows.push(String::new());
    rows.push(
        "> Numbers indicate the endpoint count available for each accessor in that NiFi version. \u{2014} means the accessor is not available in that version.".to_string(),
    );

    rows.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Endpoint, HttpMethod, SubGroup, TagGroup};

    fn make_endpoint(name: &str) -> Endpoint {
        Endpoint {
            method: HttpMethod::Get,
            path: format!("/nifi-api/{name}"),
            fn_name: name.to_string(),
            doc: None,
            description: None,
            path_params: vec![],
            request_type: None,
            body_kind: None,
            body_doc: None,
            response_type: None,
            response_inner: None,
            response_field: None,
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        }
    }

    fn make_tag(tag: &str, accessor_fn: &str, endpoint_count: usize) -> TagGroup {
        TagGroup {
            tag: tag.to_string(),
            struct_name: format!("{tag}Api"),
            module_name: accessor_fn.to_string(),
            accessor_fn: accessor_fn.to_string(),
            types: vec![],
            root_endpoints: (0..endpoint_count)
                .map(|i| make_endpoint(&format!("ep{i}")))
                .collect(),
            sub_groups: vec![],
        }
    }

    #[test]
    fn generates_table_with_version_columns() {
        let specs = vec![
            (
                "2.6.0".to_string(),
                ApiSpec {
                    tags: vec![
                        make_tag("Flow", "flow_api", 15),
                        make_tag("Processors", "processors_api", 8),
                    ],
                    all_types: vec![],
                },
            ),
            (
                "2.7.0".to_string(),
                ApiSpec {
                    tags: vec![
                        make_tag("Flow", "flow_api", 15),
                        make_tag("Processors", "processors_api", 9),
                        make_tag("AiServices", "ai_services_api", 3),
                    ],
                    all_types: vec![],
                },
            ),
        ];

        let table = generate_resource_accessors_content(&specs);

        // Header has version columns
        assert!(table.contains("2.6.0"), "should contain version 2.6.0");
        assert!(table.contains("2.7.0"), "should contain version 2.7.0");

        // Flow row has correct counts in both versions
        assert!(
            table.contains("| `client.flow_api()` | Flow | 15 | 15 |"),
            "flow should show 15 in both versions, got:\n{table}"
        );

        // Processors row shows 8 then 9
        assert!(
            table.contains("| `client.processors_api()` | Processors | 8 | 9 |"),
            "processors should show 8 then 9, got:\n{table}"
        );

        // AiServices only in 2.7.0, so 2.6.0 cell should be em dash
        assert!(
            table.contains("| `client.ai_services_api()` | Ai services | \u{2014} | 3 |"),
            "ai_services should show dash then 3, got:\n{table}"
        );
    }

    #[test]
    fn caps_at_ten_versions() {
        let specs: Vec<(String, ApiSpec)> = (0..12)
            .map(|i| {
                (
                    format!("2.{i}.0"),
                    ApiSpec {
                        tags: vec![make_tag("Flow", "flow_api", 10 + i)],
                        all_types: vec![],
                    },
                )
            })
            .collect();

        let table = generate_resource_accessors_content(&specs);

        // Oldest 2 versions should NOT appear
        assert!(
            !table.contains("2.0.0"),
            "version 2.0.0 should be capped out"
        );
        assert!(
            !table.contains("2.1.0"),
            "version 2.1.0 should be capped out"
        );

        // Version 2.2.0 through 2.11.0 should appear
        assert!(table.contains("2.2.0"), "version 2.2.0 should appear");
        assert!(table.contains("2.11.0"), "version 2.11.0 should appear");
    }

    #[test]
    fn tag_to_description_splits_pascal_case() {
        assert_eq!(tag_to_description("ControllerServices"), "Controller services");
        assert_eq!(tag_to_description("Flow"), "Flow");
        assert_eq!(tag_to_description("AiServices"), "Ai services");
        assert_eq!(
            tag_to_description("ProcessGroupAccess"),
            "Process group access"
        );
        // Tags with existing spaces (from spec)
        assert_eq!(
            tag_to_description("Controller Services"),
            "Controller services"
        );
    }

    #[test]
    fn includes_sub_group_endpoints_in_count() {
        let tag = TagGroup {
            tag: "Processors".to_string(),
            struct_name: "ProcessorsApi".to_string(),
            module_name: "processors".to_string(),
            accessor_fn: "processors_api".to_string(),
            types: vec![],
            root_endpoints: vec![make_endpoint("get_processor")],
            sub_groups: vec![SubGroup {
                name: "run_status".to_string(),
                struct_name: "ProcessorsRunStatus".to_string(),
                accessor_fn: "run_status".to_string(),
                primary_param: "id".to_string(),
                primary_param_doc: None,
                endpoints: vec![
                    make_endpoint("get_run_status"),
                    make_endpoint("set_run_status"),
                ],
            }],
        };

        assert_eq!(tag_endpoint_count(&tag), 3);
    }
}
