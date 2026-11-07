#![allow(clippy::unwrap_used, clippy::expect_used)] // integration tests are not shipped code
//! Slice 6 acceptance (R-11): milestones are made on import only when something changed,
//! and renaming adds a line without touching any existing one.

use std::cell::{Cell, RefCell};
use std::fs;

use scribetrace_core::adapters::FsHistoryStore;
use scribetrace_core::application::{
    list_milestones, Clock, HistoryStore, IdGenerator, ImportMarks, PaperItem, RenameMilestone,
    SourceError, ZoteroSource,
};
use scribetrace_core::domain::{DeviceId, EventBody, Mark, MilestoneReason, PaperId};

/// Serves whatever marks the test has set; never touches the network.
struct FakeZotero {
    marks: RefCell<Vec<Mark>>,
}

impl ZoteroSource for FakeZotero {
    fn list_papers(&self) -> Result<Vec<PaperItem>, SourceError> {
        Ok(Vec::new())
    }

    fn fetch_marks(&self, _attachment_key: &str) -> Result<Vec<Mark>, SourceError> {
        Ok(self.marks.borrow().clone())
    }
}

struct FixedClock;

impl Clock for FixedClock {
    fn now(&self) -> String {
        "2026-11-06T09:00:00Z".to_owned()
    }
}

/// Produces UUID-shaped ids in order, so every line gets a distinct, valid `id`.
struct SeqIds(Cell<u64>);

impl IdGenerator for SeqIds {
    fn next_id(&self) -> String {
        let n = self.0.get() + 1;
        self.0.set(n);
        format!("00000000-0000-4000-8000-{n:012x}")
    }
}

fn mark(key: &str) -> Mark {
    Mark {
        key: key.to_owned(),
        kind: "highlight".to_owned(),
        text: Some("some text".to_owned()),
        comment: None,
        color: Some("#ffd400".to_owned()),
        page_index: Some(0),
        added: None,
        modified: None,
    }
}

fn device() -> DeviceId {
    DeviceId::new("dev-test").unwrap()
}

fn paper() -> PaperId {
    PaperId::from_zotero("SLICE6").unwrap()
}

fn history_file(workspace: &std::path::Path) -> std::path::PathBuf {
    workspace.join(".scribetrace/papers/z_SLICE6/dev-test.jsonl")
}

fn import(
    zotero: &FakeZotero,
    store: &FsHistoryStore,
    ids: &SeqIds,
) -> scribetrace_core::application::ImportReport {
    ImportMarks::new(zotero, store, &FixedClock, ids, device())
        .run(&paper(), "ATTACH01")
        .unwrap()
}

#[test]
fn first_import_makes_one_import_milestone() {
    let workspace = tempfile::tempdir().unwrap();
    let store = FsHistoryStore::new(workspace.path());
    let zotero = FakeZotero {
        marks: RefCell::new(vec![mark("AAAA0001")]),
    };
    let ids = SeqIds(Cell::new(0));

    let report = import(&zotero, &store, &ids);

    assert_eq!(report.imported, 1);
    assert!(report.milestone.is_some());
    let milestones = list_milestones(&store, &paper()).unwrap();
    assert_eq!(milestones.len(), 1);
    assert_eq!(milestones[0].reason, MilestoneReason::Import);
    assert_eq!(milestones[0].label, None);
}

#[test]
fn refresh_with_no_change_creates_no_milestone() {
    let workspace = tempfile::tempdir().unwrap();
    let store = FsHistoryStore::new(workspace.path());
    let zotero = FakeZotero {
        marks: RefCell::new(vec![mark("AAAA0001")]),
    };
    let ids = SeqIds(Cell::new(0));
    import(&zotero, &store, &ids);
    let before = fs::read(history_file(workspace.path())).unwrap();

    let report = import(&zotero, &store, &ids);

    assert_eq!(report.imported, 0);
    assert_eq!(report.milestone, None);
    assert_eq!(list_milestones(&store, &paper()).unwrap().len(), 1);
    assert_eq!(fs::read(history_file(workspace.path())).unwrap(), before);
}

