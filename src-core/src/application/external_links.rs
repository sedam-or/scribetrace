use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::ports::SourceErrorCode;
use crate::domain::{DeviceId, Event, EventBody, PaperId, PaperMetadataPayload};

/// The only external destinations a paper can request (`DECISION-LOG.md` row z).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalTarget {
    /// The paper's DOI landing page at doi.org.
    Doi,
    /// The paper's Zotero PDF attachment.
    Pdf,
    /// The paper item in Zotero.
    Item,
}

/// Backend-produced status for one link action; the UI does not infer availability from keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkAvailabilityCode {
    /// The backend has enough validated metadata to build the target.
    Available,
    /// No valid DOI is stored for this paper.
    NoDoi,
    /// No valid PDF attachment key is stored for this paper.
    NoPdf,
    /// The paper id is not a valid Zotero item id.
    NoZoteroKey,
}

/// Availability for the three supported paper-link actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaperLinkAvailability {
    pub doi: LinkAvailabilityCode,
    pub pdf: LinkAvailabilityCode,
    pub item: LinkAvailabilityCode,
}

impl Default for PaperLinkAvailability {
    fn default() -> Self {
        Self {
            doi: LinkAvailabilityCode::NoDoi,
            pdf: LinkAvailabilityCode::NoPdf,
            item: LinkAvailabilityCode::NoZoteroKey,
        }
    }
}

/// A validated paper-link request could not be fulfilled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ExternalLinkError {
    /// The paper has no DOI accepted by the format grammar.
    #[error("paper has no DOI")]
    NoDoi,
    /// The paper has no valid Zotero PDF attachment key.
    #[error("paper has no Zotero PDF attachment")]
    NoPdf,
    /// The paper id does not contain a valid Zotero item key.
    #[error("paper has no Zotero item key")]
    NoZoteroKey,
    /// Stored or supplied metadata did not satisfy the pinned format grammar.
    #[error("paper link metadata is invalid")]
    InvalidMetadata,
}

impl ExternalLinkError {
    /// Converts the link failure to the existing machine-readable IPC error contract.
    pub fn code(self) -> SourceErrorCode {
        match self {
            Self::NoDoi => SourceErrorCode::NoDoi,
            Self::NoPdf => SourceErrorCode::NoPdf,
            Self::NoZoteroKey => SourceErrorCode::NoZoteroKey,
            Self::InvalidMetadata => SourceErrorCode::BadAnswer,
        }
    }
}

/// Builds the one allowed URI for a typed target. No caller-supplied URI is accepted.
pub fn build_external_uri(
    target: ExternalTarget,
    paper_id: &PaperId,
    metadata: &PaperMetadataPayload,
) -> Result<String, ExternalLinkError> {
    let uri = match target {
        ExternalTarget::Doi => {
            let doi = metadata.doi.as_deref().ok_or(ExternalLinkError::NoDoi)?;
            if !is_valid_doi(doi) {
                return Err(ExternalLinkError::InvalidMetadata);
            }
            let path = encode_doi_path(doi)?;
            format!("https://doi.org/{path}")
        }
        ExternalTarget::Pdf => {
            let key = metadata
                .pdf_key
                .as_deref()
                .ok_or(ExternalLinkError::NoPdf)?;
            if !is_valid_zotero_key(key) {
                return Err(ExternalLinkError::InvalidMetadata);
            }
            format!("zotero://open-pdf/library/items/{key}")
        }
        ExternalTarget::Item => {
            let key = zotero_key_from_paper_id(paper_id.as_str())
                .ok_or(ExternalLinkError::NoZoteroKey)?;
            format!("zotero://select/library/items/{key}")
        }
    };

    if is_allowed_external_uri(&uri) {
        Ok(uri)
    } else {
        Err(ExternalLinkError::InvalidMetadata)
    }
}

/// Reports availability without returning a URI or raw Zotero keys to the UI.
pub fn external_link_availability(
    paper_id: &PaperId,
    metadata: &PaperMetadataPayload,
) -> PaperLinkAvailability {
    PaperLinkAvailability {
        doi: if metadata.doi.as_deref().is_some_and(valid_linkable_doi) {
            LinkAvailabilityCode::Available
        } else {
            LinkAvailabilityCode::NoDoi
        },
        pdf: if zotero_key_from_paper_id(paper_id.as_str()).is_none() {
            LinkAvailabilityCode::NoZoteroKey
        } else if metadata.pdf_key.as_deref().is_some_and(is_valid_zotero_key) {
            LinkAvailabilityCode::Available
        } else {
            LinkAvailabilityCode::NoPdf
        },
        item: if zotero_key_from_paper_id(paper_id.as_str()).is_some() {
            LinkAvailabilityCode::Available
        } else {
            LinkAvailabilityCode::NoZoteroKey
        },
    }
}

