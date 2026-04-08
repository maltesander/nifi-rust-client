use std::collections::BTreeMap;

use crate::parser::{ApiSpec, Endpoint, QueryParam, QueryParamType};

pub fn format_source(src: &str) -> String {
    match syn::parse_file(src) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => src.to_string(),
    }
}

pub(crate) fn version_to_variant(version: &str) -> String {
    format!("V{}", version.replace('.', "_"))
}

/// Info about an endpoint in the context of a specific version.
pub(crate) struct EndpointInfo<'a> {
    pub endpoint: &'a Endpoint,
    /// None for root endpoints, Some(struct_name) for sub-group endpoints.
    pub sub_struct_name: Option<&'a str>,
    /// The field name for the primary param in the sub-struct (e.g. "id", "port_id").
    pub primary_param: Option<&'a str>,
}

/// Collect all unique (tag, struct_name, module_name, accessor_fn) across versions.
pub(crate) fn collect_all_tags(
    versions: &[(&str, &str, &str, &ApiSpec)],
) -> Vec<(String, String, String, String)> {
    let mut seen: BTreeMap<String, (String, String, String)> = BTreeMap::new();
    for (_, _, _, spec) in versions {
        for tag in &spec.tags {
            seen.entry(tag.tag.clone()).or_insert_with(|| {
                (
                    tag.struct_name.clone(),
                    tag.module_name.clone(),
                    tag.accessor_fn.clone(),
                )
            });
        }
    }
    seen.into_iter()
        .map(|(tag, (sn, mn, af))| (tag, sn, mn, af))
        .collect()
}

/// For a given tag, collect all endpoints (including sub-group endpoints flattened) across versions.
/// Returns BTreeMap<fn_name, BTreeMap<version_str, EndpointInfo>>
pub(crate) fn collect_tag_endpoints<'a>(
    versions: &[(&'a str, &'a str, &'a str, &'a ApiSpec)],
    tag: &str,
) -> BTreeMap<String, BTreeMap<&'a str, EndpointInfo<'a>>> {
    let mut result: BTreeMap<String, BTreeMap<&'a str, EndpointInfo<'a>>> = BTreeMap::new();

    for (ver, _mod_name, _feature, spec) in versions {
        for tg in &spec.tags {
            if tg.tag != tag {
                continue;
            }
            for ep in &tg.root_endpoints {
                result.entry(ep.fn_name.clone()).or_default().insert(
                    ver,
                    EndpointInfo {
                        endpoint: ep,
                        sub_struct_name: None,
                        primary_param: None,
                    },
                );
            }
            // Flatten sub-group endpoints
            for sg in &tg.sub_groups {
                for ep in &sg.endpoints {
                    result.entry(ep.fn_name.clone()).or_default().insert(
                        ver,
                        EndpointInfo {
                            endpoint: ep,
                            sub_struct_name: Some(&sg.struct_name),
                            primary_param: Some(&sg.primary_param),
                        },
                    );
                }
            }
        }
    }

    result
}

/// Merge query params from all versions of an endpoint into a superset.
/// Returns each unique param (by rust_name) along with the count of versions it appears in.
/// Preserves insertion order: params appear in the order first seen across versions.
pub(crate) fn merge_query_params(
    ep_by_version: &BTreeMap<&str, EndpointInfo<'_>>,
) -> Vec<(QueryParam, usize)> {
    let mut result: Vec<(QueryParam, usize)> = Vec::new();
    for info in ep_by_version.values() {
        for qp in &info.endpoint.query_params {
            if let Some((existing, count)) =
                result.iter_mut().find(|(q, _)| q.rust_name == qp.rust_name)
            {
                *count += 1;
                // If any version marks it optional, keep it optional
                if !qp.required {
                    existing.required = false;
                }
            } else {
                result.push((qp.clone(), 1));
            }
        }
    }
    result
}

/// For dynamic mode, enum query params use the dynamic union enum type.
pub(crate) fn dynamic_query_param_type(qp: &QueryParam) -> String {
    match &qp.ty {
        QueryParamType::Str => "&str".to_string(),
        QueryParamType::Bool => "bool".to_string(),
        QueryParamType::I32 => "i32".to_string(),
        QueryParamType::I64 => "i64".to_string(),
        QueryParamType::F64 => "f64".to_string(),
        QueryParamType::Enum(_) => {
            let type_name = qp.enum_type_name.as_deref().unwrap_or("String");
            format!("types::{type_name}")
        }
    }
}

/// Escape Rust keywords by prefixing with `r#`.
pub(crate) fn escape_keyword(name: &str) -> String {
    match name {
        "type" | "ref" | "use" | "mod" | "fn" | "let" | "match" | "for" | "if" | "else"
        | "return" | "struct" | "enum" | "impl" | "trait" | "pub" | "super" | "self" | "crate"
        | "where" | "true" | "false" | "in" | "loop" | "while" | "break" | "continue" | "mut"
        | "move" | "async" | "await" | "dyn" | "box" | "const" | "static" | "extern" | "unsafe"
        | "as" => format!("r#{name}"),
        _ => name.to_string(),
    }
}

