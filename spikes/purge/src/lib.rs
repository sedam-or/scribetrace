use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Canonical event structure.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Event {
    pub v: u32,
    pub id: String,
    pub device: String,
    pub seq: u64,
    pub ts: String, // Wall clock ISO-8601, display only
    #[serde(default = "default_author")]
    pub author: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub payload: serde_json::Value,
}

fn default_author() -> String {
    "Me".to_string()
}

impl Event {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        v: u32,
        id: impl Into<String>,
        device: impl Into<String>,
        seq: u64,
        ts: impl Into<String>,
        author: impl Into<String>,
        event_type: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            v,
            id: id.into(),
            device: device.into(),
            seq,
            ts: ts.into(),
            author: author.into(),
            event_type: event_type.into(),
            payload,
        }
    }
}

/// Represents a purge / forget event with per-device sequence coverage (ADR 0007).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Purge {
    pub id: String,
    pub device: String,
    pub seq: u64,
    pub ts: String, // Display only, never used for visibility
    pub covers: HashMap<String, u64>,
}

impl Purge {
    pub fn new(
        id: impl Into<String>,
        device: impl Into<String>,
        seq: u64,
        ts: impl Into<String>,
        covers: HashMap<String, u64>,
    ) -> Self {
        Self {
            id: id.into(),
            device: device.into(),
            seq,
            ts: ts.into(),
            covers,
        }
    }
}

/// Summary of visibility calculation per ADR 0007.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibilityResult {
    pub visible: Vec<Event>,
    pub hidden_count: usize,
    pub total_events: usize,
    pub effective_covers: HashMap<String, u64>,
}

/// Pure domain function: calculates visible events after applying all purge coverage rules.
///
/// Rules (ADR 0007):
/// 1. Events are hidden only if their device is present in `effective_covers` and `seq <= cover`.
///    Devices not present in any purge are never hidden.
/// 2. Wall clock `ts` is display-only and never affects visibility.
/// 3. In accordance with amended ADR 0007, no per-event "uncovered" flag is stored or emitted.
/// 4. Multiple purges combine cumulatively: effective coverage is the supremum (max) of all purges.
/// 5. Deterministic and commutative: 10,000 random orderings of purges produce identical visibility.
pub fn visible(events: &[Event], purges: &[Purge]) -> VisibilityResult {
    // 1. Calculate cumulative effective coverage across all purges: max seq per device
    let mut effective_covers: HashMap<String, u64> = HashMap::new();
    for p in purges {
        for (dev, &seq) in &p.covers {
            effective_covers
                .entry(dev.clone())
                .and_modify(|existing| *existing = (*existing).max(seq))
                .or_insert(seq);
        }
    }

    let mut visible = Vec::new();
    let mut hidden_count = 0;
    let mut total_events = 0;

    for ev in events {
        // Exclude purge tombstone events from active mark stream
        if ev.event_type == "purge" {
            continue;
        }

        total_events += 1;

        // Hide ONLY if the device is present in effective_covers and seq <= cover.
        if let Some(&covered_seq) = effective_covers.get(&ev.device) {
            if ev.seq <= covered_seq {
                hidden_count += 1;
                continue;
            }
        }

        // Visible event
        visible.push(ev.clone());
    }

    VisibilityResult {
        visible,
        hidden_count,
        total_events,
        effective_covers,
    }
}
