use std::collections::HashSet;

use super::common::{
    build_accessor, default_path_param_value, default_required_query_param_value, find_endpoint,
};
use super::overrides::{EndpointBehavior, lookup_endpoint_override};
use crate::diff::{EndpointSummary, VersionDiff};
use crate::parser::{ApiSpec, Endpoint, HttpMethod};
use crate::util::version_to_feature;

/// Generates integration tests verifying endpoints added in a version work on that
/// version and return `UnsupportedEndpoint` on older versions.
///
/// Returns `(generated_code, tested_keys)` where tested_keys contains
/// `"{METHOD} {path}"` strings for each endpoint that got at least one test.
pub fn emit_endpoint_availability_tests(
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
) -> (String, HashSet<String>) {
    if all_specs.is_empty() || diffs.is_empty() {
        return (String::new(), HashSet::new());
    }

    let mut tests: Vec<String> = Vec::new();
    let mut tested: HashSet<String> = HashSet::new();

    // Collect all version features for cumulative gating.
    let all_features: Vec<String> = all_specs
        .iter()
        .map(|(v, _)| version_to_feature(v))
        .collect();

    for diff in diffs {
        if diff.endpoints.added.is_empty() {
            continue;
        }

        let to_feature = version_to_feature(&diff.to);

        // Features for the "to" version and all later versions — the endpoint
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

        // Find the "to" spec so we can look up endpoint metadata.
        let to_spec = match all_specs.iter().find(|(v, _)| v == &diff.to) {
            Some((_, spec)) => spec,
            None => continue,
        };

        for added in &diff.endpoints.added {
            let ep_info = find_endpoint(to_spec, &added.method, &added.path);
            let (endpoint, tag_group) = match ep_info {
                Some(info) => info,
                None => continue,
            };

            let base_name = test_base_name(&tag_group.accessor_fn, &endpoint.fn_name);
            let accessor = build_accessor(&tag_group.accessor_fn);
            let call_args = build_call_args(endpoint);

            // Track that this endpoint has at least one test (the negative test
            // is always generated below, so every endpoint that reaches here is tested).
            tested.insert(endpoint_key(&added.method, &added.path));

            // Consult the override registry. Overrides take precedence over
            // the is_safe_endpoint heuristic — a tag-wide skip suppresses the
            // positive test even for GET-with-id endpoints that would
            // otherwise be emitted.
            let override_entry =
                lookup_endpoint_override(&added.method, &added.path, &tag_group.tag);

            // Positive test — only for safe endpoints that aren't overridden.
            match override_entry {
                Some(ov) => match &ov.behavior {
                    EndpointBehavior::SkipPositiveTest { reason } => {
                        tests.push(format!(
                            "// positive test for `{method:?} {path}` skipped by overrides\n\
                             // reason: {reason}",
                            method = added.method,
                            path = added.path,
                            reason = reason,
                        ));
                    }
                },
                None if is_safe_endpoint(added, endpoint) => {
                    let positive_test = format!(
                        r#"#[cfg(feature = "{to_feature}")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {base_name}_available() {{
    let client = helpers::dynamic_logged_in_client().await;
    let result = client.{accessor}.{fn_name}({call_args}).await;
    assert!(result.is_ok(), "expected endpoint to be available, got: {{:?}}", result.unwrap_err());
}}"#,
                        to_feature = to_feature,
                        base_name = base_name,
                        accessor = accessor,
                        fn_name = endpoint.fn_name,
                        call_args = call_args,
                    );
                    tests.push(positive_test);
                }
                None => {}
            }

            // Negative test — only on versions that lack this endpoint.
            let negative_test = format!(
                r#"#[cfg({negative_cfg})]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn {base_name}_unsupported() {{
    let client = helpers::dynamic_logged_in_client().await;
    let result = client.{accessor}.{fn_name}({call_args}).await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint {{ .. }}),
        "expected UnsupportedEndpoint, got: {{err:?}}"
    );
}}"#,
                negative_cfg = negative_cfg,
                base_name = base_name,
                accessor = accessor,
                fn_name = endpoint.fn_name,
                call_args = call_args,
            );
            tests.push(negative_test);
        }
    }

    if tests.is_empty() {
        return (String::new(), tested);
    }

    let mut out = String::new();
    out.push_str("// @generated — do not edit; run `cargo run -p nifi-openapi-gen`\n\n");
    out.push_str("#![cfg(feature = \"dynamic\")]\n\n");
    out.push_str("mod helpers;\n\n");
    for test in tests {
        out.push_str(&test);
        out.push_str("\n\n");
    }
    (out, tested)
}

