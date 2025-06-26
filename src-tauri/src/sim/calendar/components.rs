use crate::sim::sim_date::sim_date::SimDate;
use legion::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

// Additional methods for SimDate
impl SimDate {
    /// Get the month number (1-based) from week
    pub fn month(&self) -> u8 {
        ((self.week - 1) / 4) + 1
    }

    /// Get the week within the month (1-4)
    pub fn week_in_month(&self) -> u8 {
        ((self.week - 1) % 4) + 1
    }

    /// Create a SimDate for the start of a given month
    pub fn month_start(year: u16, month: u8) -> Self {
        let week = ((month - 1) * 4) + 1;
        SimDate {
            year,
            week,
            day: 1,
            quarter_tick: 1,
        }
    }

    /// Get the absolute tick within a 4-week month (0-2687)
    pub fn tick_in_month(&self) -> u32 {
        let week_in_month = self.week_in_month() as u32;
        let day_in_week = self.day as u32;
        let quarter_tick = self.quarter_tick as u32;

        ((week_in_month - 1) * 7 * 96) + ((day_in_week - 1) * 96) + (quarter_tick - 1)
    }

    /// Create SimDate from year, month, and tick within month
    pub fn from_month_tick(year: u16, month: u8, tick_in_month: u32) -> Self {
        let base_week = ((month - 1) * 4) + 1;
        let week_offset = tick_in_month / (7 * 96);
        let remaining = tick_in_month % (7 * 96);
        let day_offset = remaining / 96;
        let quarter_tick = (remaining % 96) + 1;

        SimDate {
            year,
            week: base_week + week_offset as u8,
            day: (day_offset + 1) as u8,
            quarter_tick: quarter_tick as u8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecurrencePattern {
    None,
    Daily,
    Weekly,
    Weekdays,  // Mon-Fri (days 1-5)
    Custom { days_of_week: Vec<u8> }, // 1=Monday, 7=Sunday
    EveryNWeeks { n: u8, day_of_week: u8 }, // Every N weeks on specific day
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: u64,
    pub title: String,
    pub start_time: SimDate,
    pub duration_ticks: u8,  // Duration in 15-minute increments
    pub recurrence: RecurrencePattern,
    pub participants: Vec<Entity>, // People entities
    pub event_type: EventType,
    pub priority: EventPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Meeting,
    Task,
    Break,
    Training,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EventPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl CalendarEvent {
    pub fn end_time(&self) -> SimDate {
        let end_tick = self.start_time.to_tick() + self.duration_ticks as u64;
        SimDate::from(end_tick)
    }

    /// Check if this event occurs on a specific date
    pub fn occurs_on_date(&self, date: SimDate) -> bool {
        match &self.recurrence {
            RecurrencePattern::None => {
                self.start_time.year == date.year &&
                    self.start_time.week == date.week &&
                    self.start_time.day == date.day
            },
            RecurrencePattern::Daily => true,
            RecurrencePattern::Weekly => {
                // Same day of week, and on or after start date
                date >= self.start_time && date.day == self.start_time.day
            },
            RecurrencePattern::Weekdays => {
                date.day >= 1 && date.day <= 5 && date >= self.start_time
            },
            RecurrencePattern::Custom { days_of_week } => {
                days_of_week.contains(&date.day) && date >= self.start_time
            },
            RecurrencePattern::EveryNWeeks { n, day_of_week } => {
                if date < self.start_time || date.day != *day_of_week {
                    return false;
                }

                // Calculate weeks elapsed since start
                let start_absolute_week = ((self.start_time.year - 1) as u32 * 52) + self.start_time.week as u32;
                let date_absolute_week = ((date.year - 1) as u32 * 52) + date.week as u32;
                let weeks_elapsed = date_absolute_week - start_absolute_week;

                weeks_elapsed % (*n as u32) == 0
            }
        }
    }

    /// Get all occurrences of this event within a date range
    /// Returns the start time of each occurrence, not every tick where the event is active
    pub fn get_occurrences(&self, start_date: SimDate, end_date: SimDate) -> Vec<SimDate> {
        let mut occurrences = Vec::new();

        // Start from the later of: range start or event start
        let mut current_date = if start_date > self.start_time {
            start_date
        } else {
            self.start_time
        };

        match &self.recurrence {
            RecurrencePattern::None => {
                // Single occurrence - check if it falls within the range
                if self.start_time >= start_date && self.start_time <= end_date {
                    occurrences.push(self.start_time);
                }
            },

            RecurrencePattern::Daily => {
                // Advance current_date to start of day, keeping the original time
                current_date = SimDate {
                    year: current_date.year,
                    week: current_date.week,
                    day: current_date.day,
                    quarter_tick: self.start_time.quarter_tick,
                };

                while current_date <= end_date {
                    if current_date >= start_date {
                        occurrences.push(current_date);
                    }
                    // Advance to next day
                    current_date = SimDate::from(current_date.to_tick() + 96);
                }
            },

            RecurrencePattern::Weekly => {
                // Find the first occurrence on the correct day of week
                while current_date <= end_date {
                    if current_date.day == self.start_time.day {
                        let occurrence = SimDate {
                            year: current_date.year,
                            week: current_date.week,
                            day: current_date.day,
                            quarter_tick: self.start_time.quarter_tick,
                        };
                        if occurrence >= start_date {
                            occurrences.push(occurrence);
                        }
                        // Advance by exactly one week
                        current_date = SimDate::from(current_date.to_tick() + (7 * 96));
                    } else {
                        // Advance to next day
                        current_date = SimDate::from(current_date.to_tick() + 96);
                    }
                }
            },

            RecurrencePattern::Weekdays => {
                // Advance current_date to start of day, keeping the original time
                current_date = SimDate {
                    year: current_date.year,
                    week: current_date.week,
                    day: current_date.day,
                    quarter_tick: self.start_time.quarter_tick,
                };

                while current_date <= end_date {
                    if current_date.day >= 1 && current_date.day <= 5 && current_date >= start_date {
                        occurrences.push(current_date);
                    }
                    // Advance to next day
                    current_date = SimDate::from(current_date.to_tick() + 96);
                }
            },

            RecurrencePattern::Custom { days_of_week } => {
                // Advance current_date to start of day, keeping the original time
                current_date = SimDate {
                    year: current_date.year,
                    week: current_date.week,
                    day: current_date.day,
                    quarter_tick: self.start_time.quarter_tick,
                };

                while current_date <= end_date {
                    if days_of_week.contains(&current_date.day) && current_date >= start_date {
                        occurrences.push(current_date);
                    }
                    // Advance to next day
                    current_date = SimDate::from(current_date.to_tick() + 96);
                }
            },

            RecurrencePattern::EveryNWeeks { n, day_of_week } => {
                // Find first occurrence on the correct day
                while current_date <= end_date && current_date.day != *day_of_week {
                    current_date = SimDate::from(current_date.to_tick() + 96);
                }

                while current_date <= end_date {
                    if current_date.day == *day_of_week {
                        // Check if this is the correct week interval
                        let start_absolute_week = ((self.start_time.year - 1) as u32 * 52) + self.start_time.week as u32;
                        let current_absolute_week = ((current_date.year - 1) as u32 * 52) + current_date.week as u32;
                        let weeks_elapsed = current_absolute_week - start_absolute_week;

                        if weeks_elapsed % (*n as u32) == 0 {
                            let occurrence = SimDate {
                                year: current_date.year,
                                week: current_date.week,
                                day: current_date.day,
                                quarter_tick: self.start_time.quarter_tick,
                            };
                            if occurrence >= start_date {
                                occurrences.push(occurrence);
                            }
                        }
                        // Advance to next week (same day)
                        current_date = SimDate::from(current_date.to_tick() + (7 * 96));
                    } else {
                        // This shouldn't happen if we're tracking correctly, but safety check
                        current_date = SimDate::from(current_date.to_tick() + 96);
                    }
                }
            }
        }

        occurrences
    }
}

// ECS Components
#[derive(Debug, Clone)]
pub struct PersonalCalendar {
    pub events: Vec<CalendarEvent>,
    pub blocked_times: Vec<(SimDate, SimDate)>, // Manually blocked time slots
}

#[derive(Debug, Clone)]
pub struct CompanyCalendar {
    pub events: Vec<CalendarEvent>,
    pub meeting_rooms: HashMap<String, Vec<CalendarEvent>>,
}

// Compact representation of a person's schedule for one month (4 weeks)
#[derive(Debug, Clone)]
pub struct AvailabilityBitSet {
    // Each bit represents one 15-minute tick (0 = free, 1 = busy)
    // 2688 bits = 336 bytes per month (4 weeks * 7 days * 96 ticks)
    bits: Vec<u64>, // Using u64 chunks for efficiency
}

impl AvailabilityBitSet {
    pub fn new() -> Self {
        Self {
            bits: vec![0u64; 42], // 42 * 64 = 2688 bits
        }
    }

    pub fn set_busy(&mut self, tick_in_month: u32, duration: u8) {
        for i in 0..duration as u32 {
            let bit_pos = tick_in_month + i;
            if bit_pos < 2688 {
                let chunk = (bit_pos / 64) as usize;
                let bit = bit_pos % 64;
                self.bits[chunk] |= 1u64 << bit;
            }
        }
    }

    pub fn set_free(&mut self, tick_in_month: u32, duration: u8) {
        for i in 0..duration as u32 {
            let bit_pos = tick_in_month + i;
            if bit_pos < 2688 {
                let chunk = (bit_pos / 64) as usize;
                let bit = bit_pos % 64;
                self.bits[chunk] &= !(1u64 << bit);
            }
        }
    }

    pub fn is_free(&self, tick_in_month: u32, duration: u8) -> bool {
        for i in 0..duration as u8 {
            let bit_pos = tick_in_month + i as u32;
            if bit_pos >= 2688 { return false; }

            let chunk = (bit_pos / 64) as usize;
            let bit = bit_pos % 64;
            if (self.bits[chunk] & (1u64 << bit)) != 0 {
                return false;
            }
        }
        true
    }

    // Find next free slot of given duration starting from tick_in_month
    pub fn find_next_free(&self, start_tick: u32, duration: u8) -> Option<u32> {
        let mut current = start_tick;
        while current + duration as u32 <= 2688 {
            if self.is_free(current, duration) {
                return Some(current);
            }
            current += 1;
        }
        None
    }

    // Bulk check: are ALL specified time slots free?
    pub fn are_slots_free(&self, slots: &[u32]) -> bool {
        for &slot in slots {
            if slot >= 2688 { return false; }
            let chunk = (slot / 64) as usize;
            let bit = slot % 64;
            if (self.bits[chunk] & (1u64 << bit)) != 0 {
                return false;
            }
        }
        true
    }
}

// For complex range queries and overlaps
#[derive(Debug, Default)]
pub struct IntervalTree {
    // Simplified interval tree - in practice you'd want a proper implementation
    intervals: Vec<(u64, u64, u64)>, // (start_tick, end_tick, event_id)
}

impl IntervalTree {
    pub fn insert(&mut self, start: u64, end: u64, event_id: u64) {
        self.intervals.push((start, end, event_id));
        // In a real implementation, you'd maintain a balanced tree structure
    }

    pub fn query_overlapping(&self, start: u64, end: u64) -> Vec<u64> {
        self.intervals.iter()
            .filter(|(s, e, _)| *s < end && *e > start)
            .map(|(_, _, id)| *id)
            .collect()
    }

    pub fn remove_event(&mut self, event_id: u64) {
        self.intervals.retain(|(_, _, id)| *id != event_id);
    }
}

// Month-aware availability tracking
#[derive(Debug, Default)]
pub struct MonthlyAvailability {
    // Map from month number to availability bitset
    months: HashMap<u32, AvailabilityBitSet>,
}

impl MonthlyAvailability {
    pub fn new() -> Self {
        Self {
            months: HashMap::new(),
        }
    }

    pub fn ensure_month(&mut self, year: u16, month: u8) {
        let month_key = (year as u32) * 256 + month as u32;
        self.months.entry(month_key).or_insert_with(AvailabilityBitSet::new);
    }

    fn month_key(year: u16, month: u8) -> u32 {
        (year as u32) * 256 + month as u32
    }

    pub fn set_busy(&mut self, start_time: SimDate, duration_ticks: u8) {
        let end_time = SimDate::from(start_time.to_tick() + duration_ticks as u64);

        if start_time.month() == end_time.month() && start_time.year == end_time.year {
            // Single month event
            let month_key = Self::month_key(start_time.year, start_time.month());
            if let Some(bitset) = self.months.get_mut(&month_key) {
                bitset.set_busy(start_time.tick_in_month(), duration_ticks);
            }
        } else {
            // Cross-month event - split it
            let start_month_key = Self::month_key(start_time.year, start_time.month());
            let end_month_key = Self::month_key(end_time.year, end_time.month());

            // Calculate end of start month
            let start_month_end = SimDate::month_start(start_time.year, start_time.month() + 1);
            let ticks_in_start_month = (start_month_end.to_tick() - start_time.to_tick()) as u8;

            // Set busy in start month
            if let Some(bitset) = self.months.get_mut(&start_month_key) {
                bitset.set_busy(start_time.tick_in_month(), ticks_in_start_month);
            }

            // Set busy in end month
            let remaining_ticks = duration_ticks - ticks_in_start_month;
            if let Some(bitset) = self.months.get_mut(&end_month_key) {
                bitset.set_busy(0, remaining_ticks); // Start from beginning of month
            }
        }
    }

    pub fn set_free(&mut self, start_time: SimDate, duration_ticks: u8) {
        let end_time = SimDate::from(start_time.to_tick() + duration_ticks as u64);

        if start_time.month() == end_time.month() && start_time.year == end_time.year {
            // Single month event
            let month_key = Self::month_key(start_time.year, start_time.month());
            if let Some(bitset) = self.months.get_mut(&month_key) {
                bitset.set_free(start_time.tick_in_month(), duration_ticks);
            }
        } else {
            // Cross-month event - split it
            let start_month_key = Self::month_key(start_time.year, start_time.month());
            let end_month_key = Self::month_key(end_time.year, end_time.month());

            // Calculate end of start month
            let start_month_end = SimDate::month_start(start_time.year, start_time.month() + 1);
            let ticks_in_start_month = (start_month_end.to_tick() - start_time.to_tick()) as u8;

            // Set free in start month
            if let Some(bitset) = self.months.get_mut(&start_month_key) {
                bitset.set_free(start_time.tick_in_month(), ticks_in_start_month);
            }

            // Set free in end month
            let remaining_ticks = duration_ticks - ticks_in_start_month;
            if let Some(bitset) = self.months.get_mut(&end_month_key) {
                bitset.set_free(0, remaining_ticks); // Start from beginning of month
            }
        }
    }

    pub fn is_free(&self, start_time: SimDate, duration_ticks: u8) -> bool {
        let end_time = SimDate::from(start_time.to_tick() + duration_ticks as u64);

        if start_time.month() == end_time.month() && start_time.year == end_time.year {
            // Single month check
            let month_key = Self::month_key(start_time.year, start_time.month());
            if let Some(bitset) = self.months.get(&month_key) {
                bitset.is_free(start_time.tick_in_month(), duration_ticks)
            } else {
                true // No bitset = completely free
            }
        } else {
            // Cross-month check
            let start_month_key = Self::month_key(start_time.year, start_time.month());
            let end_month_key = Self::month_key(end_time.year, end_time.month());

            // Calculate split
            let start_month_end = SimDate::month_start(start_time.year, start_time.month() + 1);
            let ticks_in_start_month = (start_month_end.to_tick() - start_time.to_tick()) as u8;
            let remaining_ticks = duration_ticks - ticks_in_start_month;

            // Check both months
            let start_month_free = if let Some(bitset) = self.months.get(&start_month_key) {
                bitset.is_free(start_time.tick_in_month(), ticks_in_start_month)
            } else {
                true
            };

            let end_month_free = if let Some(bitset) = self.months.get(&end_month_key) {
                bitset.is_free(0, remaining_ticks)
            } else {
                true
            };

            start_month_free && end_month_free
        }
    }
}

// Optimized lookup structure for queries
#[derive(Debug, Default)]
pub struct CalendarIndex {
    // Map from year-week to events that occur on that week
    pub events_by_week: BTreeMap<(u16, u8), Vec<u64>>,
    // Map from absolute tick to events at that time
    pub events_by_tick: BTreeMap<u64, Vec<u64>>,
    // Map from participant to their events
    pub events_by_participant: HashMap<Entity, Vec<u64>>,
    // All events by ID for quick lookup
    pub events: HashMap<u64, CalendarEvent>,

    // Month-aware availability tracking
    pub availability_matrix: HashMap<Entity, MonthlyAvailability>,
    // Spatial index for range queries
    pub time_ranges: IntervalTree,
}

impl CalendarIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ensure_months_for_current_and_next(&mut self, current_date: SimDate) {
        let current_month = current_date.month();
        let (next_year, next_month) = if current_month == 13 { // Assuming 13 months per year max
            (current_date.year + 1, 1)
        } else {
            (current_date.year, current_month + 1)
        };

        // Ensure current and next month bitsets exist for all people
        for availability in self.availability_matrix.values_mut() {
            availability.ensure_month(current_date.year, current_month);
            availability.ensure_month(next_year, next_month);
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
            let end_tick = start_tick + event.duration_ticks as u64;

            for tick_offset in 0..event.duration_ticks as u64 {
                self.events_by_tick.entry(start_tick + tick_offset).or_default().push(event_id);
            }

            // Update availability matrix for all participants
            for participant in &event.participants {
                let availability = self.availability_matrix.entry(*participant).or_insert_with(MonthlyAvailability::new);
                availability.ensure_month(occurrence.year, occurrence.month());
                availability.set_busy(occurrence, event.duration_ticks);
            }

            // Add to interval tree
            self.time_ranges.insert(start_tick, end_tick, event_id);
        }

        // Index by participants
        for participant in &event.participants {
            self.events_by_participant.entry(*participant).or_default().push(event_id);
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
                for participant in &event.participants {
                    if let Some(availability) = self.availability_matrix.get_mut(participant) {
                        availability.set_free(occurrence, event.duration_ticks);
                    }
                }
            }
        }
    }

    // OPTIMIZED QUERY METHODS

    pub fn is_person_free(&self, person: Entity, start_time: SimDate, duration_ticks: u8) -> bool {
        if let Some(availability) = self.availability_matrix.get(&person) {
            availability.is_free(start_time, duration_ticks)
        } else {
            true // No calendar = completely free
        }
    }

    pub fn are_people_free(&self, people: &[Entity], start_time: SimDate, duration_ticks: u8) -> bool {
        for person in people {
            if !self.is_person_free(*person, start_time, duration_ticks) {
                return false;
            }
        }
        true
    }

    pub fn find_common_free_time(&self, people: &[Entity], duration_ticks: u8, start_from: SimDate) -> Option<SimDate> {
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

    pub fn get_conflicts(&self, people: &[Entity], start_time: SimDate, duration_ticks: u8) -> Vec<(Entity, Vec<&CalendarEvent>)> {
        let start_tick = start_time.to_tick();
        let end_tick = start_tick + duration_ticks as u64;
        let mut conflicts = Vec::new();

        for &person in people {
            if !self.is_person_free(person, start_time, duration_ticks) {
                // Find which specific events are conflicting
                let conflicting_events:Vec<_> = self.time_ranges.query_overlapping(start_tick, end_tick)
                    .into_iter()
                    .filter_map(|event_id| self.events.get(&event_id))
                    .filter(|event| event.participants.contains(&person))
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

// ECS System for calendar management
pub struct CalendarSystem;

impl CalendarSystem {
    pub fn update_calendar_indices(world: &mut World) {
        // This would rebuild indices when calendars change
        // Implementation depends on your change detection strategy
    }

    pub fn process_recurring_events(world: &mut World, current_date: SimDate) {
        // Handle any recurring event logic that needs processing
    }

    pub fn advance_month(world: &mut World, new_current_date: SimDate) {
        // Create bitsets for the new next month
        // This would be called at the beginning of each new month
    }
}

// Helper functions for common operations
pub fn schedule_meeting(
    company_calendar: &mut CompanyCalendar,
    title: String,
    participants: Vec<Entity>,
    start_time: SimDate,
    duration_ticks: u8,
    recurrence: RecurrencePattern,
) -> Result<u64, String> {
    // Generate unique ID (you might want to use a proper ID generator)
    let event_id = rand::random::<u64>();

    let event = CalendarEvent {
        id: event_id,
        title,
        start_time,
        duration_ticks,
        recurrence,
        participants,
        event_type: EventType::Meeting,
        priority: EventPriority::Normal,
    };

    company_calendar.events.push(event);
    Ok(event_id)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_recurring_events_weekly() {
        let event = CalendarEvent {
            id: 1,
            title: "Weekly Meeting".to_string(),
            start_time: SimDate { year: 1, week: 1, day: 2, quarter_tick: 37 }, // Tuesday 9:15 AM
            duration_ticks: 4, // 1 hour
            recurrence: RecurrencePattern::Weekly,
            participants: vec![],
            event_type: EventType::Meeting,
            priority: EventPriority::Normal,
        };

        // Should occur on same day of week
        assert!(event.occurs_on_date(SimDate { year: 1, week: 2, day: 2, quarter_tick: 1 })); // Next Tuesday
        assert!(!event.occurs_on_date(SimDate { year: 1, week: 2, day: 3, quarter_tick: 1 })); // Wednesday
    }

    #[test]
    fn test_availability_bitset_cross_month() {
        let mut availability = MonthlyAvailability::new();
        availability.ensure_month(1, 1);
        availability.ensure_month(1, 2);

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