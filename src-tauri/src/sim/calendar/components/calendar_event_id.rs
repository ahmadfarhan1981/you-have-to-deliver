use serde::{Deserialize, Serialize};
use bincode::{Encode, Decode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Encode, Decode, Default)]
pub struct CalendarEventId(pub u64);

impl CalendarEventId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}
