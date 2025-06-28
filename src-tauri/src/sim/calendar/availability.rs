use crate::sim::sim_date::sim_date::SimDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bincode::{Decode, Encode};
use tauri::CursorIcon::Hand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Encode, Decode)]
pub struct YearMonth {
    pub year: u16,
    pub month: u8, // 1-13 for your 13-month year
}

impl YearMonth {
    pub fn new(year: u16, month: u8) -> Self {
        debug_assert!(month >= 1 && month <= 13, "Month must be between 1 and 13");
        Self { year, month }
    }

    pub fn next_month(&self) -> Self {
        if self.month == 13 {
            Self::new(self.year + 1, 1)
        } else {
            Self::new(self.year, self.month + 1)
        }
    }

    pub fn prev_month(&self) -> Self {
        if self.month == 1 {
            Self::new(self.year - 1, 13)
        } else {
            Self::new(self.year, self.month - 1)
        }
    }

    /// Get the first week of this month (1-based)
    pub fn first_week(&self) -> u8 {
        ((self.month - 1) * 4) + 1
    }

    /// Get the last week of this month (1-based)
    pub fn last_week(&self) -> u8 {
        self.month * 4
    }

    pub fn contains_week(&self, week: u8) -> bool {
        week >= self.first_week() && week <= self.last_week()
    }
}

impl From<SimDate> for YearMonth {
    fn from(date: SimDate) -> Self {
        // Convert week to month (weeks 1-4 = month 1, weeks 5-8 = month 2, etc.)
        let month = ((date.week - 1) / 4) + 1;
        Self::new(date.year, month)
    }
}

impl From<&SimDate> for YearMonth {
    fn from(date: &SimDate) -> Self {
        YearMonth::from(*date)
    }
}

#[derive(Debug, Clone, Encode, Decode, Serialize, Deserialize)]
pub struct AvailabilityDetail {
    /// Main availability bitset (0 = free, 1 = busy for any reason)
    pub availability: AvailabilityBitSet,
    /// Working hours bitset (1 = outside working hours)
    pub working_hours: AvailabilityBitSet,
}

impl AvailabilityDetail {
    pub fn new() -> Self {
        Self {
            availability: AvailabilityBitSet::new(),
            working_hours: AvailabilityBitSet::new(),
        }
    }
}

impl Default for AvailabilityDetail {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct MonthlyAvailability {
    availability: HashMap<YearMonth, AvailabilityDetail>,
}
impl Default for MonthlyAvailability{
    fn default() -> Self {
        let availability = HashMap::<YearMonth,AvailabilityDetail>::new();
        Self{
            availability,
        }   
    }
}

impl MonthlyAvailability {
    pub fn new() -> Self {
        Self {
            availability: HashMap::new(),
        }
    }

    pub fn set_busy(&mut self, start_time: SimDate, duration_ticks: u8) {
        let end_time = SimDate::from(start_time.to_tick() + duration_ticks as u64);

        if start_time.month() == end_time.month() && start_time.year == end_time.year {
            // Single month event
            let year_month = YearMonth::from(&start_time);
            let detail = self.get_or_create_month(year_month);
            detail.availability.set_busy(start_time.tick_in_month(), duration_ticks);
        } else {
            // Cross-month event - split it
            let start_month_key = YearMonth::from(&start_time);
            let end_month_key = YearMonth::from(&end_time);

            // Calculate end of start month
            let start_month_end = SimDate::month_start(start_time.year, start_time.month() + 1);
            let ticks_in_start_month = (start_month_end.to_tick() - start_time.to_tick()) as u8;

            // Set busy in start month
            let start_detail = self.get_or_create_month(start_month_key);
            start_detail.availability.set_busy(start_time.tick_in_month(), ticks_in_start_month);

            // Set busy in end month
            let remaining_ticks = duration_ticks - ticks_in_start_month;
            let end_detail = self.get_or_create_month(end_month_key);
            end_detail.availability.set_busy(0, remaining_ticks); // Start from beginning of month
        }
    }

