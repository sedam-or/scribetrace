use std::collections::HashMap;
use std::time::Duration;

use serde_json::Value;

use crate::application::{PaperItem, SourceError, ZoteroSource};
use crate::domain::Mark;

const MAX_BODY: u64 = 15 * 1024 * 1024; // 15 MB body cap (SEC-33)
const TIMEOUT: Duration = Duration::from_secs(5); // 5 s timeout budget (SEC-33)
const MAX_PAGES: usize = 100; // Cap pagination to 100 pages (10,000 items) (SEC-33)

/// Validates that `path_and_query` matches the strict broker allow-list (SEC-33).
pub fn validate_allowed_path(path_and_query: &str) -> Result<(), SourceError> {
    let path = path_and_query.split('?').next().unwrap_or(path_and_query);
    if path == "/connector/ping" || path == "/api/users/0/items" || path == "/api/users/0/items/top"
    {
        return Ok(());
    }
    if let Some(rest) = path.strip_prefix("/api/users/0/items/") {
        if let Some(key) = rest.strip_suffix("/children") {
            if !key.is_empty() && key.bytes().all(|b| b.is_ascii_alphanumeric()) {
                return Ok(());
            }
        }
    }
    Err(SourceError::BadAnswer(format!(
        "path not permitted by broker allow-list: {path}"
    )))
}

/// Zotero's local API on loopback. GET only, fixed paths, no redirects, 5 s, 15 MB.
#[derive(Debug, Clone)]
pub struct ZoteroHttp {
    port: u16,
    agent: ureq::Agent,
}

impl ZoteroHttp {
    /// A client for `127.0.0.1:port`. The host is not configurable: only loopback is ever used (SEC-1).
    pub fn new(port: u16) -> Self {
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(TIMEOUT))
            .timeout_per_call(Some(TIMEOUT))
            .timeout_recv_response(Some(TIMEOUT))
            .max_redirects(0)
            .http_status_as_error(false)
            .build()
            .into();
        Self { port, agent }
    }

    /// Healthcheck against `/connector/ping`.
    pub fn ping(&self) -> Result<(), SourceError> {
        self.get("/connector/ping")?;
        Ok(())
    }

    /// Sends a strictly GET request to the local Zotero broker, returning body and optional Total-Results count.
    pub fn get_with_total(
        &self,
        path_and_query: &str,
    ) -> Result<(String, Option<usize>), SourceError> {
        validate_allowed_path(path_and_query)?;

        let url = format!("http://127.0.0.1:{}{path_and_query}", self.port);
        let resp_result = self
            .agent
            .get(&url)
            .header("Zotero-API-Version", "3")
            .call();

        let mut resp = match resp_result {
            Ok(r) => r,
            Err(ureq::Error::Timeout(t)) => {
                return Err(SourceError::Timeout(t.to_string()));
            }
            Err(ureq::Error::TooManyRedirects) | Err(ureq::Error::RedirectFailed) => {
                return Err(SourceError::BadAnswer(
                    "redirect refused by broker".to_owned(),
                ));
            }
            Err(ureq::Error::Io(io_err)) => {
                let msg = io_err.to_string();
                match io_err.kind() {
                    std::io::ErrorKind::TimedOut => return Err(SourceError::Timeout(msg)),
                    _ => return Err(SourceError::Refused(msg)),
                }
            }
            Err(ureq::Error::ConnectionFailed) | Err(ureq::Error::HostNotFound) => {
                return Err(SourceError::Refused(
                    "cannot reach reference manager on loopback".to_owned(),
                ));
            }
            Err(other) => {
                return Err(SourceError::BadAnswer(other.to_string()));
            }
        };

        let status = resp.status().as_u16();
        if (300..=399).contains(&status) {
            return Err(SourceError::BadAnswer(format!(
                "redirect refused by broker (HTTP {status})"
            )));
        }
        if status == 403 {
            return Err(SourceError::Forbidden(
                "Local API is not enabled (HTTP 403)".to_owned(),
            ));
        }
        if status != 200 {
            return Err(SourceError::BadAnswer(format!("HTTP status {status}")));
        }

        // Require Zotero-API-Version: 3 on all /api/ endpoints (SEC-33)
        let path = path_and_query.split('?').next().unwrap_or(path_and_query);
        if path.starts_with("/api/") {
            let version = resp
                .headers()
                .get("Zotero-API-Version")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            if version != "3" {
                return Err(SourceError::Version(format!(
                    "API version {version:?}, expected \"3\""
                )));
            }
        }

        let total_results = resp
            .headers()
            .get("Total-Results")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<usize>().ok());

        // Response size protection: check Content-Length header if present
        if let Some(cl) = resp
            .headers()
            .get("Content-Length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
        {
            if cl > MAX_BODY {
                return Err(SourceError::TooLarge(format!(
                    "Content-Length {cl} exceeds maximum 15 MB limit"
                )));
            }
        }

        // Bounded stream read capped at MAX_BODY (15 MB)
        let body = match resp
            .body_mut()
            .with_config()
            .limit(MAX_BODY)
            .read_to_string()
        {
            Ok(body) => body,
            Err(ureq::Error::BodyExceedsLimit(limit)) => {
                return Err(SourceError::TooLarge(format!(
                    "response body exceeded {limit} bytes limit"
                )));
            }
            Err(e) => {
                return Err(SourceError::BadAnswer(format!("failed to read body: {e}")));
            }
        };

        Ok((body, total_results))
    }

    /// Sends a strictly GET request to the local Zotero broker.
    pub fn get(&self, path_and_query: &str) -> Result<String, SourceError> {
        self.get_with_total(path_and_query).map(|(body, _)| body)
    }
}

