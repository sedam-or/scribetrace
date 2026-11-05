use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::application::{HistoryStore, LogWriter, ReadReport, StoreError};
use crate::domain::{
    classify_line, merge_events, sort_for_display, visible, DeviceId, Event, EventBody, LineClass,
    PaperId, Purge, MAX_LINE_BYTES,
};

/// Per-device JSONL files at `<workspace>/.scribetrace/papers/<paper-id>/<device-id>.jsonl`.
#[derive(Debug, Clone)]
pub struct FsHistoryStore {
    root: PathBuf,
}

enum LineRead {
    Valid(Vec<u8>),
    TooLarge,
    Eof,
}

fn read_line_bounded<R: BufRead>(reader: &mut R, max_bytes: usize) -> std::io::Result<LineRead> {
    let mut buf = Vec::new();
    let mut too_large = false;
    let max_with_newline = max_bytes.saturating_add(1);

    loop {
        let available = reader.fill_buf()?;
        if available.is_empty() {
            if buf.is_empty() {
                return Ok(LineRead::Eof);
            }
            if too_large || buf.len() > max_bytes {
                return Ok(LineRead::TooLarge);
            } else {
                return Ok(LineRead::Valid(buf));
            }
        }

        let newline_pos = available.iter().position(|&b| b == b'\n');
        let take_bytes = match newline_pos {
            Some(pos) => pos + 1,
            None => available.len(),
        };

        if !too_large {
            let space_remaining = max_with_newline.saturating_sub(buf.len());
            if take_bytes <= space_remaining {
                buf.extend_from_slice(&available[..take_bytes]);
                if newline_pos.is_some() {
                    let content_len = if buf.ends_with(b"\n") {
                        buf.len() - 1
                    } else {
                        buf.len()
                    };
                    if content_len > max_bytes {
                        too_large = true;
                        buf.clear();
                    }
                }
            } else {
                too_large = true;
                buf.clear();
            }
        }

        reader.consume(take_bytes);

        if newline_pos.is_some() {
            return if too_large {
                Ok(LineRead::TooLarge)
            } else {
                Ok(LineRead::Valid(buf))
            };
        }
    }
}

/// Result of scanning one history file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileScan {
    pub events: Vec<Event>,
    pub skipped_lines: usize,
    pub bad_lines: HashMap<String, usize>,
    pub other_lines: usize,
    pub last_event: Option<Event>,
    pub max_seq: u64,
    pub ends_with_newline: bool,
}

