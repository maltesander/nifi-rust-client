use crate::parser::{Endpoint, HttpMethod};

/// CLI verb assigned to an endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliVerb {
    Get,
    List,
    Create,
    Update,
    Delete,
    /// Domain-specific verb rendered as a kebab-case subcommand.
    Named(String),
}

impl CliVerb {
    /// Returns the CLI command name for this verb.
    pub fn command_name(&self) -> String {
        match self {
            CliVerb::Get => "get".to_string(),
            CliVerb::List => "list".to_string(),
            CliVerb::Create => "create".to_string(),
            CliVerb::Update => "update".to_string(),
            CliVerb::Delete => "delete".to_string(),
            CliVerb::Named(s) => s.clone(),
        }
    }
}

/// Classify an endpoint into a `CliVerb`.
///
/// `tag_module` is the snake_case tag name used as a prefix to strip
/// (e.g. `"processors"` for the processors API group).
pub fn classify(endpoint: &Endpoint, tag_module: &str) -> CliVerb {
    let name = &endpoint.fn_name;

    // 1. DELETE
    if endpoint.method == HttpMethod::Delete {
        if name.starts_with("delete_") || name.starts_with("remove_") {
            return CliVerb::Delete;
        }
        return CliVerb::Named(strip_and_kebab(name, tag_module));
    }

    // 2. get_ prefix
    if let Some(rest) = name.strip_prefix("get_") {
        if is_collection_hint(rest, endpoint) {
            return CliVerb::List;
        }
        return CliVerb::Get;
    }

    // 3. search_ prefix
    if name.starts_with("search_") {
        return CliVerb::Named("search".to_string());
    }

    // 4. create_ prefix
    if name.starts_with("create_") {
        return CliVerb::Create;
    }

    // 5. update_ or set_ prefix — only when updating the tag's own resource
    if name.starts_with("update_") || name.starts_with("set_") {
        if is_tag_resource_update(name, tag_module) {
            return CliVerb::Update;
        }
        // Sub-resource update (e.g. update_run_status for processors) → Named
        return CliVerb::Named(strip_and_kebab(name, tag_module));
    }

    // 6. POST with a request body that doesn't look like an action
    if endpoint.method == HttpMethod::Post
        && endpoint.request_type.is_some()
        && !looks_like_action(name)
    {
        return CliVerb::Create;
    }

    // 7. Fallback
    CliVerb::Named(strip_and_kebab(name, tag_module))
}

/// Returns `true` when `update_<X>` or `set_<X>` is updating the tag's own
/// primary resource (singular or plural form of `tag_module`).
fn is_tag_resource_update(name: &str, tag_module: &str) -> bool {
    let resource = name
        .strip_prefix("update_")
        .or_else(|| name.strip_prefix("set_"))
        .unwrap_or("");

    // Match plural form (e.g. tag_module = "processors", resource = "processors")
    if resource == tag_module {
        return true;
    }
    // Match singular form (e.g. tag_module = "processors", resource = "processor")
    if let Some(singular) = tag_module.strip_suffix('s')
        && resource == singular
    {
        return true;
    }
    false
}

/// Returns `true` when the name/endpoint hints at a collection (List) rather
/// than a single resource (Get).
fn is_collection_hint(rest: &str, endpoint: &Endpoint) -> bool {
    // Plural ending (but exclude common false positives)
    if rest.ends_with('s')
        && !rest.ends_with("ss")
        && !rest.ends_with("status")
        && !rest.ends_with("access")
        && !rest.ends_with("details")
    {
        return true;
    }

    // Explicit "all_" prefix in rest
    if rest.starts_with("all_") {
        return true;
    }

    // No path params and name doesn't contain "about"
    if endpoint.path_params.is_empty() && !rest.contains("about") {
        return true;
    }

    false
}