#[test]
fn refresh_with_a_new_mark_creates_one_refresh_milestone() {
    let workspace = tempfile::tempdir().unwrap();
    let store = FsHistoryStore::new(workspace.path());
    let zotero = FakeZotero {
        marks: RefCell::new(vec![mark("AAAA0001")]),
    };
    let ids = SeqIds(Cell::new(0));
    import(&zotero, &store, &ids);

    zotero.marks.borrow_mut().push(mark("BBBB0002"));
    let report = import(&zotero, &store, &ids);

    assert_eq!(report.imported, 1);
    assert!(report.milestone.is_some());
    let milestones = list_milestones(&store, &paper()).unwrap();
    assert_eq!(milestones.len(), 2);
    assert_eq!(milestones[1].reason, MilestoneReason::Refresh);
    // The cut covers the new mark: seq 4 (mark 1, metadata 2, milestone 3, mark 4).
    assert_eq!(milestones[1].upto.get(&device()), Some(&4));
}

#[test]
fn a_mark_removed_in_zotero_is_not_written_again_on_refresh() {
    let workspace = tempfile::tempdir().unwrap();
    let store = FsHistoryStore::new(workspace.path());
    let zotero = FakeZotero {
        marks: RefCell::new(vec![mark("AAAA0001")]),
    };
    let ids = SeqIds(Cell::new(0));
    import(&zotero, &store, &ids);

    zotero.marks.borrow_mut().clear();
    let report = import(&zotero, &store, &ids);
    assert_eq!(report.imported, 0);

    zotero.marks.borrow_mut().push(mark("AAAA0001"));
    let report = import(&zotero, &store, &ids);
    assert_eq!(
        report.imported, 0,
        "a key already on disk is not a new mark"
    );
}

#[test]
fn rename_adds_a_line_and_changes_no_existing_line() {
    let workspace = tempfile::tempdir().unwrap();
    let store = FsHistoryStore::new(workspace.path());
    let zotero = FakeZotero {
        marks: RefCell::new(vec![mark("AAAA0001")]),
    };
    let ids = SeqIds(Cell::new(0));
    import(&zotero, &store, &ids);
    let milestone_id = list_milestones(&store, &paper()).unwrap()[0].id.clone();
    let before = fs::read_to_string(history_file(workspace.path())).unwrap();

    let renamed = RenameMilestone::new(&store, &FixedClock, &ids, device())
        .run(&paper(), &milestone_id, Some("First read".to_owned()))
        .unwrap();

    assert!(renamed);
    let after = fs::read_to_string(history_file(workspace.path())).unwrap();
    assert!(
        after.starts_with(&before),
        "existing lines must be byte-identical"
    );
    assert_eq!(after.lines().count(), before.lines().count() + 1);
    let milestones = list_milestones(&store, &paper()).unwrap();
    assert_eq!(milestones[0].label.as_deref(), Some("First read"));
}

#[test]
fn rename_of_a_missing_milestone_writes_nothing() {
    let workspace = tempfile::tempdir().unwrap();
    let store = FsHistoryStore::new(workspace.path());
    let zotero = FakeZotero {
        marks: RefCell::new(vec![mark("AAAA0001")]),
    };
    let ids = SeqIds(Cell::new(0));
    import(&zotero, &store, &ids);
    let before = fs::read(history_file(workspace.path())).unwrap();

    let renamed = RenameMilestone::new(&store, &FixedClock, &ids, device())
        .run(
            &paper(),
            "00000000-0000-4000-8000-0000000000ff",
            Some("Ghost".to_owned()),
        )
        .unwrap();

    assert!(!renamed);
    assert_eq!(fs::read(history_file(workspace.path())).unwrap(), before);
}

