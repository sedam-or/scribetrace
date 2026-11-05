use scribetrace_spike_purge::{visible, Event, Purge};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_wrong_clocks_both_ways() {
    // Case A: Purge wall clock is in the distant PAST (2020), but event clock is in FUTURE (2026).
    // Event seq is 5, Purge covers devA: 5.
    // Result: Covered event MUST be hidden despite older purge timestamp!
    let mut covers_a = HashMap::new();
    covers_a.insert("devA".to_string(), 5);

    let event_future = Event::new(
        1,
        "ev-future",
        "devA",
        5,
        "2026-11-05T12:00:00Z", // Future clock
        "Me",
        "mark_create",
        json!({"note": "mark"}),
    );

    let purge_past = Purge::new(
        "purge-past",
        "devA",
        6,
        "2020-01-01T00:00:00Z", // Past clock!
        covers_a,
    );

    let res_a = visible(&[event_future], &[purge_past]);
    assert_eq!(res_a.hidden_count, 1);
    assert_eq!(
        res_a.visible.len(),
        0,
        "Event must be hidden regardless of past purge clock!"
    );

    // Case B: Purge clock is in the future (2035), but event seq is 10 while purge covers only devA: 5.
    // Result: Event seq 10 MUST remain visible!
    let mut covers_b = HashMap::new();
    covers_b.insert("devA".to_string(), 5);

    let event_uncovered = Event::new(
        1,
        "ev-10",
        "devA",
        10,
        "2026-11-05T12:00:00Z",
        "Me",
        "mark_create",
        json!({"note": "uncovered"}),
    );

    let purge_future = Purge::new("purge-future", "devA", 6, "2035-01-01T00:00:00Z", covers_b);

    let res_b = visible(&[event_uncovered], &[purge_future]);
    assert_eq!(res_b.hidden_count, 0);
    assert_eq!(res_b.visible.len(), 1);
    assert_eq!(res_b.visible[0].seq, 10);
}

#[test]
fn test_unseen_offline_event_and_second_purge() {
    // Device B wrote events while offline: seq 1..5.
    // Device A had only seen up to seq 3 when it ran Purge 1: covers = {devB: 3}.
    let mut events = Vec::new();
    for seq in 1..=5 {
        events.push(Event::new(
            1,
            format!("devB-ev-{seq}"),
            "devB",
            seq,
            "2026-11-05T12:00:00Z",
            "Me",
            "mark_create",
            json!({"seq": seq}),
        ));
    }

    let mut covers_1 = HashMap::new();
    covers_1.insert("devB".to_string(), 3);
    let purge_1 = Purge::new("p1", "devA", 1, "2026-11-05T12:10:00Z", covers_1);

    // After Purge 1: seq 1, 2, 3 hidden; seq 4, 5 remain visible
    let res_1 = visible(&events, std::slice::from_ref(&purge_1));
    assert_eq!(res_1.hidden_count, 3);
    assert_eq!(res_1.visible.len(), 2);
    assert_eq!(res_1.visible[0].seq, 4);
    assert_eq!(res_1.visible[1].seq, 5);

    // Later: Device A syncs with Device B, sees seq 5, and triggers second Purge: covers = {devB: 5}.
    let mut covers_2 = HashMap::new();
    covers_2.insert("devB".to_string(), 5);
    let purge_2 = Purge::new("p2", "devA", 2, "2026-11-05T12:20:00Z", covers_2);

    let res_2 = visible(&events, &[purge_1, purge_2]);
    assert_eq!(res_2.hidden_count, 5);
    assert_eq!(
        res_2.visible.len(),
        0,
        "Second purge with higher coverage hides remaining events!"
    );
}

#[test]
fn test_device_not_in_purge_never_hidden_even_if_seq_zero() {
    // Verifies Architect item 2: device not present in covers is never hidden
    let ev = Event::new(
        1,
        "ev-0",
        "devUnrelated",
        0,
        "2026-11-05T10:00:00Z",
        "Me",
        "mark_create",
        json!({}),
    );
    let mut covers = HashMap::new();
    covers.insert("devOther".to_string(), 10);
    let purge = Purge::new("p1", "devOther", 1, "2026-11-05T10:05:00Z", covers);

    let res = visible(&[ev], &[purge]);
    assert_eq!(res.hidden_count, 0);
    assert_eq!(res.visible.len(), 1);
    assert_eq!(res.visible[0].device, "devUnrelated");
}

#[test]
fn test_two_purges_in_any_order() {
    let mut events = Vec::new();
    for seq in 1..=10 {
        events.push(Event::new(
            1,
            format!("devA-ev-{seq}"),
            "devA",
            seq,
            "2026-11-05T12:00:00Z",
            "Me",
            "mark_create",
            json!({"seq": seq}),
        ));
    }

    let mut cov1 = HashMap::new();
    cov1.insert("devA".to_string(), 4);
    let p1 = Purge::new("p1", "devA", 1, "2026-11-05T12:05:00Z", cov1);

    let mut cov2 = HashMap::new();
    cov2.insert("devA".to_string(), 8);
    let p2 = Purge::new("p2", "devB", 1, "2026-11-05T12:15:00Z", cov2);

    let res_12 = visible(&events, &[p1.clone(), p2.clone()]);
    let res_21 = visible(&events, &[p2, p1]);

    assert_eq!(
        res_12, res_21,
        "Purge evaluation must be identical regardless of order!"
    );
    assert_eq!(res_12.hidden_count, 8);
    assert_eq!(res_12.visible.len(), 2);
    assert_eq!(res_12.visible[0].seq, 9);
    assert_eq!(res_12.visible[1].seq, 10);
}

#[test]
fn test_new_history_after_purge_does_not_seal_paper() {
    let mut events = Vec::new();
    // Events before purge (seq 1..5)
    for seq in 1..=5 {
        events.push(Event::new(
            1,
            format!("old-{seq}"),
            "devA",
            seq,
            "2026-11-05T10:00:00Z",
            "Me",
            "mark_create",
            json!({"old": true}),
        ));
    }

    let mut cov = HashMap::new();
    cov.insert("devA".to_string(), 5);
    let purge = Purge::new("purge-1", "devA", 6, "2026-11-05T11:00:00Z", cov);

    // Fresh events created after purge (seq 6..8)
    for seq in 6..=8 {
        events.push(Event::new(
            1,
            format!("fresh-{seq}"),
            "devA",
            seq,
            "2026-11-05T12:00:00Z",
            "Me",
            "mark_create",
            json!({"fresh": true}),
        ));
    }

    let res = visible(&events, &[purge]);
    assert_eq!(res.hidden_count, 5);
    assert_eq!(res.visible.len(), 3);
    assert_eq!(res.visible[0].seq, 6);
    assert_eq!(res.visible[1].seq, 7);
    assert_eq!(res.visible[2].seq, 8);
}
