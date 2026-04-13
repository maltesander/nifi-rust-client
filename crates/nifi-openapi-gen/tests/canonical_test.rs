use nifi_openapi_gen::canonical::VersionSet;

#[test]
fn version_set_new_is_empty() {
    let vs = VersionSet::new();
    assert!(vs.is_empty());
    assert_eq!(vs.len(), 0);
}

#[test]
fn version_set_with_single_version() {
    let vs = VersionSet::with("2.8.0");
    assert!(vs.contains("2.8.0"));
    assert!(!vs.contains("2.9.0"));
    assert_eq!(vs.len(), 1);
}

#[test]
fn version_set_insert_and_iter_sorted() {
    let mut vs = VersionSet::new();
    vs.insert("2.9.0");
    vs.insert("2.6.0");
    vs.insert("2.8.0");
    let collected: Vec<&str> = vs.iter().map(String::as_str).collect();
    assert_eq!(collected, vec!["2.6.0", "2.8.0", "2.9.0"]);
}

#[test]
fn version_set_insert_duplicate_is_noop() {
    let mut vs = VersionSet::with("2.8.0");
    vs.insert("2.8.0");
    assert_eq!(vs.len(), 1);
}
