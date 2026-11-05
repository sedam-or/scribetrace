use scribetrace_spike_log_merge::{FileLogStorage, LogReader, LogWriter, MAX_LINE_BYTES};
use serde_json::json;
use std::fs::{self, OpenOptions};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_normal_appends_and_reads() {
    let tmp = tempdir().unwrap();
    let base_dir = tmp.path().join(".scribetrace");
    let app_storage = tmp.path().join("app_storage");

    let mut storage = FileLogStorage::init(&base_dir, &app_storage).unwrap();
    let initial_dev_id = storage.current_device_id().to_string();

    let ev1 = storage
        .append_event(
            "z_item123",
            "mark_import",
            json!({"text": "Hello world"}),
            "2026-11-05T12:00:00Z",
        )
        .unwrap();
    assert_eq!(ev1.seq, 1);
    assert_eq!(ev1.device, initial_dev_id);

    let ev2 = storage
        .append_event(
            "z_item123",
            "note_revision",
            json!({"content": "Updated notes"}),
            "2026-11-05T12:05:00Z",
        )
        .unwrap();
    assert_eq!(ev2.seq, 2);
    assert_eq!(ev2.device, initial_dev_id);

    let read = storage.read_paper_events("z_item123").unwrap();
    assert_eq!(read.merged_events.len(), 2);
    assert_eq!(read.merged_events[0].id, ev1.id);
    assert_eq!(read.merged_events[1].id, ev2.id);
    assert_eq!(read.skipped_lines, 0);
    assert_eq!(read.files_scanned, 1);
}

#[test]
fn test_reading_never_modifies_files_byte_identical() {
    let tmp = tempdir().unwrap();
    let base_dir = tmp.path().join(".scribetrace");
    let app_storage = tmp.path().join("app_storage");

    let mut storage = FileLogStorage::init(&base_dir, &app_storage).unwrap();
    storage
        .append_event(
            "z_testbyte",
            "mark_import",
            json!({"test": true}),
            "2026-11-05T10:00:00Z",
        )
        .unwrap();

    let dev_id = storage.current_device_id().to_string();
    let file_path = base_dir
        .join("papers")
        .join("z_testbyte")
        .join(format!("{dev_id}.jsonl"));

    let bytes_before = fs::read(&file_path).unwrap();

    // Read multiple times
    for _ in 0..10 {
        let read = storage.read_paper_events("z_testbyte").unwrap();
        assert_eq!(read.merged_events.len(), 1);
    }

    let bytes_after = fs::read(&file_path).unwrap();
    assert_eq!(
        bytes_before, bytes_after,
        "Reading MUST never rewrite or mutate file bytes!"
    );
}

#[test]
fn test_collision_cloned_machine() {
    let tmp = tempdir().unwrap();
    let base_dir = tmp.path().join(".scribetrace");
    let app_storage = tmp.path().join("app_storage");

    let mut storage = FileLogStorage::init(&base_dir, &app_storage).unwrap();
    let dev1 = storage.current_device_id().to_string();

    storage
        .append_event(
            "z_clone_test",
            "mark_import",
            json!({"item": 1}),
            "2026-11-05T10:00:00Z",
        )
        .unwrap();

    // Simulate cloned machine: another peer/sync modifies this device's file with another event
    let file_path = base_dir
        .join("papers")
        .join("z_clone_test")
        .join(format!("{dev1}.jsonl"));

    let external_line = serde_json::to_string(&json!({
        "v": 1,
        "id": "external-cloned-uuid-9999",
        "device": dev1,
        "seq": 2,
        "ts": "2026-11-05T10:05:00Z",
        "author": "Me",
        "type": "mark_create",
        "payload": {"external": true}
    }))
    .unwrap();

    let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();
    file.write_all(format!("{external_line}\n").as_bytes())
        .unwrap();
    file.sync_data().unwrap();

    // Now our local device tries to append. File's last_id is "external-cloned-uuid-9999",
    // while device.json expects the first event's id.
    // Detection: Collision! Must mint new device ID and not clobber dev1.jsonl.
    let ev_new = storage
        .append_event(
            "z_clone_test",
            "mark_update",
            json!({"item": 3}),
            "2026-11-05T10:10:00Z",
        )
        .unwrap();

    let dev2 = storage.current_device_id().to_string();
    assert_ne!(dev1, dev2, "Cloned collision MUST mint a new device ID!");
    assert_eq!(ev_new.device, dev2);
    assert_eq!(ev_new.seq, 1, "New device starts its own file at seq 1");

    // Check files: dev1.jsonl remains untouched with 2 events; dev2.jsonl has 1 event
    let read = storage.read_paper_events("z_clone_test").unwrap();
    assert_eq!(read.files_scanned, 2);
    assert_eq!(read.merged_events.len(), 3);
}

