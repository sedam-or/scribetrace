//! `FsHistoryStore::list_papers` (used by the CLI): only real, validly named folders count.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use scribetrace_core::adapters::FsHistoryStore;

#[test]
fn missing_papers_folder_is_empty() {
    let dir = tempfile::tempdir().unwrap();
    assert!(FsHistoryStore::new(dir.path())
        .list_papers()
        .unwrap()
        .is_empty());
}

#[test]
fn lists_valid_paper_folders_sorted_and_skips_the_rest() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join(".scribetrace/papers");
    for name in ["z_B", "h_a", "bad name", "z_A"] {
        std::fs::create_dir_all(root.join(name)).unwrap();
    }
    std::fs::write(root.join("z_file"), b"x").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(root.join("z_A"), root.join("z_link")).unwrap();

    let ids: Vec<String> = FsHistoryStore::new(dir.path())
        .list_papers()
        .unwrap()
        .iter()
        .map(|p| p.as_str().to_owned())
        .collect();
    assert_eq!(ids, ["h_a", "z_A", "z_B"]);
}
