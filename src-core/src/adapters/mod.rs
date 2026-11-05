//! I/O adapters for the application ports.

mod device_storage;
mod jsonl_store;
mod system;
mod zotero_http;

pub use device_storage::FsDeviceStore;
pub use jsonl_store::{FileScan, FsHistoryStore};
pub use system::{SystemClock, UuidGenerator};
pub use zotero_http::{parse_marks, parse_papers, ZoteroHttp};
