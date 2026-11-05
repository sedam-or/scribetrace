pub mod domain;
pub mod storage;

pub use domain::{merge_events, sort_for_display, Event, DEFAULT_AUTHOR, MAX_LINE_BYTES};
pub use storage::{
    validate_paper_id, DeviceState, FileLogStorage, LogReader, LogWriter, PaperDeviceState,
    ReadResult, StorageError,
};
