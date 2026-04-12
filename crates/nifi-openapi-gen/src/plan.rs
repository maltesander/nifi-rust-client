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

/// Summary of what `apply()` did.
#[derive(Debug, Default)]
pub struct ApplyReport {
    pub written: usize,
    pub unchanged: usize,
}

/// One file that drifted from the plan.
#[derive(Debug)]
pub struct DriftEntry {
    pub path: PathBuf,
    pub diff_lines: Vec<String>,
}

/// Summary of what `check()` found.
#[derive(Debug)]
pub struct CheckReport {
    pub drifted: Vec<DriftEntry>,
    pub up_to_date: usize,
}

impl CheckReport {
    pub fn has_drift(&self) -> bool {
        !self.drifted.is_empty()
    }
}

impl std::fmt::Display for CheckReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const TRUNCATE_AT: usize = 5;

        for entry in &self.drifted {
            writeln!(f, "DRIFT {}", entry.path.display())?;
            let total = entry.diff_lines.len();
            let show = if f.alternate() { total } else { total.min(TRUNCATE_AT) };
            for line in &entry.diff_lines[..show] {
                writeln!(f, "  {line}")?;
            }
            if !f.alternate() && total > TRUNCATE_AT {
                writeln!(
                    f,
                    "  ... ({} more lines, use --verbose for full diff)",
                    total - TRUNCATE_AT
                )?;
            }
        }
        writeln!(
            f,
            "\n{} file(s) drifted, {} up to date",
            self.drifted.len(),
            self.up_to_date
        )
    }
}

