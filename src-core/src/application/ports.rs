use std::collections::HashMap;

use crate::domain::{DeviceId, DeviceState, DomainError, Event, Mark, PaperId};

/// The reference manager could not be read.
#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    /// Zotero did not answer on the local port.
    #[error("cannot reach the reference manager: {0}")]
    Unreachable(String),
    /// Zotero answered, but not in a way we accept.
    #[error("unexpected answer from the reference manager: {0}")]
    BadAnswer(String),
}

/// The history store failed.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// A file operation failed.
    #[error("history file error: {0}")]
    Io(#[from] std::io::Error),
    /// An event or state could not be encoded.
    #[error("cannot encode event or state: {0}")]
    Encode(String),
    /// Domain validation failed on an ID.
    #[error("domain error: {0}")]
    Domain(#[from] DomainError),
}

/// Anything a use case can fail with.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// See [`SourceError`].
    #[error(transparent)]
    Source(#[from] SourceError),
    /// See [`StoreError`].
    #[error(transparent)]
    Store(#[from] StoreError),
    /// See [`DomainError`].
    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Statistics and events from reading paper history files (Slice 3, ADR 0007).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadReport {
    /// All valid events parsed from all scanned device files.
    pub raw_events: Vec<Event>,
    /// Merged events in total order by `(device, seq, id)`.
    pub merged_events: Vec<Event>,
    /// Visible events after applying purge coverage, sorted for display by `(ts, device, seq, id)`.
    pub visible_events: Vec<Event>,
    /// Count of skipped lines (unparsable JSON, >1 MB lines, seq == 0, bad schema).
    pub skipped_lines: usize,
    /// Breakdown of skipped lines by reason code (e.g. "too_long", "not_json", "bad_payload", etc.).
    pub bad_lines: HashMap<String, usize>,
    /// Count of kept lines of unrecognized or forward-compatible event types (FORMAT.md §3).
    pub other_lines: usize,
    /// Count of device files scanned.
    pub files_scanned: usize,
    /// Count of events hidden by purges.
    pub hidden_count: usize,
}

impl ReadReport {
    /// Returns the number of bad lines skipped for a specific reason code.
    pub fn bad_count(&self, code: &str) -> usize {
        self.bad_lines.get(code).copied().unwrap_or(0)
    }
}

/// Abstraction for atomic append operations supporting fault injection (Slice 2, R-10).
pub trait LogWriter: std::io::Write {
    /// Flushes data to disk (sync_data/fsync).
    fn sync_data(&mut self) -> std::io::Result<()>;
}

impl LogWriter for std::fs::File {
    fn sync_data(&mut self) -> std::io::Result<()> {
        std::fs::File::sync_data(self)
    }
}

/// Read side of the reference manager. Called only on a user click.
pub trait ZoteroSource {
    /// Marks attached to the PDF whose Zotero key is `attachment_key`.
    fn fetch_marks(&self, attachment_key: &str) -> Result<Vec<Mark>, SourceError>;
}

/// Append-only per-device history.
pub trait HistoryStore {
    /// The next free `seq` for this device on this paper (1 for a new file).
    fn next_seq(&self, paper: &PaperId, device: &DeviceId) -> Result<u64, StoreError>;
    /// Appends one event to the device's own file for `paper`. Never rewrites old lines.
    fn append(&self, paper: &PaperId, event: &Event) -> Result<(), StoreError>;
    /// Visible events of `paper` across devices, sorted for display.
    fn read(&self, paper: &PaperId) -> Result<Vec<Event>, StoreError> {
        self.read_report(paper).map(|r| r.visible_events)
    }
    /// Full report of all events, visible events, and bad line counts across devices.
    fn read_report(&self, paper: &PaperId) -> Result<ReadReport, StoreError>;
}

/// Device state storage manager in per-user app storage (Slice 2, R-08, SEC-34).
pub trait DeviceStore {
    /// Returns the current device ID, initializing a random UUID if not present.
    fn get_or_create_device_id(&mut self) -> Result<DeviceId, StoreError>;
    /// Returns the current device state.
    fn device_state(&self) -> Result<DeviceState, StoreError>;
    /// Checks for collisions against the file's last event. If a collision is detected,
    /// mints and persists a new random device ID and leaves the old file untouched.
    fn ensure_safe_device(
        &mut self,
        paper: &PaperId,
        file_last_event: Option<&Event>,
    ) -> Result<DeviceId, StoreError>;
    /// Records the last appended event ID for `paper`.
    fn record_last_event(&mut self, paper: &PaperId, event_id: String) -> Result<(), StoreError>;
}

/// Source of display timestamps.
pub trait Clock {
    /// Current time as `YYYY-MM-DDTHH:MM:SSZ`.
    fn now(&self) -> String;
}

/// Source of event ids.
pub trait IdGenerator {
    /// A fresh unique id.
    fn next_id(&self) -> String;
}
