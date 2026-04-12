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
