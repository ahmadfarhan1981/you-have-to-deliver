use crate::sim::sim_date::sim_date::SimDate;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU16, AtomicU64, AtomicU8, Ordering};
#[derive(Default)]
pub struct TickCounter {
    /// Absolute tick. This is the running tick counter that never resets
    tick: AtomicU64,
    year: AtomicU16,
    week: AtomicU8,
    day: AtomicU8,
    /// This is the day tick. It resets to 0 each day. Each tick = 15min, 96 ticks/day
    quarter_tick: AtomicU8,
}

impl TickCounter {
    pub fn tick(&self) {
        self.tick.fetch_add(1, Ordering::Relaxed);

        let new_qt = self.quarter_tick.fetch_add(1, Ordering::Relaxed) + 1;
        if new_qt < 96 {
            return;
        }

        self.quarter_tick.store(0, Ordering::Relaxed);
        let new_day = self.day.fetch_add(1, Ordering::Relaxed) + 1;
        if new_day < 7 {
            return;
        }

        self.day.store(0, Ordering::Relaxed);
        let new_week = self.week.fetch_add(1, Ordering::Relaxed) + 1;
        if new_week < 52 {
            return;
        }

        self.week.store(0, Ordering::Relaxed);
        self.year.fetch_add(1, Ordering::Relaxed);
    }

    /// Gets the current tick.
    pub fn value(&self) -> u64 {
        self.tick.load(Ordering::Relaxed)
    }

    pub fn reset(&self) {
        self.tick.store(0, Ordering::Relaxed);
        self.quarter_tick.store(0, Ordering::Relaxed);
        self.day.store(0, Ordering::Relaxed);
        self.week.store(0, Ordering::Relaxed);
        self.year.store(0, Ordering::Relaxed);
    }

    pub fn current_date(&self) -> SimDate {
        SimDate {
            year: self.year.load(Ordering::Relaxed),
            week: self.week.load(Ordering::Relaxed),
            day: self.day.load(Ordering::Relaxed),
            quarter_tick: self.quarter_tick.load(Ordering::Relaxed),
        }
    }
}

impl std::fmt::Debug for TickCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TickCounter")
            .field("tick", &self.value())
            .field("year", &self.year.load(Ordering::Relaxed))
            .field("week", &self.week.load(Ordering::Relaxed))
            .field("day", &self.day.load(Ordering::Relaxed))
            .field("quarter_tick", &self.quarter_tick.load(Ordering::Relaxed))
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct AssetBasePath(pub PathBuf);

#[derive(Copy, Clone, Debug, Default)]
pub struct Dirty;
