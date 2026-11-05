#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;

use scribetrace_core::domain::{visible, DeviceId, Event, EventBody, Mark, Purge};

fn dummy_mark(key: &str) -> EventBody {
    EventBody::MarkImport(Mark {
        key: key.to_string(),
        kind: "highlight".to_string(),
        text: Some("test".to_string()),
        comment: None,
        color: None,
        page_index: Some(1),
        added: None,
        modified: None,
    })
}

#[test]
fn test_wrong_clocks_both_ways() {
    let dev_a = DeviceId::new("devA").unwrap();

    // Case A: Purge wall clock is in distant PAST (2020), but event clock is in FUTURE (2026).
    // Event seq is 5, Purge covers devA: 5.
    // Result: Covered event MUST be hidden despite older purge timestamp!
    let mut covers_a = HashMap::new();
    covers_a.insert(dev_a.clone(), 5);

    let event_future = Event::new(
        1,
        "ev-future",
        dev_a.clone(),
        5,
        "2026-11-05T12:00:00Z", // Future clock
        "Me",
        dummy_mark("future"),
    );

    let purge_past = Purge::new(
        "purge-past",
        dev_a.clone(),
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
    covers_b.insert(dev_a.clone(), 5);

    let event_uncovered = Event::new(
        1,
        "ev-10",
        dev_a.clone(),
        10,
        "2026-11-05T12:00:00Z",
        "Me",
        dummy_mark("uncovered"),
    );

    let purge_future = Purge::new("purge-future", dev_a, 6, "2035-01-01T00:00:00Z", covers_b);

    let res_b = visible(&[event_uncovered], &[purge_future]);
    assert_eq!(res_b.hidden_count, 0);
    assert_eq!(res_b.visible.len(), 1);
    assert_eq!(res_b.visible[0].seq, 10);
}

#[test]
fn test_unseen_offline_event_and_second_purge() {
    let dev_a = DeviceId::new("devA").unwrap();
    let dev_b = DeviceId::new("devB").unwrap();

    // Device B wrote events while offline: seq 1..5.
    // Device A had only seen up to seq 3 when it ran Purge 1: covers = {devB: 3}.
    let mut events = Vec::new();
    for seq in 1..=5 {
        events.push(Event::new(
            1,
            format!("devB-ev-{seq}"),
            dev_b.clone(),
            seq,
            "2026-11-05T12:00:00Z",
            "Me",
            dummy_mark(&format!("b-{seq}")),
        ));
    }

    let mut covers_1 = HashMap::new();
    covers_1.insert(dev_b.clone(), 3);
    let purge_1 = Purge::new("p1", dev_a.clone(), 1, "2026-11-05T12:10:00Z", covers_1);

    // After Purge 1: seq 1, 2, 3 hidden; seq 4, 5 remain visible
    let res_1 = visible(&events, std::slice::from_ref(&purge_1));
    assert_eq!(res_1.hidden_count, 3);
    assert_eq!(res_1.visible.len(), 2);
    assert_eq!(res_1.visible[0].seq, 4);
    assert_eq!(res_1.visible[1].seq, 5);

    // Later: Device A syncs with Device B, sees seq 5, and triggers second Purge: covers = {devB: 5}.
    let mut covers_2 = HashMap::new();
    covers_2.insert(dev_b, 5);
    let purge_2 = Purge::new("p2", dev_a, 2, "2026-11-05T12:20:00Z", covers_2);

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
    let dev_unrelated = DeviceId::new("devUnrelated").unwrap();
    let dev_other = DeviceId::new("devOther").unwrap();

    let ev = Event::new(
        1,
        "ev-0",
        dev_unrelated.clone(),
        0,
        "2026-11-05T10:00:00Z",
        "Me",
        dummy_mark("unrelated"),
    );
    let mut covers = HashMap::new();
    covers.insert(dev_other.clone(), 10);
    let purge = Purge::new("p1", dev_other, 1, "2026-11-05T10:05:00Z", covers);

    let res = visible(&[ev], &[purge]);
    assert_eq!(res.hidden_count, 0);
    assert_eq!(res.visible.len(), 1);
    assert_eq!(res.visible[0].device, dev_unrelated);
}

#[test]
fn test_two_purges_in_any_order() {
    let dev_a = DeviceId::new("devA").unwrap();
    let mut events = Vec::new();
    for seq in 1..=10 {
        events.push(Event::new(
            1,
            format!("devA-ev-{seq}"),
            dev_a.clone(),
            seq,
            "2026-11-05T12:00:00Z",
            "Me",
            dummy_mark(&format!("a-{seq}")),
        ));
    }

    let mut cov1 = HashMap::new();
    cov1.insert(dev_a.clone(), 4);
    let p1 = Purge::new("p1", dev_a.clone(), 1, "2026-11-05T12:05:00Z", cov1);

    let mut cov2 = HashMap::new();
    cov2.insert(dev_a, 7);
    let p2 = Purge::new(
        "p2",
        DeviceId::new("devB").unwrap(),
        1,
        "2026-11-05T12:15:00Z",
        cov2,
    );

    let order1 = visible(&events, &[p1.clone(), p2.clone()]);
    let order2 = visible(&events, &[p2, p1]);

    assert_eq!(
        order1, order2,
        "Purges in any order must give identical visibility!"
    );
    assert_eq!(order1.hidden_count, 7);
    assert_eq!(order1.visible.len(), 3);
}
