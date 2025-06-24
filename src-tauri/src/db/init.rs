use tauri::Manager;
use std::path::PathBuf; // Consolidated imports
use serde::{Deserialize, Serialize};
use std::{fmt, fs};
use std::sync::Arc;
use std::task::Context;
use bincode::{decode_from_slice, Decode, Encode};
// rayon::in_place_scope_fifo is unused, consider removing if not needed elsewhere
use tracing::{error, info, trace, warn};
use crate::utils::errors::{BincodeError, SavesManagementError};
use crate::db::{self};
use crate::sim::sim_date::sim_date::SimDate;
use std::time::{SystemTime, UNIX_EPOCH};
use bincode::config::standard;
use sled::{Db, IVec};
use crate::db::constants::{db_keys, save_version, GAMESTATE_DB_FILENAME};
use crate::utils::errors::{LoadDataFromDBError, SaveDataToDBError};
// For generating timestamp

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSlot {
    pub slot_id: String, // This will be the sanitized or generated, path-safe directory name
    pub path: PathBuf,
    pub metadata: Option<SaveSlotMetadata>,
    pub is_empty: bool,
    #[serde(skip)]
    pub handle : Option<Db>
}
impl Default for SaveSlot {
    fn default() -> Self {
        Self {
            slot_id: "".to_string(),
            path: Default::default(),
            metadata: None,
            is_empty: true,
            handle: None,
        }
    }
}


