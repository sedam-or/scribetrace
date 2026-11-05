//! Pure domain types. No I/O, no clocks, no randomness.

mod event;
mod ids;
mod mark;

pub use event::{Event, EventBody, FORMAT_VERSION};
pub use ids::{DeviceId, DomainError, PaperId};
pub use mark::Mark;
