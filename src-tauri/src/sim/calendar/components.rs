use legion::*;
use std::collections::{HashMap, BTreeMap, HashSet};
use serde::{Serialize, Deserialize};

// Core time representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GameTime {
    pub day: u8,    // 1-28
    pub tick: u8,   // 0-95 (24 hours * 4 ticks per hour)
}

impl GameTime {
    pub fn new(day: u8, hour: u8, minute: u8) -> Self {
        assert!(day >= 1 && day <= 28);
        assert!(hour < 24 && minute < 60);
        let tick = (hour * 4) + (minute / 15);
        Self { day, tick }
    }

    pub fn to_absolute_tick(&self) -> u32 {
        (self.day as u32 - 1) * 96 + self.tick as u32
    }

    pub fn from_absolute_tick(tick: u32) -> Self {
        let day = (tick / 96) + 1;
        let tick_in_day = tick % 96;
        Self {
            day: day as u8,
            tick: tick_in_day as u8
        }
    }

    pub fn week(&self) -> u8 {
        (self.day - 1) / 7 + 1
    }

    pub fn day_of_week(&self) -> u8 {
        (self.day - 1) % 7 + 1
    }

    pub fn hour(&self) -> u8 {
        self.tick / 4
    }

    pub fn minute(&self) -> u8 {
        (self.tick % 4) * 15
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecurrencePattern {
    None,
    Daily,
    Weekly,
    Weekdays,  // Mon-Fri
    Custom { days_of_week: Vec<u8> }, // 1=Monday, 7=Sunday
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: u64,
    pub title: String,
    pub start_time: GameTime,
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
    pub fn end_time(&self) -> GameTime {
        let end_tick = self.start_time.to_absolute_tick() + self.duration_ticks as u32;
        GameTime::from_absolute_tick(end_tick)
    }

    pub fn occurs_on_day(&self, day: u8) -> bool {
        match &self.recurrence {
            RecurrencePattern::None => self.start_time.day == day,
            RecurrencePattern::Daily => true,
            RecurrencePattern::Weekly => {
                (day - self.start_time.day) % 7 == 0 && day >= self.start_time.day
            },
            RecurrencePattern::Weekdays => {
                let day_of_week = GameTime { day, tick: 0 }.day_of_week();
                day_of_week >= 1 && day_of_week <= 5
            },
            RecurrencePattern::Custom { days_of_week } => {
                let day_of_week = GameTime { day, tick: 0 }.day_of_week();
                days_of_week.contains(&day_of_week)
            }
        }
    }

    // Get all occurrences of this event within a day range
    pub fn get_occurrences(&self, start_day: u8, end_day: u8) -> Vec<GameTime> {
        let mut occurrences = Vec::new();

        for day in start_day..=end_day {
            if self.occurs_on_day(day) {
                occurrences.push(GameTime {
                    day,
                    tick: self.start_time.tick,
                });
            }
        }

        occurrences
    }
}

// ECS Components
#[derive(Debug, Clone)]
pub struct PersonalCalendar {
    pub events: Vec<CalendarEvent>,
    pub blocked_times: Vec<(GameTime, GameTime)>, // Manually blocked time slots
}

#[derive(Debug, Clone)]
pub struct CompanyCalendar {
    pub events: Vec<CalendarEvent>,
    pub meeting_rooms: HashMap<String, Vec<CalendarEvent>>,
}

// Optimized lookup structure for queries
#[derive(Debug, Default)]
pub struct CalendarIndex {
    // Map from day to events that occur on that day
    pub events_by_day: BTreeMap<u8, Vec<u64>>,
    // Map from tick (absolute) to events at that time
    pub events_by_tick: BTreeMap<u32, Vec<u64>>,
    // Map from participant to their events
    pub events_by_participant: HashMap<Entity, Vec<u64>>,
    // All events by ID for quick lookup
    pub events: HashMap<u64, CalendarEvent>,

    // NEW: Optimized availability tracking
    // BitSet per person: each bit represents a 15-min slot (28 days * 96 ticks = 2688 bits)
    pub availability_matrix: HashMap<Entity, AvailabilityBitSet>,

}

// Compact representation of a person's schedule
#[derive(Debug, Clone)]
pub struct AvailabilityBitSet {
    // Each bit represents one 15-minute tick (0 = free, 1 = busy)
    // 2688 bits = 336 bytes per person
    bits: Vec<u64>, // Using u64 chunks for efficiency
}

impl AvailabilityBitSet {
    pub fn new() -> Self {
        Self {
            bits: vec![0u64; 42], // 42 * 64 = 2688 bits
        }
    }

    pub fn set_busy(&mut self, tick: u32, duration: u8) {
        for i in 0..duration as u32 {
            let bit_pos = tick + i;
            if bit_pos < 2688 {
                let chunk = (bit_pos / 64) as usize;
                let bit = bit_pos % 64;
                self.bits[chunk] |= 1u64 << bit;
            }
        }
    }

    pub fn set_free(&mut self, tick: u32, duration: u8) {
        for i in 0..duration as u32 {
            let bit_pos = tick + i;
            if bit_pos < 2688 {
                let chunk = (bit_pos / 64) as usize;
                let bit = bit_pos % 64;
                self.bits[chunk] &= !(1u64 << bit);
            }
        }
    }

    pub fn is_free(&self, tick: u32, duration: u8) -> bool {
        for i in 0..duration as u8 {
            let bit_pos = tick + i as u32;
            if bit_pos >= 2688 { return false; }

            let chunk = (bit_pos / 64) as usize;
            let bit = bit_pos % 64;
            if (self.bits[chunk] & (1u64 << bit)) != 0 {
                return false;
            }
        }
        true
    }

    // Find next free slot of given duration starting from tick
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



impl CalendarIndex {
    pub fn add_event(&mut self, event: CalendarEvent) {
        let event_id = event.id;

        // Index by days this event occurs
        for day in 1..=28 {
            if event.occurs_on_day(day) {
                self.events_by_day.entry(day).or_default().push(event_id);
            }
        }

        // Index by absolute ticks and update availability matrix
        for day in 1..=28 {
            if event.occurs_on_day(day) {
                let start_tick = GameTime { day, tick: event.start_time.tick }.to_absolute_tick();
                let end_tick = start_tick + event.duration_ticks as u32;

                // Traditional indexing
                for tick_offset in 0..event.duration_ticks as u32 {
                    self.events_by_tick.entry(start_tick + tick_offset).or_default().push(event_id);
                }

                // Update availability matrix for all participants
                for participant in &event.participants {
                    let availability = self.availability_matrix.entry(*participant).or_insert_with(AvailabilityBitSet::new);
                    availability.set_busy(start_tick, event.duration_ticks);
                }

            }
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
            for day_events in self.events_by_day.values_mut() {
                day_events.retain(|&id| id != event_id);
            }

            for tick_events in self.events_by_tick.values_mut() {
                tick_events.retain(|&id| id != event_id);
            }

            for participant_events in self.events_by_participant.values_mut() {
                participant_events.retain(|&id| id != event_id);
            }
        }
    }

    // OPTIMIZED QUERY METHODS

    // O(1) availability check using bitset
    pub fn is_person_free_fast(&self, person: Entity, start_time: GameTime, duration_ticks: u8) -> bool {
        if let Some(availability) = self.availability_matrix.get(&person) {
            availability.is_free(start_time.to_absolute_tick(), duration_ticks)
        } else {
            true // No calendar = completely free
        }
    }

    // O(participants) bulk availability check
    pub fn are_people_free(&self, people: &[Entity], start_time: GameTime, duration_ticks: u8) -> bool {
        let start_tick = start_time.to_absolute_tick();

        for person in people {
            if let Some(availability) = self.availability_matrix.get(person) {
                if !availability.is_free(start_tick, duration_ticks) {
                    return false;
                }
            }
        }
        true
    }

    // Find common free time for multiple people
    pub fn find_common_free_time(&self, people: &[Entity], duration_ticks: u8, start_from: GameTime) -> Option<GameTime> {
        let mut current_tick = start_from.to_absolute_tick();
        let max_tick = 28 * 96;

        'outer: while current_tick + duration_ticks as u32 <= max_tick {
            // Check if ALL people are free at this time
            for person in people {
                if let Some(availability) = self.availability_matrix.get(person) {
                    if !availability.is_free(current_tick, duration_ticks) {
                        current_tick += 1;
                        continue 'outer;
                    }
                }
            }

            // All people are free!
            return Some(GameTime::from_absolute_tick(current_tick));
        }

        None
    }


    // Batch query: get availability for multiple time slots at once
    pub fn check_availability_batch(&self, person: Entity, time_slots: &[(GameTime, u8)]) -> Vec<bool> {
        if let Some(availability) = self.availability_matrix.get(&person) {
            time_slots.iter()
                .map(|(time, duration)| availability.is_free(time.to_absolute_tick(), *duration))
                .collect()
        } else {
            vec![true; time_slots.len()]
        }
    }
    pub fn events_on_day(&self, day: u8) -> Vec<&CalendarEvent> {
        self.events_by_day.get(&day)
            .map(|event_ids| {
                event_ids.iter()
                    .filter_map(|&id| self.events.get(&id))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn events_in_week(&self, week: u8) -> Vec<&CalendarEvent> {
        let start_day = (week - 1) * 7 + 1;
        let end_day = week * 7;

        let mut events = HashSet::new();
        for day in start_day..=end_day {
            if let Some(day_events) = self.events_by_day.get(&day) {
                for &event_id in day_events {
                    events.insert(event_id);
                }
            }
        }

        events.iter()
            .filter_map(|&id| self.events.get(&id))
            .collect()
    }

    pub fn events_at_time(&self, time: GameTime) -> Vec<&CalendarEvent> {
        let tick = time.to_absolute_tick();
        self.events_by_tick.get(&tick)
            .map(|event_ids| {
                event_ids.iter()
                    .filter_map(|&id| self.events.get(&id))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn is_person_free(&self, person: Entity, start_time: GameTime, duration_ticks: u8) -> bool {
        if let Some(person_events) = self.events_by_participant.get(&person) {
            for &event_id in person_events {
                if let Some(event) = self.events.get(&event_id) {
                    for day in 1..=28 {
                        if event.occurs_on_day(day) {
                            let event_start = GameTime { day, tick: event.start_time.tick };
                            let event_end_tick = event_start.to_absolute_tick() + event.duration_ticks as u32;
                            let query_start_tick = start_time.to_absolute_tick();
                            let query_end_tick = query_start_tick + duration_ticks as u32;

                            // Check for overlap
                            if query_start_tick < event_end_tick && query_end_tick > event_start.to_absolute_tick() {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }

    pub fn find_next_free_slot(&self, person: Entity, duration_ticks: u8, start_from: GameTime) -> Option<GameTime> {
        let mut current_tick = start_from.to_absolute_tick();
        let max_tick = 28 * 96; // End of month

        while current_tick + duration_ticks as u32 <= max_tick {
            let test_time = GameTime::from_absolute_tick(current_tick);
            if self.is_person_free(person, test_time, duration_ticks) {
                return Some(test_time);
            }
            current_tick += 1; // Move to next 15-minute slot
        }

        None
    }
}

// ECS System for calendar management
pub struct CalendarSystem;

impl CalendarSystem {
    pub fn update_calendar_indices(world: &mut World) {
        // This would rebuild indices when calendars change
        // Implementation depends on your change detection strategy
    }

    pub fn process_recurring_events(world: &mut World, current_day: u8) {
        // Handle any recurring event logic that needs processing
    }
}

// Helper functions for common operations
pub fn schedule_meeting(
    company_calendar: &mut CompanyCalendar,
    title: String,
    participants: Vec<Entity>,
    start_time: GameTime,
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
    fn test_game_time() {
        let time = GameTime::new(1, 9, 30); // Day 1, 9:30 AM
        assert_eq!(time.day, 1);
        assert_eq!(time.tick, 38); // 9 * 4 + 2
        assert_eq!(time.hour(), 9);
        assert_eq!(time.minute(), 30);
    }

    #[test]
    fn test_recurring_events() {
        let event = CalendarEvent {
            id: 1,
            title: "Daily Standup".to_string(),
            start_time: GameTime::new(1, 9, 0),
            duration_ticks: 2, // 30 minutes
            recurrence: RecurrencePattern::Weekdays,
            participants: vec![],
            event_type: EventType::Meeting,
            priority: EventPriority::Normal,
        };

        // Should occur on weekdays
        assert!(event.occurs_on_day(1)); // Monday
        assert!(event.occurs_on_day(5)); // Friday
        assert!(!event.occurs_on_day(6)); // Saturday
        assert!(!event.occurs_on_day(7)); // Sunday
    }
}