use crate::domain::{DeviceId, DomainError, Event, Mark, PaperId};

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
    /// An event could not be written.
    #[error("cannot encode event: {0}")]
    Encode(String),
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
    /// All readable events of `paper` across devices. Slice 3 adds bad-line counting.
    fn read(&self, paper: &PaperId) -> Result<Vec<Event>, StoreError>;
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