#[test]
fn rename_longer_than_the_limit_is_refused() {
    let workspace = tempfile::tempdir().unwrap();
    let store = FsHistoryStore::new(workspace.path());
    let zotero = FakeZotero {
        marks: RefCell::new(vec![mark("AAAA0001")]),
    };
    let ids = SeqIds(Cell::new(0));
    import(&zotero, &store, &ids);
    let milestone_id = list_milestones(&store, &paper()).unwrap()[0].id.clone();
    let before = fs::read(history_file(workspace.path())).unwrap();

    let too_long = "a".repeat(201);
    let result = RenameMilestone::new(&store, &FixedClock, &ids, device()).run(
        &paper(),
        &milestone_id,
        Some(too_long),
    );

    assert!(result.is_err());
    assert_eq!(fs::read(history_file(workspace.path())).unwrap(), before);
}

/// Appends a `purge` line that covers `device` up to `seq` (the purge is written by hand:
/// the application layer has no forget command yet).
fn append_purge(workspace: &std::path::Path, covers_seq: u64) {
    use std::io::Write;
    let line = serde_json::json!({
        "v": 1,
        "id": "6f1f2c1e-0a4b-4c55-9d3e-2b7a9a0c1e01",
        "device": "dev-test",
        "seq": covers_seq + 1,
        "ts": "2026-11-06T09:30:00Z",
        "author": "Me",
        "type": "purge",
        "payload": { "covers": { "dev-test": covers_seq } }
    });
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(history_file(workspace))
        .unwrap();
    writeln!(file, "{line}").unwrap();
}

#[test]
fn rename_of_a_hidden_milestone_writes_nothing() {
    let workspace = tempfile::tempdir().unwrap();
    let store = FsHistoryStore::new(workspace.path());
    let zotero = FakeZotero {
        marks: RefCell::new(vec![mark("AAAA0001")]),
    };
    let ids = SeqIds(Cell::new(0));
    import(&zotero, &store, &ids);
    let milestone_id = list_milestones(&store, &paper()).unwrap()[0].id.clone();
    // The import wrote mark, metadata, milestone (seq 1..=3); purge hides all three.
    append_purge(workspace.path(), 3);
    let before = fs::read(history_file(workspace.path())).unwrap();

    let renamed = RenameMilestone::new(&store, &FixedClock, &ids, device())
        .run(&paper(), &milestone_id, Some("Too late".to_owned()))
        .unwrap();

    assert!(!renamed, "a hidden milestone must not be renamed");
    assert_eq!(fs::read(history_file(workspace.path())).unwrap(), before);
}

#[test]
fn purged_mark_is_not_imported_again_on_refresh() {
    let workspace = tempfile::tempdir().unwrap();
    let store = FsHistoryStore::new(workspace.path());
    let zotero = FakeZotero {
        marks: RefCell::new(vec![mark("AAAA0001")]),
    };
    let ids = SeqIds(Cell::new(0));
    import(&zotero, &store, &ids);
    // Purge hides the mark (seq 1), metadata (seq 2), and milestone (seq 3).
    append_purge(workspace.path(), 3);
    let before = fs::read(history_file(workspace.path())).unwrap();

    let report = import(&zotero, &store, &ids);

    assert_eq!(report.imported, 0, "dedup must see purged lines on disk");
    assert!(report.milestone.is_none());
    let after = fs::read(history_file(workspace.path())).unwrap();
    assert!(
        after.starts_with(&before),
        "refresh must append without rewriting"
    );
    let raw = store.read_report(&paper()).unwrap().raw_events;
    assert_eq!(
        raw.iter()
            .filter(|event| matches!(event.body, EventBody::MarkImport(_)))
            .count(),
        1,
        "dedup must still see the purged mark on disk"
    );
    assert_eq!(
        raw.iter()
            .filter(|event| matches!(event.body, EventBody::PaperMetadata(_)))
            .count(),
        2,
        "the import must restore metadata hidden by purge"
    );
}
