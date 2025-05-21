use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;


const SETTINGS_PATH: &str = "core_settings.ron";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortraitFiles {
    pub output_dir: String,
    pub width: u32,
    pub height: u32,
}

impl Default for PortraitFiles {
    fn default() -> Self {
        Self {
            output_dir: "portraits/".to_string(),
            width: 512,
            height: 640,
        }
    }
}

/// Developer-facing configuration (not part of sim state)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSettings {
    pub asset_base_path: String,
    pub debug_mode: bool,
    pub portrait_files: PortraitFiles,
}

impl Default for CoreSettings {
    fn default() -> Self {
        Self {
            asset_base_path: "assets/".to_string(),
            debug_mode: false,
            portrait_files: PortraitFiles::default(),
        }
    }
}

impl CoreSettings {
    pub fn load_or_create() -> Self {
        let path = Path::new(SETTINGS_PATH);
        match fs::read_to_string(path) {
            Ok(s) => match ron::from_str::<Self>(&s) {
                Ok(settings) => settings,
                Err(err) => {
                    eprintln!("Failed to parse core settings file: {err}. Using defaults.");
                    let default = Self::default();
                    if let Err(e) = default.save() {
                        eprintln!("Also failed to save default settings: {e}");
                    }
                    default
                }
            },
            Err(_) => {
                eprintln!("Settings file not found. Creating default: {}", SETTINGS_PATH);
                let default = Self::default();
                if let Err(e) = default.save() {
                    eprintln!("Failed to write default settings: {e}");
                }
                default
            }
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let ron_string = ron::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .expect("Failed to serialize settings to RON");
        let mut file = fs::File::create(SETTINGS_PATH)?;
        file.write_all(ron_string.as_bytes())?;
        Ok(())
    }
}

pub type SharedCoreSettings = Arc<RwLock<CoreSettings>>;

pub fn init_core_settings() -> SharedCoreSettings {
    Arc::new(RwLock::new(CoreSettings::load_or_create()))
}

pub fn is_debug_mode_enabled(settings: &SharedCoreSettings) -> bool {
    settings.read().unwrap().debug_mode
}