/// Resolves the effective paper metadata from visible events using the FORMAT §3 clock-free rule.
/// Input order is irrelevant; duplicate per-device sequence numbers use the lexically first event
/// id as a deterministic defensive tie-break.
pub fn effective_paper_metadata(visible_events: &[Event]) -> PaperMetadataPayload {
    let mut per_device: BTreeMap<DeviceId, (u64, String, PaperMetadataPayload)> = BTreeMap::new();
    for event in visible_events {
        let EventBody::PaperMetadata(metadata) = &event.body else {
            continue;
        };
        let replace = per_device
            .get(&event.device)
            .is_none_or(|(seq, id, _)| event.seq > *seq || (event.seq == *seq && event.id < *id));
        if replace {
            per_device.insert(
                event.device.clone(),
                (event.seq, event.id.clone(), metadata.clone()),
            );
        }
    }

    let mut effective = PaperMetadataPayload {
        doi: None,
        pdf_key: None,
    };
    for (_, _, metadata) in per_device.into_values() {
        if effective.doi.is_none() {
            effective.doi = metadata.doi;
        }
        if effective.pdf_key.is_none() {
            effective.pdf_key = metadata.pdf_key;
        }
        if effective.doi.is_some() && effective.pdf_key.is_some() {
            break;
        }
    }
    effective
}

/// Returns whether a fully built URI matches the strict external-open allowlist.
/// This is repeated in the Tauri command before dispatch as defense in depth.
pub fn is_allowed_external_uri(uri: &str) -> bool {
    if let Some(path) = uri.strip_prefix("https://doi.org/") {
        return valid_encoded_doi_path(path);
    }
    if let Some(key) = uri.strip_prefix("zotero://open-pdf/library/items/") {
        return is_valid_zotero_key(key);
    }
    if let Some(key) = uri.strip_prefix("zotero://select/library/items/") {
        return is_valid_zotero_key(key);
    }
    false
}

pub(crate) fn is_valid_doi(doi: &str) -> bool {
    let Some(rest) = doi.strip_prefix("10.") else {
        return false;
    };
    let Some((registrant, suffix)) = rest.split_once('/') else {
        return false;
    };
    (4..=9).contains(&registrant.len())
        && registrant.bytes().all(|b| b.is_ascii_digit())
        && !suffix.is_empty()
        && suffix.bytes().all(|b| {
            b.is_ascii_alphanumeric()
                || matches!(b, b'-' | b'.' | b'_' | b';' | b'(' | b')' | b'/' | b':')
        })
}

fn valid_linkable_doi(doi: &str) -> bool {
    is_valid_doi(doi) && encode_doi_path(doi).is_ok()
}

fn encode_doi_path(doi: &str) -> Result<String, ExternalLinkError> {
    let mut encoded = String::new();
    for (index, segment) in doi.split('/').enumerate() {
        if index > 0 {
            encoded.push('/');
        }
        if segment.is_empty() || matches!(segment, "." | "..") {
            return Err(ExternalLinkError::InvalidMetadata);
        }
        for byte in segment.bytes() {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
                encoded.push(char::from(byte));
            } else {
                encoded.push('%');
                encoded.push(hex_digit(byte >> 4));
                encoded.push(hex_digit(byte & 0x0f));
            }
        }
    }
    Ok(encoded)
}

fn valid_encoded_doi_path(path: &str) -> bool {
    if path.is_empty() || path.contains(['?', '#', '\\']) {
        return false;
    }
    let mut decoded_segments = Vec::new();
    for segment in path.split('/') {
        if segment.is_empty() {
            return false;
        }
        let Some(decoded) = percent_decode_segment(segment) else {
            return false;
        };
        if matches!(decoded.as_str(), "." | "..")
            || decoded.contains(['/', '?', '#', '\\'])
            || !decoded.bytes().all(|b| {
                b.is_ascii_alphanumeric()
                    || matches!(b, b'-' | b'.' | b'_' | b'~' | b';' | b'(' | b')' | b':')
            })
        {
            return false;
        }
        decoded_segments.push(decoded);
    }
    let doi = decoded_segments.join("/");
    is_valid_doi(&doi) && encode_doi_path(&doi).is_ok_and(|canonical| canonical == path)
}

fn percent_decode_segment(segment: &str) -> Option<String> {
    let bytes = segment.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let hi = hex_value(*bytes.get(index + 1)?)?;
            let lo = hex_value(*bytes.get(index + 2)?)?;
            decoded.push((hi << 4) | lo);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).ok()
}

pub(crate) fn is_valid_zotero_key(key: &str) -> bool {
    key.len() == 8
        && key
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
}

fn zotero_key_from_paper_id(paper_id: &str) -> Option<&str> {
    paper_id
        .strip_prefix("z_")
        .filter(|key| is_valid_zotero_key(key))
}

fn hex_digit(value: u8) -> char {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    char::from(HEX[usize::from(value)])
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}