    pub fn set_free(&mut self, start_time: SimDate, duration_ticks: u8) {
        let end_time = SimDate::from(start_time.to_tick() + duration_ticks as u64);

        if start_time.month() == end_time.month() && start_time.year == end_time.year {
            // Single month event
            let month_key = YearMonth::from(&start_time);
            let detail = self.get_or_create_month(month_key);
            detail.availability.set_free(start_time.tick_in_month(), duration_ticks);
        } else {
            // Cross-month event - split it
            let start_month_key = YearMonth::from(&start_time);
            let end_month_key = YearMonth::from(&end_time);

            // Calculate end of start month
            let start_month_end = SimDate::month_start(start_time.year, start_time.month() + 1);
            let ticks_in_start_month = (start_month_end.to_tick() - start_time.to_tick()) as u8;

            // Set free in start month
            let start_detail = self.get_or_create_month(start_month_key);
            start_detail.availability.set_free(start_time.tick_in_month(), ticks_in_start_month);

            // Set free in end month
            let remaining_ticks = duration_ticks - ticks_in_start_month;
            let end_detail = self.get_or_create_month(end_month_key);
            end_detail.availability.set_free(0, remaining_ticks); // Start from beginning of month
        }
    }

    pub fn is_free(&self, start_time: SimDate, duration_ticks: u8) -> bool {
        let end_time = SimDate::from(start_time.to_tick() + duration_ticks as u64);

        if start_time.month() == end_time.month() && start_time.year == end_time.year {
            // Single month check
            let month_key = YearMonth::from(&start_time);
            if let Some(detail) = self.availability.get(&month_key) {
                detail.availability.is_free(start_time.tick_in_month(), duration_ticks)
            } else {
                true // No bitset = completely free
            }
        } else {
            // Cross-month check
            let start_month_key = YearMonth::from(&start_time);
            let end_month_key = YearMonth::from(&end_time);

            // Calculate split
            let start_month_end = SimDate::month_start(start_time.year, start_time.month() + 1);
            let ticks_in_start_month = (start_month_end.to_tick() - start_time.to_tick()) as u8;
            let remaining_ticks = duration_ticks - ticks_in_start_month;

            // Check both months
            let start_month_free =
                if let Some(detail) = self.availability.get(&start_month_key) {
                    detail.availability.is_free(start_time.tick_in_month(), ticks_in_start_month)
                } else {
                    true
                };

            let end_month_free = if let Some(detail) = self.availability.get(&end_month_key) {
                detail.availability.is_free(0, remaining_ticks)
            } else {
                true
            };

            start_month_free && end_month_free
        }
    }

    /// Get availability detail for a specific month, creating if it doesn't exist
    pub fn get_or_create_month(&mut self, year_month: YearMonth) -> &mut AvailabilityDetail {
        self.availability
            .entry(year_month)
            .or_insert_with(|| AvailabilityDetail::new())
    }

    /// Get availability detail for a specific month (read-only)
    pub fn get_month(&self, year_month: YearMonth) -> Option<&AvailabilityDetail> {
        self.availability.get(&year_month)
    }

    /// Get availability detail for the month containing the given date
    pub fn get_month_for_date(&self, date: &SimDate) -> Option<&AvailabilityDetail> {
        let year_month = YearMonth::from(date);
        self.get_month(year_month)
    }

    /// Get or create availability detail for the month containing the given date
    pub fn get_or_create_month_for_date(&mut self, date: &SimDate) -> &mut AvailabilityDetail {
        let year_month = YearMonth::from(date);
        self.get_or_create_month(year_month)
    }

    /// Remove a specific month's availability (for cold storage)
    pub fn remove_month(&mut self, year_month: YearMonth) -> Option<AvailabilityDetail> {
        self.availability.remove(&year_month)
    }

