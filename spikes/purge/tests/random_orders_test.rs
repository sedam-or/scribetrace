use rand::seq::SliceRandom;
use rand::thread_rng;
use scribetrace_spike_purge::{visible, Event, Purge};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_10_000_random_purge_orders_identical() {
    // Setup 50 events across 5 virtual devices
    let mut events = Vec::new();
    for dev_idx in 0..5 {
        let dev = format!("dev-{dev_idx}");
        for seq in 1..=10 {
            events.push(Event::new(
                1,
                format!("{dev}-ev-{seq}"),
                &dev,
                seq,
                "2026-11-05T12:00:00Z",
                "Me",
                "mark_create",
                json!({"seq": seq}),
            ));
        }
    }

    // Setup 6 distinct purges from different devices covering different ranges
    let mut purges = Vec::new();

    let mut cov1 = HashMap::new();
    cov1.insert("dev-0".to_string(), 4);
    cov1.insert("dev-1".to_string(), 2);
    purges.push(Purge::new("p1", "dev-0", 11, "2026-11-05T12:10:00Z", cov1));

    let mut cov2 = HashMap::new();
    cov2.insert("dev-0".to_string(), 7); // Covers higher than p1
    cov2.insert("dev-2".to_string(), 5);
    purges.push(Purge::new("p2", "dev-1", 11, "2026-11-05T12:15:00Z", cov2));

    let mut cov3 = HashMap::new();
    cov3.insert("dev-1".to_string(), 8);
    cov3.insert("dev-3".to_string(), 3);
    purges.push(Purge::new("p3", "dev-2", 11, "2026-11-05T12:20:00Z", cov3));

    let mut cov4 = HashMap::new();
    cov4.insert("dev-3".to_string(), 9);
    cov4.insert("dev-4".to_string(), 6);
    purges.push(Purge::new("p4", "dev-3", 11, "2026-11-05T12:25:00Z", cov4));

    let mut cov5 = HashMap::new();
    cov5.insert("dev-4".to_string(), 10); // Covers all dev-4
    purges.push(Purge::new("p5", "dev-0", 12, "2026-11-05T12:30:00Z", cov5));

    let mut cov6 = HashMap::new();
    cov6.insert("dev-2".to_string(), 4); // Subsumed by p2
    purges.push(Purge::new("p6", "dev-4", 11, "2026-11-05T12:35:00Z", cov6));

    // Compute baseline result
    let baseline = visible(&events, &purges);
    assert!(baseline.hidden_count > 0);
    assert!(!baseline.visible.is_empty());

    let mut rng = thread_rng();
    let mut shuffled = purges.clone();

    // Verify 10,000 random order permutations
    for iter in 0..10_000 {
        shuffled.shuffle(&mut rng);
        let current_result = visible(&events, &shuffled);
        assert_eq!(
            current_result, baseline,
            "Purge order perturbation resulted in divergent visibility on iteration {iter}!"
        );
    }

    println!("=== 10,000 Random Purge Orders Test Passed ===");
    println!("Baseline hidden count: {}", baseline.hidden_count);
    println!("Baseline visible count: {}", baseline.visible.len());
    println!("Total events: {}", baseline.total_events);
}