/// Convert snake_case to PascalCase.
pub(crate) fn pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut c = part.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
            }
        })
        .collect()
}

/// Convert SCREAMING_SNAKE_CASE to PascalCase.
pub(crate) fn wire_to_pascal(wire: &str) -> String {
    wire.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let mut s = c.to_uppercase().to_string();
                    s.extend(chars.map(|ch| ch.to_ascii_lowercase()));
                    s
                }
            }
        })
        .collect()
}

/// Convert a version string like "2.8.0" to a Rust module name like "v2_8_0".
pub fn version_to_mod_name(version: &str) -> String {
    format!("v{}", version.replace('.', "_"))
}

/// Convert a version string like "2.8.0" to a Cargo feature name like "nifi-2-8-0".
pub fn version_to_feature(version: &str) -> String {
    format!("nifi-{}", version.replace('.', "-"))
}

/// Sort version strings in semver order (ascending).
pub(crate) fn sort_versions_semver(versions: &mut [String]) {
    versions.sort_by(|a, b| {
        semver::Version::parse(a)
            .unwrap_or_else(|_| semver::Version::new(0, 0, 0))
            .cmp(&semver::Version::parse(b).unwrap_or_else(|_| semver::Version::new(0, 0, 0)))
    });
}

/// Discover all valid semver directories under `specs_dir` and return them sorted.
pub fn discover_spec_versions(specs_dir: &std::path::Path) -> Vec<String> {
    let mut versions: Vec<String> = std::fs::read_dir(specs_dir)
        .unwrap_or_else(|_| {
            panic!(
                "Cannot read specs dir {}.\nRun: ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh",
                specs_dir.display()
            )
        })
        .flatten()
        .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .filter_map(|e| {
            let name = e.file_name();
            let s = name.to_str()?.to_string();
            semver::Version::parse(&s).ok()?;
            Some(s)
        })
        .collect();
    sort_versions_semver(&mut versions);
    versions
}

/// Replace content between `start_marker` and `end_marker` (inclusive of markers) with new content.
pub(crate) fn replace_between_markers(
    content: &str,
    start_marker: &str,
    end_marker: &str,
    new_content: &str,
) -> String {
    let start_pos = content.find(start_marker).unwrap_or_else(|| {
        panic!("start marker '{start_marker}' not found");
    });
    let after_start = start_pos + start_marker.len();
    let end_pos = content[after_start..]
        .find(end_marker)
        .map(|p| p + after_start)
        .unwrap_or_else(|| {
            panic!("end marker '{end_marker}' not found");
        });
    format!(
        "{}{}\n{}\n{}{}",
        &content[..start_pos],
        start_marker,
        new_content,
        end_marker,
        &content[end_pos + end_marker.len()..],
    )
}

