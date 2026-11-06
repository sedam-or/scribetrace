//! Stand-in for Zotero's local interface (127.0.0.1:23119), built on `std` only.
//!
//! It serves recorded fixtures from a directory that mirrors the URL path:
//! `GET /api/users/0/items/AAAA1111/children?x=y` is answered from
//! `<dir>/api/users/0/items/AAAA1111/children.json`. Every response carries
//! `Zotero-API-Version: 3`. Anything that is not a GET, or has no fixture, gets an error status.

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Stand-in operational modes for testing broker refusal cases (SEC-33).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StandInMode {
    /// Normal fixture serving (HTTP 200, Zotero-API-Version: 3).
    #[default]
    Normal,
    /// Simulates HTTP 302 redirect.
    Redirect,
    /// Simulates request timeout (>5 s delay).
    Timeout,
    /// Simulates response payload exceeding 15 MB limit via Content-Length.
    TooLarge,
    /// Simulates response payload exceeding 15 MB limit via streaming body without Content-Length.
    TooLargeChunked,
    /// Simulates API version mismatch header ("2").
    WrongVersion,
    /// Simulates HTTP 403 Forbidden ("Local API is not enabled").
    Forbidden,
    /// A broken server that ignores `start` and returns the first page forever.
    IgnoreStart,
}

/// A running stand-in server. Dropping it stops the server.
#[derive(Debug)]
pub struct StandIn {
    addr: SocketAddr,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl StandIn {
    /// Starts serving `dir` on `127.0.0.1:port` in Normal mode (port 0 picks a free one).
    pub fn start(dir: impl Into<PathBuf>, port: u16) -> std::io::Result<Self> {
        Self::start_mode(dir, port, StandInMode::Normal)
    }

    /// Starts serving with an explicit [`StandInMode`].
    pub fn start_mode(
        dir: impl Into<PathBuf>,
        port: u16,
        mode: StandInMode,
    ) -> std::io::Result<Self> {
        let dir: PathBuf = dir.into();
        let listener = TcpListener::bind(("127.0.0.1", port))?;
        let addr = listener.local_addr()?;
        listener.set_nonblocking(true)?;
        let stop = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&stop);
        let handle = thread::spawn(move || {
            while !flag.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        let _ = handle_connection(stream, &dir, mode);
                    }
                    Err(_) => thread::sleep(Duration::from_millis(5)),
                }
            }
        });
        Ok(Self {
            addr,
            stop,
            handle: Some(handle),
        })
    }

    /// The port the server listens on.
    pub fn port(&self) -> u16 {
        self.addr.port()
    }
}

impl Drop for StandIn {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn handle_connection(stream: TcpStream, dir: &Path, mode: StandInMode) -> std::io::Result<()> {
    stream.set_nonblocking(false)?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    // Drain headers.
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 || line == "\r\n" || line == "\n" {
            break;
        }
    }

