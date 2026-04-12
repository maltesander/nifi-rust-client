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
