use std::time::Duration;

use serde_json::Value;

use crate::application::{SourceError, ZoteroSource};
use crate::domain::Mark;

const MAX_BODY: u64 = 15 * 1024 * 1024;
const TIMEOUT: Duration = Duration::from_secs(5);

/// Zotero's local API on loopback. GET only, fixed paths, no redirects, 5 s, 15 MB.
#[derive(Debug, Clone)]
pub struct ZoteroHttp {
    port: u16,
    agent: ureq::Agent,
}

impl ZoteroHttp {
    /// A client for `127.0.0.1:port`. The host is not configurable: only loopback is ever used.
    pub fn new(port: u16) -> Self {
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(TIMEOUT))
            .max_redirects(0)
            .http_status_as_error(false)
            .build()
            .into();
        Self { port, agent }
    }

    fn get(&self, path_and_query: &str) -> Result<String, SourceError> {
        let url = format!("http://127.0.0.1:{}{path_and_query}", self.port);
        let mut resp = self
            .agent
            .get(&url)
            .header("Zotero-API-Version", "3")
            .call()
            .map_err(|e| SourceError::Unreachable(e.to_string()))?;
        let status = resp.status().as_u16();
        if status != 200 {
            return Err(SourceError::BadAnswer(format!("HTTP status {status}")));
        }
        let version = resp
            .headers()
            .get("Zotero-API-Version")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        if version != "3" {
            return Err(SourceError::BadAnswer(format!(
                "API version {version:?}, expected \"3\""
            )));
        }
        resp.body_mut()
            .with_config()
            .limit(MAX_BODY)
            .read_to_string()
            .map_err(|e| SourceError::BadAnswer(e.to_string()))
    }
}

impl ZoteroSource for ZoteroHttp {
    fn fetch_marks(&self, attachment_key: &str) -> Result<Vec<Mark>, SourceError> {
        if attachment_key.is_empty() || !attachment_key.bytes().all(|b| b.is_ascii_alphanumeric()) {
            return Err(SourceError::BadAnswer("invalid attachment key".to_owned()));
        }
        let body = self.get(&format!(
            "/api/users/0/items/{attachment_key}/children?itemType=annotation&format=json&limit=100"
        ))?;
        parse_marks(&body)
    }
}

fn text(data: &Value, field: &str) -> Option<String> {
    data.get(field)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
}

fn parse_marks(body: &str) -> Result<Vec<Mark>, SourceError> {
    let items: Vec<Value> =
        serde_json::from_str(body).map_err(|e| SourceError::BadAnswer(e.to_string()))?;
    let mut marks = Vec::new();
    for item in &items {
        let Some(data) = item.get("data") else {
            continue;
        };
        let (Some(key), Some(kind)) = (text(data, "key"), text(data, "annotationType")) else {
            continue;
        };
        // annotationPosition is a JSON document stored as a string.
        let page_index = data
            .get("annotationPosition")
            .and_then(Value::as_str)
            .and_then(|s| serde_json::from_str::<Value>(s).ok())
            .and_then(|p| p.get("pageIndex").and_then(Value::as_u64))
            .and_then(|n| u32::try_from(n).ok());
        marks.push(Mark {
            key,
            kind,
            text: text(data, "annotationText"),
            comment: text(data, "annotationComment"),
            color: text(data, "annotationColor"),
            page_index,
            added: text(data, "dateAdded"),
            modified: text(data, "dateModified"),
        });
    }
    Ok(marks)
}

#[cfg(test)]
mod tests {
    use super::parse_marks;

    #[test]
    fn rejects_non_array() {
        assert!(parse_marks("{}").is_err());
    }

    #[test]
    fn skips_items_without_key_or_type() {
        let marks =
            parse_marks(r#"[{"data":{}},{"data":{"key":"A","annotationType":"note"}}]"#).unwrap();
        assert_eq!(marks.len(), 1);
        assert_eq!(marks[0].page_index, None);
    }
}
