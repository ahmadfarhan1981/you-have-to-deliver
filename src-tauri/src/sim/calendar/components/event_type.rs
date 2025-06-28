use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Meeting,
    Task,
    Break,
    Training,
    Custom(String),
}
