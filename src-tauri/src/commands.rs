use std::path::PathBuf;
use std::sync::Mutex;

use scribetrace_core::adapters::{
    FsDeviceStore, FsHistoryStore, SystemClock, UuidGenerator, ZoteroHttp,
};
use scribetrace_core::application::{DeviceStore, HistoryStore, ImportMarks, ImportReport};
use scribetrace_core::domain::{Event, PaperId};
use serde::Serialize;

/// The chosen project folder, app storage directory, and Zotero port; the only state the shell keeps.
#[derive(Debug)]
pub struct Session {
    workspace: Mutex<Option<PathBuf>>,
    app_storage_dir: Option<PathBuf>,
    zotero_port: u16,
}

impl Session {
    /// A session with no folder chosen yet and default app storage.
    pub fn new(zotero_port: u16) -> Self {
        Self {
            workspace: Mutex::new(None),
            app_storage_dir: None,
            zotero_port,
        }
    }

    /// A session with explicit app storage directory (used in integration testing).
    pub fn with_app_storage(zotero_port: u16, app_storage_dir: PathBuf) -> Self {
        Self {
            workspace: Mutex::new(None),
            app_storage_dir: Some(app_storage_dir),
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

    let app_dir = session
        .app_storage_dir
        .clone()
        .unwrap_or_else(FsDeviceStore::default_app_dir);
    let mut dev_store = FsDeviceStore::new(app_dir).map_err(err)?;
    let store = FsHistoryStore::new(&workspace);

    let current_id = dev_store.get_or_create_device_id().map_err(err)?;
    let scan = store
        .inspect_device_file(&paper, &current_id)
        .map_err(err)?;
    let device = dev_store
        .ensure_safe_device(&paper, scan.last_event.as_ref())
        .map_err(err)?;

    let zotero = ZoteroHttp::new(session.zotero_port);
    let report = ImportMarks::new(
        &zotero,
        &store,
        &SystemClock,
        &UuidGenerator,
        device.clone(),
    )
    .run(&paper, &attachment_key)
    .map_err(err)?;

    let scan_after = store.inspect_device_file(&paper, &device).map_err(err)?;
    if let Some(last_ev) = scan_after.last_event {
        dev_store
            .record_last_event(&paper, last_ev.id)
            .map_err(err)?;
    }

    Ok(report)
}

/// Reads the history of one paper across all devices.
#[tauri::command]
pub fn read_history(
    session: tauri::State<'_, Session>,
    paper_id: String,
) -> Result<Vec<Event>, String> {
    let workspace = session.workspace()?;
    let paper = PaperId::new(paper_id).map_err(err)?;
    FsHistoryStore::new(workspace).read(&paper).map_err(err)
}
