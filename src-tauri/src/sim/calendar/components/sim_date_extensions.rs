use crate::sim::sim_date::sim_date::SimDate;

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
