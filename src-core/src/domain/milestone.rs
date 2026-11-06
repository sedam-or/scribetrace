//! Milestones as people see them (`spec/FORMAT.md` §7). Pure functions over events: no I/O, no clock.
//! Every input is the visible events of one paper, already sorted for display.

use std::collections::BTreeMap;

use super::{DeviceId, Event, EventBody, MilestoneReason};

/// A milestone with its current name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Milestone {
    /// The `id` of the `milestone` line.
    pub id: String,
    /// Wall-clock time of the milestone line, for display only.
    pub ts: String,
    /// Why the milestone was made.
    pub reason: MilestoneReason,
    /// The current name: the latest rename by display order, else the label it was made with.
    pub label: Option<String>,
    /// The cut that defines the state at this milestone.
    pub upto: BTreeMap<DeviceId, u64>,
}

/// The cut of a set of events: for each device with events, its highest `seq`.
/// A device with no events is left out, so no entry is ever 0.
pub fn cut_of(display_events: &[Event]) -> BTreeMap<DeviceId, u64> {
    let mut cut: BTreeMap<DeviceId, u64> = BTreeMap::new();
    for ev in display_events {
        let top = cut.entry(ev.device.clone()).or_insert(0);
        if ev.seq > *top {
            *top = ev.seq;
        }
    }
    cut
}

/// Lists the milestones in display order, each with its current name.
/// A rename that names a milestone not in `display_events` has no effect.
pub fn milestones(display_events: &[Event]) -> Vec<Milestone> {
    let mut out: Vec<Milestone> = display_events
        .iter()
        .filter_map(|ev| match &ev.body {
            EventBody::Milestone(p) => Some(Milestone {
                id: ev.id.clone(),
                ts: ev.ts.clone(),
                reason: p.reason,
                label: p.label.clone(),
                upto: p.upto.clone(),
            }),
            _ => None,
        })
        .collect();
    for ev in display_events {
        if let EventBody::MilestoneRename(r) = &ev.body {
            if let Some(m) = out.iter_mut().find(|m| m.id == r.milestone) {
                m.label = r.label.clone();
            }
        }
    }
    out
}

/// True when the history has changed since the last milestone: a mark was imported with a `seq`
/// above that milestone's cut. Before any milestone, any mark counts as a change.
/// Renames and milestones themselves are not changes to the paper's content.
pub fn changed_since_last(display_events: &[Event]) -> bool {
    let cut = milestones(display_events)
        .last()
        .map(|m| m.upto.clone())
        .unwrap_or_default();
    display_events.iter().any(|ev| {
        matches!(ev.body, EventBody::MarkImport(_))
            && ev.seq > cut.get(&ev.device).copied().unwrap_or(0)
    })
}
