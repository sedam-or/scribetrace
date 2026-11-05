use crate::domain::{merge_events, Event, MAX_LINE_BYTES};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid paper ID: {0}")]
    InvalidPaperId(String),
}

/// Statistics and output from reading paper events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadResult {
    pub raw_events: Vec<Event>,
    pub merged_events: Vec<Event>,
    pub skipped_lines: usize,
    pub files_scanned: usize,
}

/// Device state stored locally in app storage (not synced across machines).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceState {
    pub device_id: String,
    #[serde(default)]
    pub papers: HashMap<String, PaperDeviceState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaperDeviceState {
    pub last_id: String,
}

impl DeviceState {
    pub fn new_random() -> Self {
        Self {
            device_id: Uuid::new_v4().to_string(),
            papers: HashMap::new(),
        }
    }
}

/// Validates paper-id per ADR 0007: `z_<itemKey>` or `h_<hash>`, characters `[A-Za-z0-9_-]`.
pub fn validate_paper_id(paper_id: &str) -> Result<(), StorageError> {
    if paper_id.is_empty() {
        return Err(StorageError::InvalidPaperId(
            "Paper ID cannot be empty".to_string(),
        ));
    }
    if !(paper_id.starts_with("z_") || paper_id.starts_with("h_")) {
        return Err(StorageError::InvalidPaperId(format!(
            "Paper ID must start with 'z_' or 'h_': '{paper_id}'"
        )));
    }
    for c in paper_id.chars() {
        if !c.is_ascii_alphanumeric() && c != '_' && c != '-' {
            return Err(StorageError::InvalidPaperId(format!(
                "Invalid character '{c}' in paper ID: '{paper_id}'"
            )));
        }
    }
    Ok(())
}

/// Abstraction for reading paper logs (Domain / Clean Architecture port).
pub trait LogReader {
    fn read_paper_events(&self, paper_id: &str) -> Result<ReadResult, StorageError>;
}

/// Abstraction for appending paper logs (Domain / Clean Architecture port).
pub trait LogWriter {
    fn append_event(
        &mut self,
        paper_id: &str,
        event_type: &str,
        payload: serde_json::Value,
        ts: &str,
    ) -> Result<Event, StorageError>;
}

/// Filesystem adapter for ScribeTrace per-device append-only JSONL storage.
pub struct FileLogStorage {
    base_dir: PathBuf,
    app_storage_dir: PathBuf,
    device_state: DeviceState,
}

impl FileLogStorage {
    /// Initializes or loads storage from given paths.
    pub fn init(
        base_dir: impl Into<PathBuf>,
        app_storage_dir: impl Into<PathBuf>,
    ) -> Result<Self, StorageError> {
        let base_dir = base_dir.into();
        let app_storage_dir = app_storage_dir.into();

        fs::create_dir_all(&base_dir)?;
        fs::create_dir_all(&app_storage_dir)?;

        let state_path = app_storage_dir.join("device.json");
        let device_state = if state_path.exists() {
            let data = fs::read(&state_path)?;
            match serde_json::from_slice::<DeviceState>(&data) {
                Ok(state) => state,
                Err(_) => {
                    // Corrupted state file -> mint new device ID
                    let fresh = DeviceState::new_random();
                    let bytes = serde_json::to_vec_pretty(&fresh)?;
                    fs::write(&state_path, bytes)?;
                    fresh
                }
            }
        } else {
            let fresh = DeviceState::new_random();
            let bytes = serde_json::to_vec_pretty(&fresh)?;
            fs::write(&state_path, bytes)?;
            fresh
        };

        Ok(Self {
            base_dir,
            app_storage_dir,
            device_state,
        })
    }

    pub fn current_device_id(&self) -> &str {
        &self.device_state.device_id
    }

    pub fn device_state(&self) -> &DeviceState {
        &self.device_state
    }

    fn paper_dir(&self, paper_id: &str) -> PathBuf {
        self.base_dir.join("papers").join(paper_id)
    }

    fn device_log_path(&self, paper_id: &str, device_id: &str) -> PathBuf {
        self.paper_dir(paper_id).join(format!("{device_id}.jsonl"))
    }

    fn persist_device_state(&self) -> Result<(), StorageError> {
        let state_path = self.app_storage_dir.join("device.json");
        let bytes = serde_json::to_vec_pretty(&self.device_state)?;
        // Write atomic via temp file
        let tmp_path = self.app_storage_dir.join("device.json.tmp");
        fs::write(&tmp_path, bytes)?;
        fs::rename(tmp_path, state_path)?;
        Ok(())
    }

