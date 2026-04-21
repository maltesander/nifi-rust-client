//! External unit tests for `MutationPlan` — exercise each `FileEdit` variant
//! and the `validate` / `apply` / `check` entry points through the public API
//! surface (complements the inline tests at the bottom of `plan.rs`).

use std::io::Write as _;
use std::path::PathBuf;

use nifi_openapi_gen::plan::{FileEdit, MutationPlan, PlanError};
use tempfile::{NamedTempFile, tempdir};

fn tmp_file(content: &str) -> NamedTempFile {
    let mut f = NamedTempFile::new().expect("create temp file");
    f.write_all(content.as_bytes()).expect("write temp file");
    f
}

#[test]
fn file_edit_path_accessor_returns_inner_path() {
    let p1 = PathBuf::from("/tmp/a.rs");
    let p2 = PathBuf::from("/tmp/b.rs");
    let p3 = PathBuf::from("/tmp/c.rs");

    let overwrite = FileEdit::Overwrite {
        path: p1.clone(),
        content: "x".into(),
    };
    let replace = FileEdit::ReplaceBlock {
        path: p2.clone(),
        start_marker: "S".into(),
        end_marker: "E".into(),
        content: "x".into(),
    };
    let create_or_replace = FileEdit::CreateOrReplaceBlock {
        path: p3.clone(),
        start_marker: "S".into(),
        end_marker: "E".into(),
        content: "x".into(),
        template: "tpl".into(),
    };

    assert_eq!(overwrite.path(), p1.as_path());
    assert_eq!(replace.path(), p2.as_path());
    assert_eq!(create_or_replace.path(), p3.as_path());
}

#[test]
fn apply_overwrite_reports_unchanged_when_content_matches() {
    let f = tmp_file("same content");
    let plan = MutationPlan {
        edits: vec![FileEdit::Overwrite {
            path: f.path().to_path_buf(),
            content: "same content".into(),
        }],
    };
    let report = plan.apply().expect("apply succeeds");
    assert_eq!(report.written, 0);
    assert_eq!(report.unchanged, 1);
    // File content is untouched.
    let on_disk = std::fs::read_to_string(f.path()).unwrap();
    assert_eq!(on_disk, "same content");
}

#[test]
fn apply_reports_mixed_written_and_unchanged_across_multiple_edits() {
    let f_changed = tmp_file("before\n<!-- S -->\nold\n<!-- E -->\nafter");
    let f_same = tmp_file("identical");
    let f_overwrite = tmp_file("<!-- S -->\nold\n<!-- E -->");

    let plan = MutationPlan {
        edits: vec![
            FileEdit::ReplaceBlock {
                path: f_changed.path().to_path_buf(),
                start_marker: "<!-- S -->".into(),
                end_marker: "<!-- E -->".into(),
                content: "new".into(),
            },
            FileEdit::Overwrite {
                path: f_same.path().to_path_buf(),
                content: "identical".into(),
            },
            FileEdit::ReplaceBlock {
                path: f_overwrite.path().to_path_buf(),
                start_marker: "<!-- S -->".into(),
                end_marker: "<!-- E -->".into(),
                content: "old".into(), // already matches
            },
        ],
    };
    let report = plan.apply().expect("apply succeeds");
    assert_eq!(report.written, 1, "one file rewrote");
    assert_eq!(report.unchanged, 2, "two files already matched");
}

#[test]
fn validate_detects_duplicate_overwrite_targets() {
    let f = tmp_file("initial");
    let path = f.path().to_path_buf();
    let plan = MutationPlan {
        edits: vec![
            FileEdit::Overwrite {
                path: path.clone(),
                content: "first".into(),
            },
            FileEdit::Overwrite {
                path,
                content: "second".into(),
            },
        ],
    };
    let errors = plan.validate().expect_err("duplicate targets rejected");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, PlanError::DuplicateTarget { .. })),
        "expected DuplicateTarget, got {errors:?}"
    );
}

