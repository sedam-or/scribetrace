#![allow(clippy::unwrap_used, clippy::expect_used)] // integration tests are not shipped code
//! Slice 1 acceptance (a): core + adapters against a loopback stand-in Zotero server
//! write exactly one `mark_import` line and leave the sources byte-identical.

use std::fs;
use std::path::{Path, PathBuf};

use scribetrace_core::adapters::{FsHistoryStore, SystemClock, UuidGenerator, ZoteroHttp};
use scribetrace_core::application::{HistoryStore, ImportMarks};
use scribetrace_core::domain::{DeviceId, EventBody, MilestoneReason, PaperId};
use zotero_standin::StandIn;

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/fixtures/zotero")
}

fn snapshot(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in fs::read_dir(d).unwrap() {
            let p = entry.unwrap().path();
            if p.is_dir() {
                stack.push(p);
            } else {
                let bytes = fs::read(&p).unwrap();
                out.push((p, bytes));
            }
        }
    }
    out.sort();
    out
}

#[test]
fn import_writes_exactly_one_mark_import_line_and_keeps_sources_untouched() {
    let server = StandIn::start(fixtures(), 0).unwrap();
    let workspace = tempfile::tempdir().unwrap();
    fs::write(workspace.path().join("paper.pdf"), b"%PDF original").unwrap();
    let before = snapshot(workspace.path());

    let zotero = ZoteroHttp::new(server.port());
    let store = FsHistoryStore::new(workspace.path());
    let device = DeviceId::new("dev-test").unwrap();
    let paper = PaperId::from_zotero("EFGH6789").unwrap();
    let report = ImportMarks::new(
        &zotero,
        &store,
        &SystemClock,
        &UuidGenerator,
        device.clone(),
    )
    .run(&paper, "EFGH6789")
    .unwrap();
    assert_eq!(report.imported, 1);

    let file = workspace
        .path()
        .join(".scribetrace/papers/z_EFGH6789/dev-test.jsonl");
    // One mark_import line plus the first milestone (reason import) for this import.
    let raw = fs::read_to_string(&file).unwrap();
    assert_eq!(raw.lines().count(), 2);
    let events = store.read(&paper).unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].author, "Me");
    assert_eq!(events[0].seq, 1);
    let EventBody::MarkImport(mark) = &events[0].body else {
        panic!("expected MarkImport event body");
    };
    assert_eq!(mark.key, "WXYZ2345");
    assert_eq!(mark.page_index, Some(3));
    assert!(raw.contains("\"type\":\"mark_import\""));
    let EventBody::Milestone(ms) = &events[1].body else {
        panic!("expected Milestone event body");
    };
    assert_eq!(ms.reason, MilestoneReason::Import);
    assert_eq!(events[1].seq, 2);
    assert!(raw.contains("\"type\":\"milestone\""));

    // Everything that existed before is byte-identical; only the history file is new.
    let after = snapshot(workspace.path());
    let untouched: Vec<_> = after.iter().filter(|(p, _)| *p != file).cloned().collect();
    assert_eq!(untouched, before);
}

#[test]
fn unreachable_zotero_is_a_typed_error_and_writes_nothing() {
    let workspace = tempfile::tempdir().unwrap();
    let zotero = ZoteroHttp::new(1); // nothing listens on port 1
    let store = FsHistoryStore::new(workspace.path());
    let device = DeviceId::new("dev-test").unwrap();
    let paper = PaperId::from_zotero("EFGH6789").unwrap();
    let result = ImportMarks::new(&zotero, &store, &SystemClock, &UuidGenerator, device)
        .run(&paper, "EFGH6789");
    assert!(result.is_err());
    assert!(!workspace.path().join(".scribetrace").exists());
}

#[test]
fn hostile_ids_are_refused() {
    assert!(PaperId::new("../x").is_err());
    assert!(PaperId::new("").is_err());
    assert!(DeviceId::new("a/b").is_err());
}
