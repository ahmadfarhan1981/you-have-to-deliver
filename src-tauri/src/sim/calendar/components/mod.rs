pub mod recurrence_pattern;
pub mod attendance_status;
pub mod calendar_event_attendee;
pub mod event_type;
pub mod event_priority;
pub mod calendar_event;
pub mod personal_calendar;
pub mod company_calendar;
pub mod interval_tree;
pub mod calendar_index;
pub mod calendar_system;
pub mod helpers;
mod sim_date_extensions;
pub mod event_details;
pub mod recurring_event_template;
pub mod calendar_event_id;
pub mod calendar_event_registry;

pub use recurrence_pattern::RecurrencePattern;
pub use attendance_status::AttendanceStatus;
pub use calendar_event_attendee::CalendarEventAttendee;
pub use event_type::EventType;
pub use event_priority::EventPriority;
pub use calendar_event::CalendarEvent;
pub use personal_calendar::PersonalCalendar;
pub use company_calendar::CompanyCalendar;
pub use interval_tree::IntervalTree;
pub use calendar_index::CalendarIndex;
pub use calendar_system::CalendarSystem;
pub use helpers::schedule_meeting;
pub use event_details::EventDetails;
pub use recurring_event_template::RecurringEventTemplate;
pub use calendar_event_id::CalendarEventId;
pub use calendar_event_registry::CalendarEventRegistry;

#[cfg(test)]
mod tests {
    use crate::sim::sim_date::sim_date::SimDate;
    use super::*;
    use crate::sim::calendar::availability::{MonthlyAvailability, YearMonth};

    #[test]
    fn test_simdate_month_calculation() {
        assert_eq!(SimDate { year: 1, week: 1, day: 1, quarter_tick: 1 }.month(), 1);
        assert_eq!(SimDate { year: 1, week: 4, day: 7, quarter_tick: 96 }.month(), 1);
        assert_eq!(SimDate { year: 1, week: 5, day: 1, quarter_tick: 1 }.month(), 2);
        assert_eq!(SimDate { year: 1, week: 8, day: 7, quarter_tick: 96 }.month(), 2);
    }

    #[test]
    fn test_tick_in_month_calculation() {
        let start_of_month = SimDate { year: 1, week: 1, day: 1, quarter_tick: 1 };
        assert_eq!(start_of_month.tick_in_month(), 0);

        let second_quarter_tick = SimDate { year: 1, week: 1, day: 1, quarter_tick: 2 };
        assert_eq!(second_quarter_tick.tick_in_month(), 1);

        let second_day = SimDate { year: 1, week: 1, day: 2, quarter_tick: 1 };
        assert_eq!(second_day.tick_in_month(), 96);
    }

    #[test]
    fn test_availability_bitset_cross_month() {
        let mut availability = MonthlyAvailability::new();
        availability.get_or_create_month(YearMonth::new(1, 1));
        availability.get_or_create_month(YearMonth::new(1, 2));

        // Event that spans from end of month 1 to beginning of month 2
        let end_of_month1 = SimDate { year: 1, week: 4, day: 7, quarter_tick: 95 }; // Almost end of month 1
        availability.set_busy(end_of_month1, 4); // 1 hour duration crosses months

        // Should be busy at the end of month 1
        assert!(!availability.is_free(end_of_month1, 4));

        // Should be busy at the start of month 2
        let start_of_month2 = SimDate { year: 1, week: 5, day: 1, quarter_tick: 1 };
        assert!(!availability.is_free(start_of_month2, 2)); // Remaining ticks from cross-month event
    }
}
