#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::Instant;

use scribetrace_core::adapters::FsHistoryStore;
use scribetrace_core::application::HistoryStore;
use scribetrace_core::domain::PaperId;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn test_replay_10_000_events_benchmark() {
    let tmp = tempdir().unwrap();
    let workspace = tmp.path();

    let paper = PaperId::new("z_bench10k").unwrap();
    let store = FsHistoryStore::new(workspace);
    let paper_dir = store.paper_dir(&paper);
    fs::create_dir_all(&paper_dir).unwrap();

    let num_devices = 5;
    let events_per_device = 2000;

    for dev_idx in 0..num_devices {
        let dev_id = format!("bench-device-{dev_idx}");
        let file_path = paper_dir.join(format!("{dev_id}.jsonl"));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)
            .unwrap();

        let mut buffer = Vec::new();
        for seq in 1..=events_per_device {
            let ev = json!({
                "v": 1,
                "id": format!("6f1f2c1e-0a4b-4c55-9d3e-{:04x}{:08x}", dev_idx, seq),
                "device": dev_id,
                "seq": seq,
                "ts": "2026-11-05T12:00:00Z",
                "author": "Me",
                "type": "mark_import",
                "payload": {
                    "key": format!("k-{dev_idx}-{seq}"),
                    "kind": "highlight",
                    "text": "Benchmark annotation highlight text for latency verification",
                    "comment": null,
                    "color": "#ffff00",
                    "page_index": seq % 100,
                    "added": "2026-11-05T12:00:00Z",
                    "modified": "2026-11-05T12:00:00Z"
                }
            });
            serde_json::to_writer(&mut buffer, &ev).unwrap();
            buffer.push(b'\n');
        }
        file.write_all(&buffer).unwrap();
        file.sync_data().unwrap();
    }

    // Benchmark the full replay (read all 5 files + parse + union merge + purge visibility + sort)
    let start = Instant::now();
    let report = store.read_report(&paper).unwrap();
    let elapsed = start.elapsed();

    assert_eq!(report.raw_events.len(), 10_000);
    assert_eq!(report.merged_events.len(), 10_000);
    assert_eq!(report.visible_events.len(), 10_000);
    assert_eq!(report.files_scanned, 5);
    assert_eq!(report.skipped_lines, 0);

    let elapsed_ms = elapsed.as_millis();
    println!("=== 10,000 Event Replay Benchmark ===");
    println!("Total events: 10,000 across 5 device files");
    println!("Elapsed time: {elapsed_ms} ms ({:?})", elapsed);
    println!("QA Budget: <= 500 ms");

    assert!(
        elapsed_ms <= 500,
        "Replay of 10,000 events took {elapsed_ms} ms, exceeding the 500 ms budget!"
    );
}