impl ZoteroSource for ZoteroHttp {
    fn list_papers(&self) -> Result<Vec<PaperItem>, SourceError> {
        let start_time = std::time::Instant::now();
        let mut all_items: Vec<Value> = Vec::new();
        let mut start = 0;
        let mut page_count = 0;

        loop {
            if page_count >= MAX_PAGES {
                return Err(SourceError::TooLarge(
                    "library exceeds maximum pagination limit of 100 pages".to_owned(),
                ));
            }
            if start_time.elapsed() >= TIMEOUT {
                return Err(SourceError::Timeout(
                    "library pagination exceeded 5 s time budget".to_owned(),
                ));
            }

            let path = format!("/api/users/0/items?limit=100&start={start}");
            let (body, total_results) = self.get_with_total(&path)?;
            let page_items: Vec<Value> = serde_json::from_str(&body)
                .map_err(|e| SourceError::BadAnswer(format!("invalid JSON in items page: {e}")))?;

            let count = page_items.len();
            all_items.extend(page_items);
            page_count += 1;
            start += count;

            if count < 100 {
                break;
            }
            if let Some(total) = total_results {
                if start >= total {
                    break;
                }
            }
        }

        parse_papers_from_items(&all_items)
    }

    fn fetch_marks(&self, attachment_key: &str) -> Result<Vec<Mark>, SourceError> {
        if attachment_key.is_empty() || !attachment_key.bytes().all(|b| b.is_ascii_alphanumeric()) {
            return Err(SourceError::BadAnswer("invalid attachment key".to_owned()));
        }

        let start_time = std::time::Instant::now();
        let mut all_marks: Vec<Mark> = Vec::new();
        let mut seen_keys = std::collections::HashSet::new();
        let mut start = 0;
        let mut page_count = 0;

        loop {
            if page_count >= MAX_PAGES {
                return Err(SourceError::TooLarge(
                    "annotations exceed maximum pagination limit of 100 pages".to_owned(),
                ));
            }
            if start_time.elapsed() >= TIMEOUT {
                return Err(SourceError::Timeout(
                    "annotations pagination exceeded 5 s time budget".to_owned(),
                ));
            }

            let path = format!(
                "/api/users/0/items/{attachment_key}/children?itemType=annotation&format=json&limit=100&start={start}"
            );
            let (body, total_results) = self.get_with_total(&path)?;
            let page_marks = parse_marks(&body)?;

            let count = page_marks.len();
            for mark in page_marks {
                if seen_keys.insert(mark.key.clone()) {
                    all_marks.push(mark);
                }
            }
            page_count += 1;
            start += count;

            if count < 100 {
                break;
            }
            if let Some(total) = total_results {
                if start >= total {
                    break;
                }
            }
        }

        Ok(all_marks)
    }
}

