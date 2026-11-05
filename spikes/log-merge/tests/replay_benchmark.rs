use scribetrace_spike_log_merge::{FileLogStorage, LogReader};
use serde_json::json;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::Instant;
use tempfile::tempdir;

#[test]
fn test_replay_10_000_events_benchmark() {
    let tmp = tempdir().unwrap();
    let base_dir = tmp.path().join(".scribetrace");
    let app_storage = tmp.path().join("app_storage");

    let paper_id = "z_bench10k";
    let paper_dir = base_dir.join("papers").join(paper_id);
    fs::create_dir_all(&paper_dir).unwrap();

    // Generate 10,000 events distributed across 5 virtual devices
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
                "id": format!("bench-ev-{dev_idx}-{seq}"),
                "device": dev_id,
                "seq": seq,
                "ts": "2026-11-05T12:00:00Z",
                "author": "Me",
                "type": "mark_create",
                "payload": {"page": seq % 100, "text": "Benchmark annotation highlight"}
            });
            serde_json::to_writer(&mut buffer, &ev).unwrap();
            buffer.push(b'\n');
        }
        file.write_all(&buffer).unwrap();
        file.sync_data().unwrap();
    }

    let storage = FileLogStorage::init(&base_dir, &app_storage).unwrap();

    // Benchmark the full replay (read all 5 files + parse + union merge + sort)
    let start = Instant::now();
    let read_result = storage.read_paper_events(paper_id).unwrap();
    let elapsed = start.elapsed();

    assert_eq!(read_result.merged_events.len(), 10_000);
    assert_eq!(read_result.files_scanned, 5);
    assert_eq!(read_result.skipped_lines, 0);

    let elapsed_ms = elapsed.as_millis();
    println!("=== 10,000 Event Replay Benchmark ===");
    println!("Total events: 10,000 across 5 device files");
    println!("Elapsed time: {elapsed_ms} ms ({:?})", elapsed);
    println!("QA Budget: < 500 ms");

    assert!(
        elapsed_ms < 500,
        "Replay of 10,000 events took {elapsed_ms} ms, exceeding the 500 ms QA budget!"
    );
}
