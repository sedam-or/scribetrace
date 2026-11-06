use super::ports::{AppError, Clock, HistoryStore, IdGenerator};
use crate::domain::{
    milestones, DeviceId, DomainError, Event, EventBody, Milestone, MilestoneRenamePayload,
    PaperId, FORMAT_VERSION, MAX_LABEL_CHARS,
};

/// Use case: give or change a milestone's name by adding a `milestone_rename` line (R-11).
/// No existing line is edited.
#[derive(Debug)]
pub struct RenameMilestone<'a, S, C, I> {
    store: &'a S,
    clock: &'a C,
    ids: &'a I,
    device: DeviceId,
}

impl<'a, S, C, I> RenameMilestone<'a, S, C, I>
where
    S: HistoryStore,
    C: Clock,
    I: IdGenerator,
{
    /// Wires the use case to its ports.
    pub fn new(store: &'a S, clock: &'a C, ids: &'a I, device: DeviceId) -> Self {
        Self {
            store,
            clock,
            ids,
            device,
        }
    }

    /// Names `milestone_id`. Returns `Ok(false)` and writes nothing when that milestone is
    /// missing or hidden (`spec/FORMAT.md` §7: such a rename has no effect).
    pub fn run(
        &self,
        paper: &PaperId,
        milestone_id: &str,
        label: Option<String>,
    ) -> Result<bool, AppError> {
        if label
            .as_ref()
            .is_some_and(|l| l.chars().count() > MAX_LABEL_CHARS)
        {
            return Err(DomainError::LabelTooLong {
                max: MAX_LABEL_CHARS,
            }
            .into());
        }
        let visible = self.store.read(paper)?;
        if !milestones(&visible).iter().any(|m| m.id == milestone_id) {
            return Ok(false);
        }
        let seq = self.store.next_seq(paper, &self.device)?;
        let event = Event {
            v: FORMAT_VERSION,
            id: self.ids.next_id(),
            device: self.device.clone(),
            seq,
            ts: self.clock.now(),
            author: "Me".to_owned(),
            body: EventBody::MilestoneRename(MilestoneRenamePayload {
                milestone: milestone_id.to_owned(),
                label,
            }),
        };
        self.store.append(paper, &event)?;
        Ok(true)
    }
}

/// Lists a paper's milestones in display order, each with its current name.
pub fn list_milestones<S: HistoryStore>(
    store: &S,
    paper: &PaperId,
) -> Result<Vec<Milestone>, AppError> {
    Ok(milestones(&store.read(paper)?))
}
