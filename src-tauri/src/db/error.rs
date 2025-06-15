use std::time::SystemTimeError;
use bincode::error::{DecodeError, EncodeError};

#[derive(Debug)]
pub enum BincodeError{
    Encode(bincode::error::EncodeError),
    Decode(bincode::error::DecodeError),
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
    TimeError(std::time::SystemTimeError),
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
            SavesManagementError::TimeError(e) => write!(f, "Time error: {}", e)
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
            SavesManagementError::TimeError(e) => Some(e)
        }
    }
}

impl From<SystemTimeError> for SavesManagementError {
    fn from(value: SystemTimeError) -> Self {
        Self::TimeError(value)
    }
}
impl From<bincode::error::EncodeError> for SavesManagementError {
    fn from(value: EncodeError) -> Self {
        Self::Bincode(BincodeError::Encode(value))
    }
}

impl From<bincode::error::DecodeError> for SavesManagementError {
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