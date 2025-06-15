use tauri::Manager;
use std::path::PathBuf; // Consolidated imports
use serde::{Deserialize, Serialize};
use std::fs;
use bincode::{Decode, Encode};
// rayon::in_place_scope_fifo is unused, consider removing if not needed elsewhere
use tracing::info;
use crate::db::error::SavesManagementError;
use crate::sim::sim_date::sim_date::SimDate;
use std::time::{SystemTime, UNIX_EPOCH}; // For generating timestamp

#[derive(Debug, Clone)]
pub struct SavesDirectory(pub PathBuf);


#[derive(Serialize, Deserialize, Debug, Clone, Decode, Encode)]
pub struct SaveSlotMetadata {
    pub name: String, // This will store the user-visible slot name
    pub employee_count: u32,
    pub sim_date: SimDate,
    pub save_version: String,
    pub last_saved_timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Decode, Encode)]
pub struct SaveSlot {
    pub slot_id: String, // This will be the sanitized or generated, path-safe directory name
    pub path: PathBuf,
    pub metadata: Option<SaveSlotMetadata>,
    pub is_empty: bool,
}

const METADATA_KEY: &str = "save_slot_metadata";


pub fn scan_save_slots(saves_directory: &SavesDirectory) -> Result<Vec<SaveSlot>, SavesManagementError> {
    let saves_base_path = &saves_directory.0;
    let mut save_slots = Vec::new();

    if !saves_base_path.exists() {
        fs::create_dir_all(saves_base_path)?;
        return Ok(save_slots);
    }

    for entry_result in fs::read_dir(saves_base_path)? {
        let entry = entry_result?;
        let path = entry.path();

        if path.is_dir() {
            let directory_name = match path.file_name() {
                Some(name) => name.to_string_lossy().into_owned(),
                None => continue,
            };

            let gamestate_db_path = path.join("gamestate.sled");
            let mut slot_metadata: Option<SaveSlotMetadata> = None;
            let mut is_empty_slot = true;

            if gamestate_db_path.exists() && gamestate_db_path.is_dir() {
                let db_config = sled::Config::default()
                    .path(&gamestate_db_path)
                    .cache_capacity(1_000_000)
                    .flush_every_ms(None);

                match db_config.open() {
                    Ok(db) => {
                        match db.get(METADATA_KEY)? {
                            Some(ivec_data) => {
                                match bincode::decode_from_slice(&ivec_data[..], bincode::config::standard()) {
                                    Ok((data, _size)) => {
                                        slot_metadata = Some(data);
                                        is_empty_slot = false;
                                    }
                                    Err(e) => {
                                        eprintln!("Error deserializing bincode metadata for slot dir {}: {}", directory_name, e);
                                    }
                                }
                            }
                            None => {
                                eprintln!("Metadata key '{}' not found in slot dir {}", METADATA_KEY, directory_name);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error opening Sled DB for slot dir {}: {}", directory_name, e);
                    }
                }
            }

            save_slots.push(SaveSlot {
                slot_id: directory_name,
                path,
                metadata: slot_metadata,
                is_empty: is_empty_slot,
            });
        }
    }

    save_slots.sort_by(|a, b| {
        let a_time = a.metadata.as_ref().map_or(0, |m| m.last_saved_timestamp);
        let b_time = b.metadata.as_ref().map_or(0, |m| m.last_saved_timestamp);
        b_time.cmp(&a_time)
            .then_with(|| a.slot_id.cmp(&b.slot_id))
    });
    Ok(save_slots)
}

/// Sanitizes a string to be a folder-name-friendly format.
/// Replaces problematic characters with underscores and trims excessive underscores.
///
/// # Arguments
/// * `name` - The string to sanitize.
///
/// # Returns
/// A sanitized string suitable for folder names.
pub fn sanitize_foldername(name: &str) -> String {
    let mut sanitized = String::new();
    let mut last_char_was_underscore = false;

    for char_code in name.chars() {
        match char_code {
            // Allowed characters (alphanumeric, hyphen, underscore, period)
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '.' => {
                sanitized.push(char_code);
                last_char_was_underscore = false;
            }
            // Convert spaces to a single underscore
            ' ' => {
                if !last_char_was_underscore {
                    sanitized.push('_');
                    last_char_was_underscore = true;
                }
            }
            // Other problematic characters are converted to a single underscore
            _ => {
                if !last_char_was_underscore {
                    sanitized.push('_');
                    last_char_was_underscore = true;
                }
            }
        }
    }

    // Remove leading/trailing underscores if any were added
    let mut result = sanitized.trim_matches('_').to_string();

    // If the string became empty after sanitization (e.g., input was "///"),
    // provide a default name.
    if result.is_empty() {
        result = "default_slot_name".to_string();
    }

    // Optional: Truncate to a reasonable length if necessary, e.g., 200 characters
    // let max_len = 200;
    // if result.len() > max_len {
    //     result.truncate(max_len);
    //     // Ensure it doesn't end with a partial multi-byte char or trailing underscore after truncate
    //     result = result.trim_matches('_').to_string();
    // }

    result
}


/// Creates a new save slot directory with a sanitized name and initializes it with metadata.
/// The `user_visible_name` is stored in the metadata and also used to generate the sanitized folder name.
/// Returns the sanitized folder name (slot_id) and the created metadata.
pub fn create_new_save_slot(
    saves_directory: &SavesDirectory,
    user_visible_name: &str, // This is the name the user sees and provides
) -> Result<(String, SaveSlotMetadata), SavesManagementError> {
    let sanitized_folder_name = sanitize_foldername(user_visible_name);
    info!("Creating new save slot. User name: '{}', Sanitized folder: '{}'", user_visible_name, sanitized_folder_name);

    let mut final_folder_name = sanitized_folder_name.clone();
    let mut counter = 1;
    // Handle potential collisions by appending a number
    while saves_directory.0.join(&final_folder_name).exists() {
        final_folder_name = format!("{}_{}", sanitized_folder_name, counter);
        counter += 1;
    }
    info!("Final folder name after collision check: '{}'", final_folder_name);


    let slot_path = saves_directory.0.join(&final_folder_name);
    info!("Full slot path: {:?}", slot_path);

    fs::create_dir_all(&slot_path)?;
    info!("Slot directory created/ensured.");

    let gamestate_db_path = slot_path.join("gamestate.sled");

    let db_config = sled::Config::default()
        .path(&gamestate_db_path)
        .cache_capacity(10_000_000)
        .flush_every_ms(Some(1000));

    let db = db_config.open()?;
    info!("Sled DB opened for new slot.");

    let current_timestamp_secs = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    let metadata = SaveSlotMetadata {
        name: user_visible_name.to_string(), // Store the original user-visible name here
        employee_count: 0,
        sim_date: SimDate { year: 1, week: 1, day: 1, quarter_tick: 42 },
        save_version: env!("CARGO_PKG_VERSION").to_string(),
        last_saved_timestamp: current_timestamp_secs,
    };
    info!("Generated metadata: {:?}", metadata);

    let encoded_metadata = bincode::encode_to_vec(&metadata, bincode::config::standard())?;
    info!("Metadata encoded ({} bytes).", encoded_metadata.len());

    db.insert(METADATA_KEY, encoded_metadata)?;
    info!("Metadata inserted into DB.");

    db.flush()?;
    info!("DB flushed.");

    Ok((final_folder_name, metadata)) // Return the actually used folder name
}