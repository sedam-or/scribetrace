use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{DeviceId, Mark};

/// Format version written on every line (`v`).
pub const FORMAT_VERSION: u32 = 1;

/// Maximum allowed line size for an event record (1 MB) per ADR 0007 / SEC-12.
pub const MAX_LINE_BYTES: usize = 1_048_576;

/// Default author for local events.
pub const DEFAULT_AUTHOR: &str = "Me";

fn default_author() -> String {
    DEFAULT_AUTHOR.to_string()
}

/// Payload for a `purge` event, specifying the highest covered sequence per device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PurgePayload {
    /// Map of device id to the highest sequence number covered/hidden on that device.
    pub covers: HashMap<DeviceId, u64>,
}

/// The typed part of a line: `type` and `payload`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum EventBody {
    /// A mark seen for the first time in a paper's history.
    MarkImport(Mark),
    /// Forgetting history up to specific sequence numbers across devices (ADR 0007).
    Purge(PurgePayload),
    /// Mark created manually.
    MarkCreate(serde_json::Value),
    /// Mark updated.
    MarkUpdate(serde_json::Value),
    /// Mark deleted.
    MarkDelete(serde_json::Value),
    /// Mark restored.
    MarkRestore(serde_json::Value),
    /// Mark gone from source.
    MarkGone(serde_json::Value),
    /// Saved revision of a paper note.
    NoteRevision(serde_json::Value),
    /// Named milestone in paper history.
    Milestone(serde_json::Value),
    /// Generic/extensible event body for forward compatibility.
    Other(serde_json::Value),
}

/// One line of a per-device history file (ADR 0007).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    /// Format version.
    pub v: u32,
    /// Globally unique id (UUID text); merging is a union by this id.
    pub id: String,
    /// The device that wrote the line.
    pub device: DeviceId,
    /// Per-device counter, 1-based.
    pub seq: u64,
    /// Wall-clock time for display only; never used to decide anything.
    pub ts: String,
    /// Who made the change; "Me" until a person sets a name.
    #[serde(default = "default_author")]
    pub author: String,
    /// What happened.
    #[serde(flatten)]
    pub body: EventBody,
}

impl Event {
    /// Constructs a new history event.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        v: u32,
        id: impl Into<String>,
        device: DeviceId,
        seq: u64,
        ts: impl Into<String>,
        author: impl Into<String>,
        body: EventBody,
    ) -> Self {
        Self {
            v,
            id: id.into(),
            device,
            seq,
            ts: ts.into(),
            author: author.into(),
            body,
        }
    }
}

/// Representation of a purge event for visibility filtering (ADR 0007).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Purge {
    /// Unique identifier of the purge event.
    pub id: String,
    /// Device that issued the purge.
    pub device: DeviceId,
    /// Sequence number of the purge event.
    pub seq: u64,
    /// Timestamp for display only.
    pub ts: String,
    /// Map of covered devices to their highest covered sequence number.
    pub covers: HashMap<DeviceId, u64>,
}

impl Purge {
    /// Constructs a purge record.
    pub fn new(
        id: impl Into<String>,
        device: DeviceId,
        seq: u64,
        ts: impl Into<String>,
        covers: HashMap<DeviceId, u64>,
    ) -> Self {
        Self {
            id: id.into(),
            device,
            seq,
            ts: ts.into(),
            covers,
        }
    }
}

/// Summary of visibility calculation per ADR 0007.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibilityResult {
    /// Visible non-purge events.
    pub visible: Vec<Event>,
    /// Number of events hidden by purges.
    pub hidden_count: usize,
    /// Total events evaluated (excluding purge tombstones).
    pub total_events: usize,
    /// Cumulative supremum of all covers across devices.
    pub effective_covers: HashMap<DeviceId, u64>,
}

/// Pure domain function: performs union merge over any sequence of events by `id`,
/// returning deterministically ordered events sorted by `(device, seq, id)`.
///
/// Properties (ADR 0007):
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
                // spec/FORMAT.md §4: keep the line from the device whose id is smaller
                // in byte order; within one device, the earlier file line wins (keep existing).
                if ev.device < existing.device {
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

/// Sorts events for UI timeline display by `(ts, device, seq, id)` per ADR 0007.
pub fn sort_for_display(events: &[Event]) -> Vec<Event> {
    let mut display_list = events.to_vec();
    display_list
        .sort_by(|a, b| (&a.ts, &a.device, a.seq, &a.id).cmp(&(&b.ts, &b.device, b.seq, &b.id)));
    display_list
}

/// Pure domain function: calculates visible events after applying all purge coverage rules.
///
/// Rules (ADR 0007):
/// 1. Events are hidden only if their device is present in `effective_covers` and `seq <= cover`.
///    Devices not present in any purge are never hidden.
/// 2. Wall clock `ts` is display-only and never affects visibility.
/// 3. In accordance with amended ADR 0007, no per-event "uncovered" flag is stored or emitted.
/// 4. Multiple purges combine cumulatively: effective coverage is the supremum (max) of all purges.
/// 5. Deterministic and commutative: random orderings of purges produce identical visibility.
pub fn visible(events: &[Event], purges: &[Purge]) -> VisibilityResult {
    let mut effective_covers: HashMap<DeviceId, u64> = HashMap::new();
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
        if matches!(ev.body, EventBody::Purge(_)) {
            continue;
        }

        total_events += 1;

        if let Some(&covered_seq) = effective_covers.get(&ev.device) {
            if covered_seq > 0 && ev.seq <= covered_seq {
                hidden_count += 1;
                continue;
            }
        }

        visible.push(ev.clone());
    }

    VisibilityResult {
        visible,
        hidden_count,
        total_events,
        effective_covers,
    }
}