/// Returns `true` when the endpoint name looks like a domain action rather
/// than a CRUD operation.
fn looks_like_action(name: &str) -> bool {
    const ACTION_WORDS: &[&str] = &[
        "analyze",
        "verify",
        "validate",
        "clear",
        "submit",
        "initiate",
        "export",
        "import",
        "upload",
        "download",
        "schedule",
        "terminate",
    ];
    ACTION_WORDS.iter().any(|w| name.contains(w))
}

/// Strip the tag_module prefix (or its singular form) from `name`, also
/// stripping any leading CRUD verb prefix, then convert remaining underscores
/// to hyphens for use as a kebab-case subcommand name.
fn strip_and_kebab(name: &str, tag_module: &str) -> String {
    // First strip any CRUD verb prefix so "update_run_status" → "run_status"
    const VERB_PREFIXES: &[&str] = &[
        "update_", "set_", "delete_", "remove_", "create_", "get_", "search_",
    ];
    let after_verb = VERB_PREFIXES
        .iter()
        .find_map(|pfx| name.strip_prefix(pfx))
        .unwrap_or(name);

    // Try stripping tag_module prefix (plural)
    let prefix_with_underscore = format!("{tag_module}_");
    if let Some(rest) = after_verb.strip_prefix(prefix_with_underscore.as_str()) {
        return rest.replace('_', "-");
    }

    // Try singular: strip trailing 's' from tag_module
    if let Some(singular) = tag_module.strip_suffix('s') {
        let singular_prefix = format!("{singular}_");
        if let Some(rest) = after_verb.strip_prefix(singular_prefix.as_str()) {
            return rest.replace('_', "-");
        }
    }

    after_verb.replace('_', "-")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content_type::ResponseBodyKind;

    fn make_endpoint(
        method: HttpMethod,
        fn_name: &str,
        path_params: Vec<crate::parser::PathParam>,
        request_type: Option<&str>,
    ) -> Endpoint {
        Endpoint {
            method,
            path: "/test".to_string(),
            fn_name: fn_name.to_string(),
            doc: None,
            description: None,
            path_params,
            request_type: request_type.map(str::to_string),
            body_kind: None,
            body_doc: None,
            response_type: None,
            response_inner: None,
            response_field: None,
            response_kind: ResponseBodyKind::Empty,
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        }
    }

    fn path_param(name: &str) -> crate::parser::PathParam {
        crate::parser::PathParam {
            name: name.to_string(),
            doc: None,
        }
    }

    #[test]
    fn get_single_resource() {
        // GET "get_processor" with a path param → Get
        let ep = make_endpoint(
            HttpMethod::Get,
            "get_processor",
            vec![path_param("id")],
            None,
        );
        assert_eq!(classify(&ep, "processors"), CliVerb::Get);
    }

    #[test]
    fn get_collection() {
        // GET "get_processors" with no path params → List
        let ep = make_endpoint(HttpMethod::Get, "get_processors", vec![], None);
        assert_eq!(classify(&ep, "processors"), CliVerb::List);
    }

    #[test]
    fn delete_resource() {
        // DELETE "delete_processor" → Delete
        let ep = make_endpoint(
            HttpMethod::Delete,
            "delete_processor",
            vec![path_param("id")],
            None,
        );
        assert_eq!(classify(&ep, "processors"), CliVerb::Delete);
    }

    #[test]
    fn update_resource() {
        // PUT "update_processor" → Update
        let ep = make_endpoint(
            HttpMethod::Put,
            "update_processor",
            vec![path_param("id")],
            Some("ProcessorEntity"),
        );
        assert_eq!(classify(&ep, "processors"), CliVerb::Update);
    }

    #[test]
    fn create_resource() {
        // POST "create_processor" with request_type → Create
        let ep = make_endpoint(
            HttpMethod::Post,
            "create_processor",
            vec![],
            Some("ProcessorEntity"),
        );
        assert_eq!(classify(&ep, "processors"), CliVerb::Create);
    }

    #[test]
    fn named_action() {
        // PUT "update_run_status" → Named("run-status")
        // starts with "update_" → Update... wait, that would be Update.
        // Actually per the spec: Named("run-status") — so this must NOT match
        // the plain "update_" path. Looking at the strategy: update_ → Update.
        // But the test says Named("run-status"). Re-reading: step 5 catches
        // update_ → Update unconditionally. The task description says
        // "PUT update_run_status → Named('run-status')". This implies
        // update_run_status should not match step 5 as Update but fall through.
        // On reflection: update_run_status has "run_status" as the resource —
        // this is a sub-resource action, not a top-level update. The test
        // expectation is Named("run-status"). To satisfy this, step 5 should
        // only apply when the remainder after "update_" matches the tag.
        // Re-implementing: update_ → Update only when stripping update_ leaves
        // something that is the tag resource itself (or empty). Otherwise Named.
        //
        // Actually the simplest reading: the classify function as specced
        // step 5 says update_ OR set_ → Update. But the test contradicts that
        // for update_run_status. The resolution: step 5 (Update) applies when
        // the name is simply "update_{tag_singular}" (i.e. updating the main
        // resource). Otherwise it's a Named action.
        // We'll adjust classify() accordingly and verify all 9 tests pass.
        let ep = make_endpoint(
            HttpMethod::Put,
            "update_run_status",
            vec![path_param("id")],
            Some("ProcessorRunStatusEntity"),
        );
        assert_eq!(
            classify(&ep, "processors"),
            CliVerb::Named("run-status".to_string())
        );
    }

    #[test]
    fn search_verb() {
        // GET "search_flow" → Named("search")
        let ep = make_endpoint(HttpMethod::Get, "search_flow", vec![], None);
        assert_eq!(classify(&ep, "flow"), CliVerb::Named("search".to_string()));
    }

    #[test]
    fn get_about_is_not_list() {
        // GET "get_about_info" no params → Get (contains "about")
        let ep = make_endpoint(HttpMethod::Get, "get_about_info", vec![], None);
        assert_eq!(classify(&ep, "flow"), CliVerb::Get);
    }

    #[test]
    fn analyze_is_action() {
        // POST "analyze_configuration" with request_type → Named("analyze-configuration")
        let ep = make_endpoint(
            HttpMethod::Post,
            "analyze_configuration",
            vec![path_param("id")],
            Some("ConfigurationAnalysisEntity"),
        );
        assert_eq!(
            classify(&ep, "controller_services"),
            CliVerb::Named("analyze-configuration".to_string())
        );
    }

    #[test]
    fn classify_all_endpoints_from_latest_spec() {
        let specs_dir = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/specs"));
        let mut versions: Vec<_> = std::fs::read_dir(specs_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().ok().is_some_and(|t| t.is_dir()))
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        versions.sort();
        let latest = versions.last().expect("no spec versions found");
        let spec_file = specs_dir.join(latest).join("nifi-api.json");
        let spec = crate::parser::load(&spec_file.to_string_lossy());

        let mut verb_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for tag in &spec.tags {
            for ep in &tag.root_endpoints {
                let verb = classify(ep, &tag.module_name);
                *verb_counts.entry(verb.command_name()).or_default() += 1;
            }
            for sg in &tag.sub_groups {
                for ep in &sg.endpoints {
                    let verb = classify(ep, &tag.module_name);
                    *verb_counts.entry(verb.command_name()).or_default() += 1;
                }
            }
        }

        // Sanity: standard verbs should have reasonable counts
        assert!(
            *verb_counts.get("get").unwrap_or(&0) > 5,
            "expected many get verbs, got {:?}",
            verb_counts.get("get")
        );
        assert!(
            *verb_counts.get("delete").unwrap_or(&0) > 5,
            "expected many delete verbs"
        );
        assert!(
            *verb_counts.get("create").unwrap_or(&0) > 0,
            "expected some create verbs"
        );
        assert!(
            *verb_counts.get("update").unwrap_or(&0) > 0,
            "expected some update verbs"
        );
    }
}
