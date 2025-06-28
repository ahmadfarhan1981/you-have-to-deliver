use serde::{Deserialize, Serialize};
use bincode::{Encode, Decode};
use crate::sim::person::components::PersonId;
use super::attendance_status::AttendanceStatus;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct CalendarEventAttendee {
    pub person_id: PersonId,
    pub status: AttendanceStatus,
}
