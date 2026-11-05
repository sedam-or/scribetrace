use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Maximum allowed line size for an event record (1 MB) per ADR 0007 / SEC-8.
pub const MAX_LINE_BYTES: usize = 1_048_576;

/// Default author for local events.
pub const DEFAULT_AUTHOR: &str = "Me";

/// Canonical event structure in ScribeTrace log format (ADR 0007).
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
    DEFAULT_AUTHOR.to_string()
}

impl Event {
    /// Constructs a valid new event.
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

/// Pure domain function: performs union merge over any sequence of events by `id`,
/// returning deterministically ordered events sorted by `(device, seq, id)`.
///
/// Properties:
/// - Commutative: merge(A, B) == merge(B, A)
/// - Idempotent: merge(merge(A, B), B) == merge(A, B)
/// - Clock-independent: `ts` is display-only and does not affect domain ordering.
pub fn merge_events(events: impl IntoIterator<Item = Event>) -> Vec<Event> {
    let mut by_id: HashMap<String, Event> = HashMap::new();

    for ev in events {
        by_id
            .entry(ev.id.clone())
            .and_modify(|existing| {
                // Deterministic canonical tie-breaker if two conflicting records share an ID
                let key_new = (&ev.device, ev.seq, &ev.ts, &ev.event_type);
                let key_existing = (
                    &existing.device,
                    existing.seq,
                    &existing.ts,
                    &existing.event_type,
                );
                if key_new > key_existing {
                    *existing = ev.clone();
                }
            })
            .or_insert(ev);
    }

    let mut merged: Vec<Event> = by_id.into_values().collect();
    // Deterministic total ordering by (device, seq, id)
    merged.sort_by(|a, b| (&a.device, a.seq, &a.id).cmp(&(&b.device, b.seq, &b.id)));
    merged
}

/// Sorts events for UI timeline display by `(ts, device, seq)` per ADR 0007.
pub fn sort_for_display(events: &[Event]) -> Vec<Event> {
    let mut display_list = events.to_vec();
    display_list.sort_by(|a, b| (&a.ts, &a.device, a.seq).cmp(&(&b.ts, &b.device, b.seq)));
    display_list
}
