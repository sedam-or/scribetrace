#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use scribetrace_core::adapters::FsHistoryStore;
use scribetrace_core::application::{HistoryStore, LogWriter, StoreError};
use scribetrace_core::domain::{DeviceId, Event, EventBody, Mark, PaperId, MAX_LINE_BYTES};
use serde_json::{json, Value};
use tempfile::tempdir;

/// Fault-injecting writer that simulates process/power cut after exactly `cut_after` bytes.
struct FaultyWriter<W: Write> {
    inner: W,
    cut_after: usize,
    bytes_written: usize,
}

impl<W: Write> FaultyWriter<W> {
    fn new(inner: W, cut_after: usize) -> Self {
        Self {
            inner,
            cut_after,
            bytes_written: 0,
        }
    }
}

impl<W: Write> Write for FaultyWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.bytes_written >= self.cut_after {
            return Err(std::io::Error::new(
                std::io::ErrorKind::WriteZero,
                "simulated power/crash cut",
            ));
        }

        let remaining = self.cut_after - self.bytes_written;
        let to_write = buf.len().min(remaining);
        let written = self.inner.write(&buf[..to_write])?;
        self.bytes_written += written;

        if self.bytes_written >= self.cut_after {
            Err(std::io::Error::new(
                std::io::ErrorKind::WriteZero,
                "simulated power/crash cut",
            ))
        } else {
            Ok(written)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl<W: Write> LogWriter for FaultyWriter<W> {
    fn sync_data(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

fn test_uuid(id: &str) -> String {
    if id.len() == 36 && id.chars().filter(|&c| c == '-').count() == 4 {
        id.to_string()
    } else {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        let h1 = hasher.finish();
        "crash-append-test-salt".hash(&mut hasher);
        let h2 = hasher.finish();
        format!(
            "{:08x}-{:04x}-4000-8000-{:012x}",
            (h1 >> 32) as u32,
            (h1 & 0xffff) as u16,
            h2 & 0x0000_ffff_ffff_ffff
        )
    }
}

fn make_test_event(id: &str, device: DeviceId, seq: u64) -> Event {
    Event::new(
        1,
        test_uuid(id),
        device,
        seq,
        "2026-11-05T12:00:00Z",
        "Me",
        EventBody::MarkImport(Mark {
            key: format!("k-{seq}"),
            kind: "highlight".to_string(),
            text: Some(format!("Highlight for event {seq}")),
            comment: None,
            color: Some("#ff0000".to_string()),
            page_index: Some(seq as u32),
            added: Some("2026-11-05T12:00:00Z".to_string()),
            modified: None,
        }),
    )
}

#[test]
fn test_fault_injection_cut_after_every_byte_k_of_n() {
    let dev = DeviceId::new("dev-crash").unwrap();
    let paper = PaperId::new("z_crash_proof").unwrap();

    let target_event = make_test_event("target-ev", dev.clone(), 2);
    let mut line_bytes = serde_json::to_vec(&target_event).unwrap();
    line_bytes.push(b'\n');
    let total_len = line_bytes.len();

    // Test cutting after every byte k from 1 to total_len
    for k in 1..total_len {
        let tmp = tempdir().unwrap();
        let store = FsHistoryStore::new(tmp.path());

        // 1. Append valid baseline event 1
        let ev1 = make_test_event("ev-base-1", dev.clone(), 1);
        store.append(&paper, &ev1).unwrap();

        // 2. Attempt append of event 2 through FaultyWriter cut at byte k
        let file_path = store.device_file(&paper, &dev);
        let file = OpenOptions::new().append(true).open(&file_path).unwrap();
        let mut faulty_writer = FaultyWriter::new(file, k);

        let append_res = store.append_with_writer(&paper, &target_event, &mut faulty_writer);
        assert!(
            append_res.is_err(),
            "FaultyWriter must fail append at cut point {k}"
        );

        // 3. Read history report after the crash
        let report = store.read_report(&paper).unwrap();

        // INVARIANTS (R-10, QA gate 2):
        // (a) No malformed line returned in visible_events
        for ev in &report.visible_events {
            assert!(ev.seq >= 1, "Visible event must have valid sequence number");
            assert_eq!(ev.v, 1);
        }

        // (b) At most the last unfinished action is lost: baseline event 1 is preserved
        assert!(
            report.visible_events.len() == 1 || report.visible_events.len() == 2,
            "At most the last action is lost at byte cut {k}"
        );
        assert_eq!(report.visible_events[0].id, test_uuid("ev-base-1"));
        if report.visible_events.len() == 2 {
            assert_eq!(report.visible_events[1].id, test_uuid("target-ev"));
            assert_eq!(report.skipped_lines, 0);
        } else {
            assert_eq!(report.skipped_lines, 1);
        }

        // 4. Test recovery: subsequent append works and repairs unterminated newline
        let ev3 = make_test_event("ev-recovered-3", dev.clone(), 3);
        store.append(&paper, &ev3).unwrap();

        let report_after = store.read_report(&paper).unwrap();
        assert!(
            report_after.visible_events.len() == 2 || report_after.visible_events.len() == 3,
            "History recovered cleanly after append"
        );
        assert_eq!(report_after.visible_events[0].id, test_uuid("ev-base-1"));
        assert_eq!(
            report_after.visible_events.last().unwrap().id,
            test_uuid("ev-recovered-3")
        );
    }
}

#[test]
fn test_bad_and_torn_lines_skipped_and_counted() {
    let tmp = tempdir().unwrap();
    let workspace = tmp.path();
    let store = FsHistoryStore::new(workspace);
    let dev = DeviceId::new("dev-bad").unwrap();
    let paper = PaperId::new("z_bad_lines").unwrap();

    let ev1 = make_test_event("valid-1", dev.clone(), 1);
    store.append(&paper, &ev1).unwrap();

    let file_path = store.device_file(&paper, &dev);

    // Inject corrupted lines:
    // 1. Invalid JSON
    // 2. Over 1 MB line
    // 3. Line with seq == 0
    // 4. Unterminated trailing line
    let big_junk = "X".repeat(MAX_LINE_BYTES + 50);
    let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();

    // Bad line 1: truncated/invalid JSON
    file.write_all(b"{\"invalid\": truncated json line\n")
        .unwrap();

    // Bad line 2: over 1 MB line
    file.write_all(format!("{{\"v\":1,\"id\":\"big\",\"payload\":\"{big_junk}\"}}\n").as_bytes())
        .unwrap();

    // Bad line 3: seq == 0
    let seq_zero = serde_json::to_string(&json!({
        "v": 1,
        "id": test_uuid("seq-zero"),
        "device": dev.as_str(),
        "seq": 0,
        "ts": "2026-11-05T12:00:00Z",
        "author": "Me",
        "type": "mark_import",
        "payload": {
            "key": "k0",
            "kind": "highlight",
            "text": "zero seq",
            "comment": null,
            "color": null,
            "page_index": 0,
            "added": null,
            "modified": null
        }
    }))
    .unwrap();
    file.write_all(format!("{seq_zero}\n").as_bytes()).unwrap();

    // Bad line 4: unterminated trailing junk (no newline)
    file.write_all(b"unterminated trailing junk").unwrap();
    file.sync_data().unwrap();

    let report = store.read_report(&paper).unwrap();

    // Valid event 1 is preserved
    assert_eq!(report.visible_events.len(), 1);
    assert_eq!(report.visible_events[0].id, test_uuid("valid-1"));

    // All 4 bad lines are counted and skipped
    assert_eq!(
        report.skipped_lines, 4,
        "All 4 bad/corrupted lines must be skipped and counted!"
    );

    // Now test recovery: appending a new valid event repairs the unterminated line
    let ev2 = make_test_event("valid-2", dev, 2);
    store.append(&paper, &ev2).unwrap();

    let report2 = store.read_report(&paper).unwrap();
    assert_eq!(report2.visible_events.len(), 2);
    assert_eq!(report2.visible_events[0].id, test_uuid("valid-1"));
    assert_eq!(report2.visible_events[1].id, test_uuid("valid-2"));
}

#[test]
fn test_bounded_reader_exactly_max_and_over_max_bytes() {
    let tmp = tempdir().unwrap();
    let workspace = tmp.path();
    let store = FsHistoryStore::new(workspace);
    let dev = DeviceId::new("dev-bound").unwrap();
    let paper = PaperId::new("z_bound_test").unwrap();

    let file_path = store.device_file(&paper, &dev);
    fs::create_dir_all(store.paper_dir(&paper)).unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();

    // 1. Line of exactly MAX_LINE_BYTES (1,048,576) content bytes without \n, followed by \n.
    let prefix = format!(
        r#"{{"v":1,"id":"{}","device":"dev-bound","seq":1,"ts":"2026-11-05T12:00:00Z","type":"mark_import","payload":{{"key":"k1","kind":"highlight","text":""#,
        test_uuid("ev-exact-max")
    );
    let suffix = r#""}}"#;
    let filler_len = MAX_LINE_BYTES - prefix.len() - suffix.len();
    let mut exact_line = Vec::with_capacity(MAX_LINE_BYTES + 1);
    exact_line.extend_from_slice(prefix.as_bytes());
    exact_line.resize(exact_line.len() + filler_len, b'a');
    exact_line.extend_from_slice(suffix.as_bytes());
    assert_eq!(exact_line.len(), MAX_LINE_BYTES);
    exact_line.push(b'\n');

    file.write_all(&exact_line).unwrap();

    // 2. Line of exactly MAX_LINE_BYTES + 1 content bytes without \n, followed by \n.
    let over_prefix = format!(
        r#"{{"v":1,"id":"{}","device":"dev-bound","seq":2,"ts":"2026-11-05T12:00:00Z","type":"mark_import","payload":{{"key":"k2","kind":"highlight","text":""#,
        test_uuid("ev-over-max")
    );
    let over_filler_len = (MAX_LINE_BYTES + 1) - over_prefix.len() - suffix.len();
    let mut over_line = Vec::with_capacity(MAX_LINE_BYTES + 2);
    over_line.extend_from_slice(over_prefix.as_bytes());
    over_line.resize(over_line.len() + over_filler_len, b'b');
    over_line.extend_from_slice(suffix.as_bytes());
    assert_eq!(over_line.len(), MAX_LINE_BYTES + 1);
    over_line.push(b'\n');

    file.write_all(&over_line).unwrap();
    file.sync_data().unwrap();

    let report = store.read_report(&paper).unwrap();

    // Exactly MAX_LINE_BYTES content line is valid and preserved!
    assert_eq!(report.raw_events.len(), 1);
    assert_eq!(report.raw_events[0].id, test_uuid("ev-exact-max"));

    // Over MAX_LINE_BYTES line is skipped as too_long!
    assert_eq!(report.skipped_lines, 1);
    assert_eq!(report.bad_count("too_long"), 1);
}

#[test]
fn test_store_conformance_with_spec_vectors() {
    let tmp = tempdir().unwrap();
    let workspace = tmp.path();
    let store = FsHistoryStore::new(workspace);
    let dev = DeviceId::new("dev-a").unwrap();
    let paper = PaperId::new("z_spec_vectors").unwrap();

    let file_path = store.device_file(&paper, &dev);
    fs::create_dir_all(store.paper_dir(&paper)).unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();

    let spec_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../spec/vectors");
    let manifest_bytes = fs::read(spec_dir.join("manifest.json")).unwrap();
    let manifest: Value = serde_json::from_slice(&manifest_bytes).unwrap();
    let vectors = manifest["vectors"].as_array().unwrap();

    let mut expected_valid = 0;
    let mut expected_other = 0;
    let mut expected_bad = 0;
    let mut expected_reasons: HashMap<String, usize> = HashMap::new();

    for v in vectors {
        let rel_file = v["file"].as_str().unwrap();
        let expect = v["expect"].as_str().unwrap();
        let bytes = fs::read(spec_dir.join(rel_file)).unwrap();
        file.write_all(&bytes).unwrap();
        if !bytes.ends_with(b"\n") {
            file.write_all(b"\n").unwrap();
        }

        match expect {
            "valid" => expected_valid += 1,
            "other" => expected_other += 1,
            bad_code => {
                expected_bad += 1;
                *expected_reasons.entry(bad_code.to_string()).or_insert(0) += 1;
            }
        }
    }
    file.sync_data().unwrap();

    let report = store.read_report(&paper).unwrap();

    assert_eq!(
        report.raw_events.len(),
        expected_valid,
        "Valid lines must match spec vectors"
    );
    assert_eq!(
        report.other_lines, expected_other,
        "Other lines must be kept and counted"
    );
    assert_eq!(
        report.skipped_lines, expected_bad,
        "Bad lines must match skipped count"
    );

    for (code, count) in &expected_reasons {
        assert_eq!(
            report.bad_count(code),
            *count,
            "Reason code '{code}' count mismatch"
        );
    }

    // Next sequence must take max_seq across both valid and other lines + 1
    // In good vectors, max seq is 8, so next_seq must be 9
    let next = store.next_seq(&paper, &dev).unwrap();
    assert_eq!(
        next, 9,
        "next_seq must count valid and other lines without reuse"
    );
}

#[test]
fn test_next_seq_tracks_other_lines_and_bad_lines_with_seq() {
    let tmp = tempdir().unwrap();
    let workspace = tmp.path();
    let store = FsHistoryStore::new(workspace);
    let dev = DeviceId::new("dev-seq").unwrap();
    let paper = PaperId::new("z_next_seq").unwrap();

    let file_path = store.device_file(&paper, &dev);
    fs::create_dir_all(store.paper_dir(&paper)).unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();

    // 1. Valid event with seq: 1
    let ev1 = json!({
        "v": 1,
        "id": test_uuid("seq-1"),
        "device": "dev-seq",
        "seq": 1,
        "ts": "2026-11-05T12:00:00Z",
        "type": "mark_import",
        "payload": {
            "key": "k1",
            "kind": "highlight"
        }
    });
    file.write_all(format!("{ev1}\n").as_bytes()).unwrap();

    // 2. Forward-compatible 'other' event with seq: 5
    let other_ev = json!({
        "v": 1,
        "id": test_uuid("seq-5"),
        "device": "dev-seq",
        "seq": 5,
        "ts": "2026-11-05T12:05:00Z",
        "author": "Me",
        "type": "future_milestone",
        "payload": { "future": true }
    });
    file.write_all(format!("{other_ev}\n").as_bytes()).unwrap();

    // 3. Corrupt/bad-payload event with seq: 9
    let bad_ev = json!({
        "v": 1,
        "id": test_uuid("seq-9"),
        "device": "dev-seq",
        "seq": 9,
        "ts": "2026-11-05T12:10:00Z",
        "type": "mark_import",
        "payload": { "invalid_missing_key": true }
    });
    file.write_all(format!("{bad_ev}\n").as_bytes()).unwrap();
    file.sync_data().unwrap();

    let next = store.next_seq(&paper, &dev).unwrap();
    assert_eq!(
        next, 10,
        "next_seq must be highest sequence across valid, other and bad lines + 1 (never reuses!)"
    );
}

#[test]
fn test_next_seq_overflow_returns_typed_store_error() {
    let tmp = tempdir().unwrap();
    let workspace = tmp.path();
    let store = FsHistoryStore::new(workspace);
    let dev = DeviceId::new("dev-overflow").unwrap();
    let paper = PaperId::new("z_overflow").unwrap();

    let file_path = store.device_file(&paper, &dev);
    fs::create_dir_all(store.paper_dir(&paper)).unwrap();

    // 1. Hostile / corrupt line with u64::MAX seq: 18446744073709551615
    let hostile_bad = json!({
        "v": 1,
        "id": test_uuid("hostile-bad"),
        "device": "dev-overflow",
        "seq": u64::MAX,
        "ts": "2026-11-05T12:00:00Z",
        "type": "mark_import",
        "payload": { "bad_missing_key": true }
    });
    fs::write(&file_path, format!("{hostile_bad}\n")).unwrap();

    let err = store.next_seq(&paper, &dev).unwrap_err();
    assert!(
        matches!(err, StoreError::SequenceOverflow(ref d) if d == &dev),
        "Sequence overflow on bad line must return StoreError::SequenceOverflow"
    );

    // 2. Forward-compatible 'other' line with u64::MAX seq: 18446744073709551615
    let hostile_other = json!({
        "v": 1,
        "id": test_uuid("hostile-other"),
        "device": "dev-overflow",
        "seq": u64::MAX,
        "ts": "2026-11-05T12:00:00Z",
        "author": "Me",
        "type": "future_milestone",
        "payload": { "future": true }
    });
    fs::write(&file_path, format!("{hostile_other}\n")).unwrap();

    let err2 = store.next_seq(&paper, &dev).unwrap_err();
    assert!(
        matches!(err2, StoreError::SequenceOverflow(ref d) if d == &dev),
        "Sequence overflow on other line must return StoreError::SequenceOverflow"
    );
}
