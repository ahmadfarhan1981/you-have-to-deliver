use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub const MAX_TICK:u64 = 2_290_657_279;


/// Represents in-game simulation time using a structured calendar format.
///
/// Simulation time advances in fixed 15-minute units called "quarter ticks":
/// - 96 quarter ticks per day (24 hours × 4)
/// - 7 days per week
/// - 4 weeks per year (28 days total)
///
/// All fields are 1-based:
/// - `year` starts at 1
/// - `week` ranges from 1 to 4
/// - `day` ranges from 1 to 7
/// - `quarter_tick` ranges from 1 to 96
///
/// Provides `From<u64>` for converting from raw tick count,
/// and a `.to_tick()` method for converting back.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, Encode, Decode)]
pub struct SimDate {
    pub year: u16,
    pub week: u8,
    pub day: u8,
    pub quarter_tick: u8,
}
impl Default for SimDate {
    fn default() -> Self {
        SimDate {
            year: 1,
            week: 1,
            day: 1,
            quarter_tick: 1,
        }
    }
}
impl PartialOrd for SimDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SimDate {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.year, self.week, self.day, self.quarter_tick)
            .cmp(&(other.year, other.week, other.day, other.quarter_tick))
    }
}
impl From<u64> for SimDate {
    fn from(ticks: u64) -> Self {
        const TICKS_PER_DAY: u64 = 96;
        const DAYS_PER_WEEK: u64 = 7;
        const WEEKS_PER_YEAR: u64 = 52;
        const DAYS_PER_YEAR: u64 = DAYS_PER_WEEK * WEEKS_PER_YEAR;
        const TICKS_PER_YEAR: u64 = DAYS_PER_YEAR * TICKS_PER_DAY;

        let year = (ticks / TICKS_PER_YEAR) + 1;
        let remaining = ticks % TICKS_PER_YEAR;

        let day_index = remaining / TICKS_PER_DAY;
        let quarter_tick = (remaining % TICKS_PER_DAY) + 1;

        let week = (day_index / DAYS_PER_WEEK) + 1;
        let day = (day_index % DAYS_PER_WEEK) + 1;

        SimDate {
            year: year as u16,
            week: week as u8,
            day: day as u8,
            quarter_tick: quarter_tick as u8,
        }
    }
}