fn scan_file(path: &Path) -> Result<FileScan, StoreError> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(FileScan {
                events: Vec::new(),
                skipped_lines: 0,
                bad_lines: HashMap::new(),
                other_lines: 0,
                last_event: None,
                max_seq: 0,
                ends_with_newline: true,
            });
        }
        Err(e) => return Err(e.into()),
    };

    let expected_device = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

    let mut reader = BufReader::new(file);
    let mut events = Vec::new();
    let mut skipped_lines = 0;
    let mut bad_lines: HashMap<String, usize> = HashMap::new();
    let mut other_lines = 0;
    let mut last_event = None;
    let mut max_seq = 0;
    let mut ends_with_newline = true;

    loop {
        match read_line_bounded(&mut reader, MAX_LINE_BYTES)? {
            LineRead::Eof => break,
            LineRead::TooLarge => {
                skipped_lines += 1;
                *bad_lines.entry("too_long".to_string()).or_insert(0) += 1;
                ends_with_newline = true;
            }
            LineRead::Valid(mut bytes) => {
                if bytes.is_empty() {
                    continue;
                }
                ends_with_newline = bytes.ends_with(b"\n");
                while bytes.ends_with(b"\n") || bytes.ends_with(b"\r") {
                    bytes.pop();
                }
                if bytes.iter().all(|b| b.is_ascii_whitespace()) {
                    continue;
                }
                match classify_line(&bytes) {
                    LineClass::Valid(ev) => {
                        if ev.device.as_str() != expected_device {
                            // spec/FORMAT.md §2: device MUST equal the file name without .jsonl
                            skipped_lines += 1;
                            *bad_lines.entry("bad_value".to_string()).or_insert(0) += 1;
                            if ev.seq > max_seq {
                                max_seq = ev.seq;
                            }
                        } else {
                            if ev.seq > max_seq {
                                max_seq = ev.seq;
                            }
                            last_event = Some((*ev).clone());
                            events.push(*ev);
                        }
                    }
                    LineClass::Other { .. } => {
                        // Envelope is valid. Enforce device == file stem.
                        if let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(&bytes) {
                            let dev_str = map.get("device").and_then(|v| v.as_str()).unwrap_or("");
                            if dev_str != expected_device {
                                skipped_lines += 1;
                                *bad_lines.entry("bad_value".to_string()).or_insert(0) += 1;
                            } else {
                                other_lines += 1;
                            }
                            if let Some(s) = map.get("seq").and_then(|v| v.as_u64()) {
                                if s > max_seq {
                                    max_seq = s;
                                }
                            }
                        } else {
                            other_lines += 1;
                        }
                    }
                    LineClass::Bad(bad) => {
                        skipped_lines += 1;
                        *bad_lines.entry(bad.code().to_string()).or_insert(0) += 1;
                        // Lenient sequence recovery from corrupted line
                        if let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(&bytes) {
                            if let Some(s) = map.get("seq").and_then(|v| v.as_u64()) {
                                if s > max_seq {
                                    max_seq = s;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(FileScan {
        events,
        skipped_lines,
        bad_lines,
        other_lines,
        last_event,
        max_seq,
        ends_with_newline,
    })
}

impl FsHistoryStore {
    /// A store rooted at the project folder `workspace`.
    pub fn new(workspace: impl AsRef<Path>) -> Self {
        Self {
            root: workspace.as_ref().join(".scribetrace").join("papers"),
        }
    }

    /// Returns the directory path for `paper`.
    pub fn paper_dir(&self, paper: &PaperId) -> PathBuf {
        self.root.join(paper.as_str())
    }

    /// Returns the file path for `paper` and `device`.
    pub fn device_file(&self, paper: &PaperId, device: &DeviceId) -> PathBuf {
        self.paper_dir(paper)
            .join(format!("{}.jsonl", device.as_str()))
    }

    /// Inspects the device file for `paper` and `device`, returning the scan summary.
    pub fn inspect_device_file(
        &self,
        paper: &PaperId,
        device: &DeviceId,
    ) -> Result<FileScan, StoreError> {
        scan_file(&self.device_file(paper, device))
    }

    /// Appends an event using an injectable [`LogWriter`] implementation (Slice 2, R-10).
    pub fn append_with_writer<W: LogWriter>(
        &self,
        paper: &PaperId,
        event: &Event,
        writer: &mut W,
    ) -> Result<(), StoreError> {
        fs::create_dir_all(self.paper_dir(paper))?;

        let mut line =
            serde_json::to_string(event).map_err(|e| StoreError::Encode(e.to_string()))?;
        line.push('\n');

        writer.write_all(line.as_bytes())?;
        writer.sync_data()?;
        Ok(())
    }
}

impl HistoryStore for FsHistoryStore {
    fn next_seq(&self, paper: &PaperId, device: &DeviceId) -> Result<u64, StoreError> {
        let scan = self.inspect_device_file(paper, device)?;
        scan.max_seq
            .checked_add(1)
            .ok_or_else(|| StoreError::SequenceOverflow(device.clone()))
    }

    fn append(&self, paper: &PaperId, event: &Event) -> Result<(), StoreError> {
        fs::create_dir_all(self.paper_dir(paper))?;
        let target_file = self.device_file(paper, &event.device);

        // Check if existing file ended without a newline and repair it before appending
        if target_file.exists() {
            let scan = scan_file(&target_file)?;
            if !scan.ends_with_newline {
                let mut repair_file = OpenOptions::new().append(true).open(&target_file)?;
                repair_file.write_all(b"\n")?;
                repair_file.sync_data()?;
            }
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&target_file)?;

        self.append_with_writer(paper, event, &mut file)
    }

    fn read(&self, paper: &PaperId) -> Result<Vec<Event>, StoreError> {
        self.read_report(paper).map(|r| r.visible_events)
    }

    fn read_report(&self, paper: &PaperId) -> Result<ReadReport, StoreError> {
        let dir = self.paper_dir(paper);
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(ReadReport {
                    raw_events: Vec::new(),
                    merged_events: Vec::new(),
                    visible_events: Vec::new(),
                    skipped_lines: 0,
                    bad_lines: HashMap::new(),
                    other_lines: 0,
                    files_scanned: 0,
                    hidden_count: 0,
                });
            }
            Err(e) => return Err(e.into()),
        };

        let mut paths: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "jsonl"))
            .collect();
        paths.sort();

        let mut raw_events = Vec::new();
        let mut skipped_lines = 0;
        let mut bad_lines: HashMap<String, usize> = HashMap::new();
        let mut other_lines = 0;
        let mut files_scanned = 0;

        for path in paths {
            files_scanned += 1;
            let scan = scan_file(&path)?;
            raw_events.extend(scan.events);
            skipped_lines += scan.skipped_lines;
            other_lines += scan.other_lines;
            for (code, count) in scan.bad_lines {
                *bad_lines.entry(code).or_insert(0) += count;
            }
        }

        let merged_events = merge_events(raw_events.clone());

        let purges: Vec<Purge> = merged_events
            .iter()
            .filter_map(|ev| match &ev.body {
                EventBody::Purge(p) => Some(Purge::new(
                    ev.id.clone(),
                    ev.device.clone(),
                    ev.seq,
                    ev.ts.clone(),
                    p.covers.clone(),
                )),
                _ => None,
            })
            .collect();

        let vis = visible(&merged_events, &purges);
        let visible_events = sort_for_display(&vis.visible);

        Ok(ReadReport {
            raw_events,
            merged_events,
            visible_events,
            skipped_lines,
            bad_lines,
            other_lines,
            files_scanned,
            hidden_count: vis.hidden_count,
        })
    }
}
