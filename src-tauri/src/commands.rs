use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use scribetrace_core::adapters::{
    FsDeviceStore, FsHistoryStore, SystemClock, UuidGenerator, ZoteroHttp,
};
use scribetrace_core::application::{
    AppError, DeviceStore, HistoryStore, ImportMarks, ImportReport, PaperItem, SourceError,
    SourceErrorCode, StoreError, ZoteroSource,
};
use scribetrace_core::domain::{DomainError, Event, PaperId};
use serde::{Deserialize, Serialize};

/// Port for folder selection. Production shell uses native OS dialog (rfd);
/// integration tests inject a deterministic picker without UI.
pub trait FolderPicker: Send + Sync {
    fn pick_folder<'a>(&'a self) -> Pin<Box<dyn Future<Output = Option<PathBuf>> + Send + 'a>>;
}

/// Native OS folder dialog picker using the `rfd` crate asynchronously on the main loop.
#[derive(Debug)]
pub struct RfdFolderPicker;

impl FolderPicker for RfdFolderPicker {
    fn pick_folder<'a>(&'a self) -> Pin<Box<dyn Future<Output = Option<PathBuf>> + Send + 'a>> {
        Box::pin(async move {
            let handle = rfd::AsyncFileDialog::new().pick_folder().await;
            handle.map(|h| h.path().to_path_buf())
        })
    }
}

/// The chosen project folder, app storage directory, and Zotero port; the only state the shell keeps.
pub struct Session {
    workspace: Mutex<Option<PathBuf>>,
    app_storage_dir: Option<PathBuf>,
    zotero_port: u16,
    picker: Arc<dyn FolderPicker>,
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session")
            .field("workspace", &self.workspace)
            .field("app_storage_dir", &self.app_storage_dir)
            .field("zotero_port", &self.zotero_port)
            .finish()
    }
}

impl Session {
    /// A new session with the given loopback port.
    pub fn new(zotero_port: u16) -> Self {
        Self {
            workspace: Mutex::new(None),
            app_storage_dir: None,
            zotero_port,
            picker: Arc::new(RfdFolderPicker),
        }
    }

    /// A session with explicit app storage directory (used for tests).
    pub fn with_app_storage(zotero_port: u16, app_storage_dir: PathBuf) -> Self {
        Self {
            workspace: Mutex::new(None),
            app_storage_dir: Some(app_storage_dir),
            zotero_port,
            picker: Arc::new(RfdFolderPicker),
        }
    }

    /// A session with an injected folder picker (used for integration tests).
    pub fn with_picker(zotero_port: u16, picker: Arc<dyn FolderPicker>) -> Self {
        Self {
            workspace: Mutex::new(None),
            app_storage_dir: None,
            zotero_port,
            picker,
        }
    }

    /// A session with explicit app storage directory and custom picker.
    pub fn with_app_storage_and_picker(
        zotero_port: u16,
        app_storage_dir: PathBuf,
        picker: Arc<dyn FolderPicker>,
    ) -> Self {
        Self {
            workspace: Mutex::new(None),
            app_storage_dir: Some(app_storage_dir),
            zotero_port,
            picker,
        }
    }

    fn workspace(&self) -> Result<PathBuf, CommandError> {
        self.workspace
            .lock()
            .map_err(|_| CommandError::new(SourceErrorCode::Internal))?
            .clone()
            .ok_or_else(|| CommandError::new(SourceErrorCode::Internal))
    }
}

/// Answer of `choose_folder` and `get_workspace`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    /// Canonical path of the folder.
    pub path: String,
}

/// Typed error returned across the IPC seam (SEC-39).
/// Contains only the machine-readable error code; details are recorded to local process logs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandError {
    pub code: SourceErrorCode,
}

impl CommandError {
    pub fn new(code: SourceErrorCode) -> Self {
        Self { code }
    }
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code.as_str())
    }
}

impl std::error::Error for CommandError {}

impl From<SourceErrorCode> for CommandError {
    fn from(code: SourceErrorCode) -> Self {
        Self { code }
    }
}

impl From<SourceError> for CommandError {
    fn from(e: SourceError) -> Self {
        eprintln!("Zotero source error: {e}");
        Self { code: e.code() }
    }
}

impl From<AppError> for CommandError {
    fn from(e: AppError) -> Self {
        eprintln!("App error: {e}");
        Self { code: e.code() }
    }
}

