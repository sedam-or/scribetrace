#![allow(clippy::unwrap_used, clippy::expect_used)]

use proptest::prelude::*;
use scribetrace_core::domain::{merge_events, DeviceId, Event, EventBody};

fn arb_event_pool() -> impl Strategy<Value = Vec<Event>> {
    // Generate up to 40 unique events across 5 virtual devices
    prop::collection::vec(
        (
            0u8..5,       // device index (0..4)
            1u64..100,    // sequence number (seq >= 1)
            0u32..10_000, // fake timestamp
        ),
        1..40,
    )
    .prop_map(|raw_tuples| {
        raw_tuples
            .into_iter()
            .enumerate()
            .map(|(idx, (dev_idx, seq, ts_sec))| {
                Event::new(
                    1,
                    format!("event-uuid-{idx}-{dev_idx}-{seq}"),
                    DeviceId::new(format!("virtual-device-{dev_idx}")).unwrap(),
                    seq,
                    format!(
                        "2026-11-05T{:02}:{:02}:{:02}Z",
                        (ts_sec / 3600) % 24,
                        (ts_sec % 3600) / 60,
                        ts_sec % 60
                    ),
                    "Me",
                    EventBody::MarkImport(scribetrace_core::domain::Mark {
                        key: format!("k-{idx}"),
                        kind: "highlight".to_string(),
                        text: Some(format!("text-{seq}")),
                        comment: None,
                        color: None,
                        page_index: Some(seq as u32),
                        added: None,
                        modified: None,
                    }),
                )
            })
            .collect()
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]

    #[test]
    fn prop_merge_commutativity_and_idempotency(
        pool in arb_event_pool(),
        split_mask in prop::collection::vec(any::<bool>(), 1..40),
        duplicate_indices in prop::collection::vec(0usize..40, 0..15),
    ) {
        let mut list_a = Vec::new();
        let mut list_b = Vec::new();

        for (i, ev) in pool.iter().enumerate() {
            let take_a = split_mask.get(i).copied().unwrap_or(true);
            if take_a {
                list_a.push(ev.clone());
            } else {
                list_b.push(ev.clone());
            }
        }

        for &idx in &duplicate_indices {
            if let Some(ev) = pool.get(idx % pool.len()) {
                if idx % 2 == 0 {
                    list_a.push(ev.clone());
                } else {
                    list_b.push(ev.clone());
                }
            }
        }

        // Commutativity: merge(A, B) == merge(B, A)
        let mut union_ab = list_a.clone();
        union_ab.extend(list_b.clone());

        let mut union_ba = list_b.clone();
        union_ba.extend(list_a);

        let merged_ab = merge_events(union_ab);
        let merged_ba = merge_events(union_ba);

        prop_assert_eq!(&merged_ab, &merged_ba, "merge(A, B) must equal merge(B, A)");

        // Idempotency: merge(merge(A, B), B) == merge(A, B)
        let mut union_with_b = merged_ab.clone();
        union_with_b.extend(list_b);

        let merged_idempotent = merge_events(union_with_b);
        prop_assert_eq!(
            &merged_idempotent,
            &merged_ab,
            "merge(merge(A, B), B) must equal merge(A, B)"
        );
    }

    #[test]
    fn prop_same_id_conflicting_devices_commutativity(
        id in "[a-z0-9-]{8}",
        payload_a in prop::collection::vec(any::<u8>(), 1..20),
        payload_b in prop::collection::vec(any::<u8>(), 1..20),
    ) {
        let dev_a = DeviceId::new("dev-aaa").unwrap();
        let dev_b = DeviceId::new("dev-bbb").unwrap();
        let ev_a = Event::new(
            1,
            id.clone(),
            dev_a.clone(),
            1,
            "2026-11-05T10:00:00Z",
            "Me",
            EventBody::MarkImport(scribetrace_core::domain::Mark {
                key: format!("k-a-{payload_a:?}"),
                kind: "highlight".to_string(),
                text: Some("text a".to_string()),
                comment: None,
                color: None,
                page_index: Some(1),
                added: None,
                modified: None,
            }),
        );
        let ev_b = Event::new(
            1,
            id,
            dev_b,
            1,
            "2026-11-05T10:00:00Z",
            "Me",
            EventBody::MarkImport(scribetrace_core::domain::Mark {
                key: format!("k-b-{payload_b:?}"),
                kind: "note".to_string(),
                text: Some("text b".to_string()),
                comment: None,
                color: None,
                page_index: Some(2),
                added: None,
                modified: None,
            }),
        );

        let ab = merge_events(vec![ev_a.clone(), ev_b.clone()]);
        let ba = merge_events(vec![ev_b, ev_a]);

        prop_assert_eq!(&ab, &ba, "Conflicting device IDs must deterministically pick smaller device ID!");
        prop_assert_eq!(&ab[0].device, &dev_a);
    }
}

#[test]
fn test_conflicting_copies_both_rules() {
    let dev_a = DeviceId::new("dev-a").unwrap();
    let dev_b = DeviceId::new("dev-b").unwrap();

    let ev_a = Event::new(
        1,
        "same-id",
        dev_a.clone(),
        1,
        "2026-11-05T10:00:00Z",
        "Me",
        EventBody::MarkImport(scribetrace_core::domain::Mark {
            key: "k1".into(),
            kind: "highlight".into(),
            text: Some("A".into()),
            comment: None,
            color: None,
            page_index: Some(1),
            added: None,
            modified: None,
        }),
    );
    let ev_b = Event::new(
        1,
        "same-id",
        dev_b,
        1,
        "2026-11-05T10:00:00Z",
        "Me",
        EventBody::MarkImport(scribetrace_core::domain::Mark {
            key: "k2".into(),
            kind: "highlight".into(),
            text: Some("B".into()),
            comment: None,
            color: None,
            page_index: Some(1),
            added: None,
            modified: None,
        }),
    );

    // Cross-device: smaller device id wins regardless of order
    let res1 = merge_events(vec![ev_a.clone(), ev_b.clone()]);
    let res2 = merge_events(vec![ev_b, ev_a]);
    assert_eq!(res1, res2);
    assert_eq!(res1[0].device, dev_a);

    // Within-device: earlier file line wins
    let ev1 = Event::new(
        1,
        "same-id-2",
        dev_a.clone(),
        1,
        "2026-11-05T10:00:00Z",
        "Me",
        EventBody::MarkImport(scribetrace_core::domain::Mark {
            key: "k1".into(),
            kind: "highlight".into(),
            text: Some("1".into()),
            comment: None,
            color: None,
            page_index: Some(1),
            added: None,
            modified: None,
        }),
    );
    let ev2 = Event::new(
        1,
        "same-id-2",
        dev_a,
        1,
        "2026-11-05T10:00:00Z",
        "Me",
        EventBody::MarkImport(scribetrace_core::domain::Mark {
            key: "k2".into(),
            kind: "highlight".into(),
            text: Some("2".into()),
            comment: None,
            color: None,
            page_index: Some(1),
            added: None,
            modified: None,
        }),
    );

    let order1 = merge_events(vec![ev1.clone(), ev2.clone()]);
    let order2 = merge_events(vec![ev2.clone(), ev1.clone()]);
    assert_eq!(order1[0].body, ev1.body);
    assert_eq!(order2[0].body, ev2.body);
}
