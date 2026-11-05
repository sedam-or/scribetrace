use std::fmt;

use serde::{Deserialize, Serialize};

/// Why a domain value was refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DomainError {
    /// Identifiers become file and folder names, so only `[A-Za-z0-9_-]` is allowed.
    #[error("invalid {what}: must be 1..={max} characters from [A-Za-z0-9_-]")]
    InvalidId {
        /// Which identifier was refused.
        what: &'static str,
        /// Maximum length.
        max: usize,
    },
}

const MAX_ID_LEN: usize = 128;

fn check(value: &str, what: &'static str) -> Result<(), DomainError> {
    let ok = !value.is_empty()
        && value.len() <= MAX_ID_LEN
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');
    if ok {
        Ok(())
    } else {
        Err(DomainError::InvalidId {
            what,
            max: MAX_ID_LEN,
        })
    }
}

macro_rules! id_type {
    ($name:ident, $what:literal) => {
        #[doc = concat!("A validated ", $what, ".")]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            /// Validates and wraps `value`.
            pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
                let value = value.into();
                check(&value, $what)?;
                Ok(Self(value))
            }

            /// The identifier as text.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = DomainError;
            fn try_from(value: String) -> Result<Self, DomainError> {
                Self::new(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> String {
                value.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

id_type!(PaperId, "paper id");
id_type!(DeviceId, "device id");

impl PaperId {
    /// The paper id for a Zotero item key: `z_<itemKey>`.
    pub fn from_zotero(item_key: &str) -> Result<Self, DomainError> {
        Self::new(format!("z_{item_key}"))
    }
}
