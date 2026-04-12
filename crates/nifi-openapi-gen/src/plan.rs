use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// A single file mutation the generator wants to make.
#[derive(Debug, Clone)]
pub enum FileEdit {
    /// Replace content between a marker pair.
    ReplaceBlock {
        path: PathBuf,
        start_marker: String,
        end_marker: String,
        content: String,
    },
    /// Overwrite the entire file.
    Overwrite {
        path: PathBuf,
        content: String,
    },
    /// Create the file from a template if missing, then replace a marker block.
    CreateOrReplaceBlock {
        path: PathBuf,
        start_marker: String,
        end_marker: String,
        content: String,
        template: String,
    },
}

impl FileEdit {
    pub fn path(&self) -> &Path {
        match self {
            Self::ReplaceBlock { path, .. }
            | Self::Overwrite { path, .. }
            | Self::CreateOrReplaceBlock { path, .. } => path,
        }
    }
}

#[derive(Debug)]
pub enum PlanError {
    FileNotFound { path: PathBuf },
    MarkerNotFound { path: PathBuf, marker: String },
    DuplicateTarget { path: PathBuf, marker: String },
    IoError { path: PathBuf, source: std::io::Error },
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound { path } => write!(f, "file not found: {}", path.display()),
            Self::MarkerNotFound { path, marker } => {
                write!(f, "marker '{}' not found in {}", marker, path.display())
            }
            Self::DuplicateTarget { path, marker } => {
                write!(f, "duplicate write to marker '{}' in {}", marker, path.display())
            }
            Self::IoError { path, source } => {
                write!(f, "I/O error on {}: {}", path.display(), source)
            }
        }
    }
}

impl std::error::Error for PlanError {}

/// Collects all `FileEdit`s and validates/applies them.
pub struct MutationPlan {
    pub edits: Vec<FileEdit>,
}

impl MutationPlan {
    /// Validate all edits without writing anything.
    pub fn validate(&self) -> Result<(), Vec<PlanError>> {
        let mut errors = Vec::new();
        let mut seen_targets: HashSet<(PathBuf, String)> = HashSet::new();

        for edit in &self.edits {
            match edit {
                FileEdit::Overwrite { path, .. } => {
                    if !path.exists() {
                        errors.push(PlanError::FileNotFound { path: path.clone() });
                        continue;
                    }
                    let key = (path.clone(), String::new());
                    if !seen_targets.insert(key) {
                        errors.push(PlanError::DuplicateTarget {
                            path: path.clone(),
                            marker: "(full overwrite)".into(),
                        });
                    }
                }
                FileEdit::ReplaceBlock {
                    path, start_marker, end_marker, ..
                } => {
                    if !path.exists() {
                        errors.push(PlanError::FileNotFound { path: path.clone() });
                        continue;
                    }
                    let content = match std::fs::read_to_string(path) {
                        Ok(c) => c,
                        Err(e) => {
                            errors.push(PlanError::IoError { path: path.clone(), source: e });
                            continue;
                        }
                    };
                    if !content.contains(start_marker.as_str()) {
                        errors.push(PlanError::MarkerNotFound { path: path.clone(), marker: start_marker.clone() });
                    }
                    if !content.contains(end_marker.as_str()) {
                        errors.push(PlanError::MarkerNotFound { path: path.clone(), marker: end_marker.clone() });
                    }
                    let key = (path.clone(), start_marker.clone());
                    if !seen_targets.insert(key) {
                        errors.push(PlanError::DuplicateTarget { path: path.clone(), marker: start_marker.clone() });
                    }
                }
                FileEdit::CreateOrReplaceBlock {
                    path, start_marker, end_marker, template, ..
                } => {
                    let content = if path.exists() {
                        match std::fs::read_to_string(path) {
                            Ok(c) => c,
                            Err(e) => {
                                errors.push(PlanError::IoError { path: path.clone(), source: e });
                                continue;
                            }
                        }
                    } else {
                        template.clone()
                    };
                    if !content.contains(start_marker.as_str()) {
                        errors.push(PlanError::MarkerNotFound { path: path.clone(), marker: start_marker.clone() });
                    }
                    if !content.contains(end_marker.as_str()) {
                        errors.push(PlanError::MarkerNotFound { path: path.clone(), marker: end_marker.clone() });
                    }
                    let key = (path.clone(), start_marker.clone());
                    if !seen_targets.insert(key) {
                        errors.push(PlanError::DuplicateTarget { path: path.clone(), marker: start_marker.clone() });
                    }
                }
            }
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;
    use tempfile::NamedTempFile;

    fn tmp_file(content: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f
    }

    #[test]
    fn validate_passes_for_valid_replace_block() {
        let f = tmp_file("before\n<!-- START -->\nold\n<!-- END -->\nafter");
        let plan = MutationPlan {
            edits: vec![FileEdit::ReplaceBlock {
                path: f.path().to_path_buf(),
                start_marker: "<!-- START -->".into(),
                end_marker: "<!-- END -->".into(),
                content: "new".into(),
            }],
        };
        assert!(plan.validate().is_ok());
    }

    #[test]
    fn validate_fails_for_missing_file() {
        let plan = MutationPlan {
            edits: vec![FileEdit::Overwrite {
                path: PathBuf::from("/nonexistent/file.txt"),
                content: "x".into(),
            }],
        };
        let errors = plan.validate().unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(matches!(&errors[0], PlanError::FileNotFound { .. }));
    }

    #[test]
    fn validate_fails_for_missing_start_marker() {
        let f = tmp_file("no markers here");
        let plan = MutationPlan {
            edits: vec![FileEdit::ReplaceBlock {
                path: f.path().to_path_buf(),
                start_marker: "<!-- START -->".into(),
                end_marker: "<!-- END -->".into(),
                content: "new".into(),
            }],
        };
        let errors = plan.validate().unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, PlanError::MarkerNotFound { .. })));
    }

    #[test]
    fn validate_fails_for_duplicate_marker_target() {
        let f = tmp_file("<!-- START -->\nold\n<!-- END -->");
        let path = f.path().to_path_buf();
        let plan = MutationPlan {
            edits: vec![
                FileEdit::ReplaceBlock {
                    path: path.clone(),
                    start_marker: "<!-- START -->".into(),
                    end_marker: "<!-- END -->".into(),
                    content: "first".into(),
                },
                FileEdit::ReplaceBlock {
                    path,
                    start_marker: "<!-- START -->".into(),
                    end_marker: "<!-- END -->".into(),
                    content: "second".into(),
                },
            ],
        };
        let errors = plan.validate().unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, PlanError::DuplicateTarget { .. })));
    }

    #[test]
    fn validate_allows_create_or_replace_on_missing_file() {
        let plan = MutationPlan {
            edits: vec![FileEdit::CreateOrReplaceBlock {
                path: PathBuf::from("/tmp/test-plan-nonexistent.md"),
                start_marker: "<!-- START -->".into(),
                end_marker: "<!-- END -->".into(),
                content: "body".into(),
                template: "# Title\n\n<!-- START -->\n<!-- END -->\n".into(),
            }],
        };
        assert!(plan.validate().is_ok());
    }
}
