use crate::layout::RepoLayout;
use crate::plan::FileEdit;
use std::path::Path;

/// Returns a `FileEdit::Overwrite` for the docker-compose file.
pub fn emit_docker_compose_default(
    layout: &RepoLayout,
    current_content: &str,
    latest: &str,
) -> Vec<FileEdit> {
    let patched = replace_image_tag_default(current_content, latest);
    vec![FileEdit::Overwrite {
        path: layout.docker_compose.clone(),
        content: patched,
    }]
}

/// Replaces the `${NIFI_IMAGE_TAG:-<old>}` default tag in a docker-compose file content.
pub fn replace_image_tag_default(content: &str, latest: &str) -> String {
    const PREFIX: &str = "${NIFI_IMAGE_TAG:-";
    if let Some(start) = content.find(PREFIX) {
        let ver_start = start + PREFIX.len();
        if let Some(end_offset) = content[ver_start..].find('}') {
            let ver_end = ver_start + end_offset;
            return format!("{}{}{}", &content[..ver_start], latest, &content[ver_end..]);
        }
    }
    content.to_string()
}

/// Read a docker-compose file, replace the default image tag, and write back if changed.
pub fn update_docker_compose_default(path: &Path, latest: &str) {
    let on_disk =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("read {}", path.display()));
    let patched = replace_image_tag_default(&on_disk, latest);
    if on_disk != patched {
        std::fs::write(path, &patched).unwrap_or_else(|_| panic!("write {}", path.display()));
        println!("  wrote {}", path.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::RepoLayout;
    use crate::plan::FileEdit;
    use std::path::Path;

    #[test]
    fn emit_docker_compose_default_returns_overwrite() {
        let layout = RepoLayout::from_workspace_root(Path::new("/fake"));
        let content = "image: apache/nifi:${NIFI_IMAGE_TAG:-2.7.2}\n";
        let edits = super::emit_docker_compose_default(&layout, content, "2.8.0");
        assert_eq!(edits.len(), 1);
        assert!(matches!(&edits[0], FileEdit::Overwrite { path, content }
            if *path == Path::new("/fake/tests/docker-compose.yml")
            && content.contains("2.8.0")
        ));
    }

    #[test]
    fn test_replace_image_tag_default_updates_version() {
        let input = "image: apache/nifi:${NIFI_IMAGE_TAG:-2.7.2}\n";
        let result = replace_image_tag_default(input, "2.8.0");
        assert_eq!(result, "image: apache/nifi:${NIFI_IMAGE_TAG:-2.8.0}\n");
    }

    #[test]
    fn test_replace_image_tag_default_idempotent() {
        let input = "image: apache/nifi:${NIFI_IMAGE_TAG:-2.8.0}\n";
        let result = replace_image_tag_default(input, "2.8.0");
        assert_eq!(result, input);
    }
}
