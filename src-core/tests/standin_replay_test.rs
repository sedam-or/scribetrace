#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::{Path, PathBuf};

use scribetrace_core::adapters::{FsHistoryStore, SystemClock, UuidGenerator, ZoteroHttp};
use scribetrace_core::application::{HistoryStore, ImportMarks, ZoteroSource};
use scribetrace_core::domain::{DeviceId, EventBody, PaperId};
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
fn test_list_papers_from_standin_fixtures() {
    let server = StandIn::start(fixtures(), 0).unwrap();
    let zotero = ZoteroHttp::new(server.port());

    let papers = zotero.list_papers().unwrap();
    assert_eq!(papers.len(), 3, "Expected 3 top-level arXiv papers");

    let vaswani = papers.iter().find(|p| p.id == "z_6WG6QNHB").unwrap();
    assert_eq!(vaswani.title, "Attention is all you need");
    assert_eq!(vaswani.authors.as_deref(), Some("Vaswani et al."));
    assert_eq!(vaswani.year.as_deref(), Some("2017"));
    assert_eq!(vaswani.attachment_key.as_deref(), Some("H3E7BPJW"));
    assert_eq!(vaswani.marks_count, 2);

    let he = papers.iter().find(|p| p.id == "z_KBPLWQT5").unwrap();
    assert_eq!(he.title, "Deep residual learning for image recognition");
    assert_eq!(he.authors.as_deref(), Some("He et al."));
    assert_eq!(he.year.as_deref(), Some("2016"));
    assert_eq!(he.attachment_key.as_deref(), Some("45Y8JGAU"));
    assert_eq!(he.marks_count, 2);

    let brown = papers.iter().find(|p| p.id == "z_HMIBIMH7").unwrap();
    assert_eq!(brown.title, "Language models are few-shot learners");
    assert_eq!(brown.authors.as_deref(), Some("Brown et al."));
    assert_eq!(brown.year.as_deref(), Some("2020"));
    assert_eq!(brown.attachment_key.as_deref(), Some("RNBV2SQM"));
    assert_eq!(brown.marks_count, 2);
}

#[test]
fn test_import_and_refresh_marks_through_slices_2_3_store() {
    let server = StandIn::start(fixtures(), 0).unwrap();
    let workspace = tempfile::tempdir().unwrap();

    let pdf_path = workspace.path().join("Vaswani_2017.pdf");
    fs::write(&pdf_path, b"%PDF-1.5 original document content").unwrap();
    let before_snapshot = snapshot(workspace.path());

    let zotero = ZoteroHttp::new(server.port());
    let store = FsHistoryStore::new(workspace.path());
    let device = DeviceId::new("dev-primary").unwrap();
    let paper = PaperId::from_zotero("6WG6QNHB").unwrap();

    // 1. Initial Import
    let report = ImportMarks::new(
        &zotero,
        &store,
        &SystemClock,
        &UuidGenerator,
        device.clone(),
    )
    .run(&paper, "H3E7BPJW")
    .unwrap();
    assert_eq!(report.imported, 2, "Expected 2 marks imported");

    let read_report = store.read_report(&paper).unwrap();
    assert_eq!(read_report.files_scanned, 1);
    assert_eq!(read_report.skipped_lines, 0);
    assert_eq!(read_report.visible_events.len(), 2);

    let mut keys: Vec<String> = read_report
        .visible_events
        .iter()
        .filter_map(|e| match &e.body {
            EventBody::MarkImport(m) => Some(m.key.clone()),
            _ => None,
        })
        .collect();
    keys.sort();
    assert_eq!(keys, vec!["JYRYDGDI", "QYLASC88"]);

    // Verify mark fields: text, comment, color, position/pageIndex, dates (R-01, R-03)
    let note_mark = read_report
        .visible_events
        .iter()
        .find_map(|e| match &e.body {
            EventBody::MarkImport(m) if m.key == "QYLASC88" => Some(m),
            _ => None,
        })
        .unwrap();
    assert_eq!(note_mark.kind, "note");
    assert_eq!(
        note_mark.comment.as_deref(),
        Some("Self-attention mechanism replaces recurrent layers entirely.")
    );
    assert_eq!(note_mark.page_index, Some(0)); // Page 1 = index 0
    assert_eq!(note_mark.color.as_deref(), Some("#ff6666"));
    assert_eq!(note_mark.added.as_deref(), Some("2026-11-05T18:25:32Z"));
    assert_eq!(note_mark.modified.as_deref(), Some("2026-11-05T18:25:32Z"));

    let highlight_mark = read_report
        .visible_events
        .iter()
        .find_map(|e| match &e.body {
            EventBody::MarkImport(m) if m.key == "JYRYDGDI" => Some(m),
            _ => None,
        })
        .unwrap();
    assert_eq!(highlight_mark.kind, "highlight");
    assert!(highlight_mark
        .text
        .as_deref()
        .unwrap()
        .contains("The dominant sequence transduction models"));
    assert_eq!(
        highlight_mark.comment.as_deref(),
        Some("Background context on previous state of the art.")
    );
    assert_eq!(highlight_mark.page_index, Some(0));
    assert_eq!(highlight_mark.color.as_deref(), Some("#ffd400"));

    // 2. Refresh (monotonically appends events, next_seq increments)
    let refresh_report = ImportMarks::new(
        &zotero,
        &store,
        &SystemClock,
        &UuidGenerator,
        device.clone(),
    )
    .run(&paper, "H3E7BPJW")
    .unwrap();
    assert_eq!(refresh_report.imported, 2);

    let read_report_after = store.read_report(&paper).unwrap();
    // 4 raw events across 2 imports
    assert_eq!(read_report_after.raw_events.len(), 4);
    assert_eq!(read_report_after.raw_events[0].seq, 1);
    assert_eq!(read_report_after.raw_events[1].seq, 2);
    assert_eq!(read_report_after.raw_events[2].seq, 3);
    assert_eq!(read_report_after.raw_events[3].seq, 4);

    // SEC-19: Source documents in workspace remain byte-identical
    let after_pdf = fs::read(&pdf_path).unwrap();
    let original_pdf = before_snapshot
        .iter()
        .find(|(p, _)| p == &pdf_path)
        .map(|(_, b)| b)
        .unwrap();
    assert_eq!(
        &after_pdf, original_pdf,
        "Source PDF must remain strictly untouched and byte-identical"
    );
}
