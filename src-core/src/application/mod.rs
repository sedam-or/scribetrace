//! Use cases and the ports they need. Adapters implement the ports.

mod external_links;
mod import;
mod milestones;
mod ports;

pub use external_links::{
    build_external_uri, effective_paper_metadata, external_link_availability,
    is_allowed_external_uri, ExternalLinkError, ExternalTarget, LinkAvailabilityCode,
    PaperLinkAvailability,
};
pub(crate) use external_links::{is_valid_doi, is_valid_zotero_key};
pub use import::{ImportMarks, ImportReport};
pub use milestones::{list_milestones, RenameMilestone};
pub use ports::{
    AppError, Clock, DeviceStore, HistoryStore, IdGenerator, LogWriter, PaperItem, ReadReport,
    SourceError, SourceErrorCode, StoreError, ZoteroSource,
};
