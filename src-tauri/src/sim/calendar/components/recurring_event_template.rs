use serde::{Deserialize, Serialize};
use crate::sim::sim_date::sim_date::SimDate;
use super::recurrence_pattern::RecurrencePattern;
use super::event_details::EventDetails;
use super::calendar_event::CalendarEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringEventTemplate {
    pub id: u64,
    pub start_date: SimDate,
    pub end_date: SimDate,
    pub recurrence: RecurrencePattern,
    pub details: EventDetails,
}

impl RecurringEventTemplate {
    pub fn expand(&self, start_date: SimDate, end_date: SimDate) -> Vec<CalendarEvent> {
        let mut occurrences = Vec::new();
        let mut current_date = self.start_date;

        while current_date <= self.end_date && current_date <= end_date {
            if current_date >= start_date {
                let event = CalendarEvent {
                    id: rand::random(),
                    start_time: current_date,
                    details: self.details.clone(),
                    template_id: Some(self.id),
                };
                occurrences.push(event);
            }

            current_date = match self.recurrence {
                RecurrencePattern::None => break,
                RecurrencePattern::Daily => SimDate::from(current_date.to_tick() + 96),
                RecurrencePattern::Weekly => SimDate::from(current_date.to_tick() + (7 * 96)),
                RecurrencePattern::Weekdays => {
                    let mut next_date = SimDate::from(current_date.to_tick() + 96);
                    while next_date.day > 5 {
                        next_date = SimDate::from(next_date.to_tick() + 96);
                    }
                    next_date
                }
                RecurrencePattern::Custom { ref days_of_week } => {
                    let mut next_date = SimDate::from(current_date.to_tick() + 96);
                    while !days_of_week.contains(&next_date.day) {
                        next_date = SimDate::from(next_date.to_tick() + 96);
                    }
                    next_date
                }
                RecurrencePattern::EveryNWeeks { n, day_of_week } => {
                    let mut next_date = SimDate::from(current_date.to_tick() + (n as u64 * 7 * 96));
                    while next_date.day != day_of_week {
                        next_date = SimDate::from(next_date.to_tick() + 96);
                    }
                    next_date
                }
            };
        }
        occurrences
    }
}