/// Build a key matching the format used by integration_coverage.rs for endpoints.
fn endpoint_key(method: &HttpMethod, path: &str) -> String {
    let m = match method {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Put => "PUT",
        HttpMethod::Delete => "DELETE",
    };
    format!("{m} {path}")
}

// ─── Private helpers ──────────────────────────────────────────────────────────

/// Determine whether it's safe to call an endpoint positively in a test.
///
/// Safe means: GET with no required resource-specific path params,
/// or POST on a process-groups path where we can use "root" as the ID.
fn is_safe_endpoint(summary: &EndpointSummary, endpoint: &Endpoint) -> bool {
    // Only GET endpoints without resource-specific path params are safe.
    // POST/PUT/DELETE may have required body fields that Default::default()
    // cannot satisfy, causing MissingRequiredField errors.
    if summary.method != HttpMethod::Get {
        return false;
    }

    endpoint.path_params.iter().all(|pp| {
        matches!(
            pp.name.as_str(),
            "id" | "process_group_id" | "parent_group_id" | "producer"
        )
    })
}

/// Build the argument list for the function call.
/// All path params are passed as regular arguments in declaration order.
fn build_call_args(endpoint: &Endpoint) -> String {
    let mut args: Vec<String> = Vec::new();

    // Path params — all passed in declaration order.
    for pp in &endpoint.path_params {
        let val = default_path_param_value(&pp.name);
        args.push(format!("\"{val}\""));
    }

    // Add request body (if any).
    match &endpoint.body_kind {
        Some(crate::parser::RequestBodyKind::Json) => {
            args.push("&Default::default()".to_string());
        }
        Some(crate::parser::RequestBodyKind::OctetStream) => {
            args.push("None".to_string()); // filename
            args.push("vec![]".to_string()); // data
        }
        Some(crate::parser::RequestBodyKind::Multipart) => {
            args.push("\"test.xml\"".to_string()); // filename
            args.push("vec![]".to_string()); // data
        }
        Some(crate::parser::RequestBodyKind::Wildcard)
        | Some(crate::parser::RequestBodyKind::FormEncoded)
        | None => {}
    }

    // Add query params — all None (optional) or a default for required ones.
    for qp in &endpoint.query_params {
        if qp.required {
            args.push(default_required_query_param_value(&qp.ty));
        } else {
            args.push("None".to_string());
        }
    }

    args.join(",\n        ")
}

/// Collect the set of tested endpoint keys without generating code.
pub fn collect_endpoint_metadata(
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
) -> HashSet<String> {
    let (_, tested) = emit_endpoint_availability_tests(all_specs, diffs);
    tested
}

