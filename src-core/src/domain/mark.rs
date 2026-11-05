use serde::{Deserialize, Serialize};

/// One mark (highlight, note, ...) as read from the reference manager.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mark {
    /// The reference manager's key for this mark.
    pub key: String,
    /// `highlight`, `note`, `underline`, `image` or `ink`.
    pub kind: String,
    /// Highlighted text, when there is any.
    pub text: Option<String>,
    /// The reader's own comment, when there is any.
    pub comment: Option<String>,
    /// Colour as `#rrggbb`, when known.
    pub color: Option<String>,
    /// Zero-based page index, when known.
    pub page_index: Option<u32>,
    /// When the mark was created, as given by the source.
    pub added: Option<String>,
    /// When the mark was last changed, as given by the source.
    pub modified: Option<String>,
}
