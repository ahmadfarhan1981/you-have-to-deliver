use std::sync::atomic::{AtomicU16, AtomicU64, AtomicU8, Ordering};
use serde::{Deserialize, Serialize};
use crate::sim::resources::global::TickCounter;
use crate::sim::sim_date::sim_date::SimDate;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TickSnapshot {
    tick: AtomicU64,
    year: AtomicU16,
    week: AtomicU8,
    day: AtomicU8,
    quarter_tick: AtomicU8,
}

impl Clone for TickSnapshot {
    fn clone(&self) -> Self {
        Self {
            tick: AtomicU64::new(self.tick.load(Ordering::Relaxed)),
            year: AtomicU16::new(self.year.load(Ordering::Relaxed)),
            week: AtomicU8::new(self.week.load(Ordering::Relaxed)),
            day: AtomicU8::new(self.day.load(Ordering::Relaxed)),
            quarter_tick: AtomicU8::new(self.quarter_tick.load(Ordering::Relaxed)),
        }
    }
}

impl From<TickCounter> for TickSnapshot {
    fn from(value: TickCounter) -> Self {
        let date = value.current_date();
        Self {
            tick: value.value().into(),
            year: date.year.into(),
            week: date.week.into(),
            day: date.day.into(),
            quarter_tick: date.quarter_tick.into(),
        }
    }
}

impl TickSnapshot {
    /// Updates the snapshot to match the given value
    pub fn set(&self, val: &TickCounter) {
        self.tick.store(val.value(), Ordering::Relaxed);

        let SimDate {
            year,
            week,
            day,
            quarter_tick,
        } = val.current_date();
        if self.year.load(Ordering::Relaxed) != year {
            self.year.store(year, Ordering::Relaxed);
        }
        if self.week.load(Ordering::Relaxed) != week {
            self.week.store(week, Ordering::Relaxed);
        }
        if self.day.load(Ordering::Relaxed) != day {
            self.day.store(day, Ordering::Relaxed);
        }
        if self.quarter_tick.load(Ordering::Relaxed) != quarter_tick {
            self.quarter_tick.store(quarter_tick, Ordering::Relaxed);
        }
    }

    /// Reads the current snapshot value
    pub fn get(&self) -> u64 {
        self.tick.load(Ordering::Relaxed)
    }
    pub fn get_date(&self) -> (u16, u8, u8, u8, u64) {
        (
            self.year.load(Ordering::Relaxed),
            self.week.load(Ordering::Relaxed),
            self.day.load(Ordering::Relaxed),
            self.quarter_tick.load(Ordering::Relaxed),
            self.tick.load(Ordering::Relaxed),
        )
    }
}