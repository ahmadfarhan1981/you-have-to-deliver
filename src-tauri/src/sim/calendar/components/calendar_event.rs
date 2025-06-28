use crate::sim::sim_date::sim_date::SimDate;
use serde::{Deserialize, Serialize};
use super::event_details::EventDetails;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: u64,
    pub start_time: SimDate,
    pub details: EventDetails,
    pub template_id: Option<u64>,
}

impl CalendarEvent {
    pub fn end_time(&self) -> SimDate {
        let end_tick = self.start_time.to_tick() + self.details.duration_ticks as u64;
        SimDate::from(end_tick)
    }

    /// Get all occurrences of this event within a date range
    /// Returns the start time of each occurrence, not every tick where the event is active
    pub fn get_occurrences(&self, start_date: SimDate, end_date: SimDate) -> Vec<SimDate> {
        let mut occurrences = Vec::new();

        // Single occurrence - check if it falls within the range
        if self.start_time >= start_date && self.start_time <= end_date {
            occurrences.push(self.start_time);
        }

        occurrences
    }
}