/// Produce the test function base name, e.g. `endpoint_flow_get_listen_ports`.
fn test_base_name(accessor_fn: &str, fn_name: &str) -> String {
    format!("endpoint_{accessor_fn}_{fn_name}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff::{EndpointDiff, EndpointSummary, TypesDiff, VersionDiff};
    use crate::parser::{ApiSpec, Endpoint, HttpMethod, PathParam, TagGroup};

    fn empty_spec() -> ApiSpec {
        ApiSpec {
            tags: vec![],
            all_types: vec![],
        }
    }

    fn single_get_endpoint(path: &str, fn_name: &str) -> Endpoint {
        Endpoint {
            method: HttpMethod::Get,
            path: path.to_string(),
            fn_name: fn_name.to_string(),
            raw_operation_id: String::new(),
            doc: None,
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            request_type: None,
            body_kind: None,
            body_doc: None,
            response_type: None,
            response_inner: None,
            response_field: None,
            response_kind: crate::content_type::ResponseBodyKind::Empty,
            query_params: vec![],
            header_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        }
    }

    fn spec_with_tag(tag: &str, endpoint: Endpoint) -> ApiSpec {
        ApiSpec {
            tags: vec![TagGroup {
                tag: tag.to_string(),
                struct_name: tag.to_string(),
                module_name: tag.to_lowercase(),
                accessor_fn: tag.to_lowercase(),
                types: vec![],
                endpoints: vec![endpoint],
            }],
            all_types: vec![],
        }
    }

    fn diff_with_added_endpoint(
        from: &str,
        to: &str,
        method: HttpMethod,
        path: &str,
        tag: &str,
    ) -> VersionDiff {
        VersionDiff {
            from: from.to_string(),
            to: to.to_string(),
            endpoints: EndpointDiff {
                added: vec![EndpointSummary {
                    method,
                    path: path.to_string(),
                    tag: tag.to_string(),
                    doc: None,
                }],
                removed: vec![],
                changed: vec![],
            },
            types: TypesDiff {
                added: vec![],
                removed: vec![],
                changed: vec![],
            },
        }
    }

    #[test]
    fn empty_inputs_returns_empty_string() {
        let (result, tested) = emit_endpoint_availability_tests(&[], &[]);
        assert_eq!(result, "");
        assert!(tested.is_empty());
    }

    #[test]
    fn no_added_endpoints_returns_empty_string() {
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
                changed: vec![],
            },
        }];
        let (result, tested) = emit_endpoint_availability_tests(&specs, &diffs);
        assert_eq!(result, "");
        assert!(tested.is_empty());
    }

    #[test]
    fn connectors_tag_endpoint_omits_positive_test_but_keeps_negative() {
        let ep = single_get_endpoint("/connectors/{id}", "get_connector");
        let to_spec = spec_with_tag("Connectors", ep);
        let specs = vec![
            ("2.8.0".to_string(), empty_spec()),
            ("2.9.0".to_string(), to_spec),
        ];
        let diffs = vec![diff_with_added_endpoint(
            "2.8.0",
            "2.9.0",
            HttpMethod::Get,
            "/connectors/{id}",
            "Connectors",
        )];

        let (result, _) = emit_endpoint_availability_tests(&specs, &diffs);

        assert!(
            !result.contains("endpoint_connectors_get_connector_available"),
            "positive test must be skipped for Connectors tag, got:\n{result}"
        );
        assert!(
            result.contains("endpoint_connectors_get_connector_unsupported"),
            "negative test must still be emitted, got:\n{result}"
        );
        assert!(
            result.contains("skipped by overrides"),
            "generated file should explain why the positive test was skipped, got:\n{result}"
        );
    }

    #[test]
    fn non_overridden_tag_still_gets_positive_test() {
        let mut ep = single_get_endpoint("/flow/about", "get_about_info");
        ep.path_params.clear();
        let to_spec = spec_with_tag("Flow", ep);
        let specs = vec![
            ("2.8.0".to_string(), empty_spec()),
            ("2.9.0".to_string(), to_spec),
        ];
        let diffs = vec![diff_with_added_endpoint(
            "2.8.0",
            "2.9.0",
            HttpMethod::Get,
            "/flow/about",
            "Flow",
        )];

        let (result, _) = emit_endpoint_availability_tests(&specs, &diffs);

        assert!(
            result.contains("endpoint_flow_get_about_info_available"),
            "positive test should be emitted for non-overridden Flow tag, got:\n{result}"
        );
    }

    #[test]
    fn test_base_name_joins_accessor_and_fn() {
        assert_eq!(
            test_base_name("flow", "get_listen_ports"),
            "endpoint_flow_get_listen_ports"
        );
        assert_eq!(
            test_base_name("processors", "get_processor"),
            "endpoint_processors_get_processor"
        );
    }
}
