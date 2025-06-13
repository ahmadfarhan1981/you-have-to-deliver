use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct DebugDisplayEntrySnapshot {
    pub label: String,
    pub value: String,
}
impl From<(String, String)> for DebugDisplayEntrySnapshot {
    fn from(value: (String, String)) -> Self {
        Self {
            label: value.0,
            value: value.1,
        }
    }
}

impl DebugDisplayEntrySnapshot {
    pub fn new(label: String, value: String) -> Self {
        Self { label, value }
    }
}

