use std::collections::HashMap;
use super::calendar_event::CalendarEvent;

#[derive(Debug, Clone)]
pub struct CompanyCalendar {
    pub events: Vec<CalendarEvent>,
    pub meeting_rooms: HashMap<String, Vec<CalendarEvent>>,
}
