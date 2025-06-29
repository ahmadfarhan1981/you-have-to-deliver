use bincode::{Decode, Encode};
use crate::sim::sim_date::sim_date::SimDate;
use serde::{Deserialize, Serialize};
use crate::sim::calendar::components::recurring_event_template::RecurringEventTemplateId;
use super::event_details::EventDetails;
use super::calendar_event_id::CalendarEventId;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct CalendarEvent {
    pub id: CalendarEventId,
    pub start_time: SimDate,
    pub details: EventDetails,
    pub template_id: Option<RecurringEventTemplateId>,
}

impl CalendarEvent {
    pub fn end_time(&self) -> SimDate {
        let end_tick = self.start_time.to_tick() + self.details.duration_ticks as u64;
        SimDate::from(end_tick)
    }
    
    pub fn is_recurring(&self)->bool{
        self.template_id.is_some()
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
