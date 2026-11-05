//! ScribeTrace core. Layers: `domain` (pure) <- `application` (use cases + ports) <- `adapters` (I/O).
//! No UI, no Tauri, no TLS: see docs/ARCHITECTURE.md and AGENTS.md.

pub mod adapters;
pub mod application;
pub mod domain;
