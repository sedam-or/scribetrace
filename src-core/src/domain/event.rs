use serde::{Deserialize, Serialize};

use super::{DeviceId, Mark};

/// Format version written on every line (`v`).
pub const FORMAT_VERSION: u32 = 1;

/// The typed part of a line: `type` and `payload`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum EventBody {
    /// A mark seen for the first time in a paper's history.
    MarkImport(Mark),
}

/// One line of a per-device history file.
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
    pub author: String,
    /// What happened.
    #[serde(flatten)]
    pub body: EventBody,
}
