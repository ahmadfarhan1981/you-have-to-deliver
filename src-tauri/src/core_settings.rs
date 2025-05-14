use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, RwLock};

/// Developer-facing configuration (not part of sim state)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSettings {
    pub asset_base_path: String,
    pub debug_mode: bool,
}

impl Default for CoreSettings {
    fn default() -> Self {
        Self {
            asset_base_path: "assets/".to_string(),
            debug_mode: false,
        }
    }
}

impl CoreSettings {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Self {
        let content = fs::read_to_string(&path);
        match content {
            Ok(s) => match ron::from_str::<Self>(&s) {
                Ok(settings) => settings,
                Err(err) => {
                    eprintln!("Failed to parse core settings file: {err}. Falling back to defaults.");
                    Self::default()
                }
            },
            Err(_) => {
                eprintln!(
                    "Core settings file not found or unreadable: {}. Using default settings.",
                    path.as_ref().display()
                );
                Self::default()
            }
        }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let ron_string = ron::to_string_pretty(self, ron::ser_
