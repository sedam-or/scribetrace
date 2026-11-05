use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::{DeviceId, DeviceState, DomainError, Event, Mark, PaperId};

/// Machine-readable error code contract shared across the IPC boundary with the frontend (Slice 4 / SEC-7, SEC-9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceErrorCode {
    /// Reference manager is not running on loopback or connection was refused.
    Refused,
    /// HTTP 403 Forbidden ("Local API is not enabled" in Zotero settings).
    Forbidden,
    /// Request exceeded the 5-second timeout budget.
    Timeout,
    /// Zotero-API-Version header missing or != "3".
    Version,
    /// Response payload exceeded the 15 MB body cap.
    TooLarge,
    /// Non-200 HTTP status, malformed JSON, off-list path, or redirect refused by broker.
    BadAnswer,
    /// Internal store or IO failure.
    Internal,
}

impl SourceErrorCode {
    /// Returns the snake_case string representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Refused => "refused",
            Self::Forbidden => "forbidden",
            Self::Timeout => "timeout",
            Self::Version => "version",
            Self::TooLarge => "too_large",
            Self::BadAnswer => "bad_answer",
            Self::Internal => "internal",
        }
    }
}

/// The reference manager could not be read.
#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    /// Zotero is not running or connection was refused on loopback.
    #[error("cannot reach the reference manager: {0}")]
    Refused(String),
    /// Zotero did not answer on the local port (alias for Refused).
    #[error("cannot reach the reference manager: {0}")]
    Unreachable(String),
    /// HTTP 403 Forbidden ("Local API is not enabled").
    #[error("access forbidden: {0}")]
    Forbidden(String),
    /// Request timed out (>5 seconds).
    #[error("request timed out: {0}")]
    Timeout(String),
    /// API version mismatch (expected "3").
    #[error("unsupported API version: {0}")]
    Version(String),
    /// Response body exceeded 15 MB limit.
    #[error("response too large: {0}")]
    TooLarge(String),
    /// Zotero answered, but not in a way we accept.
    #[error("unexpected answer from the reference manager: {0}")]
    BadAnswer(String),
    /// Internal broker or serialization failure.
    #[error("internal error: {0}")]
    Internal(String),
}

impl SourceError {
    /// Returns the associated typed error code.
    pub fn code(&self) -> SourceErrorCode {
        match self {
            Self::Refused(_) | Self::Unreachable(_) => SourceErrorCode::Refused,
            Self::Forbidden(_) => SourceErrorCode::Forbidden,
            Self::Timeout(_) => SourceErrorCode::Timeout,
            Self::Version(_) => SourceErrorCode::Version,
            Self::TooLarge(_) => SourceErrorCode::TooLarge,
            Self::BadAnswer(_) => SourceErrorCode::BadAnswer,
            Self::Internal(_) => SourceErrorCode::Internal,
        }
    }
}

/// Metadata of a paper discovered in Zotero, returned to the UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaperItem {
    /// Paper identifier in ScribeTrace format (`z_<zoteroKey>`).
    pub id: String,
    /// Title of the paper.
    pub title: String,
    /// Formatted author summary (e.g. "Vaswani et al.").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authors: Option<String>,
    /// Publication year.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
    /// Total number of marks/annotations attached to this paper.
    #[serde(rename = "marksCount")]
    pub marks_count: usize,
    /// Attachment key of the primary PDF enclosure in Zotero.
    #[serde(
        rename = "attachmentKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attachment_key: Option<String>,
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
    /// Sequence number reached maximum representable integer (u64::MAX).
    #[error("sequence overflow on device {0}: device must be retired")]
    SequenceOverflow(DeviceId),
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

impl AppError {
    /// Returns the associated typed error code.
    pub fn code(&self) -> SourceErrorCode {
        match self {
            Self::Source(s) => s.code(),
            Self::Store(_) => SourceErrorCode::Internal,
            Self::Domain(_) => SourceErrorCode::BadAnswer,
        }
    }
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
    /// Discovers available papers from the local reference manager.
    fn list_papers(&self) -> Result<Vec<PaperItem>, SourceError>;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_and_source_error_codes() {
        let errs = [
            (SourceError::Refused("x".into()), SourceErrorCode::Refused),
            (
                SourceError::Unreachable("x".into()),
                SourceErrorCode::Refused,
            ),
            (
                SourceError::Forbidden("x".into()),
                SourceErrorCode::Forbidden,
            ),
            (SourceError::Timeout("x".into()), SourceErrorCode::Timeout),
            (SourceError::Version("x".into()), SourceErrorCode::Version),
            (SourceError::TooLarge("x".into()), SourceErrorCode::TooLarge),
            (
                SourceError::BadAnswer("x".into()),
                SourceErrorCode::BadAnswer,
            ),
            (SourceError::Internal("x".into()), SourceErrorCode::Internal),
        ];

        for (src_err, expected_code) in errs {
            assert_eq!(src_err.code(), expected_code);
            let app_err = AppError::from(src_err);
            assert_eq!(app_err.code(), expected_code);
        }

        let store_err = AppError::from(StoreError::Encode("fail".into()));
        assert_eq!(store_err.code(), SourceErrorCode::Internal);

        let domain_err = AppError::from(DomainError::InvalidId {
            what: "test",
            max: 10,
        });
        assert_eq!(domain_err.code(), SourceErrorCode::BadAnswer);
    }
}
