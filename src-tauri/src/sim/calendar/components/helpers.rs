use super::company_calendar::CompanyCalendar;
use super::calendar_event_attendee::CalendarEventAttendee;
use crate::sim::sim_date::sim_date::SimDate;
use super::calendar_event::CalendarEvent;
use super::event_type::EventType;
use super::event_priority::EventPriority;
use super::event_details::EventDetails;

// Helper functions for common operations
pub fn schedule_meeting(
    company_calendar: &mut CompanyCalendar,
    title: String,
    participants: Vec<CalendarEventAttendee>,
    start_time: SimDate,
    duration_ticks: u8,
) -> Result<u64, String> {
    // Generate unique ID (you might want to use a proper ID generator)
    let event_id = rand::random::<u64>();

    let event = CalendarEvent {
        id: event_id,
        start_time,
        details: EventDetails {
            title,
            duration_ticks,
            participants,
            event_type: EventType::Meeting,
            priority: EventPriority::Normal,
        },
        template_id: None,
    };

    company_calendar.events.push(event);
    Ok(event_id)
}
