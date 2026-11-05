use std::path::PathBuf;
use std::sync::Mutex;

use scribetrace_core::adapters::{FsHistoryStore, SystemClock, UuidGenerator, ZoteroHttp};
use scribetrace_core::application::{HistoryStore, ImportMarks, ImportReport};
use scribetrace_core::domain::{DeviceId, Event, PaperId};
use serde::Serialize;

/// Device id used until slice 2 mints and stores a real one per machine.
const SLICE1_DEVICE: &str = "local-1";

/// The chosen project folder and the Zotero port; the only state the shell keeps.
#[derive(Debug)]
pub struct Session {
    workspace: Mutex<Option<PathBuf>>,
    zotero_port: u16,
}

impl Session {
    /// A session with no folder chosen yet.
    pub fn new(zotero_port: u16) -> Self {
        Self {
            workspace: Mutex::new(None),
            zotero_port,
        }
    }

    fn workspace(&self) -> Result<PathBuf, String> {
        self.workspace
            .lock()
            .map_err(|_| "internal error: state poisoned".to_owned())?
            .clone()
            .ok_or_else(|| "No project folder is open.".to_owned())
    }
}

/// Answer of `open_workspace`.
#[derive(Debug, Clone, Serialize)]
pub struct WorkspaceInfo {
    /// Canonical path of the folder.
    pub path: String,
}

fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}

/// Opens a project folder. Must be an existing directory; stored in canonical form.
#[tauri::command]
pub fn open_workspace(
    session: tauri::State<'_, Session>,
    path: String,
) -> Result<WorkspaceInfo, String> {
    let canonical = std::fs::canonicalize(&path).map_err(err)?;
    if !canonical.is_dir() {
        return Err("That path is not a folder.".to_owned());
    }
    let info = WorkspaceInfo {
        path: canonical.to_string_lossy().into_owned(),
    };
    *session
        .workspace
        .lock()
        .map_err(|_| "internal error".to_owned())? = Some(canonical);
    Ok(info)
}

/// Imports the marks of one paper from Zotero. Runs only when the user clicks.
#[tauri::command]
pub fn import_marks(
    session: tauri::State<'_, Session>,
    paper_id: String,
    attachment_key: String,
) -> Result<ImportReport, String> {
    let workspace = session.workspace()?;
    let paper = PaperId::new(paper_id).map_err(err)?;
    let device = DeviceId::new(SLICE1_DEVICE).map_err(err)?;
    let zotero = ZoteroHttp::new(session.zotero_port);
    let store = FsHistoryStore::new(workspace);
    ImportMarks::new(&zotero, &store, &SystemClock, &UuidGenerator, device)
        .run(&paper, &attachment_key)
        .map_err(err)
}

/// Reads the history of one paper.
#[tauri::command]
pub fn read_history(
    session: tauri::State<'_, Session>,
    paper_id: String,
) -> Result<Vec<Event>, String> {
    let workspace = session.workspace()?;
    let paper = PaperId::new(paper_id).map_err(err)?;
    FsHistoryStore::new(workspace).read(&paper).map_err(err)
}
