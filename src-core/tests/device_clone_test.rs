#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs::{self, OpenOptions};
use std::io::Write;

use scribetrace_core::adapters::{FsDeviceStore, FsHistoryStore};
use scribetrace_core::application::{DeviceStore, HistoryStore};
use scribetrace_core::domain::{DeviceId, Event, EventBody, Mark, PaperId};
use serde_json::json;
use tempfile::tempdir;

fn sample_event(id: &str, device: DeviceId, seq: u64) -> Event {
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
            text: Some("Sample mark text".to_string()),
            comment: None,
            color: None,
            page_index: Some(seq as u32),
            added: None,
            modified: None,
        }),
    )
}

#[test]
fn test_normal_appends_and_reads() {
    let tmp = tempdir().unwrap();
    let workspace = tmp.path().join("workspace");
    let app_storage = tmp.path().join("app_storage");

    let history_store = FsHistoryStore::new(&workspace);
    let mut dev_store = FsDeviceStore::new(&app_storage).unwrap();
    let dev1 = dev_store.get_or_create_device_id().unwrap();

    let paper = PaperId::new("z_item123").unwrap();

    let ev1 = sample_event("ev-uuid-1", dev1.clone(), 1);
    history_store.append(&paper, &ev1).unwrap();
    dev_store.record_last_event(&paper, ev1.id.clone()).unwrap();

    let next = history_store.next_seq(&paper, &dev1).unwrap();
    assert_eq!(next, 2);

    let ev2 = sample_event("ev-uuid-2", dev1.clone(), 2);
    history_store.append(&paper, &ev2).unwrap();
    dev_store.record_last_event(&paper, ev2.id.clone()).unwrap();

    let report = history_store.read_report(&paper).unwrap();
    assert_eq!(report.raw_events.len(), 2);
    assert_eq!(report.merged_events.len(), 2);
    assert_eq!(report.visible_events.len(), 2);
    assert_eq!(report.visible_events[0].id, "ev-uuid-1");
    assert_eq!(report.visible_events[1].id, "ev-uuid-2");
    assert_eq!(report.skipped_lines, 0);
    assert_eq!(report.files_scanned, 1);
}

#[test]
fn test_reading_never_modifies_files_byte_identical() {
    let tmp = tempdir().unwrap();
    let workspace = tmp.path().join("workspace");
    let store = FsHistoryStore::new(&workspace);
    let paper = PaperId::new("z_testbyte").unwrap();
    let dev = DeviceId::new("dev-byte-test").unwrap();

    let ev = sample_event("ev-byte-1", dev.clone(), 1);
    store.append(&paper, &ev).unwrap();

    let file_path = store.device_file(&paper, &dev);
    let bytes_before = fs::read(&file_path).unwrap();

    // Read multiple times
    for _ in 0..10 {
        let report = store.read_report(&paper).unwrap();
        assert_eq!(report.visible_events.len(), 1);
    }

    let bytes_after = fs::read(&file_path).unwrap();
    assert_eq!(
        bytes_before, bytes_after,
        "Reading MUST never mutate or touch file bytes (SEC-19)!"
    );
}

#[test]
fn test_collision_cloned_machine() {
    let tmp = tempdir().unwrap();
    let workspace = tmp.path().join("workspace");
    let app_storage = tmp.path().join("app_storage");

    let history_store = FsHistoryStore::new(&workspace);
    let mut dev_store = FsDeviceStore::new(&app_storage).unwrap();
    let dev1 = dev_store.get_or_create_device_id().unwrap();

    let paper = PaperId::new("z_clone_test").unwrap();

    let ev1 = sample_event("ev-clone-1", dev1.clone(), 1);
    history_store.append(&paper, &ev1).unwrap();
    dev_store.record_last_event(&paper, ev1.id.clone()).unwrap();

    // Simulate cloned machine: another peer/sync modifies this device's file with another event
    let file_path = history_store.device_file(&paper, &dev1);
    let external_line = serde_json::to_string(&json!({
        "v": 1,
        "id": "external-cloned-uuid-9999",
        "device": dev1.as_str(),
        "seq": 2,
        "ts": "2026-11-05T10:05:00Z",
        "author": "Me",
        "type": "mark_import",
        "payload": {
            "key": "ext",
            "kind": "highlight",
            "text": "External clone text",
            "comment": null,
            "color": null,
            "page_index": 2,
            "added": null,
            "modified": null
        }
    }))
    .unwrap();

    let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();
    file.write_all(format!("{external_line}\n").as_bytes())
        .unwrap();
    file.sync_data().unwrap();

    // Now our local device prepares to append.
    // Inconsistency detected: file's last_id is "external-cloned-uuid-9999",
    // while device.json expects "ev-clone-1".
    // Safe resolution: mint new device ID, leave dev1.jsonl untouched.
    let scan = history_store.inspect_device_file(&paper, &dev1).unwrap();
    let dev2 = dev_store
        .ensure_safe_device(&paper, scan.last_event.as_ref())
        .unwrap();

    assert_ne!(
        dev1, dev2,
        "Cloned machine collision MUST mint a new device ID (R-08, SEC-34)!"
    );

    let ev_new = sample_event("ev-fresh-1", dev2.clone(), 1);
    history_store.append(&paper, &ev_new).unwrap();
    dev_store
        .record_last_event(&paper, ev_new.id.clone())
        .unwrap();

    // Verify files: dev1.jsonl remains untouched with 2 events; dev2.jsonl has 1 event
    let report = history_store.read_report(&paper).unwrap();
    assert_eq!(report.files_scanned, 2);
    assert_eq!(report.merged_events.len(), 3);
    assert_eq!(report.visible_events.len(), 3);
}