impl SimDate {
    pub fn to_tick(&self) -> u64 {
        const TICKS_PER_DAY: u64 = 96;
        const DAYS_PER_WEEK: u64 = 7;
        const WEEKS_PER_YEAR: u64 = 52;
        const DAYS_PER_YEAR: u64 = WEEKS_PER_YEAR * DAYS_PER_WEEK;
        const TICKS_PER_YEAR: u64 = DAYS_PER_YEAR * TICKS_PER_DAY;

        let year_part = (self.year as u64 - 1) * TICKS_PER_YEAR;
        let week_part = (self.week as u64 - 1) * DAYS_PER_WEEK * TICKS_PER_DAY;
        let day_part = (self.day as u64 - 1) * TICKS_PER_DAY;
        let tick_part = self.quarter_tick as u64 - 1;

        year_part + week_part + day_part + tick_part
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering as CmpOrdering;

    // --- Constants for testing, reflecting the updated calendar ---
    const TICKS_PER_DAY_TEST: u64 = 96;
    const DAYS_PER_WEEK_TEST: u64 = 7;
    const WEEKS_PER_YEAR_TEST: u64 = 52;
    const DAYS_PER_YEAR_TEST: u64 = DAYS_PER_WEEK_TEST * WEEKS_PER_YEAR_TEST;
    const TICKS_PER_YEAR_TEST: u64 = DAYS_PER_YEAR_TEST * TICKS_PER_DAY_TEST;
    // -----------------------------------------------------------

    // Helper function to create a SimDate for testing
    fn sim_date(year: u16, week: u8, day: u8, quarter_tick: u8) -> SimDate {
        SimDate { year, week, day, quarter_tick }
    }

    #[test]
    fn test_from_ticks_zero() {
        let date = SimDate::from(0);
        assert_eq!(date.year, 1);
        assert_eq!(date.week, 1);
        assert_eq!(date.day, 1);
        assert_eq!(date.quarter_tick, 1);
    }

    #[test]
    fn test_to_tick_zero() {
        let date = sim_date(1, 1, 1, 1);
        assert_eq!(date.to_tick(), 0);
    }

    #[test]
    fn test_from_to_tick_roundtrip() {
        // Calculate the maximum representable date's tick value
        // Max year: u16::MAX
        // Max week: WEEKS_PER_YEAR_TEST
        // Max day: DAYS_PER_WEEK_TEST
        // Max quarter_tick: TICKS_PER_DAY_TEST
        let max_representable_date = sim_date(
            u16::MAX,
            WEEKS_PER_YEAR_TEST as u8,
            DAYS_PER_WEEK_TEST as u8,
            TICKS_PER_DAY_TEST as u8,
        );
        let max_representable_ticks = max_representable_date.to_tick();


        let ticks_to_test = vec![
            0,
            1,
            TICKS_PER_DAY_TEST - 1,       // Last tick of day 1, week 1, year 1
            TICKS_PER_DAY_TEST,         // First tick of day 2, week 1, year 1
            (DAYS_PER_WEEK_TEST * TICKS_PER_DAY_TEST) - 1, // Last tick of week 1, year 1
            (DAYS_PER_WEEK_TEST * TICKS_PER_DAY_TEST),     // First tick of week 2, year 1
            TICKS_PER_YEAR_TEST - 1,    // Last tick of year 1 (important boundary)
            TICKS_PER_YEAR_TEST,        // First tick of year 2 (important boundary)
            5000,
            100_000,
            1_000_000,
            TICKS_PER_YEAR_TEST * 3 + (TICKS_PER_DAY_TEST * 10), // A few years in, some days
            max_representable_ticks,    // The maximum possible tick value for a u16 year
            // Test a value just below the maximum to ensure it's handled
            max_representable_ticks - 1,
        ];

        for ticks in ticks_to_test {
            let date = SimDate::from(ticks);
            assert_eq!(date.to_tick(), ticks, "Roundtrip failed for ticks: {}", ticks);
        }
    }

    #[test]
    fn test_from_ticks_boundary_day() {
        // Just before new day
        let date = SimDate::from(TICKS_PER_DAY_TEST - 1); // 96th quarter_tick of day 1
        assert_eq!(date.year, 1);
        assert_eq!(date.week, 1);
        assert_eq!(date.day, 1);
        assert_eq!(date.quarter_tick, 96);

        // First tick of new day
        let date = SimDate::from(TICKS_PER_DAY_TEST); // 1st quarter_tick of day 2
        assert_eq!(date.year, 1);
        assert_eq!(date.week, 1);
        assert_eq!(date.day, 2);
        assert_eq!(date.quarter_tick, 1);
    }

    #[test]
    fn test_from_ticks_boundary_week() {
        // Just before new week
        let date = SimDate::from((DAYS_PER_WEEK_TEST * TICKS_PER_DAY_TEST) - 1); // Last tick of day 7, week 1
        assert_eq!(date.year, 1);
        assert_eq!(date.week, 1);
        assert_eq!(date.day, 7);
        assert_eq!(date.quarter_tick, 96);

        // First tick of new week
        let date = SimDate::from(DAYS_PER_WEEK_TEST * TICKS_PER_DAY_TEST); // First tick of day 1, week 2
        assert_eq!(date.year, 1);
        assert_eq!(date.week, 2);
        assert_eq!(date.day, 1);
        assert_eq!(date.quarter_tick, 1);
    }

    #[test]
    fn test_from_ticks_boundary_year() {
        // Just before new year (last tick of last day of last week of year 1)
        let date = SimDate::from(TICKS_PER_YEAR_TEST - 1);
        assert_eq!(date.year, 1);
        assert_eq!(date.week, WEEKS_PER_YEAR_TEST as u8); // Should be week 52
        assert_eq!(date.day, DAYS_PER_WEEK_TEST as u8); // Should be day 7
        assert_eq!(date.quarter_tick, TICKS_PER_DAY_TEST as u8); // Should be tick 96

        // First tick of new year (first tick of day 1, week 1, year 2)
        let date = SimDate::from(TICKS_PER_YEAR_TEST);
        assert_eq!(date.year, 2);
        assert_eq!(date.week, 1);
        assert_eq!(date.day, 1);
        assert_eq!(date.quarter_tick, 1);

        // Test a few ticks into the second year
        let date = SimDate::from(TICKS_PER_YEAR_TEST + TICKS_PER_DAY_TEST); // First tick of day 2, year 2
        assert_eq!(date.year, 2);
        assert_eq!(date.week, 1);
        assert_eq!(date.day, 2);
        assert_eq!(date.quarter_tick, 1);
    }

    #[test]
    fn test_ordering() {
        let d1 = sim_date(1, 1, 1, 1);
        let d2 = sim_date(1, 1, 1, 2);
        let d3 = sim_date(1, 1, 2, 1);
        let d4 = sim_date(1, 2, 1, 1);
        let d5 = sim_date(2, 1, 1, 1);
        let d6 = sim_date(1, 1, 1, 1); // Same as d1

        assert!(d1 < d2);
        assert!(d2 < d3);
        assert!(d3 < d4);
        assert!(d4 < d5);
        assert_eq!(d1, d6);
        assert_eq!(d1.cmp(&d6), CmpOrdering::Equal);
        assert_eq!(d1.cmp(&d2), CmpOrdering::Less);
        assert_eq!(d2.cmp(&d1), CmpOrdering::Greater);

        // Test ordering across year boundary (specific to new 52-week year)
        let end_of_year_1 = sim_date(1, 52, 7, 96);
        let start_of_year_2 = sim_date(2, 1, 1, 1);
        assert!(end_of_year_1 < start_of_year_2);
        assert_eq!(end_of_year_1.to_tick(), TICKS_PER_YEAR_TEST - 1);
        assert_eq!(start_of_year_2.to_tick(), TICKS_PER_YEAR_TEST);

        // Test ordering at the maximum year boundary
        let max_representable_date = sim_date(
            u16::MAX,
            WEEKS_PER_YEAR_TEST as u8,
            DAYS_PER_WEEK_TEST as u8,
            TICKS_PER_DAY_TEST as u8,
        );
        let date_just_before_max = sim_date(
            u16::MAX,
            WEEKS_PER_YEAR_TEST as u8,
            DAYS_PER_WEEK_TEST as u8,
            TICKS_PER_DAY_TEST as u8 - 1,
        );
        assert!(date_just_before_max < max_representable_date);
    }

    #[test]
    fn test_default() {
        let default_date = SimDate::default();
        // Default should be equivalent to SimDate::from(0)
        assert_eq!(default_date.year, 1);
        assert_eq!(default_date.week, 1);
        assert_eq!(default_date.day, 1);
        assert_eq!(default_date.quarter_tick, 1);
        assert_eq!(default_date.to_tick(), 0);
    }
}