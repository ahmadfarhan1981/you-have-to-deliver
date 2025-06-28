use crate::sim::sim_date::sim_date::SimDate;
use super::calendar_event::CalendarEvent;

// ECS Components
#[derive(Debug, Clone)]
pub struct PersonalCalendar {
    pub events: Vec<CalendarEvent>,
    pub blocked_times: Vec<(SimDate, SimDate)>, // Manually blocked time slots
}
