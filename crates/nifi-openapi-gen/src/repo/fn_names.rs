use crate::layout::RepoLayout;
use crate::parser::ApiSpec;
use crate::plan::FileEdit;

/// Build one `FileEdit::Overwrite` per version, writing
/// `<specs_dir>/<version>/fn_names.txt` with the sorted, column-padded
/// table of `(tag, method, path) -> fn_name` for every endpoint in the
/// parsed spec.
pub fn emit_fn_names_goldens(
    layout: &RepoLayout,
    all_parsed: &[(String, ApiSpec)],
) -> Vec<FileEdit> {
    all_parsed
        .iter()
        .map(|(version, spec)| {
            let content = format_fn_names_table(spec);
            FileEdit::Overwrite {
                path: layout.specs_dir.join(version).join("fn_names.txt"),
                content,
            }
        })
        .collect()
}

/// Format a single spec as a sorted, space-padded table. One line per
/// endpoint, no header, trailing newline. Columns are padded to the
/// maximum width observed for this spec so diffs stay clean.
pub fn format_fn_names_table(spec: &ApiSpec) -> String {
    let mut rows: Vec<(String, String, String, String)> = Vec::new();
    for tag in &spec.tags {
        for ep in &tag.endpoints {
            rows.push((
                tag.tag.clone(),
                ep.method.as_str().to_string(),
                ep.path.clone(),
                ep.fn_name.clone(),
            ));
        }
    }
    rows.sort_by(|a, b| (&a.0, &a.1, &a.2).cmp(&(&b.0, &b.1, &b.2)));

    let tag_w = rows.iter().map(|r| r.0.len()).max().unwrap_or(0);
    let method_w = rows.iter().map(|r| r.1.len()).max().unwrap_or(0);
    let path_w = rows.iter().map(|r| r.2.len()).max().unwrap_or(0);

    let mut out = String::new();
    for (tag, method, path, fn_name) in rows {
        out.push_str(&format!(
            "{tag:<tag_w$}  {method:<method_w$}  {path:<path_w$}  -> {fn_name}\n"
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content_type::ResponseBodyKind;
    use crate::parser::{Endpoint, HttpMethod, TagGroup};

    fn mk_ep(method: HttpMethod, path: &str, fn_name: &str) -> Endpoint {
        Endpoint {
            method,
            path: path.to_string(),
            fn_name: fn_name.to_string(),
            raw_operation_id: String::new(),
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
            header_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        }
    }

    fn spec_with(eps: Vec<Endpoint>) -> ApiSpec {
        ApiSpec {
            tags: vec![TagGroup {
                tag: "Flow".to_string(),
                struct_name: "Flow".to_string(),
                module_name: "flow".to_string(),
                accessor_fn: "flow".to_string(),
                types: vec![],
                endpoints: eps,
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn format_sorts_by_tag_method_path() {
        let spec = spec_with(vec![
            mk_ep(HttpMethod::Post, "/b", "post_b"),
            mk_ep(HttpMethod::Get, "/a", "get_a"),
            mk_ep(HttpMethod::Get, "/b", "get_b"),
        ]);
        let out = format_fn_names_table(&spec);
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].contains("GET"));
        assert!(lines[0].contains("/a"));
        assert!(lines[0].contains("-> get_a"));
        assert!(lines[1].contains("GET"));
        assert!(lines[1].contains("/b"));
        assert!(lines[2].contains("POST"));
    }

    #[test]
    fn format_has_trailing_newline_and_no_header() {
        let spec = spec_with(vec![mk_ep(HttpMethod::Get, "/a", "get_a")]);
        let out = format_fn_names_table(&spec);
        assert!(out.ends_with('\n'));
        assert!(!out.starts_with("tag"));
        assert!(!out.starts_with('#'));
    }

    #[test]
    fn emit_builds_one_overwrite_per_version() {
        let layout = RepoLayout::from_workspace_root(std::path::Path::new("/fake"));
        let v1 = spec_with(vec![mk_ep(HttpMethod::Get, "/a", "get_a")]);
        let v2 = spec_with(vec![mk_ep(HttpMethod::Get, "/a", "get_a")]);
        let edits = emit_fn_names_goldens(
            &layout,
            &[("2.8.0".to_string(), v1), ("2.9.0".to_string(), v2)],
        );
        assert_eq!(edits.len(), 2);
        match &edits[0] {
            FileEdit::Overwrite { path, .. } => {
                assert_eq!(
                    path,
                    std::path::Path::new("/fake/crates/nifi-openapi-gen/specs/2.8.0/fn_names.txt")
                );
            }
            _ => panic!("expected Overwrite"),
        }
    }
}
