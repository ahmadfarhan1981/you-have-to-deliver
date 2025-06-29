use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::sim::calendar::components::calendar_event_attendee::CalendarEventAttendee;
use crate::sim::calendar::components::event_type::EventType;
use crate::sim::calendar::components::event_priority::EventPriority;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct EventDetails {
    pub title: String,
    pub duration_ticks: u8,  // Duration in 15-minute increments
    pub participants: Vec<CalendarEventAttendee>, // People entities
    pub event_type: EventType,
    pub priority: EventPriority,
}