impl SaveSlot {
    /// Ensures the Sled database handle is open if the slot is not empty and the handle is None.
    ///
    /// # Arguments
    /// * `_saves_directory_arc` - An Arc reference to the saves directory. Currently unused if `self.path` is absolute,
    ///                            but included as per requirements for potential future use or consistency.
    ///
    /// # Returns
    /// * `Ok(())` if the handle is successfully opened or was already open/slot is empty.
    /// * `Err(SavesManagementError)` if an error occurs while trying to open the database.
    pub fn ensure_db_handle_is_open(
        &mut self,
        _saves_directory_arc: &Arc<SavesDirectory>, // Included as requested
    ) -> Result<(), SavesManagementError> {
        if self.handle.is_some() {
            trace!("DB handle for slot '{}' is already open.", self.slot_id);
            return Ok(());
        }

        if self.is_empty {
            warn!("Slot '{}' is empty, not opening DB handle.", self.slot_id);
            return Err(SavesManagementError::EmptySaveSlotError);
        }

        let gamestate_db_path = self.path.join(GAMESTATE_DB_FILENAME);
        info!("Attempting to open DB for slot '{}' at path: {:?}", self.slot_id, gamestate_db_path);
        

        if !gamestate_db_path.exists() || !gamestate_db_path.is_dir() {
            error!("Gamestate DB directory not found or is not a directory for slot '{}' at: {:?}", self.slot_id, gamestate_db_path);
            return Err(SavesManagementError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Gamestate DB directory not found for slot '{}': {:?}",
                    self.slot_id, gamestate_db_path
                ),
            )));
        }

        // Configure Sled DB. These settings are similar to when creating a new slot,
        // assuming the handle will be used for active game operations.
        let db_config = sled::Config::default()
            .path(&gamestate_db_path)
            .cache_capacity(10_000_000) // Consistent with create_new_save_slot
            .flush_every_ms(Some(1000)); // Consistent with create_new_save_slot

        match db_config.open() {
            Ok(db) => {
                self.handle = Some(db);
                info!("Successfully opened DB handle for slot '{}'.", self.slot_id);
                Ok(())
            }
            Err(e) => {
                error!("Failed to open Sled DB for slot '{}': {:?}", self.slot_id, e);
                // Assuming SavesManagementError implements From<sled::Error>
                Err(e.into())
            }
        }
    }

    /// Loads a specific save slot by its ID.
    ///
    /// This function constructs a `SaveSlot`, loads its metadata from the `gamestate.sled` database,
    /// and then attempts to open the database handle if the slot is not empty.
    ///
    /// # Arguments
    /// * `slot_id` - The ID (directory name) of the save slot to load.
    /// * `saves_directory_arc` - An Arc reference to the saves directory.
    ///
    /// # Returns
    /// * `Ok(SaveSlot)` if the slot is successfully loaded (metadata may be None if slot is empty but dir exists).
    /// * `Err(SavesManagementError)` if the slot directory doesn't exist, metadata is corrupt, or DB errors occur.
    pub fn load(
        slot_id: String,
        saves_directory_arc: Arc<SavesDirectory>,
    ) -> Result<Self, SavesManagementError> {
        let slot_path = saves_directory_arc.0.join(slot_id.as_str());
        info!("Attempting to load save slot '{}' from path: {:?}", slot_id, slot_path);

        if !slot_path.exists() {
            error!("Save slot directory not found for slot_id '{}' at: {:?}", slot_id, slot_path);
            return Err(SavesManagementError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Save slot directory not found for slot_id '{}': {:?}", slot_id, slot_path),
            )));
        }
        if !slot_path.is_dir() {
            error!("Save slot path is not a directory for slot_id '{}' at: {:?}", slot_id, slot_path);
            return Err(SavesManagementError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput, // Or NotADirectory if more specific
                format!("Save slot path is not a directory for slot_id '{}': {:?}", slot_id, slot_path),
            )));
        }

        let gamestate_db_path = slot_path.join(GAMESTATE_DB_FILENAME);
        let mut loaded_metadata: Option<SaveSlotMetadata> = None;
        let mut is_slot_empty = true; // Assume empty until metadata is successfully loaded

        if gamestate_db_path.exists() && gamestate_db_path.is_dir() {
            // Attempt to open DB for metadata reading (try read-only first, then read-write fallback)
            let db_meta_config_ro = sled::Config::default().path(&gamestate_db_path).cache_capacity(1_000_000).flush_every_ms(None);
            let db_meta = match db_meta_config_ro.open() {
                Ok(db) => Ok(db),
                Err(_e_ro) => {
                    info!("Failed to open DB in read-only mode for slot '{}'. Attempting read-write mode for metadata.", slot_id);
                    sled::Config::default().path(&gamestate_db_path).cache_capacity(1_000_000).flush_every_ms(None).open()
                }
            };

            match db_meta {
                Ok(db) => {
                    match db.get(db_keys::METADATA)? { // Propagates SledError
                        Some(ivec_data) => {
                            match decode_from_slice(&ivec_data[..], standard()) {
                                Ok((data, _size)) => {
                                    loaded_metadata = Some(data);
                                    is_slot_empty = false; // Metadata found and decoded
                                    info!("Successfully loaded metadata for slot '{}'.", slot_id);
                                }
                                Err(e) => {
                                    error!("Failed to deserialize metadata for slot '{}': {}", slot_id, e);
                                    return Err(SavesManagementError::Bincode(BincodeError::Decode(e)));
                                }
                            }
                        }
                        None => {
                            info!("Metadata key '{}' not found in slot '{}'. Slot is considered empty.", db_keys::METADATA, slot_id);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to open Sled DB for metadata check for slot '{}': {}", slot_id, e);
                    return Err(e.into());
                }
            }
        } else {
            info!("Gamestate DB directory not found for slot '{}' at: {:?}. Slot is considered empty.", slot_id, gamestate_db_path);
        }

        let mut save_slot = Self { slot_id, path: slot_path, metadata: loaded_metadata, is_empty: is_slot_empty, handle: None };
        save_slot.ensure_db_handle_is_open(&saves_directory_arc)?;
        Ok(save_slot)
    }

    // Saving related fn
    /// Custom error type for saving to the database, without external crates.


    /// Save a encoded value to the sled database, logging any errors that occur.
    pub fn save_entry<T: bincode::Encode>(&mut self,
        key: &str,
        value: &T,
    ) -> Result<Option<IVec>, SaveDataToDBError> {
        match bincode::encode_to_vec(value, standard()) {
            Ok(encoded) => match self.handle.as_ref().unwrap().insert(key, encoded) {
                Ok(ivec) => Ok(ivec),
                Err(e) => {
                    error!("Failed to save key '{}': {}", key, e);
                    Err(SaveDataToDBError::Db(e))
                }
            },
            Err(e) => {
                error!("Failed to encode key '{}': {}", key, e);
                Err(SaveDataToDBError::Encoding(e))
            }
        }
    }
    

    /// Load and decode a value from the sled database using the given key.
    ///
    /// Returns `Ok(None)` if the key does not exist.
    pub fn load_entry<T>(
        &self,
        key: &str,
    ) -> Result<Option<T>, LoadDataFromDBError>
    where
        T: bincode::Decode<()>,
    {
        let handle = self.handle.as_ref().ok_or(LoadDataFromDBError::MissingHandle)?;

        match handle.get(key) {
            Ok(Some(bytes)) => {
                match decode_from_slice::<T, _>(&bytes, standard()) {
                    Ok((value, _)) => Ok(Some(value)),
                    Err(e) => {
                        error!("Failed to decode key '{}': {}", key, e);
                        Err(LoadDataFromDBError::Decoding(e))
                    }
                }
            }
            Ok(None) => Ok(None),
            Err(e) => {
                error!("Failed to load key '{}': {}", key, e);
                Err(LoadDataFromDBError::Db(e))
            }
        }
    }


}


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

            let gamestate_db_path = path.join(GAMESTATE_DB_FILENAME);
            let mut slot_metadata: Option<SaveSlotMetadata> = None;
            let mut is_empty_slot = true;

            if gamestate_db_path.exists() && gamestate_db_path.is_dir() {
                let db_config = sled::Config::default()
                    .path(&gamestate_db_path)
                    .cache_capacity(1_000_000)
                    .flush_every_ms(None);

                match db_config.open() {
                    Ok(db) => {
                        match db.get(db_keys::METADATA)? {
                            Some(ivec_data) => {
                                match decode_from_slice(&ivec_data[..], standard()) {
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
                                eprintln!("Metadata key '{}' not found in slot dir {}", db_keys::METADATA, directory_name);
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
                handle: None,
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
    company_name: &str,
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

    let gamestate_db_path = slot_path.join(GAMESTATE_DB_FILENAME);

    let db_config = sled::Config::default()
        .path(&gamestate_db_path)
        .cache_capacity(10_000_000)
        .flush_every_ms(Some(1000));

    let db = db_config.open()?;
    info!("Sled DB opened for new slot.");

    let current_timestamp_secs = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    let metadata = SaveSlotMetadata {
        name: company_name.to_string(), // Store the original user-visible name here
        employee_count: 0,
        sim_date: SimDate { year: 1, week: 1, day: 1, quarter_tick: 42 },
        save_version: save_version::SAVE_VERSION.to_string(),
        last_saved_timestamp: current_timestamp_secs,
    };
    info!("Generated metadata: {:?}", metadata);

    let encoded_metadata = bincode::encode_to_vec(&metadata, standard())?;
    info!("Metadata encoded ({} bytes).", encoded_metadata.len());

    db.insert(db_keys::METADATA, encoded_metadata)?;
    info!("Metadata inserted into DB.");

    db.flush()?;
    info!("DB flushed.");

    Ok((final_folder_name, metadata)) // Return the actually used folder name
}