use serde::{Deserialize, Serialize};

use super::ports::{AppError, Clock, HistoryStore, IdGenerator, ZoteroSource};
use crate::domain::{DeviceId, Event, EventBody, PaperId, FORMAT_VERSION};

/// What an import did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportReport {
    /// Marks written to history by this run.
    pub imported: usize,
}

/// Use case: read a paper's marks from Zotero and record each as a `mark_import` event.
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

    /// Imports every mark of `attachment_key` into the history of `paper`.
    pub fn run(&self, paper: &PaperId, attachment_key: &str) -> Result<ImportReport, AppError> {
        let marks = self.zotero.fetch_marks(attachment_key)?;
        let first = self.store.next_seq(paper, &self.device)?;
        let mut imported = 0;
        for (seq, mark) in (first..).zip(marks) {
            let event = Event {
                v: FORMAT_VERSION,
                id: self.ids.next_id(),
                device: self.device.clone(),
                seq,
                ts: self.clock.now(),
                author: "Me".to_owned(),
                body: EventBody::MarkImport(mark),
            };
            self.store.append(paper, &event)?;
            imported += 1;
        }
        Ok(ImportReport { imported })
    }
}
