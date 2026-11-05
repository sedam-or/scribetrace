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

impl Mark {
    /// Checks the rules of `spec/FORMAT.md` for a `mark_import` payload.
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.key.is_empty() || self.key.len() > 128 {
            return Err("key");
        }
        if !["highlight", "note", "underline", "image", "ink"].contains(&self.kind.as_str()) {
            return Err("kind");
        }
        if let Some(c) = &self.color {
            let b = c.as_bytes();
            if !(b.len() == 7 && b[0] == b'#' && b[1..].iter().all(u8::is_ascii_hexdigit)) {
                return Err("color");
            }
        }
        Ok(())
    }
}