fn text(data: &Value, field: &str) -> Option<String> {
    data.get(field)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
}

fn extract_attachment_key(href: &str) -> Option<&str> {
    let key = href.trim_end_matches('/').rsplit('/').next()?;
    if !key.is_empty() && key.bytes().all(|b| b.is_ascii_alphanumeric()) {
        Some(key)
    } else {
        None
    }
}

fn format_creators(creators: Option<&Value>) -> Option<String> {
    let arr = creators?.as_array()?;
    if arr.is_empty() {
        return None;
    }
    let first = arr.first()?;
    let last_name = first.get("lastName").and_then(Value::as_str)?;
    if arr.len() == 1 {
        Some(last_name.to_owned())
    } else {
        Some(format!("{last_name} et al."))
    }
}

fn extract_year(date: Option<&str>) -> Option<String> {
    let s = date?.trim();
    if s.len() >= 4 && s[..4].chars().all(|c| c.is_ascii_digit()) {
        Some(s[..4].to_owned())
    } else if !s.is_empty() {
        Some(s.to_owned())
    } else {
        None
    }
}

/// Parses top-level papers, links, and annotation counts from Zotero items JSON.
pub fn parse_papers(body: &str) -> Result<Vec<PaperItem>, SourceError> {
    let items: Vec<Value> =
        serde_json::from_str(body).map_err(|e| SourceError::BadAnswer(e.to_string()))?;
    parse_papers_from_items(&items)
}

/// Parses top-level papers, links, and annotation counts from a slice of item JSON values.
pub fn parse_papers_from_items(items: &[Value]) -> Result<Vec<PaperItem>, SourceError> {
    let mut annotation_counts: HashMap<String, usize> = HashMap::new();
    let mut paper_to_attachment: HashMap<String, String> = HashMap::new();
    let mut seen_annotation_keys: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    for item in items {
        let Some(data) = item.get("data") else {
            continue;
        };
        let item_type = text(data, "itemType").unwrap_or_default();
        let key = text(data, "key").unwrap_or_default();
        let parent = text(data, "parentItem");

        if item_type == "annotation" {
            if let Some(parent_key) = parent {
                if !key.is_empty() && seen_annotation_keys.insert(key) {
                    *annotation_counts.entry(parent_key).or_insert(0) += 1;
                }
            }
        } else if item_type == "attachment" {
            if let Some(paper_key) = parent {
                if !key.is_empty() {
                    paper_to_attachment.entry(paper_key).or_insert(key);
                }
            }
        }
    }

    let mut papers = Vec::new();
    let mut seen_paper_keys = std::collections::HashSet::new();

    for item in items {
        let Some(data) = item.get("data") else {
            continue;
        };
        let item_type = text(data, "itemType").unwrap_or_default();
        if item_type == "annotation" || item_type == "attachment" {
            continue;
        }
        let Some(key) = text(data, "key") else {
            continue;
        };
        if !seen_paper_keys.insert(key.clone()) {
            continue;
        }
        let title = text(data, "title").unwrap_or_else(|| key.clone());

        let attachment_key = item
            .get("links")
            .and_then(|l| l.get("attachment"))
            .and_then(|a| a.get("href"))
            .and_then(Value::as_str)
            .and_then(extract_attachment_key)
            .map(str::to_owned)
            .or_else(|| paper_to_attachment.get(&key).cloned());

        let authors = item
            .get("meta")
            .and_then(|m| m.get("creatorSummary"))
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .or_else(|| format_creators(data.get("creators")));

        let year = item
            .get("meta")
            .and_then(|m| m.get("parsedDate"))
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .or_else(|| extract_year(text(data, "date").as_deref()));

        let marks_count = attachment_key
            .as_ref()
            .and_then(|att| annotation_counts.get(att))
            .copied()
            .unwrap_or(0);

        papers.push(PaperItem {
            id: format!("z_{key}"),
            title,
            authors,
            year,
            marks_count,
            attachment_key,
        });
    }

    Ok(papers)
}