    /// Reads the last valid event and seq from a device file, also checking for unterminated trailing line.
    fn inspect_last_event(file_path: &Path) -> Result<(Option<Event>, u64, bool), StorageError> {
        if !file_path.exists() {
            return Ok((None, 0, true));
        }

        let mut file = File::open(file_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        if content.is_empty() {
            return Ok((None, 0, true));
        }

        let ends_with_newline = content.last() == Some(&b'\n');

        // Parse lines in reverse to find the last valid event
        let mut last_event = None;
        let mut max_seq = 0;

        for line in content.split(|&b| b == b'\n') {
            if line.is_empty() {
                continue;
            }
            if line.len() > MAX_LINE_BYTES {
                continue;
            }
            if let Ok(event) = serde_json::from_slice::<Event>(line) {
                if event.seq > max_seq {
                    max_seq = event.seq;
                }
                last_event = Some(event);
            }
        }

        Ok((last_event, max_seq, ends_with_newline))
    }

    /// Performs the collision check before appending.
    /// If there is any mismatch (cloned machine, restored backup, crash before state save, missing file),
    /// mints a brand new device ID and leaves the old file untouched.
    fn ensure_safe_device_for_append(
        &mut self,
        paper_id: &str,
    ) -> Result<(String, u64), StorageError> {
        let current_dev = self.device_state.device_id.clone();
        let log_path = self.device_log_path(paper_id, &current_dev);

        let (last_event_opt, max_seq, ends_with_newline) = Self::inspect_last_event(&log_path)?;

        let recorded_state = self.device_state.papers.get(paper_id);

        let needs_new_device = match (&last_event_opt, recorded_state) {
            (None, None) => false, // Brand new paper on this device
            (Some(last_ev), Some(rec)) => {
                // Both exist: check if last_id matches exactly
                last_ev.id != rec.last_id
            }
            (Some(_), None) => {
                // File has events, but device.json has no record (e.g. restored backup or clone)
                true
            }
            (None, Some(_)) => {
                // device.json expects a last_id, but file is missing or empty (file deleted or cloned state)
                true
            }
        };

        if needs_new_device {
            // Mint new device id
            let new_dev = Uuid::new_v4().to_string();
            self.device_state.device_id = new_dev.clone();
            // The paper starts clean for this new device
            self.device_state.papers.remove(paper_id);
            self.persist_device_state()?;
            return Ok((new_dev, 0));
        }

        // If file didn't end with newline, repair unterminated line before appending
        if !ends_with_newline && log_path.exists() {
            let mut file = OpenOptions::new().append(true).open(&log_path)?;
            file.write_all(b"\n")?;
        }

        Ok((current_dev, max_seq))
    }
}

impl LogReader for FileLogStorage {
    fn read_paper_events(&self, paper_id: &str) -> Result<ReadResult, StorageError> {
        validate_paper_id(paper_id)?;

        let paper_dir = self.paper_dir(paper_id);
        if !paper_dir.exists() {
            return Ok(ReadResult {
                raw_events: Vec::new(),
                merged_events: Vec::new(),
                skipped_lines: 0,
                files_scanned: 0,
            });
        }

        let mut raw_events = Vec::new();
        let mut skipped_lines = 0;
        let mut files_scanned = 0;

        for entry in fs::read_dir(&paper_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                files_scanned += 1;
                // Reading opens STRICTLY read-only
                let file = File::open(&path)?;
                let reader = BufReader::new(file);

                for line_result in reader.lines() {
                    match line_result {
                        Ok(line) => {
                            let trimmed = line.trim();
                            if trimmed.is_empty() {
                                continue;
                            }
                            if trimmed.len() > MAX_LINE_BYTES {
                                skipped_lines += 1;
                                continue;
                            }
                            match serde_json::from_str::<Event>(trimmed) {
                                Ok(event) => {
                                    if event.seq == 0 {
                                        skipped_lines += 1;
                                    } else {
                                        raw_events.push(event);
                                    }
                                }
                                Err(_) => skipped_lines += 1,
                            }
                        }
                        Err(_) => {
                            skipped_lines += 1;
                        }
                    }
                }
            }
        }

        let merged_events = merge_events(raw_events.clone());

        Ok(ReadResult {
            raw_events,
            merged_events,
            skipped_lines,
            files_scanned,
        })
    }
}

impl LogWriter for FileLogStorage {
    fn append_event(
        &mut self,
        paper_id: &str,
        event_type: &str,
        payload: serde_json::Value,
        ts: &str,
    ) -> Result<Event, StorageError> {
        validate_paper_id(paper_id)?;

        let (device_id, last_seq) = self.ensure_safe_device_for_append(paper_id)?;
        let next_seq = last_seq + 1;
        let event_id = Uuid::new_v4().to_string();

        let event = Event::new(
            1,
            event_id.clone(),
            device_id.clone(),
            next_seq,
            ts,
            crate::domain::DEFAULT_AUTHOR,
            event_type,
            payload,
        );

        let paper_dir = self.paper_dir(paper_id);
        fs::create_dir_all(&paper_dir)?;
        let log_path = self.device_log_path(paper_id, &device_id);

        let mut line_bytes = serde_json::to_vec(&event)?;
        line_bytes.push(b'\n');

        // Append-only write
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;
        file.write_all(&line_bytes)?;
        file.sync_data()?;

        // Update local device state
        self.device_state
            .papers
            .insert(paper_id.to_string(), PaperDeviceState { last_id: event_id });
        self.persist_device_state()?;

        Ok(event)
    }
}
