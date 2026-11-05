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

/// An event that remains visible to the user after purge evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibleEvent {
    pub event: Event,
    /// True if a purge occurred but this event was not covered (e.g. written offline).
    pub is_uncovered_after_purge: bool,
}

/// Summary of visibility calculation per ADR 0007.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibilityResult {
    pub visible: Vec<VisibleEvent>,
    pub hidden_count: usize,
    pub total_events: usize,
    pub effective_covers: HashMap<String, u64>,
}

/// Pure domain function: calculates visible events after applying all purge coverage rules.
///
/// Rules (ADR 0007):
/// 1. Events with `seq <= covers[device]` are hidden everywhere, regardless of wall clock `ts`.
/// 2. Events not covered by any purge (written offline or fresh history) remain visible.
/// 3. If a purge exists, uncovered events are flagged as `is_uncovered_after_purge`.
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

    let has_purges = !purges.is_empty();
    let mut visible = Vec::new();
    let mut hidden_count = 0;
    let mut total_events = 0;

    for ev in events {
        // Exclude purge tombstone events from mark stream
        if ev.event_type == "purge" {
            continue;
        }

        total_events += 1;
        let covered_seq = effective_covers.get(&ev.device).copied().unwrap_or(0);

        if ev.seq <= covered_seq {
            // Hidden per ADR 0007
            hidden_count += 1;
        } else {
            // Visible
            visible.push(VisibleEvent {
                event: ev.clone(),
                is_uncovered_after_purge: has_purges,
            });
        }
    }

    VisibilityResult {
        visible,
        hidden_count,
        total_events,
        effective_covers,
    }
}
