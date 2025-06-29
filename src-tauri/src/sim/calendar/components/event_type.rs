use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum EventType {
    Meeting,
    Task,
    Break,
    Training,
    Custom(String),
}
