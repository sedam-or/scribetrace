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

/// A running stand-in server. Dropping it stops the server.
#[derive(Debug)]
pub struct StandIn {
    addr: SocketAddr,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl StandIn {
    /// Starts serving `dir` on `127.0.0.1:port` (port 0 picks a free one).
    pub fn start(dir: impl Into<PathBuf>, port: u16) -> std::io::Result<Self> {
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
                        let _ = handle_connection(stream, &dir);
                    }
                    Err(_) => thread::sleep(std::time::Duration::from_millis(5)),
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

fn handle_connection(stream: TcpStream, dir: &Path) -> std::io::Result<()> {
    stream.set_nonblocking(false)?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    // Drain the headers.
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 || line == "\r\n" || line == "\n" {
            break;
        }
    }
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");
    let (status, body) = if method != "GET" {
        (405, String::from("{\"error\":\"method not allowed\"}"))
    } else {
        match fixture_for(dir, target) {
            Some(body) => (200, body),
            None => (404, String::from("{\"error\":\"no fixture\"}")),
        }
    };
    let mut out = stream;
    write!(
        out,
        "HTTP/1.1 {status} X\r\nZotero-API-Version: 3\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )?;
    out.flush()?;
    out.shutdown(Shutdown::Both)
}

fn fixture_for(dir: &Path, target: &str) -> Option<String> {
    let path = target.split('?').next()?;
    if path.split('/').any(|seg| seg == ".." || seg.contains('\\')) {
        return None;
    }
    let file = dir.join(format!("{}.json", path.trim_start_matches('/')));
    fs::read_to_string(file).ok()
}
