use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use crate::application::{HistoryStore, StoreError};
use crate::domain::{DeviceId, Event, PaperId};

/// Per-device JSONL files at `<workspace>/.scribetrace/papers/<paper-id>/<device-id>.jsonl`.
#[derive(Debug, Clone)]
pub struct FsHistoryStore {
    root: PathBuf,
}

impl FsHistoryStore {
    /// A store rooted at the project folder `workspace`.
    pub fn new(workspace: impl AsRef<Path>) -> Self {
        Self {
            root: workspace.as_ref().join(".scribetrace").join("papers"),
        }
    }

    fn paper_dir(&self, paper: &PaperId) -> PathBuf {
        self.root.join(paper.as_str())
    }

    fn device_file(&self, paper: &PaperId, device: &DeviceId) -> PathBuf {
        self.paper_dir(paper)
            .join(format!("{}.jsonl", device.as_str()))
    }
}

fn read_events(path: &Path) -> Result<Vec<Event>, StoreError> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };
    let mut events = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        // Slice 1 skips lines it cannot parse; slice 3 counts and reports them.
        if let Ok(event) = serde_json::from_str::<Event>(&line) {
            events.push(event);
        }
    }
    Ok(events)
}

impl HistoryStore for FsHistoryStore {
    fn next_seq(&self, paper: &PaperId, device: &DeviceId) -> Result<u64, StoreError> {
        let events = read_events(&self.device_file(paper, device))?;
        Ok(events.iter().map(|e| e.seq).max().unwrap_or(0) + 1)
    }

    fn append(&self, paper: &PaperId, event: &Event) -> Result<(), StoreError> {
        fs::create_dir_all(self.paper_dir(paper))?;
        let mut line =
            serde_json::to_string(event).map_err(|e| StoreError::Encode(e.to_string()))?;
        line.push('\n');
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.device_file(paper, &event.device))?;
        // One write call per line keeps a line whole; slice 2 proves crash behaviour.
        file.write_all(line.as_bytes())?;
        file.sync_data()?;
        Ok(())
    }

    fn read(&self, paper: &PaperId) -> Result<Vec<Event>, StoreError> {
        let dir = self.paper_dir(paper);
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e.into()),
        };
        let mut paths: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "jsonl"))
            .collect();
        paths.sort();
        let mut all = Vec::new();
        for path in paths {
            all.extend(read_events(&path)?);
        }
        all.sort_by(|a, b| (&a.ts, &a.device, a.seq).cmp(&(&b.ts, &b.device, b.seq)));
        Ok(all)
    }
}
