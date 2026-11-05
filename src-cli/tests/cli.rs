//! Golden and behaviour tests for `scribetrace-cli` (slice 13, ADR 0010).
//!
//! The golden files are the contract other tools may rely on: change them only together with
//! `spec/FORMAT.md`. Set `UPDATE_GOLDEN=1` to rewrite them after an intended change.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::process::Command;

fn sample() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../spec/sample-project")
}

fn golden(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden")
        .join(name)
}

fn cli(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_scribetrace-cli"))
        .args(args)
        .output()
        .unwrap();
    (
        out.status.code().unwrap(),
        String::from_utf8(out.stdout).unwrap(),
        String::from_utf8(out.stderr).unwrap(),
    )
}

fn assert_golden(name: &str, actual: &str) {
    let path = golden(name);
    if std::env::var_os("UPDATE_GOLDEN").is_some() {
        std::fs::write(&path, actual).unwrap();
    }
    let expected = std::fs::read_to_string(&path).unwrap();
    assert_eq!(actual, expected, "golden file {name} differs");
}

#[test]
fn check_matches_golden_and_exits_1_on_bad_lines() {
    let (code, out, err) = cli(&["check", sample().to_str().unwrap()]);
    assert_eq!(code, 1);
    assert_eq!(err, "");
    assert_golden("check.txt", &out);
}

#[test]
fn export_matches_golden() {
    let (code, out, err) = cli(&["export", sample().to_str().unwrap()]);
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert_golden("export.md", &out);
}

#[test]
fn export_one_paper_hides_purged_marks_and_dedupes() {
    let (code, out, _) = cli(&["export", sample().to_str().unwrap(), "z_ABCD1234"]);
    assert_eq!(code, 0);
    assert!(!out.contains("Erased on purpose"), "purged mark leaked");
    assert!(!out.contains("QQ11WW22"), "other paper leaked");
    assert_eq!(out.matches("AB12CD34").count(), 1);
}

#[test]
fn export_is_safe_for_terminals_and_markdown() {
    let (_, out, _) = cli(&["export", sample().to_str().unwrap()]);
    assert!(!out.contains('\u{1b}'), "terminal escape reached output");
    assert!(!out.contains('\u{202e}'), "bidi override reached output");
    assert!(!out.contains("<b>"), "raw html reached output");
    assert!(out.contains(r"\<b\>this\</b\>"));
}

#[test]
fn clean_project_exits_0() {
    let dir = tempfile::tempdir().unwrap();
    let paper = dir.path().join(".scribetrace/papers/z_AAAA1111");
    std::fs::create_dir_all(&paper).unwrap();
    std::fs::write(
        paper.join("dev-a.jsonl"),
        concat!(
            r#"{"v":1,"id":"22222222-0000-4000-8000-000000000001","device":"dev-a","seq":1,"#,
            r#""ts":"2026-11-06T09:00:00Z","type":"mark_import","payload":{"key":"AAAA1111","kind":"note"}}"#,
            "\n"
        ),
    )
    .unwrap();
    let (code, out, _) = cli(&["check", dir.path().to_str().unwrap()]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("checked 1 papers, 0 bad lines"));
}

#[test]
fn usage_errors_exit_2_with_a_message_on_stderr() {
    let s = sample();
    let s = s.to_str().unwrap();
    for args in [
        vec!["bogus"],
        vec!["check"],
        vec!["check", s, "extra"],
        vec!["export", s, "no_such_paper"],
        vec!["export", s, "../etc"],
        vec!["check", "/nonexistent-scribetrace-folder"],
    ] {
        let (code, out, err) = cli(&args);
        assert_eq!(code, 2, "{args:?}");
        assert_eq!(out, "", "{args:?}");
        assert!(err.starts_with("scribetrace-cli: "), "{args:?}: {err}");
    }
}

#[test]
fn help_and_version_exit_0() {
    let (code, out, _) = cli(&["--help"]);
    assert_eq!(code, 0);
    assert!(out.contains("usage:"));
    let (code, out, _) = cli(&["--version"]);
    assert_eq!(code, 0);
    assert!(out.starts_with("scribetrace-cli "));
}

#[test]
fn commands_never_change_the_folder() {
    let dir = tempfile::tempdir().unwrap();
    copy_dir(&sample(), dir.path());
    let before = snapshot(dir.path());
    cli(&["check", dir.path().to_str().unwrap()]);
    cli(&["export", dir.path().to_str().unwrap()]);
    assert_eq!(snapshot(dir.path()), before);
}

fn copy_dir(from: &Path, to: &Path) {
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let dest = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            std::fs::create_dir_all(&dest).unwrap();
            copy_dir(&entry.path(), &dest);
        } else {
            std::fs::copy(entry.path(), dest).unwrap();
        }
    }
}

fn snapshot(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    fn walk(dir: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap())
            .collect();
        entries.sort_by_key(|e| e.file_name());
        for e in entries {
            if e.file_type().unwrap().is_dir() {
                walk(&e.path(), out);
            } else {
                out.push((e.path(), std::fs::read(e.path()).unwrap()));
            }
        }
    }
    let mut out = Vec::new();
    walk(root, &mut out);
    out
}