/// Parses annotation items from Zotero child items JSON.
pub fn parse_marks(body: &str) -> Result<Vec<Mark>, SourceError> {
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
    use super::*;

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

    #[test]
    fn allowed_path_validation() {
        assert!(validate_allowed_path("/connector/ping").is_ok());
        assert!(validate_allowed_path("/api/users/0/items").is_ok());
        assert!(validate_allowed_path("/api/users/0/items/top").is_ok());
        assert!(validate_allowed_path("/api/users/0/items/EFGH6789/children?limit=100").is_ok());
        assert!(validate_allowed_path("/api/users/0/items/H3E7BPJW/children").is_ok());

        // Negative cases
        assert!(validate_allowed_path("/admin").is_err());
        assert!(validate_allowed_path("/etc/passwd").is_err());
        assert!(validate_allowed_path("/api/users/0/items/../secret").is_err());
        assert!(validate_allowed_path("/api/users/0/collections").is_err());
    }

    #[test]
    fn parse_papers_extracts_metadata_and_mark_counts() {
        let json = r#"[
            {
                "key": "PAPER1",
                "links": {
                    "attachment": {
                        "href": "http://localhost:23119/api/users/0/items/ATT1"
                    }
                },
                "meta": {
                    "creatorSummary": "Alice et al.",
                    "parsedDate": "2024"
                },
                "data": {
                    "key": "PAPER1",
                    "itemType": "journalArticle",
                    "title": "Quantum Computing Advances"
                }
            },
            {
                "key": "ATT1",
                "data": {
                    "key": "ATT1",
                    "itemType": "attachment",
                    "parentItem": "PAPER1"
                }
            },
            {
                "key": "ANN1",
                "data": {
                    "key": "ANN1",
                    "itemType": "annotation",
                    "parentItem": "ATT1"
                }
            },
            {
                "key": "ANN2",
                "data": {
                    "key": "ANN2",
                    "itemType": "annotation",
                    "parentItem": "ATT1"
                }
            }
        ]"#;

        let papers = parse_papers(json).unwrap();
        assert_eq!(papers.len(), 1);
        assert_eq!(papers[0].id, "z_PAPER1");
        assert_eq!(papers[0].title, "Quantum Computing Advances");
        assert_eq!(papers[0].authors.as_deref(), Some("Alice et al."));
        assert_eq!(papers[0].year.as_deref(), Some("2024"));
        assert_eq!(papers[0].attachment_key.as_deref(), Some("ATT1"));
        assert_eq!(papers[0].marks_count, 2);
    }

    #[test]
    fn format_creators_and_extract_year_helpers() {
        assert_eq!(format_creators(None), None);
        let empty = serde_json::json!([]);
        assert_eq!(format_creators(Some(&empty)), None);

        let single = serde_json::json!([{"lastName": "Turing"}]);
        assert_eq!(format_creators(Some(&single)), Some("Turing".to_owned()));

        let multiple = serde_json::json!([{"lastName": "Turing"}, {"lastName": "Church"}]);
        assert_eq!(
            format_creators(Some(&multiple)),
            Some("Turing et al.".to_owned())
        );

        assert_eq!(extract_year(None), None);
        assert_eq!(extract_year(Some("")), None);
        assert_eq!(extract_year(Some("2023-05-01")), Some("2023".to_owned()));
        assert_eq!(
            extract_year(Some("circa 1999")),
            Some("circa 1999".to_owned())
        );
    }
}
