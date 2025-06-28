use serde::{Deserialize, Serialize};
use bincode::{Encode, Decode};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, Default)]
pub enum AttendanceStatus {
    Accepted,
    Declined,
    #[default]
    Pending,
}