#[test]
fn test_collision_restored_backup_or_missing_state() {
    let tmp = tempdir().unwrap();
    let workspace = tmp.path().join("workspace");
    let app_storage = tmp.path().join("app_storage");

    let history_store = FsHistoryStore::new(&workspace);
    let mut dev_store = FsDeviceStore::new(&app_storage).unwrap();
    let dev1 = dev_store.get_or_create_device_id().unwrap();

    let paper = PaperId::new("z_restore_test").unwrap();

    let ev1 = sample_event("ev-restore-1", dev1.clone(), 1);
    history_store.append(&paper, &ev1).unwrap();
    dev_store.record_last_event(&paper, ev1.id.clone()).unwrap();

    // Simulate restored backup: delete device.json
    let state_file = app_storage.join("device.json");
    fs::remove_file(&state_file).unwrap();

    // Re-initialize storage (as if restarted on new machine or from backup)
    let mut dev_store2 = FsDeviceStore::new(&app_storage).unwrap();
    let dev_restored = dev_store2.get_or_create_device_id().unwrap();
    assert_ne!(
        dev1, dev_restored,
        "Missing device.json on restart creates fresh device ID"
    );

    let scan = history_store
        .inspect_device_file(&paper, &dev_restored)
        .unwrap();
    let safe_dev = dev_store2
        .ensure_safe_device(&paper, scan.last_event.as_ref())
        .unwrap();
    assert_eq!(safe_dev, dev_restored);

    let ev2 = sample_event("ev-restore-2", safe_dev.clone(), 1);
    history_store.append(&paper, &ev2).unwrap();
    dev_store2
        .record_last_event(&paper, ev2.id.clone())
        .unwrap();

    let report = history_store.read_report(&paper).unwrap();
    assert_eq!(report.files_scanned, 2);
    assert_eq!(report.merged_events.len(), 2);
}

#[test]
fn test_collision_crash_between_append_and_state_update() {
    let tmp = tempdir().unwrap();
    let workspace = tmp.path().join("workspace");
    let app_storage = tmp.path().join("app_storage");

    let history_store = FsHistoryStore::new(&workspace);
    let mut dev_store = FsDeviceStore::new(&app_storage).unwrap();
    let dev1 = dev_store.get_or_create_device_id().unwrap();

    let paper = PaperId::new("z_crash_test").unwrap();

    let ev1 = sample_event("ev-crash-1", dev1.clone(), 1);
    history_store.append(&paper, &ev1).unwrap();
    dev_store.record_last_event(&paper, ev1.id.clone()).unwrap();

    // Simulate crash where an extra event was written to the file,
    // but the app crashed before device.json was updated with its id
    let file_path = history_store.device_file(&paper, &dev1);
    let crashed_event = serde_json::to_string(&json!({
        "v": 1,
        "id": "crashed-event-uuid-8888",
        "device": dev1.as_str(),
        "seq": 2,
        "ts": "2026-11-05T08:05:00Z",
        "author": "Me",
        "type": "mark_import",
        "payload": {
            "key": "crash",
            "kind": "highlight",
            "text": "Crash event",
            "comment": null,
            "color": null,
            "page_index": 1,
            "added": null,
            "modified": null
        }
    }))
    .unwrap();

    let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();
    file.write_all(format!("{crashed_event}\n").as_bytes())
        .unwrap();
    file.sync_data().unwrap();

    // On next append, storage sees last event has id "crashed-event-uuid-8888",
    // which does NOT match the state saved in device.json ("ev-crash-1").
    let scan = history_store.inspect_device_file(&paper, &dev1).unwrap();
    let dev2 = dev_store
        .ensure_safe_device(&paper, scan.last_event.as_ref())
        .unwrap();

    assert_ne!(
        dev1, dev2,
        "Crash inconsistency MUST mint a new device ID (R-08)!"
    );

    let ev_new = sample_event("ev-post-crash", dev2.clone(), 1);
    history_store.append(&paper, &ev_new).unwrap();
    dev_store
        .record_last_event(&paper, ev_new.id.clone())
        .unwrap();

    let report = history_store.read_report(&paper).unwrap();
    assert_eq!(report.files_scanned, 2);
    assert_eq!(report.merged_events.len(), 3);
}
