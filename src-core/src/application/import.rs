use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::ports::{AppError, Clock, HistoryStore, IdGenerator, ZoteroSource};
use crate::domain::{
    changed_since_last, cut_of, milestones, DeviceId, Event, EventBody, Mark, MilestonePayload,
    MilestoneReason, PaperId, FORMAT_VERSION,
};

/// What an import did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportReport {
    /// Marks written to history by this run: only those not already recorded.
    pub imported: usize,
    /// The id of the milestone this run made, or `None` when nothing differed from the last one.
    pub milestone: Option<String>,
}

/// Use case: read a paper's marks from Zotero, record each new one as a `mark_import` event,
/// and, when anything changed, make one milestone (R-11).
#[derive(Debug)]
pub struct ImportMarks<'a, Z, S, C, I> {
    zotero: &'a Z,
    store: &'a S,
    clock: &'a C,
    ids: &'a I,
    device: DeviceId,
}

impl<'a, Z, S, C, I> ImportMarks<'a, Z, S, C, I>
where
    Z: ZoteroSource,
    S: HistoryStore,
    C: Clock,
    I: IdGenerator,
{
    /// Wires the use case to its ports.
    pub fn new(zotero: &'a Z, store: &'a S, clock: &'a C, ids: &'a I, device: DeviceId) -> Self {
        Self {
            zotero,
            store,
            clock,
            ids,
            device,
        }
    }

    /// Imports the marks of `attachment_key` into the history of `paper`.
    ///
    /// A mark is recorded once (`spec/FORMAT.md` §3: "seen for the first time"). Its key is
    /// checked against every line on disk, purged ones included, so a forgotten mark does not
    /// come back on refresh. Then one milestone is written if the history changed since the last.
    pub fn run(&self, paper: &PaperId, attachment_key: &str) -> Result<ImportReport, AppError> {
        let fetched = self.zotero.fetch_marks(attachment_key)?;
        let mut known: HashSet<String> = self
            .store
            .read_report(paper)?
            .raw_events
            .iter()
            .filter_map(|ev| match &ev.body {
                EventBody::MarkImport(m) => Some(m.key.clone()),
                _ => None,
            })
            .collect();
        let new_marks: Vec<Mark> = fetched
            .into_iter()
            .filter(|m| known.insert(m.key.clone()))
            .collect();

        let first = self.store.next_seq(paper, &self.device)?;
        let mut imported = 0;
        for (seq, mark) in (first..).zip(new_marks) {
            let event = self.event(seq, EventBody::MarkImport(mark));
            self.store.append(paper, &event)?;
            imported += 1;
        }

        let milestone = self.record_milestone_if_changed(paper)?;
        Ok(ImportReport {
            imported,
            milestone,
        })
    }

    /// Writes one milestone when the visible history differs from the last milestone's cut.
    /// The first milestone of a paper is `import`; any later one is `refresh`.
    fn record_milestone_if_changed(&self, paper: &PaperId) -> Result<Option<String>, AppError> {
        let visible = self.store.read(paper)?;
        if !changed_since_last(&visible) {
            return Ok(None);
        }
        let reason = if milestones(&visible).is_empty() {
            MilestoneReason::Import
        } else {
            MilestoneReason::Refresh
        };
        let seq = self.store.next_seq(paper, &self.device)?;
        let event = self.event(
            seq,
            EventBody::Milestone(MilestonePayload {
                label: None,
                reason,
                upto: cut_of(&visible),
            }),
        );
        let id = event.id.clone();
        self.store.append(paper, &event)?;
        Ok(Some(id))
    }

    fn event(&self, seq: u64, body: EventBody) -> Event {
        Event {
            v: FORMAT_VERSION,
            id: self.ids.next_id(),
            device: self.device.clone(),
            seq,
            ts: self.clock.now(),
            author: "Me".to_owned(),
            body,
        }
    }
}
