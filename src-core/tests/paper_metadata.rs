#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use scribetrace_core::adapters::FsHistoryStore;
use scribetrace_core::application::{
    Clock, HistoryStore, IdGenerator, ImportMarks, PaperItem, SourceError, ZoteroSource,
};
use scribetrace_core::domain::{
    DeviceId, Event, EventBody, Mark, PaperId, PaperMetadataPayload, PurgePayload, FORMAT_VERSION,
};

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
        "2026-11-07T04:00:00Z".to_owned()
    }
}

struct SeqIds(Cell<u64>);

impl IdGenerator for SeqIds {
    fn next_id(&self) -> String {
        let next = self.0.get() + 1;
        self.0.set(next);
        format!("00000000-0000-4000-8000-{next:012x}")
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

fn paper() -> PaperId {
    PaperId::from_zotero("PAPER001").unwrap()
}

fn device() -> DeviceId {
    DeviceId::new("dev-test").unwrap()
}

fn metadata(doi: Option<&str>, pdf_key: Option<&str>) -> PaperMetadataPayload {
    PaperMetadataPayload {
        doi: doi.map(str::to_owned),
        pdf_key: pdf_key.map(str::to_owned),
    }
}

fn metadata_event(device_id: &str, seq: u64, ts: &str, metadata: PaperMetadataPayload) -> Event {
    Event {
        v: FORMAT_VERSION,
        id: format!("00000000-0000-4000-8000-{seq:012x}"),
        device: DeviceId::new(device_id).unwrap(),
        seq,
        ts: ts.to_owned(),
        author: "Me".to_owned(),
        body: EventBody::PaperMetadata(metadata),
    }
}

fn import(
    zotero: &FakeZotero,
    store: &FsHistoryStore,
    ids: &SeqIds,
    attachment_key: &str,
    paper_metadata: PaperMetadataPayload,
) -> scribetrace_core::application::ImportReport {
    ImportMarks::new(zotero, store, &FixedClock, ids, device())
        .run_with_metadata(&paper(), attachment_key, paper_metadata)
        .unwrap()
}

fn append_purge(store: &FsHistoryStore, ids: &SeqIds) {
    let next_seq = store.next_seq(&paper(), &device()).unwrap();
    let event = Event {
        v: FORMAT_VERSION,
        id: ids.next_id(),
        device: device(),
        seq: next_seq,
        ts: FixedClock.now(),
        author: "Me".to_owned(),
        body: EventBody::Purge(PurgePayload {
            covers: HashMap::from([(device(), next_seq - 1)]),
        }),
    };
    store.append(&paper(), &event).unwrap();
}

#[test]
fn import_records_paper_metadata_only_when_a_source_field_changes() {
    let workspace = tempfile::tempdir().unwrap();
    let store = FsHistoryStore::new(workspace.path());
    let zotero = FakeZotero {
        marks: RefCell::new(vec![mark("AAAA0001")]),
    };
    let ids = SeqIds(Cell::new(0));
    let first = metadata(Some("10.12345/example"), Some("ATTACH01"));

    let first_report = import(&zotero, &store, &ids, "ATTACH01", first.clone());
    assert_eq!(first_report.imported, 1);
    assert_eq!(
        store
            .read_report(&paper())
            .unwrap()
            .raw_events
            .iter()
            .filter(|event| matches!(event.body, EventBody::PaperMetadata(_)))
            .count(),
        1
    );

    let unchanged_report = import(&zotero, &store, &ids, "ATTACH01", first);
    assert_eq!(unchanged_report.imported, 0);
    assert!(unchanged_report.milestone.is_none());
    assert_eq!(
        store
            .read_report(&paper())
            .unwrap()
            .raw_events
            .iter()
            .filter(|event| matches!(event.body, EventBody::PaperMetadata(_)))
            .count(),
        1,
        "an unchanged refresh must not append duplicate metadata"
    );

    let changed_report = import(
        &zotero,
        &store,
        &ids,
        "ATTACH02",
        metadata(None, Some("ATTACH02")),
    );
    assert_eq!(changed_report.imported, 0);
    assert!(changed_report.milestone.is_none());
    let visible_metadata: Vec<_> = store
        .read(&paper())
        .unwrap()
        .into_iter()
        .filter_map(|event| match event.body {
            EventBody::PaperMetadata(value) => Some(value),
            _ => None,
        })
        .collect();
    assert_eq!(visible_metadata.len(), 2);
    assert_eq!(visible_metadata[1], metadata(None, Some("ATTACH02")));
}

#[test]
fn import_rewrites_metadata_hidden_by_purge_without_reimporting_the_mark() {
    let workspace = tempfile::tempdir().unwrap();
    let store = FsHistoryStore::new(workspace.path());
    let zotero = FakeZotero {
        marks: RefCell::new(vec![mark("AAAA0001")]),
    };
    let ids = SeqIds(Cell::new(0));
    let source = metadata(Some("10.12345/example"), Some("ATTACH01"));
    import(&zotero, &store, &ids, "ATTACH01", source.clone());
    append_purge(&store, &ids);

    let report = import(&zotero, &store, &ids, "ATTACH01", source.clone());

    assert_eq!(report.imported, 0, "purged mark keys remain deduplicated");
    assert!(report.milestone.is_none(), "no new mark means no milestone");
    let raw = store.read_report(&paper()).unwrap().raw_events;
    assert_eq!(
        raw.iter()
            .filter(|event| matches!(event.body, EventBody::PaperMetadata(_)))
            .count(),
        2,
        "the hidden metadata snapshot is written again for the new local history"
    );
    let visible_metadata: Vec<_> = store
        .read(&paper())
        .unwrap()
        .into_iter()
        .filter_map(|event| match event.body {
            EventBody::PaperMetadata(value) => Some(value),
            _ => None,
        })
        .collect();
    assert_eq!(visible_metadata, vec![source]);
}

#[test]
fn effective_metadata_uses_per_device_seq_then_first_device_with_a_value() {
    let events = vec![
        metadata_event(
            "dev-b",
            1,
            "2026-11-07T05:00:00Z",
            metadata(Some("10.12345/device-b"), Some("PDFKEY02")),
        ),
        metadata_event(
            "dev-a",
            4,
            "2026-11-07T05:10:00Z",
            metadata(Some("10.12345/older-a"), Some("PDFKEY01")),
        ),
        metadata_event(
            "dev-a",
            5,
            "2026-11-07T04:00:00Z",
            metadata(None, Some("PDFKEY03")),
        ),
    ];

    let effective = scribetrace_core::application::effective_paper_metadata(&events);

    assert_eq!(effective.doi.as_deref(), Some("10.12345/device-b"));
    assert_eq!(effective.pdf_key.as_deref(), Some("PDFKEY03"));
}