    match mode {
        StandInMode::Timeout => {
            thread::sleep(Duration::from_secs(10));
            return Ok(());
        }
        StandInMode::Redirect => {
            let mut out = stream;
            write!(
                out,
                "HTTP/1.1 302 Found\r\nLocation: /api/users/0/items\r\nConnection: close\r\nContent-Length: 0\r\n\r\n"
            )?;
            out.flush()?;
            return out.shutdown(Shutdown::Both);
        }
        StandInMode::Forbidden => {
            let mut out = stream;
            let body = "Local API is not enabled";
            write!(
                out,
                "HTTP/1.1 403 Forbidden\r\nZotero-API-Version: 3\r\nContent-Type: text/plain\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}",
                body.len()
            )?;
            out.flush()?;
            return out.shutdown(Shutdown::Both);
        }
        StandInMode::WrongVersion => {
            let mut out = stream;
            let body = "[]";
            write!(
                out,
                "HTTP/1.1 200 OK\r\nZotero-API-Version: 2\r\nContent-Type: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}",
                body.len()
            )?;
            out.flush()?;
            return out.shutdown(Shutdown::Both);
        }
        StandInMode::TooLarge => {
            let mut out = stream;
            // 16 MB oversized payload (>15 MB limit) with Content-Length
            let size = 16 * 1024 * 1024;
            write!(
                out,
                "HTTP/1.1 200 OK\r\nZotero-API-Version: 3\r\nContent-Type: application/json\r\nConnection: close\r\nContent-Length: {size}\r\n\r\n"
            )?;
            let chunk = vec![b' '; 64 * 1024];
            for _ in 0..(size / chunk.len()) {
                out.write_all(&chunk)?;
            }
            out.flush()?;
            return out.shutdown(Shutdown::Both);
        }
        StandInMode::TooLargeChunked => {
            let mut out = stream;
            // Oversized payload (>15 MB limit) without Content-Length
            let size = 16 * 1024 * 1024;
            write!(
                out,
                "HTTP/1.1 200 OK\r\nZotero-API-Version: 3\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n"
            )?;
            let chunk = vec![b' '; 64 * 1024];
            for _ in 0..(size / chunk.len()) {
                out.write_all(&chunk)?;
            }
            out.flush()?;
            return out.shutdown(Shutdown::Both);
        }
        StandInMode::Normal | StandInMode::IgnoreStart => {}
    }

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");
    let (status, body, content_type, total_results) = if method != "GET" {
        (
            405,
            String::from("{\"error\":\"method not allowed\"}"),
            "application/json",
            None,
        )
    } else {
        match fixture_for(dir, target) {
            Some((content, ct)) => {
                if ct == "application/json" && target.contains('?') {
                    let (paginated, total) = paginate_json_if_requested(&content, target, mode);
                    (200, paginated, ct, total)
                } else {
                    (200, content, ct, None)
                }
            }
            None => (
                404,
                String::from("{\"error\":\"no fixture\"}"),
                "application/json",
                None,
            ),
        }
    };
    let mut out = stream;
    let total_header = match total_results {
        Some(total) => format!("Total-Results: {total}\r\n"),
        None => String::new(),
    };
    write!(
        out,
        "HTTP/1.1 {status} OK\r\nZotero-API-Version: 3\r\nContent-Type: {content_type}\r\n{total_header}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )?;
    out.flush()?;
    out.shutdown(Shutdown::Both)
}

fn parse_query_param(target: &str, param: &str) -> Option<usize> {
    let query = target.split('?').nth(1)?;
    for pair in query.split('&') {
        let mut kv = pair.split('=');
        if kv.next() == Some(param) {
            return kv.next().and_then(|v| v.parse::<usize>().ok());
        }
    }
    None
}

fn paginate_json_if_requested(
    body: &str,
    target: &str,
    mode: StandInMode,
) -> (String, Option<usize>) {
    let Ok(items) = serde_json::from_str::<Vec<serde_json::Value>>(body) else {
        return (body.to_owned(), None);
    };

    let total = items.len();
    let start = if mode == StandInMode::IgnoreStart {
        0
    } else {
        parse_query_param(target, "start").unwrap_or(0)
    };
    let limit = parse_query_param(target, "limit").unwrap_or(25);

    let end = std::cmp::min(total, start.saturating_add(limit));
    let slice = if start < total {
        &items[start..end]
    } else {
        &[]
    };
    let paginated_body = match serde_json::to_string(slice) {
        Ok(s) => s,
        Err(_) => return (body.to_owned(), Some(total)),
    };
    // The broken IgnoreStart server also omits Total-Results, so only the page cap can stop a client.
    let total_header = (mode != StandInMode::IgnoreStart).then_some(total);
    (paginated_body, total_header)
}

fn fixture_for(dir: &Path, target: &str) -> Option<(String, &'static str)> {
    let path = target.split('?').next()?;
    if path.split('/').any(|seg| seg == ".." || seg.contains('\\')) {
        return None;
    }
    let trimmed = path.trim_start_matches('/');

    // 1. Direct mirror file: <dir>/<path>.json
    let json_file = dir.join(format!("{trimmed}.json"));
    if let Ok(content) = fs::read_to_string(json_file) {
        return Some((content, "application/json"));
    }

    // 2. Direct mirror file: <dir>/<path>.html
    let html_file = dir.join(format!("{trimmed}.html"));
    if let Ok(content) = fs::read_to_string(html_file) {
        return Some((content, "text/html"));
    }

    // 3. Fallback mappings for standard recorded test fixtures
    match path {
        "/connector/ping" => fs::read_to_string(dir.join("connector_ping.html"))
            .ok()
            .map(|c| (c, "text/html")),
        "/api/users/0/items" => fs::read_to_string(dir.join("items_all.json"))
            .ok()
            .map(|c| (c, "application/json")),
        "/api/users/0/items/top" => fs::read_to_string(dir.join("items_top.json"))
            .ok()
            .map(|c| (c, "application/json")),
        "/api/users/0/items/H3E7BPJW/children" => {
            fs::read_to_string(dir.join("annotations_vaswani_attachment.json"))
                .ok()
                .map(|c| (c, "application/json"))
        }
        "/api/users/0/items/45Y8JGAU/children" => {
            fs::read_to_string(dir.join("annotations_he_attachment.json"))
                .ok()
                .map(|c| (c, "application/json"))
        }
        "/api/users/0/items/RNBV2SQM/children" => {
            fs::read_to_string(dir.join("annotations_brown_attachment.json"))
                .ok()
                .map(|c| (c, "application/json"))
        }
        _ => None,
    }
}
