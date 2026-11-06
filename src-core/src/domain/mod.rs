//! Pure domain types. No I/O, no clocks, no randomness.

mod device;
mod event;
mod ids;
mod line;
mod mark;
mod milestone;

pub use device::{DeviceState, PaperDeviceState};
pub use event::{
    merge_events, sort_for_display, visible, Event, EventBody, MilestonePayload, MilestoneReason,
    MilestoneRenamePayload, Purge, PurgePayload, VisibilityResult, DEFAULT_AUTHOR, FORMAT_VERSION,
    MAX_LABEL_CHARS, MAX_LINE_BYTES,
};
pub use ids::{DeviceId, DomainError, PaperId};
pub use line::{classify_line, BadLine, LineClass};
pub use mark::Mark;
pub use milestone::{changed_since_last, cut_of, milestones, Milestone};
