#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;

use proptest::prelude::*;
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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]

    #[test]
    fn prop_random_purge_orders_identical(
        seed in any::<u64>(),
        shuffle_indices in prop::collection::vec(0usize..6, 6),
    ) {
        let _ = seed;
        // Setup 50 events across 5 virtual devices
        let mut events = Vec::new();
        for dev_idx in 0..5 {
            let dev = DeviceId::new(format!("dev-{dev_idx}")).unwrap();
            for seq in 1..=10 {
                events.push(Event::new(
                    1,
                    format!("dev-{dev_idx}-ev-{seq}"),
                    dev.clone(),
                    seq,
                    "2026-11-05T12:00:00Z",
                    "Me",
                    dummy_mark(&format!("ev-{dev_idx}-{seq}")),
                ));
            }
        }

        let dev0 = DeviceId::new("dev-0").unwrap();
        let dev1 = DeviceId::new("dev-1").unwrap();
        let dev2 = DeviceId::new("dev-2").unwrap();
        let dev3 = DeviceId::new("dev-3").unwrap();
        let dev4 = DeviceId::new("dev-4").unwrap();

        let mut purges = Vec::new();

        let mut cov1 = HashMap::new();
        cov1.insert(dev0.clone(), 4);
        cov1.insert(dev1.clone(), 2);
        purges.push(Purge::new("p1", dev0.clone(), 11, "2026-11-05T12:10:00Z", cov1));

        let mut cov2 = HashMap::new();
        cov2.insert(dev0.clone(), 7); // Covers higher than p1
        cov2.insert(dev2.clone(), 5);
        purges.push(Purge::new("p2", dev1.clone(), 11, "2026-11-05T12:15:00Z", cov2));

        let mut cov3 = HashMap::new();
        cov3.insert(dev1.clone(), 8);
        cov3.insert(dev3.clone(), 3);
        purges.push(Purge::new("p3", dev2.clone(), 11, "2026-11-05T12:20:00Z", cov3));

        let mut cov4 = HashMap::new();
        cov4.insert(dev3.clone(), 9);
        cov4.insert(dev4.clone(), 6);
        purges.push(Purge::new("p4", dev3, 11, "2026-11-05T12:25:00Z", cov4));

        let mut cov5 = HashMap::new();
        cov5.insert(dev4.clone(), 10); // Covers all dev-4
        purges.push(Purge::new("p5", dev0, 12, "2026-11-05T12:30:00Z", cov5));

        let mut cov6 = HashMap::new();
        cov6.insert(dev2, 4); // Subsumed by p2
        purges.push(Purge::new("p6", dev4, 11, "2026-11-05T12:35:00Z", cov6));

        let baseline = visible(&events, &purges);

        // Permute order according to shuffle_indices
        let mut shuffled = Vec::with_capacity(purges.len());
        let mut indices_pool: Vec<usize> = (0..purges.len()).collect();
        for &idx in &shuffle_indices {
            let pick = idx % indices_pool.len();
            let orig_idx = indices_pool.remove(pick);
            shuffled.push(purges[orig_idx].clone());
        }

        let permuted_result = visible(&events, &shuffled);
        prop_assert_eq!(
            permuted_result,
            baseline,
            "Purge order perturbation must produce identical visibility!"
        );
    }
}
