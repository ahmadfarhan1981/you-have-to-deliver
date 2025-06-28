use std::collections::{BTreeMap, HashMap};
use crate::sim::person::components::PersonId;
use crate::sim::calendar::availability::{MonthlyAvailability, YearMonth};
use crate::sim::sim_date::sim_date::SimDate;
use super::calendar_event::CalendarEvent;
use super::interval_tree::IntervalTree;

// Optimized lookup structure for queries
#[derive(Debug, Default)]
pub struct CalendarIndex {
    // Map from year-week to events that occur on that week
    pub events_by_week: BTreeMap<(u16, u8), Vec<u64>>,
    // Map from absolute tick to events at that time
    pub events_by_tick: BTreeMap<u64, Vec<u64>>,
    // Map from participant to their events
    pub events_by_participant: HashMap<PersonId, Vec<u64>>,
    // All events by ID for quick lookup
    pub events: HashMap<u64, CalendarEvent>,

    // Month-aware availability tracking
    pub availability_matrix: HashMap<PersonId, MonthlyAvailability>,
    // Spatial index for range queries
    pub time_ranges: IntervalTree,
}

impl CalendarIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ensure_months_for_current_and_next(&mut self, current_date: SimDate) {
        let current_year_month =YearMonth::from(&current_date);
        // Ensure current and next month bitsets exist for all people
        for availability in self.availability_matrix.values_mut() {
            availability.get_or_create_month(current_year_month);
            availability.get_or_create_month(current_year_month.next_month());
        }
    }

    pub fn add_event(&mut self, event: CalendarEvent) {
        let event_id = event.id;

        // Get all occurrences for the next year (reasonable window)
        let current_date = SimDate::default(); // You might want to pass current date
        let end_date = SimDate {
            year: current_date.year + 1,
            week: current_date.week,
            day: current_date.day,
            quarter_tick: current_date.quarter_tick,
        };

        let occurrences = event.get_occurrences(event.start_time, end_date);

        for occurrence in occurrences {
            // Index by week
            let week_key = (occurrence.year, occurrence.week);
            self.events_by_week.entry(week_key).or_default().push(event_id);

            // Index by absolute tick
            let start_tick = occurrence.to_tick();
            let end_tick = start_tick + event.details.duration_ticks as u64;

            for tick_offset in 0..event.details.duration_ticks as u64 {
                self.events_by_tick.entry(start_tick + tick_offset).or_default().push(event_id);
            }

            // Update availability matrix for all participants
            for participant in &event.details.participants {
                let availability = self.availability_matrix.entry(participant.person_id).or_insert_with(MonthlyAvailability::new);
                availability.get_or_create_month(YearMonth::new(occurrence.year, occurrence.month()));
                availability.set_busy(occurrence, event.details.duration_ticks);
            }

            // Add to interval tree
            self.time_ranges.insert(start_tick, end_tick, event_id);
        }

        // Index by participants
        for participant in &event.details.participants {
            self.events_by_participant.entry(participant.person_id).or_default().push(event_id);
        }

        self.events.insert(event_id, event);
    }

    pub fn remove_event(&mut self, event_id: u64) {
        if let Some(event) = self.events.remove(&event_id) {
            // Remove from all indices
            for week_events in self.events_by_week.values_mut() {
                week_events.retain(|&id| id != event_id);
            }

            for tick_events in self.events_by_tick.values_mut() {
                tick_events.retain(|&id| id != event_id);
            }

            for participant_events in self.events_by_participant.values_mut() {
                participant_events.retain(|&id| id != event_id);
            }

            self.time_ranges.remove_event(event_id);

            // Update availability matrix (set back to free)
            let current_date = SimDate::default(); // You might want to pass current date
            let end_date = SimDate {
                year: current_date.year + 1,
                week: current_date.week,
                day: current_date.day,
                quarter_tick: current_date.quarter_tick,
            };

            let occurrences = event.get_occurrences(event.start_time, end_date);

            for occurrence in occurrences {
                for participant in &event.details.participants {
                    if let Some(availability) = self.availability_matrix.get_mut(&participant.person_id) {
                        availability.set_free(occurrence, event.details.duration_ticks);
                    }
                }
            }
        }
    }

    // OPTIMIZED QUERY METHODS

    pub fn is_person_free(&self, person: PersonId, start_time: SimDate, duration_ticks: u8) -> bool {
        if let Some(availability) = self.availability_matrix.get(&person) {
            availability.is_free(start_time, duration_ticks)
        } else {
            true // No calendar = completely free
        }
    }

    pub fn are_people_free(&self, people: &[PersonId], start_time: SimDate, duration_ticks: u8) -> bool {
        for person in people {
            if !self.is_person_free(*person, start_time, duration_ticks) {
                return false;
            }
        }
        true
    }

    pub fn find_common_free_time(&self, people: &[PersonId], duration_ticks: u8, start_from: SimDate) -> Option<SimDate> {
        let mut current_tick = start_from.to_tick();
        let max_tick = start_from.to_tick() + (30 * 7 * 96); // Search within 30 weeks

        while current_tick + duration_ticks as u64 <= max_tick {
            let test_time = SimDate::from(current_tick);
            if self.are_people_free(people, test_time, duration_ticks) {
                return Some(test_time);
            }
            current_tick += 1; // Move to next 15-minute slot
        }

        None
    }

    pub fn get_conflicts(&self, people: &[PersonId], start_time: SimDate, duration_ticks: u8) -> Vec<(PersonId, Vec<&CalendarEvent>)> {
        let start_tick = start_time.to_tick();
        let end_tick = start_tick + duration_ticks as u64;
        let mut conflicts = Vec::new();

        for &person in people {
            if !self.is_person_free(person, start_time, duration_ticks) {
                // Find which specific events are conflicting
                let conflicting_events:Vec<_> = self.time_ranges.query_overlapping(start_tick, end_tick)
                    .into_iter()
                    .filter_map(|event_id| self.events.get(&event_id))
                    .filter(|event| event.details.participants.iter().any(|p| p.person_id == person))
                    .collect();

                if !conflicting_events.is_empty() {
                    conflicts.push((person, conflicting_events));
                }
            }
        }

        conflicts
    }

    pub fn events_in_week(&self, year: u16, week: u8) -> Vec<&CalendarEvent> {
        let week_key = (year, week);
        self.events_by_week.get(&week_key)
            .map(|event_ids| {
                event_ids.iter()
                    .filter_map(|&id| self.events.get(&id))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn events_at_time(&self, time: SimDate) -> Vec<&CalendarEvent> {
        let tick = time.to_tick();
        self.events_by_tick.get(&tick)
            .map(|event_ids| {
                event_ids.iter()
                    .filter_map(|&id| self.events.get(&id))
                    .collect()
            })
            .unwrap_or_default()
    }
}
