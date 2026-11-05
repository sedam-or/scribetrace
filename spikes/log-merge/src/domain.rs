use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Maximum allowed line size for an event record (1 MB) per ADR 0007 / SEC-12.
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
    /// Constructs a valid new event. Sequence numbers must be >= 1 per ADR 0007.
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
/// - Full-record canonical tie-breaker: if two records share an ID with differing payloads,
///   the full canonical serialization is compared to guarantee strict commutativity.
pub fn merge_events(events: impl IntoIterator<Item = Event>) -> Vec<Event> {
    let mut by_id: HashMap<String, Event> = HashMap::new();

    for ev in events {
        // Enforce sequence number >= 1
        if ev.seq == 0 {
            continue;
        }

        by_id
            .entry(ev.id.clone())
            .and_modify(|existing| {
                // Canonical tie-breaker comparing full serialized JSON to cover payload and all fields
                if let (Ok(json_new), Ok(json_existing)) =
                    (serde_json::to_string(&ev), serde_json::to_string(existing))
                {
                    if json_new > json_existing {
                        *existing = ev.clone();
                    }
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
