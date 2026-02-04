use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunCommandStore {
    pub version: u32,
    pub commands: HashMap<String, String>,
}

impl RunCommandStore {
    pub fn load() -> Self {
        let path = match Self::storage_path() {
            Some(path) => path,
            None => return Self::default_store(),
        };

        let Ok(data) = fs::read_to_string(path) else {
            return Self::default_store();
        };

        serde_json::from_str(&data).unwrap_or_else(|_| Self::default_store())
    }

    pub fn save(&self) {
        let Some(path) = Self::storage_path() else {
            return;
        };

        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, data);
        }
    }

    fn default_store() -> Self {
        Self {
            version: 1,
            commands: HashMap::new(),
        }
    }

    fn storage_path() -> Option<PathBuf> {
        let proj = ProjectDirs::from("io", "openpad", "Openpad")?;
        Some(proj.config_dir().join("run-commands.json"))
    }
}