    /// Remove all months before the specified month (for cold storage cleanup)
    pub fn remove_months_before(&mut self, cutoff: YearMonth) -> Vec<(YearMonth, AvailabilityDetail)> {
        let keys_to_remove: Vec<YearMonth> = self
            .availability
            .keys()
            .filter(|&&key| key.year < cutoff.year || (key.year == cutoff.year && key.month < cutoff.month))
            .copied()
            .collect();

        let mut removed = Vec::new();
        for key in keys_to_remove {
            if let Some(detail) = self.availability.remove(&key) {
                removed.push((key, detail));
            }
        }
        removed
    }

    /// Get all available months
    pub fn get_available_months(&self) -> Vec<YearMonth> {
        let mut months: Vec<_> = self.availability.keys().copied().collect();
        months.sort_by(|a, b| {
            a.year.cmp(&b.year).then_with(|| a.month.cmp(&b.month))
        });
        months
    }

    /// Check if we have availability data for a specific month
    pub fn has_month(&self, year_month: YearMonth) -> bool {
        self.availability.contains_key(&year_month)
    }

    /// Get the total number of months stored
    pub fn month_count(&self) -> usize {
        self.availability.len()
    }

    /// Clear all stored availability data
    pub fn clear(&mut self) {
        self.availability.clear();
    }
}




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvailabilityState {
    Free,   // 0 in bitset
    Busy,   // 1 in bitset
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct AvailabilityBitSet {
    // Each bit represents one 15-minute tick (0 = free, 1 = busy)
    // MAX_TICKS_IN_MONTH bits = 336 bytes per month (4 weeks * 7 days * 96 ticks)
    bits: Vec<u64>, // Using u64 chunks for efficiency
}

impl AvailabilityBitSet {
    pub const MAX_TICKS_IN_MONTH: u32 = 4 * 7 * 96; // 4 weeks * 7 days/week * 96 ticks/day
    pub const TICKS_PER_DAY: u32 = 96; // 24 hours * 4 ticks/hour
    pub const TICKS_PER_WEEK: u32 = 7 * 96; // 7 days * 96 ticks/day

    pub fn new() -> Self {
        Self {
            bits: vec![0u64; 42], // 42 * 64 = 2688 bits
        }
    }

    pub fn set_busy(&mut self, tick_in_month: u32, duration: u8) {
        for i in 0..duration as u32 {
            let bit_pos = tick_in_month + i;
            if bit_pos < Self::MAX_TICKS_IN_MONTH {
                let chunk = (bit_pos / 64) as usize;
                let bit = bit_pos % 64;
                self.bits[chunk] |= 1u64 << bit;
            }
        }
    }

    pub fn set_free(&mut self, tick_in_month: u32, duration: u8) {
        for i in 0..duration as u32 {
            let bit_pos = tick_in_month + i;
            if bit_pos < Self::MAX_TICKS_IN_MONTH {
                let chunk = (bit_pos / 64) as usize;
                let bit = bit_pos % 64;
                self.bits[chunk] &= !(1u64 << bit);
            }
        }
    }

    pub fn is_free(&self, tick_in_month: u32, duration: u8) -> bool {
        for i in 0..duration {
            let bit_pos = tick_in_month + i as u32;
            if bit_pos >= Self::MAX_TICKS_IN_MONTH { return false; }
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
        while current + duration as u32 <= Self::MAX_TICKS_IN_MONTH {
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
            if slot >= Self::MAX_TICKS_IN_MONTH { return false; }
            let chunk = (slot / 64) as usize;
            let bit = slot % 64;
            if (self.bits[chunk] & (1u64 << bit)) != 0 {
                return false;
            }
        }
        true
    }

    // ===== NEW RECURRING PATTERN API =====

    /// Convert 1-based day and 1-based tick to absolute tick in month
    ///
    /// # Arguments
    /// * `week` - Week number (1-4)
    /// * `day` - Day of week (1=Monday, 7=Sunday) 
    /// * `tick` - Tick of day (1=00:00-00:15, 2=00:15-00:30, etc.)
    ///
    /// # Returns
    /// Absolute tick position in month (0-based internally)
    pub fn day_tick_to_month_tick(week: u8, day: u8, tick: u8) -> Option<u32> {
        if week < 1 || week > 4 || day < 1 || day > 7 || tick < 1 || tick > 96 {
            return None;
        }
        let week_offset = (week - 1) as u32 * Self::TICKS_PER_WEEK;
        let day_offset = (day - 1) as u32 * Self::TICKS_PER_DAY;
        let tick_offset = (tick - 1) as u32;

        Some(week_offset + day_offset + tick_offset)
    }

    /// Set availability for a time range on specific days with recurrence pattern
    ///
    /// # Arguments
    /// * `start_tick` - Starting tick of day (1-96, where 1 = 00:00-00:15)
    /// * `end_tick` - Ending tick of day (1-96, INCLUSIVE)
    /// * `weekdays` - Which days to apply this pattern to
    /// * `state` - Whether to mark as Free or Busy
    ///
    /// # Example
    /// ```
    /// // Set busy 9:00 AM to 11:30 AM (ticks 37-46) on Monday, Wednesday, Friday
    /// availability.set_recurring_pattern(
    ///     37, 46, 
    ///     &[Weekday::Monday, Weekday::Wednesday, Weekday::Friday],
    ///     AvailabilityState::Busy
    /// );
    /// ```
    pub fn set_recurring_pattern(
        &mut self,
        start_tick: u8,
        end_tick: u8,
        weekdays: &[Weekday],
        state: AvailabilityState
    ) {
        if start_tick < 1 || start_tick > 96 || end_tick < 1 || end_tick > 96 || start_tick > end_tick {
            return; // Invalid input
        }

        let duration = (end_tick - start_tick + 1) as u8;

        // Apply pattern to each week (1-4) and each specified weekday
        for week in 1..=4 {
            for &weekday in weekdays {
                if let Some(month_tick) = Self::day_tick_to_month_tick(week, weekday as u8, start_tick) {
                    match state {
                        AvailabilityState::Free => self.set_free(month_tick, duration),
                        AvailabilityState::Busy => self.set_busy(month_tick, duration),
                    }
                }
            }
        }
    }

    /// Convenience method: Set busy for recurring pattern
    pub fn set_recurring_busy(&mut self, start_tick: u8, end_tick: u8, weekdays: &[Weekday]) {
        self.set_recurring_pattern(start_tick, end_tick, weekdays, AvailabilityState::Busy);
    }

    /// Convenience method: Set free for recurring pattern
    pub fn set_recurring_free(&mut self, start_tick: u8, end_tick: u8, weekdays: &[Weekday]) {
        self.set_recurring_pattern(start_tick, end_tick, weekdays, AvailabilityState::Free);
    }

    /// Set availability for ALL days of the week (daily pattern)
    pub fn set_daily_pattern(&mut self, start_tick: u8, end_tick: u8, state: AvailabilityState) {
        let all_days = [
            Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday,
            Weekday::Thursday, Weekday::Friday, Weekday::Saturday, Weekday::Sunday
        ];
        self.set_recurring_pattern(start_tick, end_tick, &all_days, state);
    }

    /// Convenience method: Set busy every day
    pub fn set_daily_busy(&mut self, start_tick: u8, end_tick: u8) {
        self.set_daily_pattern(start_tick, end_tick, AvailabilityState::Busy);
    }

    /// Convenience method: Set free every day  
    pub fn set_daily_free(&mut self, start_tick: u8, end_tick: u8) {
        self.set_daily_pattern(start_tick, end_tick, AvailabilityState::Free);
    }

    /// Set availability for weekdays only (Monday-Friday)
    pub fn set_weekdays_pattern(&mut self, start_tick: u8, end_tick: u8, state: AvailabilityState) {
        let weekdays = [
            Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday,
            Weekday::Thursday, Weekday::Friday
        ];
        self.set_recurring_pattern(start_tick, end_tick, &weekdays, state);
    }

    /// Set availability for weekends only (Saturday-Sunday)
    pub fn set_weekends_pattern(&mut self, start_tick: u8, end_tick: u8, state: AvailabilityState) {
        let weekends = [Weekday::Saturday, Weekday::Sunday];
        self.set_recurring_pattern(start_tick, end_tick, &weekends, state);
    }

    // ===== HELPER METHODS FOR TIME CONVERSION =====

    /// Convert hour and minute to day tick (1-based)
    ///
    /// # Arguments  
    /// * `hour` - Hour (0-23)
    /// * `minute` - Minute (0, 15, 30, or 45)
    ///
    /// # Returns
    /// Day tick (1-96) or None if invalid
    ///
    /// # Example
    /// ```
    /// let tick = AvailabilityBitSet::time_to_day_tick(9, 0); // 9:00 AM = tick 37
    /// ```
    pub fn time_to_day_tick(hour: u8, minute: u8) -> Option<u8> {
        if hour > 23 || minute > 59 || minute % 15 != 0 {
            return None;
        }
        let tick = (hour as u8 * 4) + (minute / 15) + 1; // +1 for 1-based
        Some(tick)
    }

    /// Convert day tick back to hour and minute
    ///
    /// # Arguments
    /// * `tick` - Day tick (1-96)
    ///
    /// # Returns  
    /// (hour, minute) or None if invalid tick
    pub fn day_tick_to_time(tick: u8) -> Option<(u8, u8)> {
        if tick < 1 || tick > 96 {
            return None;
        }
        let zero_based_tick = tick - 1;
        let hour = zero_based_tick / 4;
        let minute = (zero_based_tick % 4) * 15;
        Some((hour, minute))
    }

    /// Check if a specific day and time slot is free
    ///
    /// # Arguments
    /// * `week` - Week number (1-4)
    /// * `weekday` - Day of week
    /// * `tick` - Day tick (1-96)
    /// * `duration` - Duration in ticks
    pub fn is_day_slot_free(&self, week: u8, weekday: Weekday, tick: u8, duration: u8) -> bool {
        if let Some(month_tick) = Self::day_tick_to_month_tick(week, weekday as u8, tick) {
            self.is_free(month_tick, duration)
        } else {
            false
        }
    }
}

// ===== EXAMPLE USAGE =====
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurring_patterns() {
        let mut availability = AvailabilityBitSet::new();

        // Example: Set busy 9:00 AM to 11:30 AM on Monday, Wednesday, Friday
        let start_tick = AvailabilityBitSet::time_to_day_tick(9, 0).unwrap(); // tick 37
        let end_tick = AvailabilityBitSet::time_to_day_tick(11, 30).unwrap(); // tick 46

        availability.set_recurring_busy(
            start_tick,
            end_tick,
            &[Weekday::Monday, Weekday::Wednesday, Weekday::Friday]
        );

        // Check that Monday week 1 at 9:00 AM is busy
        assert!(!availability.is_day_slot_free(1, Weekday::Monday, start_tick, 1));

        // Check that Tuesday week 1 at 9:00 AM is still free
        assert!(availability.is_day_slot_free(1, Weekday::Tuesday, start_tick, 1));
    }

    #[test]
    fn test_daily_pattern() {
        let mut availability = AvailabilityBitSet::new();

        // Set lunch break busy every day 12:00-13:00
        let lunch_start = AvailabilityBitSet::time_to_day_tick(12, 0).unwrap();
        let lunch_end = AvailabilityBitSet::time_to_day_tick(13, 0).unwrap() - 1; // Exclusive end

        availability.set_daily_busy(lunch_start, lunch_end);

        // Check all days have lunch break busy
        for day in [Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday,
            Weekday::Thursday, Weekday::Friday, Weekday::Saturday, Weekday::Sunday] {
            assert!(!availability.is_day_slot_free(1, day, lunch_start, 1));
        }
    }
}
impl Default for AvailabilityBitSet {
    fn default() -> Self {
        Self::new()
    }
}