#[test]
fn apply_bails_before_writing_any_files_when_validation_fails() {
    // A valid edit paired with an invalid one — nothing should be written.
    let f = tmp_file("original");
    let missing = PathBuf::from("/nonexistent/definitely-not-there.txt");
    let plan = MutationPlan {
        edits: vec![
            FileEdit::Overwrite {
                path: f.path().to_path_buf(),
                content: "rewritten".into(),
            },
            FileEdit::Overwrite {
                path: missing,
                content: "irrelevant".into(),
            },
        ],
    };
    let err = plan.apply().expect_err("apply rejects invalid plan");
    assert!(
        err.iter()
            .any(|e| matches!(e, PlanError::FileNotFound { .. })),
        "expected FileNotFound, got {err:?}"
    );
    // Critically: the *valid* edit's file must be untouched.
    let on_disk = std::fs::read_to_string(f.path()).unwrap();
    assert_eq!(on_disk, "original");
}

#[test]
fn create_or_replace_block_then_replace_block_on_same_plan() {
    // First run creates the file; second run (new plan) replaces the marker
    // block in the now-existing file.
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("nested/sub/out.md");
    let template = "# Heading\n\n<!-- S -->\n<!-- E -->\n".to_string();

    let plan1 = MutationPlan {
        edits: vec![FileEdit::CreateOrReplaceBlock {
            path: path.clone(),
            start_marker: "<!-- S -->".into(),
            end_marker: "<!-- E -->".into(),
            content: "first".into(),
            template: template.clone(),
        }],
    };
    let r1 = plan1.apply().expect("first apply");
    assert_eq!(r1.written, 1);
    let after_first = std::fs::read_to_string(&path).unwrap();
    assert!(after_first.contains("first"));
    assert!(after_first.contains("# Heading"));

    // Second apply via CreateOrReplaceBlock — file exists, template ignored.
    let plan2 = MutationPlan {
        edits: vec![FileEdit::CreateOrReplaceBlock {
            path: path.clone(),
            start_marker: "<!-- S -->".into(),
            end_marker: "<!-- E -->".into(),
            content: "second".into(),
            template,
        }],
    };
    let r2 = plan2.apply().expect("second apply");
    assert_eq!(r2.written, 1);
    let after_second = std::fs::read_to_string(&path).unwrap();
    assert!(after_second.contains("second"));
    assert!(!after_second.contains("first"));
}

#[test]
fn check_does_not_mutate_filesystem() {
    let f = tmp_file("before\n<!-- S -->\nold\n<!-- E -->\nafter");
    let plan = MutationPlan {
        edits: vec![FileEdit::ReplaceBlock {
            path: f.path().to_path_buf(),
            start_marker: "<!-- S -->".into(),
            end_marker: "<!-- E -->".into(),
            content: "completely different".into(),
        }],
    };
    let report = plan.check().expect("check succeeds");
    assert!(report.has_drift());
    assert_eq!(report.drifted.len(), 1);
    assert_eq!(report.up_to_date, 0);
    // File is untouched after check().
    let on_disk = std::fs::read_to_string(f.path()).unwrap();
    assert!(on_disk.contains("old"));
    assert!(!on_disk.contains("completely different"));
}

#[test]
fn plan_error_display_mentions_path_and_marker() {
    let path = PathBuf::from("/tmp/example.rs");
    let not_found = PlanError::FileNotFound { path: path.clone() };
    let marker_missing = PlanError::MarkerNotFound {
        path: path.clone(),
        marker: "<!-- START -->".into(),
    };
    let dup = PlanError::DuplicateTarget {
        path,
        marker: "<!-- START -->".into(),
    };

    let s1 = format!("{not_found}");
    let s2 = format!("{marker_missing}");
    let s3 = format!("{dup}");

    assert!(s1.contains("/tmp/example.rs"), "got: {s1}");
    assert!(
        s2.contains("<!-- START -->") && s2.contains("/tmp/example.rs"),
        "got: {s2}"
    );
    assert!(
        s3.contains("duplicate") && s3.contains("<!-- START -->"),
        "got: {s3}"
    );
}
