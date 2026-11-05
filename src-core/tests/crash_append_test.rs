#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs::OpenOptions;
use std::io::Write;

use scribetrace_core::adapters::FsHistoryStore;
use scribetrace_core::application::{HistoryStore, LogWriter};
use scribetrace_core::domain::{DeviceId, Event, EventBody, Mark, PaperId, MAX_LINE_BYTES};
use serde_json::json;
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

fn make_test_event(id: &str, device: DeviceId, seq: u64) -> Event {
    Event::new(
        1,
        id,
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
        assert_eq!(report.visible_events[0].id, "ev-base-1");
        if report.visible_events.len() == 2 {
            assert_eq!(report.visible_events[1].id, "target-ev");
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
        assert_eq!(report_after.visible_events[0].id, "ev-base-1");
        assert_eq!(
            report_after.visible_events.last().unwrap().id,
            "ev-recovered-3"
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
        "id": "seq-zero",
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
    assert_eq!(report.visible_events[0].id, "valid-1");

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
    assert_eq!(report2.visible_events[0].id, "valid-1");
    assert_eq!(report2.visible_events[1].id, "valid-2");
}