/// Produce a minimal line-level diff.
fn simple_diff(old: &str, new: &str) -> Vec<String> {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();
    let mut diff = Vec::new();

    let max_len = old_lines.len().max(new_lines.len());
    for i in 0..max_len {
        let old_line = old_lines.get(i).copied();
        let new_line = new_lines.get(i).copied();
        match (old_line, new_line) {
            (Some(o), Some(n)) if o != n => {
                diff.push(format!("-{o}"));
                diff.push(format!("+{n}"));
            }
            (Some(o), None) => {
                diff.push(format!("-{o}"));
            }
            (None, Some(n)) => {
                diff.push(format!("+{n}"));
            }
            _ => {}
        }
    }
    diff
}

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

    /// Validate, then diff the plan against disk without writing.
    /// Use `format!("{report}")` for truncated output, `format!("{report:#}")` for full.
    pub fn check(&self, _verbose: bool) -> Result<CheckReport, Vec<PlanError>> {
        self.validate()?;
        let mut report = CheckReport {
            drifted: Vec::new(),
            up_to_date: 0,
        };

        for edit in &self.edits {
            let (on_disk, planned) = match edit {
                FileEdit::Overwrite { path, content } => {
                    let on_disk = std::fs::read_to_string(path).unwrap_or_default();
                    (on_disk, content.clone())
                }
                FileEdit::ReplaceBlock { path, start_marker, end_marker, content } => {
                    let on_disk = std::fs::read_to_string(path).map_err(|e| {
                        vec![PlanError::IoError { path: path.clone(), source: e }]
                    })?;
                    let planned = crate::util::replace_between_markers(
                        &on_disk, start_marker, end_marker, content,
                    );
                    (on_disk, planned)
                }
                FileEdit::CreateOrReplaceBlock { path, start_marker, end_marker, content, template } => {
                    let on_disk = if path.exists() {
                        std::fs::read_to_string(path).map_err(|e| {
                            vec![PlanError::IoError { path: path.clone(), source: e }]
                        })?
                    } else {
                        String::new()
                    };
                    let base = if path.exists() { &on_disk } else { template.as_str() };
                    let planned = crate::util::replace_between_markers(
                        base, start_marker, end_marker, content,
                    );
                    (on_disk, planned)
                }
            };

            if on_disk == planned {
                report.up_to_date += 1;
            } else {
                let diff_lines = simple_diff(&on_disk, &planned);
                report.drifted.push(DriftEntry {
                    path: edit.path().to_path_buf(),
                    diff_lines,
                });
            }
        }

        Ok(report)
    }

    /// Validate, then apply all edits. Returns a report of what changed.
    pub fn apply(&self) -> Result<ApplyReport, Vec<PlanError>> {
        self.validate()?;
        let mut report = ApplyReport::default();

        for edit in &self.edits {
            match edit {
                FileEdit::Overwrite { path, content } => {
                    let on_disk = std::fs::read_to_string(path).unwrap_or_default();
                    if on_disk == *content {
                        report.unchanged += 1;
                    } else {
                        std::fs::write(path, content).map_err(|e| {
                            vec![PlanError::IoError { path: path.clone(), source: e }]
                        })?;
                        println!("  wrote {}", path.display());
                        report.written += 1;
                    }
                }
                FileEdit::ReplaceBlock { path, start_marker, end_marker, content } => {
                    let on_disk = std::fs::read_to_string(path).map_err(|e| {
                        vec![PlanError::IoError { path: path.clone(), source: e }]
                    })?;
                    let patched = crate::util::replace_between_markers(&on_disk, start_marker, end_marker, content);
                    if on_disk == patched {
                        report.unchanged += 1;
                    } else {
                        std::fs::write(path, &patched).map_err(|e| {
                            vec![PlanError::IoError { path: path.clone(), source: e }]
                        })?;
                        println!("  wrote {}", path.display());
                        report.written += 1;
                    }
                }
                FileEdit::CreateOrReplaceBlock { path, start_marker, end_marker, content, template } => {
                    let on_disk = if path.exists() {
                        std::fs::read_to_string(path).map_err(|e| {
                            vec![PlanError::IoError { path: path.clone(), source: e }]
                        })?
                    } else {
                        if let Some(parent) = path.parent() {
                            std::fs::create_dir_all(parent).map_err(|e| {
                                vec![PlanError::IoError { path: path.clone(), source: e }]
                            })?;
                        }
                        println!("  created {}", path.display());
                        template.clone()
                    };
                    let patched = crate::util::replace_between_markers(&on_disk, start_marker, end_marker, content);
                    if on_disk == patched && path.exists() {
                        report.unchanged += 1;
                    } else {
                        std::fs::write(path, &patched).map_err(|e| {
                            vec![PlanError::IoError { path: path.clone(), source: e }]
                        })?;
                        if on_disk != patched {
                            println!("  wrote {}", path.display());
                        }
                        report.written += 1;
                    }
                }
            }
        }
        Ok(report)
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
    fn apply_replace_block_updates_content() {
        let f = tmp_file("before\n<!-- START -->\nold\n<!-- END -->\nafter");
        let plan = MutationPlan {
            edits: vec![FileEdit::ReplaceBlock {
                path: f.path().to_path_buf(),
                start_marker: "<!-- START -->".into(),
                end_marker: "<!-- END -->".into(),
                content: "new content".into(),
            }],
        };
        let report = plan.apply().unwrap();
        assert_eq!(report.written, 1);
        assert_eq!(report.unchanged, 0);
        let result = std::fs::read_to_string(f.path()).unwrap();
        assert!(result.contains("new content"));
        assert!(result.contains("before"));
        assert!(result.contains("after"));
    }

    #[test]
    fn apply_overwrite_replaces_file() {
        let f = tmp_file("old content");
        let plan = MutationPlan {
            edits: vec![FileEdit::Overwrite {
                path: f.path().to_path_buf(),
                content: "new content".into(),
            }],
        };
        let report = plan.apply().unwrap();
        assert_eq!(report.written, 1);
        let result = std::fs::read_to_string(f.path()).unwrap();
        assert_eq!(result, "new content");
    }

    #[test]
    fn apply_skips_unchanged_file() {
        let f = tmp_file("before\n<!-- START -->\nold\n<!-- END -->\nafter");
        let plan = MutationPlan {
            edits: vec![FileEdit::ReplaceBlock {
                path: f.path().to_path_buf(),
                start_marker: "<!-- START -->".into(),
                end_marker: "<!-- END -->".into(),
                content: "old".into(),
            }],
        };
        let report = plan.apply().unwrap();
        assert_eq!(report.written, 0);
        assert_eq!(report.unchanged, 1);
    }

    #[test]
    fn apply_create_or_replace_creates_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("new-file.md");
        let plan = MutationPlan {
            edits: vec![FileEdit::CreateOrReplaceBlock {
                path: path.clone(),
                start_marker: "<!-- START -->".into(),
                end_marker: "<!-- END -->".into(),
                content: "body".into(),
                template: "# Title\n\n<!-- START -->\n<!-- END -->\n".into(),
            }],
        };
        let report = plan.apply().unwrap();
        assert_eq!(report.written, 1);
        let result = std::fs::read_to_string(&path).unwrap();
        assert!(result.contains("body"));
        assert!(result.contains("# Title"));
    }

    #[test]
    fn check_reports_no_drift_when_up_to_date() {
        let f = tmp_file("before\n<!-- START -->\nold\n<!-- END -->\nafter");
        let plan = MutationPlan {
            edits: vec![FileEdit::ReplaceBlock {
                path: f.path().to_path_buf(),
                start_marker: "<!-- START -->".into(),
                end_marker: "<!-- END -->".into(),
                content: "old".into(),
            }],
        };
        let report = plan.check(false).unwrap();
        assert!(!report.has_drift());
        assert_eq!(report.up_to_date, 1);
    }

    #[test]
    fn check_reports_drift_when_content_differs() {
        let f = tmp_file("before\n<!-- START -->\nold\n<!-- END -->\nafter");
        let plan = MutationPlan {
            edits: vec![FileEdit::ReplaceBlock {
                path: f.path().to_path_buf(),
                start_marker: "<!-- START -->".into(),
                end_marker: "<!-- END -->".into(),
                content: "new content here".into(),
            }],
        };
        let report = plan.check(false).unwrap();
        assert!(report.has_drift());
        assert_eq!(report.drifted.len(), 1);
    }

    #[test]
    fn check_truncates_diff_when_not_verbose() {
        let old_lines = (0..20).map(|i| format!("line{i}")).collect::<Vec<_>>().join("\n");
        let new_lines = (0..20).map(|i| format!("changed{i}")).collect::<Vec<_>>().join("\n");
        let file_content = format!("<!-- S -->\n{old_lines}\n<!-- E -->");
        let f = tmp_file(&file_content);
        let plan = MutationPlan {
            edits: vec![FileEdit::ReplaceBlock {
                path: f.path().to_path_buf(),
                start_marker: "<!-- S -->".into(),
                end_marker: "<!-- E -->".into(),
                content: new_lines,
            }],
        };
        let report = plan.check(false).unwrap();
        let output = format!("{report}");
        assert!(output.contains("..."));
        assert!(output.contains("--verbose"));
    }

    #[test]
    fn check_shows_full_diff_when_verbose() {
        let f = tmp_file("<!-- S -->\nold1\nold2\nold3\nold4\nold5\nold6\nold7\n<!-- E -->");
        let plan = MutationPlan {
            edits: vec![FileEdit::ReplaceBlock {
                path: f.path().to_path_buf(),
                start_marker: "<!-- S -->".into(),
                end_marker: "<!-- E -->".into(),
                content: "new1\nnew2\nnew3\nnew4\nnew5\nnew6\nnew7".into(),
            }],
        };
        let report = plan.check(true).unwrap();
        let output = format!("{report:#}");
        assert!(!output.contains("--verbose"));
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