impl From<StoreError> for CommandError {
    fn from(e: StoreError) -> Self {
        eprintln!("Store error: {e}");
        Self {
            code: SourceErrorCode::Internal,
        }
    }
}

impl From<DomainError> for CommandError {
    fn from(e: DomainError) -> Self {
        eprintln!("Domain error: {e}");
        Self {
            code: SourceErrorCode::Internal,
        }
    }
}

impl From<std::io::Error> for CommandError {
    fn from(e: std::io::Error) -> Self {
        eprintln!("IO error: {e}");
        Self {
            code: SourceErrorCode::Internal,
        }
    }
}

/// Opens the native OS folder chooser dialog. Confined by design: the UI never supplies a path (SEC-18).
#[tauri::command]
pub async fn choose_folder(
    session: tauri::State<'_, Session>,
) -> Result<Option<WorkspaceInfo>, CommandError> {
    let picked = match session.picker.pick_folder().await {
        Some(p) => p,
        None => return Ok(None),
    };
    let canonical = std::fs::canonicalize(&picked).map_err(|e| {
        eprintln!("canonicalize error: {e}");
        CommandError::new(SourceErrorCode::Internal)
    })?;
    if !canonical.is_dir() {
        return Err(CommandError::new(SourceErrorCode::Internal));
    }
    let info = WorkspaceInfo {
        path: canonical.to_string_lossy().into_owned(),
    };
    *session
        .workspace
        .lock()
        .map_err(|_| CommandError::new(SourceErrorCode::Internal))? = Some(canonical);
    Ok(Some(info))
}

/// Returns the currently active workspace info, if any.
#[tauri::command]
pub fn get_workspace(
    session: tauri::State<'_, Session>,
) -> Result<Option<WorkspaceInfo>, CommandError> {
    let lock = session
        .workspace
        .lock()
        .map_err(|_| CommandError::new(SourceErrorCode::Internal))?;
    Ok(lock.as_ref().map(|p| WorkspaceInfo {
        path: p.to_string_lossy().into_owned(),
    }))
}

/// Lists available papers from the local Zotero library. Runs only when the user clicks.
#[tauri::command]
pub fn list_zotero_papers(
    session: tauri::State<'_, Session>,
) -> Result<Vec<PaperItem>, CommandError> {
    let zotero = ZoteroHttp::new(session.zotero_port);
    zotero.list_papers().map_err(CommandError::from)
}

/// Imports the marks of one paper from Zotero. Runs only when the user clicks.
#[tauri::command]
pub fn import_marks(
    session: tauri::State<'_, Session>,
    paper_id: String,
    attachment_key: String,
) -> Result<ImportReport, CommandError> {
    let workspace = session.workspace()?;
    let paper = PaperId::new(paper_id).map_err(CommandError::from)?;

    let app_dir = session
        .app_storage_dir
        .clone()
        .unwrap_or_else(FsDeviceStore::default_app_dir);
    let mut dev_store = FsDeviceStore::new(app_dir).map_err(CommandError::from)?;
    let store = FsHistoryStore::new(&workspace);

    let current_id = dev_store
        .get_or_create_device_id()
        .map_err(CommandError::from)?;
    let scan = store
        .inspect_device_file(&paper, &current_id)
        .map_err(CommandError::from)?;
    let device = dev_store
        .ensure_safe_device(&paper, scan.last_event.as_ref())
        .map_err(CommandError::from)?;

    let zotero = ZoteroHttp::new(session.zotero_port);
    let report = ImportMarks::new(
        &zotero,
        &store,
        &SystemClock,
        &UuidGenerator,
        device.clone(),
    )
    .run(&paper, &attachment_key)
    .map_err(CommandError::from)?;

    let scan_after = store
        .inspect_device_file(&paper, &device)
        .map_err(CommandError::from)?;
    if let Some(last_ev) = scan_after.last_event {
        dev_store
            .record_last_event(&paper, last_ev.id)
            .map_err(CommandError::from)?;
    }

    Ok(report)
}

/// Reads the history of one paper across all devices.
#[tauri::command]
pub fn read_history(
    session: tauri::State<'_, Session>,
    paper_id: String,
) -> Result<Vec<Event>, CommandError> {
    let workspace = session.workspace()?;
    let paper = PaperId::new(paper_id).map_err(CommandError::from)?;
    FsHistoryStore::new(workspace)
        .read(&paper)
        .map_err(CommandError::from)
}
