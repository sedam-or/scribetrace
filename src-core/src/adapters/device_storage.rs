use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use uuid::Uuid;

use crate::application::{DeviceStore, StoreError};
use crate::domain::{DeviceId, DeviceState, Event, PaperDeviceState, PaperId};

/// Per-user application storage adapter for device identity and state (ADR 0007 / SEC-34).
/// Files are stored in the user's OS application data directory (not in the workspace).
#[derive(Debug, Clone)]
pub struct FsDeviceStore {
    app_dir: PathBuf,
    state: DeviceState,
    recovered_corrupt: bool,
}

impl FsDeviceStore {
    /// Initializes device storage rooted at `app_dir`.
    /// If `device.json` exists, it is loaded; if corrupt or missing, a fresh random UUID
    /// device ID is minted and atomically persisted.
    pub fn new(app_dir: impl AsRef<Path>) -> Result<Self, StoreError> {
        let app_dir = app_dir.as_ref().to_path_buf();
        fs::create_dir_all(&app_dir)?;

        let mut recovered_corrupt = false;
        let state_path = app_dir.join("device.json");
        let state = if state_path.exists() {
            let data = fs::read(&state_path)?;
            match serde_json::from_slice::<DeviceState>(&data) {
                Ok(s) => s,
                Err(_) => {
                    recovered_corrupt = true;
                    let fresh = Self::fresh_random_state()?;
                    Self::persist_state_to(&app_dir, &fresh)?;
                    fresh
                }
            }
        } else {
            let fresh = Self::fresh_random_state()?;
            Self::persist_state_to(&app_dir, &fresh)?;
            fresh
        };

        Ok(Self {
            app_dir,
            state,
            recovered_corrupt,
        })
    }

    /// Indicates whether a corrupt `device.json` file was encountered on initialization
    /// and safely recovered with a newly minted device ID.
    pub fn was_corrupt_recovered(&self) -> bool {
        self.recovered_corrupt
    }

    /// Resolves the default OS application config directory for ScribeTrace.
    /// Linux/Unix: `$XDG_CONFIG_HOME/scribetrace` or `~/.config/scribetrace`.
    pub fn default_app_dir() -> PathBuf {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            if !xdg.is_empty() {
                return PathBuf::from(xdg).join("scribetrace");
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            if !home.is_empty() {
                return PathBuf::from(home).join(".config").join("scribetrace");
            }
        }
        PathBuf::from(".scribetrace-app-storage")
    }

    fn fresh_random_state() -> Result<DeviceState, StoreError> {
        let raw_id = Uuid::new_v4().to_string();
        let device_id = DeviceId::new(raw_id)?;
        Ok(DeviceState::new(device_id))
    }

    fn persist_state_to(app_dir: &Path, state: &DeviceState) -> Result<(), StoreError> {
        let bytes =
            serde_json::to_vec_pretty(state).map_err(|e| StoreError::Encode(e.to_string()))?;
        let tmp_path = app_dir.join("device.json.tmp");
        let target_path = app_dir.join("device.json");

        let mut file = File::create(&tmp_path)?;
        file.write_all(&bytes)?;
        file.sync_data()?;
        fs::rename(tmp_path, target_path)?;
        Ok(())
    }

    fn persist_state(&self) -> Result<(), StoreError> {
        Self::persist_state_to(&self.app_dir, &self.state)
    }
}

impl DeviceStore for FsDeviceStore {
    fn get_or_create_device_id(&mut self) -> Result<DeviceId, StoreError> {
        Ok(self.state.device_id.clone())
    }

    fn device_state(&self) -> Result<DeviceState, StoreError> {
        Ok(self.state.clone())
    }

    fn ensure_safe_device(
        &mut self,
        paper: &PaperId,
        file_last_event: Option<&Event>,
    ) -> Result<DeviceId, StoreError> {
        let recorded = self.state.papers.get(paper);
        let collision = DeviceState::is_collision(recorded, file_last_event);

        if collision {
            let new_dev = DeviceId::new(Uuid::new_v4().to_string())?;
            self.state.device_id = new_dev.clone();
            self.state.papers.remove(paper);
            self.persist_state()?;
            return Ok(new_dev);
        }

        Ok(self.state.device_id.clone())
    }

    fn record_last_event(&mut self, paper: &PaperId, event_id: String) -> Result<(), StoreError> {
        self.state
            .papers
            .insert(paper.clone(), PaperDeviceState { last_id: event_id });
        self.persist_state()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_corrupt_device_json_recovers_with_fresh_id() {
        let tmp = tempdir().unwrap();
        let state_path = tmp.path().join("device.json");
        fs::write(&state_path, b"corrupted non json content").unwrap();

        let mut store = FsDeviceStore::new(tmp.path()).unwrap();
        let dev_id = store.get_or_create_device_id().unwrap();
        assert!(!dev_id.as_str().is_empty());

        let state = store.device_state().unwrap();
        assert_eq!(state.device_id, dev_id);
    }

    #[test]
    fn test_default_app_dir_resolves() {
        let dir = FsDeviceStore::default_app_dir();
        assert!(dir.to_string_lossy().contains("scribetrace"));
    }
}
