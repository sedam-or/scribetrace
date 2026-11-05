use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{DeviceId, Event, PaperId};

/// Per-machine device state stored in per-user application storage (ADR 0007 / SEC-34).
/// Never synced across machines; kept strictly out of the shared workspace folder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceState {
    /// Random UUID device identifier for this machine.
    pub device_id: DeviceId,
    /// Per-paper sequence tracking to detect collisions from cloned or restored files.
    #[serde(default)]
    pub papers: HashMap<PaperId, PaperDeviceState>,
}

/// Recorded state for one paper on this device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaperDeviceState {
    /// ID of the last event written by this device to this paper's log.
    pub last_id: String,
}

impl DeviceState {
    /// Creates a new device state with a given device id.
    pub fn new(device_id: DeviceId) -> Self {
        Self {
            device_id,
            papers: HashMap::new(),
        }
    }

    /// Evaluates whether a collision or state mismatch occurred for `paper`
    /// between the local `device.json` state and the actual disk file's last event.
    ///
    /// Per ADR 0007:
    /// - If neither exists: fresh paper, no collision.
    /// - If both exist and `last_event.id == recorded.last_id`: matching, no collision.
    /// - If file has events but state is missing, or state exists but file is missing/empty,
    ///   or IDs do not match: collision! A new device ID must be minted to prevent history corruption.
    pub fn is_collision(
        recorded_state: Option<&PaperDeviceState>,
        file_last_event: Option<&Event>,
    ) -> bool {
        match (file_last_event, recorded_state) {
            (None, None) => false,
            (Some(last_ev), Some(rec)) => last_ev.id != rec.last_id,
            (Some(_), None) => true,
            (None, Some(_)) => true,
        }
    }
}
