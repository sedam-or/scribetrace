//! Use cases and the ports they need. Adapters implement the ports.

mod import;
mod ports;

pub use import::{ImportMarks, ImportReport};
pub use ports::{
    AppError, Clock, DeviceStore, HistoryStore, IdGenerator, LogWriter, PaperItem, ReadReport,
    SourceError, SourceErrorCode, StoreError, ZoteroSource,
};
