//! I/O adapters for the application ports.

mod jsonl_store;
mod system;
mod zotero_http;

pub use jsonl_store::FsHistoryStore;
pub use system::{SystemClock, UuidGenerator};
pub use zotero_http::ZoteroHttp;