#[test]
fn test_collision_restored_backup_or_missing_state() {
    let tmp = tempdir().unwrap();
    let base_dir = tmp.path().join(".scribetrace");
    let app_storage = tmp.path().join("app_storage");

    let mut storage = FileLogStorage::init(&base_dir, &app_storage).unwrap();
    let dev1 = storage.current_device_id().to_string();

    storage
        .append_event(
            "z_restore_test",
            "mark_import",
            json!({"initial": true}),
            "2026-11-05T09:00:00Z",
        )
        .unwrap();

    // Simulate restored backup: delete device.json or replace with empty
    let state_file = app_storage.join("device.json");
    fs::remove_file(&state_file).unwrap();

    // Re-initialize storage (as if restarted from backup)
    let mut storage2 = FileLogStorage::init(&base_dir, &app_storage).unwrap();
    let dev_restored = storage2.current_device_id().to_string();
    assert_ne!(
        dev1, dev_restored,
        "Missing device.json on restart creates fresh device ID"
    );

    let ev = storage2
        .append_event(
            "z_restore_test",
            "mark_create",
            json!({"fresh": true}),
            "2026-11-05T09:30:00Z",
        )
        .unwrap();

    assert_eq!(ev.seq, 1);
    assert_eq!(ev.device, dev_restored);

    let read = storage2.read_paper_events("z_restore_test").unwrap();
    assert_eq!(read.files_scanned, 2);
    assert_eq!(read.merged_events.len(), 2);
}

#[test]
fn test_collision_crash_between_append_and_state_update() {
    let tmp = tempdir().unwrap();
    let base_dir = tmp.path().join(".scribetrace");
    let app_storage = tmp.path().join("app_storage");

    let mut storage = FileLogStorage::init(&base_dir, &app_storage).unwrap();
    let dev1 = storage.current_device_id().to_string();

    storage
        .append_event(
            "z_crash_test",
            "mark_import",
            json!({"first": true}),
            "2026-11-05T08:00:00Z",
        )
        .unwrap();

    // Simulate crash where an extra event was written to the file,
    // but the app crashed before device.json was updated with its id
    let file_path = base_dir
        .join("papers")
        .join("z_crash_test")
        .join(format!("{dev1}.jsonl"));

    let crashed_event = serde_json::to_string(&json!({
        "v": 1,
        "id": "crashed-event-uuid-8888",
        "device": dev1,
        "seq": 2,
        "ts": "2026-11-05T08:05:00Z",
        "author": "Me",
        "type": "mark_create",
        "payload": {"crashed": true}
    }))
    .unwrap();

    let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();
    file.write_all(format!("{crashed_event}\n").as_bytes())
        .unwrap();
    file.sync_data().unwrap();

    // On next append, storage sees last event has id "crashed-event-uuid-8888",
    // which does NOT match the state saved in device.json.
    // It must detect this inconsistency, mint a new device ID, and keep the existing file safe.
    let ev = storage
        .append_event(
            "z_crash_test",
            "mark_update",
            json!({"post_crash": true}),
            "2026-11-05T08:10:00Z",
        )
        .unwrap();

    let dev2 = storage.current_device_id().to_string();
    assert_ne!(dev1, dev2, "Crash inconsistency MUST mint a new device ID!");
    assert_eq!(ev.device, dev2);

    let read = storage.read_paper_events("z_crash_test").unwrap();
    assert_eq!(read.files_scanned, 2);
    assert_eq!(read.merged_events.len(), 3);
}

#[test]
fn test_bad_and_torn_lines_skipped_and_counted() {
    let tmp = tempdir().unwrap();
    let base_dir = tmp.path().join(".scribetrace");
    let app_storage = tmp.path().join("app_storage");

    let mut storage = FileLogStorage::init(&base_dir, &app_storage).unwrap();
    let dev1 = storage.current_device_id().to_string();

    let ev1 = storage
        .append_event(
            "z_bad_lines",
            "mark_import",
            json!({"valid": 1}),
            "2026-11-05T12:00:00Z",
        )
        .unwrap();

    let file_path = base_dir
        .join("papers")
        .join("z_bad_lines")
        .join(format!("{dev1}.jsonl"));

    // Inject corrupted lines:
    // 1. Invalid JSON
    // 2. Over 1MB line
    // 3. Unterminated last line
    let big_junk = "X".repeat(MAX_LINE_BYTES + 50);
    let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();
    file.write_all(b"{\"invalid\": truncated json line\n")
        .unwrap();
    file.write_all(format!("{{\"v\":1,\"id\":\"big\",\"payload\":\"{big_junk}\"}}\n").as_bytes())
        .unwrap();
    // Unterminated trailing line
    file.write_all(b"unterminated trailing junk").unwrap();
    file.sync_data().unwrap();

    let read = storage.read_paper_events("z_bad_lines").unwrap();
    // Valid event 1 is preserved
    assert_eq!(read.merged_events.len(), 1);
    assert_eq!(read.merged_events[0].id, ev1.id);
    // 3 bad lines skipped
    assert_eq!(read.skipped_lines, 3);
}
