use nifi_openapi_gen::parser::load;

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

#[test]
fn suffix_stripped_from_fn_name_and_raw_id_preserved() {
    let spec = load(&fixture("suffixed_op.json"));
    let ep = spec
        .tags
        .iter()
        .flat_map(|t| t.root_endpoints.iter())
        .find(|e| e.path == "/foo/bar")
        .expect("endpoint /foo/bar not found");
    assert_eq!(ep.fn_name, "get_foo");
    assert_eq!(ep.raw_operation_id, "getFoo_5");
}

#[test]
fn passthrough_without_suffix() {
    let spec = load(&fixture("unsuffixed_op.json"));
    let ep = spec
        .tags
        .iter()
        .flat_map(|t| t.root_endpoints.iter())
        .next()
        .expect("endpoint");
    assert_eq!(ep.fn_name, "get_about_info");
    assert_eq!(ep.raw_operation_id, "getAboutInfo");
}

#[test]
fn multi_digit_suffix_stripped() {
    let spec = load(&fixture("multi_digit_suffix_op.json"));
    let ep = spec
        .tags
        .iter()
        .flat_map(|t| t.root_endpoints.iter())
        .next()
        .expect("endpoint");
    assert_eq!(ep.fn_name, "get_foo");
    assert_eq!(ep.raw_operation_id, "getFoo_42");
}

#[test]
fn only_trailing_group_stripped() {
    let spec = load(&fixture("double_suffix_op.json"));
    let ep = spec
        .tags
        .iter()
        .flat_map(|t| t.root_endpoints.iter())
        .next()
        .expect("endpoint");
    // "getFoo_1_2" strips only the trailing _2 -> "getFoo_1" -> "get_foo_1"
    assert_eq!(ep.fn_name, "get_foo_1");
    assert_eq!(ep.raw_operation_id, "getFoo_1_2");
}

use nifi_openapi_gen::content_type::ResponseBodyKind;
use nifi_openapi_gen::naming;
use nifi_openapi_gen::parser::{ApiSpec, Endpoint, HttpMethod, TagGroup};

fn make_endpoint(method: HttpMethod, path: &str, fn_name: &str, raw_op_id: &str) -> Endpoint {
    Endpoint {
        method,
        path: path.to_string(),
        fn_name: fn_name.to_string(),
        raw_operation_id: raw_op_id.to_string(),
        doc: None,
        description: None,
        path_params: vec![],
        request_type: None,
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

fn make_spec(tag: &str, endpoints: Vec<Endpoint>) -> ApiSpec {
    ApiSpec {
        tags: vec![TagGroup {
            tag: tag.to_string(),
            struct_name: format!("{}Api", tag),
            module_name: tag.to_lowercase(),
            accessor_fn: format!("{}_api", tag.to_lowercase()),
            types: vec![],
            root_endpoints: endpoints,
            sub_groups: vec![],
        }],
        all_types: vec![],
    }
}

#[test]
fn apply_overrides_rewrites_matching_fn_name() {
    let mut overrides = std::collections::HashMap::new();
    overrides.insert(
        ("test-1.0".to_string(), "getFoo_5".to_string()),
        "get_foo_special".to_string(),
    );

    let mut spec = make_spec(
        "Foo",
        vec![make_endpoint(HttpMethod::Get, "/foo", "get_foo", "getFoo_5")],
    );

    naming::apply_overrides_with_table(&mut spec, "test-1.0", &overrides);

    let ep = &spec.tags[0].root_endpoints[0];
    assert_eq!(ep.fn_name, "get_foo_special");
}

#[test]
fn apply_overrides_leaves_unmatched_names_alone() {
    let overrides = std::collections::HashMap::new();
    let mut spec = make_spec(
        "Foo",
        vec![make_endpoint(HttpMethod::Get, "/foo", "get_foo", "getFoo")],
    );

    naming::apply_overrides_with_table(&mut spec, "test-1.0", &overrides);

    let ep = &spec.tags[0].root_endpoints[0];
    assert_eq!(ep.fn_name, "get_foo");
}

#[test]
#[should_panic(expected = "fn_name collision")]
fn collision_same_tag_panics() {
    let mut spec = make_spec(
        "Foo",
        vec![
            make_endpoint(HttpMethod::Put, "/foo/a", "update", "updateFoo_1"),
            make_endpoint(HttpMethod::Put, "/foo/b", "update", "updateFoo_2"),
        ],
    );
    let overrides = std::collections::HashMap::new();
    naming::apply_overrides_with_table(&mut spec, "2.9.0", &overrides);
    naming::check_collisions(&spec, "2.9.0");
}

#[test]
fn no_collision_when_fn_names_differ() {
    let mut spec = make_spec(
        "Foo",
        vec![
            make_endpoint(HttpMethod::Get, "/foo/a", "get_foo_a", "getFooA"),
            make_endpoint(HttpMethod::Get, "/foo/b", "get_foo_b", "getFooB"),
        ],
    );
    let overrides = std::collections::HashMap::new();
    naming::apply_overrides_with_table(&mut spec, "2.9.0", &overrides);
    naming::check_collisions(&spec, "2.9.0"); // must not panic
}

#[test]
fn collision_message_is_actionable() {
    let mut spec = make_spec(
        "Foo",
        vec![
            make_endpoint(HttpMethod::Put, "/foo/a", "update", "updateFoo_1"),
            make_endpoint(HttpMethod::Put, "/foo/b", "update", "updateFoo_2"),
        ],
    );
    let overrides = std::collections::HashMap::new();
    naming::apply_overrides_with_table(&mut spec, "2.9.0", &overrides);

    let result = std::panic::catch_unwind(|| naming::check_collisions(&spec, "2.9.0"));
    let err = result.expect_err("check_collisions must panic on collision");
    let msg = err
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| err.downcast_ref::<&'static str>().copied())
        .unwrap_or("<non-string panic>");
    assert!(msg.contains("fn_name collision"), "got: {msg}");
    assert!(msg.contains("2.9.0"), "got: {msg}");
    assert!(msg.contains("Foo"), "got: {msg}");
    assert!(msg.contains("updateFoo_1"), "got: {msg}");
    assert!(msg.contains("updateFoo_2"), "got: {msg}");
    assert!(msg.contains("naming_overrides.rs"), "got: {msg}");
}
