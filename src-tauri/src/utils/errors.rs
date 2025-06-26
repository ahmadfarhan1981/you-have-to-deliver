use bincode::error::{DecodeError, EncodeError};
use std::fmt;
use std::time::SystemTimeError;

#[derive(Debug)]
pub enum SaveDataToDBError {
    Encoding(bincode::error::EncodeError),
    Db(sled::Error),
    
}

#[derive(Debug)]
pub enum LoadDataFromDBError{
    Decoding(bincode::error::DecodeError),
    MissingHandle,
    Db(sled::Error),
    KeyNotFound(String)
}

impl fmt::Display for LoadDataFromDBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadDataFromDBError::Decoding(e) => write!(f, "Decoding failed: {}", e),
            LoadDataFromDBError::MissingHandle => write!(f, "Database handle is missing"),
            LoadDataFromDBError::Db(e) => write!(f, "Database load failed: {}", e),
            LoadDataFromDBError::KeyNotFound(key) => write!(f, "Key '{}' not found in database.", key )
        }
    }
}

impl std::error::Error for LoadDataFromDBError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoadDataFromDBError::Decoding(e) => Some(e),
            LoadDataFromDBError::Db(e) => Some(e),
            LoadDataFromDBError::MissingHandle => None,
            LoadDataFromDBError::KeyNotFound(_) => None,
        }
    }
}

impl fmt::Display for SaveDataToDBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SaveDataToDBError::Encoding(e) => write!(f, "Encoding failed: {}", e),
            SaveDataToDBError::Db(e) => write!(f, "Database insert failed: {}", e),
        }
    }
}

impl std::error::Error for SaveDataToDBError {}

/// Represents errors that can occur during `Decimal33` operations.
#[derive(Debug)]
pub enum DecimalError {
    /// Error indicating that the input value was negative, which is not supported by `Decimal33::new`.
    NegativeInput,
    /// Error indicating that the input float value, when scaled, exceeded the representable range.
    /// Contains the original problematic float value.
    Overflow(f32),
    /// Error indicating an overflow during an addition operation.
    /// Note: Current addition methods in `Decimal33` use saturating arithmetic,
    /// so this variant might be intended for other contexts or future uses.
    OverflowAdd,
    /// Error indicating a failure to parse a string into a float.
    /// Wraps the underlying `std::num::ParseFloatError`.
    ParseFloatError(std::num::ParseFloatError),
}

impl From<std::num::ParseFloatError> for DecimalError {
    /// Converts a `std::num::ParseFloatError` into a `DecimalError::ParseFloatError`.
    fn from(err: std::num::ParseFloatError) -> Self {
        DecimalError::ParseFloatError(err)
    }
}

#[derive(Debug)]
pub enum BincodeError{
    Encode(EncodeError),
    Decode(DecodeError),
}

impl std::fmt::Display for BincodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BincodeError::Encode(e) => {write!(f, "Encode error: {}", e)}
            BincodeError::Decode(e) => {write!(f, "Decode error: {}", e)}
        }
    }
}

impl std::error::Error for BincodeError{

}

#[derive(Debug)]
pub enum SavesManagementError {
    Io(std::io::Error),
    TauriPath(tauri::Error),
    Sled(sled::Error),
    Bincode(BincodeError),
    MetadataKeyNotFound(String),
    TimeError(SystemTimeError),
    EmptySaveSlotError,
    LoadError(LoadDataFromDBError),
}

// Implement Display for manual error message formatting if needed
impl std::fmt::Display for SavesManagementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SavesManagementError::Io(e) => write!(f, "IO error: {}", e),
            SavesManagementError::TauriPath(e) => write!(f, "Tauri path error: {}", e),
            SavesManagementError::Sled(e) => write!(f, "Sled DB error: {}", e),
            SavesManagementError::Bincode(e) => write!(f, "Bincode error: {}", e),
            SavesManagementError::MetadataKeyNotFound(slot_id) => write!(f, "Metadata key not found in save slot: {}", slot_id),
            SavesManagementError::TimeError(e) => write!(f, "Time error: {}", e),
            SavesManagementError::EmptySaveSlotError => write!(f, "Slot is empty"),
            SavesManagementError::LoadError(e) => write!(f, "Load data error: {}", e),
        }
    }
}

// Implement Error for compatibility, though not strictly necessary for simple cases
impl std::error::Error for SavesManagementError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SavesManagementError::Io(e) => Some(e),
            SavesManagementError::TauriPath(e) => Some(e),
            SavesManagementError::Sled(e) => Some(e),
            SavesManagementError::Bincode(e) => Some(e),
            SavesManagementError::MetadataKeyNotFound(_) => None,
            SavesManagementError::TimeError(e) => Some(e),
            SavesManagementError::EmptySaveSlotError => Some(self),
            SavesManagementError::LoadError(e) => Some(e),
        }
    }
}

impl From<SystemTimeError> for SavesManagementError {
    fn from(value: SystemTimeError) -> Self {
        Self::TimeError(value)
    }
}

impl From<EncodeError> for SavesManagementError {
    fn from(value: EncodeError) -> Self {
        Self::Bincode(BincodeError::Encode(value))
    }
}

impl From<DecodeError> for SavesManagementError {
    fn from(value: DecodeError) -> Self {
        Self::Bincode(BincodeError::Decode(value))
    }
}

// Manual From implementations
impl From<std::io::Error> for SavesManagementError {
    fn from(err: std::io::Error) -> SavesManagementError {
        SavesManagementError::Io(err)
    }
}

impl From<tauri::Error> for SavesManagementError {
    fn from(err: tauri::Error) -> SavesManagementError {
        SavesManagementError::TauriPath(err)
    }
}

impl From<sled::Error> for SavesManagementError {
    fn from(err: sled::Error) -> SavesManagementError {
        SavesManagementError::Sled(err)
    }
}

impl From<LoadDataFromDBError> for SavesManagementError {
    fn from(err: LoadDataFromDBError) -> SavesManagementError {
        SavesManagementError::LoadError(err)
    }
}