/// Read a file, replace content between markers, and write back if changed.
pub fn update_file_between_markers(
    path: &std::path::Path,
    start_marker: &str,
    end_marker: &str,
    new_content: &str,
) {
    let on_disk =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("read {}", path.display()));
    let patched = replace_between_markers(&on_disk, start_marker, end_marker, new_content);
    if on_disk != patched {
        std::fs::write(path, &patched).unwrap_or_else(|_| panic!("write {}", path.display()));
        println!("  wrote {}", path.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- pascal_case ---

    #[test]
    fn pascal_case_simple() {
        assert_eq!(pascal_case("hello_world"), "HelloWorld");
    }

    #[test]
    fn pascal_case_single_word() {
        assert_eq!(pascal_case("hello"), "Hello");
    }

    #[test]
    fn pascal_case_already_pascal() {
        assert_eq!(pascal_case("Hello"), "Hello");
    }

    #[test]
    fn pascal_case_multiple_underscores() {
        assert_eq!(pascal_case("one_two_three_four"), "OneTwoThreeFour");
    }

    #[test]
    fn pascal_case_empty() {
        assert_eq!(pascal_case(""), "");
    }

    #[test]
    fn pascal_case_trailing_underscore() {
        assert_eq!(pascal_case("hello_"), "Hello");
    }

    #[test]
    fn pascal_case_upper_input() {
        assert_eq!(pascal_case("HELLO_WORLD"), "HelloWorld");
    }

    // --- wire_to_pascal ---

    #[test]
    fn wire_to_pascal_screaming_snake() {
        assert_eq!(wire_to_pascal("KEEP_EXISTING"), "KeepExisting");
    }

    #[test]
    fn wire_to_pascal_single_word() {
        assert_eq!(wire_to_pascal("BASIC"), "Basic");
    }

    #[test]
    fn wire_to_pascal_three_words() {
        assert_eq!(wire_to_pascal("ALL_PROCESS_GROUPS"), "AllProcessGroups");
    }

    #[test]
    fn wire_to_pascal_empty() {
        assert_eq!(wire_to_pascal(""), "");
    }

    #[test]
    fn wire_to_pascal_lowercase_input() {
        assert_eq!(wire_to_pascal("keep_existing"), "KeepExisting");
    }

    // --- version_to_mod_name ---

    #[test]
    fn version_to_mod_name_basic() {
        assert_eq!(version_to_mod_name("2.8.0"), "v2_8_0");
    }

    #[test]
    fn version_to_mod_name_single_digit() {
        assert_eq!(version_to_mod_name("1.0.0"), "v1_0_0");
    }

    // --- version_to_feature ---

    #[test]
    fn version_to_feature_basic() {
        assert_eq!(version_to_feature("2.8.0"), "nifi-2-8-0");
    }

    #[test]
    fn version_to_feature_double_digit() {
        assert_eq!(version_to_feature("2.10.1"), "nifi-2-10-1");
    }

    // --- sort_versions_semver ---

    #[test]
    fn sort_versions_semver_basic() {
        let mut versions = vec![
            "2.8.0".to_string(),
            "1.0.0".to_string(),
            "2.10.0".to_string(),
            "2.2.0".to_string(),
        ];
        sort_versions_semver(&mut versions);
        assert_eq!(versions, ["1.0.0", "2.2.0", "2.8.0", "2.10.0"]);
    }

    #[test]
    fn sort_versions_semver_empty() {
        let mut versions: Vec<String> = vec![];
        sort_versions_semver(&mut versions);
        assert!(versions.is_empty());
    }

    #[test]
    fn sort_versions_semver_already_sorted() {
        let mut versions = vec!["1.0.0".to_string(), "2.0.0".to_string()];
        sort_versions_semver(&mut versions);
        assert_eq!(versions, ["1.0.0", "2.0.0"]);
    }

    // --- discover_spec_versions ---

    fn make_temp_dir(test_name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("nifi_openapi_gen_test_{}", test_name));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn discover_spec_versions_finds_valid_dirs() {
        let tmp = make_temp_dir("discover_valid");
        std::fs::create_dir(tmp.join("2.8.0")).unwrap();
        std::fs::create_dir(tmp.join("2.2.0")).unwrap();
        std::fs::create_dir(tmp.join("not-semver")).unwrap();
        // Create a file (should be ignored)
        std::fs::write(tmp.join("1.0.0"), "file").unwrap();

        let versions = discover_spec_versions(&tmp);
        assert_eq!(versions, ["2.2.0", "2.8.0"]);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn discover_spec_versions_empty_dir() {
        let tmp = make_temp_dir("discover_empty");
        let versions = discover_spec_versions(&tmp);
        assert!(versions.is_empty());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[should_panic(expected = "Cannot read specs dir")]
    fn discover_spec_versions_missing_dir() {
        discover_spec_versions(std::path::Path::new("/nonexistent/path/specs"));
    }

    // --- replace_between_markers ---

    #[test]
    fn replace_between_markers_basic() {
        let content = "before\n// START\nold content\n// END\nafter";
        let result = replace_between_markers(content, "// START", "// END", "new content");
        assert_eq!(result, "before\n// START\nnew content\n// END\nafter");
    }

    #[test]
    fn replace_between_markers_preserves_surrounding() {
        let content = "header\n<!-- BEGIN -->\nstuff\n<!-- END -->\nfooter\n";
        let result = replace_between_markers(content, "<!-- BEGIN -->", "<!-- END -->", "replaced");
        assert_eq!(result, "header\n<!-- BEGIN -->\nreplaced\n<!-- END -->\nfooter\n");
    }

    #[test]
    #[should_panic(expected = "start marker")]
    fn replace_between_markers_missing_start() {
        replace_between_markers("no markers here", "// START", "// END", "x");
    }

    #[test]
    #[should_panic(expected = "end marker")]
    fn replace_between_markers_missing_end() {
        replace_between_markers("// START\ncontent", "// START", "// END", "x");
    }

    // --- update_file_between_markers ---

    #[test]
    fn update_file_between_markers_writes_when_changed() {
        let tmp = make_temp_dir("update_changed");
        let path = tmp.join("test.txt");
        std::fs::write(&path, "before\n// START\nold\n// END\nafter").unwrap();

        update_file_between_markers(&path, "// START", "// END", "new");

        let result = std::fs::read_to_string(&path).unwrap();
        assert_eq!(result, "before\n// START\nnew\n// END\nafter");

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn update_file_between_markers_no_write_when_unchanged() {
        let tmp = make_temp_dir("update_unchanged");
        let path = tmp.join("test.txt");
        let content = "before\n// START\nnew\n// END\nafter";
        std::fs::write(&path, content).unwrap();

        // This should not panic or error — content already matches
        update_file_between_markers(&path, "// START", "// END", "new");

        let result = std::fs::read_to_string(&path).unwrap();
        assert_eq!(result, content);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- escape_keyword ---

    #[test]
    fn escape_keyword_escapes_type() {
        assert_eq!(escape_keyword("type"), "r#type");
    }

    #[test]
    fn escape_keyword_passes_through_normal() {
        assert_eq!(escape_keyword("name"), "name");
    }

    // --- version_to_variant ---

    #[test]
    fn version_to_variant_basic() {
        assert_eq!(version_to_variant("2.8.0"), "V2_8_0");
    }
}
