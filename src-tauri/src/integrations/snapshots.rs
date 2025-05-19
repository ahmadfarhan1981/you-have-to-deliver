use std::sync::Arc;
use std::sync::atomic::{AtomicU16, AtomicU64, AtomicU8, Ordering};
use serde::{Deserialize, Serialize};
use crate::sim::person::components::{Gender, PersonId, ProfilePictureCategory};
use crate::sim::person::stats::Stats;
use crate::sim::resources::global::TickCounter;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TickSnapshot{
    tick: AtomicU64,
    year: AtomicU16,
    week: AtomicU8,
    day: AtomicU8,
    quarter_tick: AtomicU8,
}

impl TickSnapshot {
    /// Updates the snapshot to match the given value
    pub fn set(&self, val: &TickCounter) {
        self.tick.store(val.value(), Ordering::Relaxed);

        let (year, week, day, quarter_tick) = val.current_date();
        self.quarter_tick.store(quarter_tick, Ordering::Relaxed);
        self.day.store(day, Ordering::Relaxed);
        self.week.store(week, Ordering::Relaxed);
        self.year.store(year, Ordering::Relaxed);
    }

    /// Reads the current snapshot value
    pub fn get(&self) -> u64 {
        self.tick.load(Ordering::Relaxed)
    }
     pub fn get_date(&self) -> ( u16, u8, u8, u8, u64) {
         (self.year.load(Ordering::Relaxed) ,
          self.week.load(Ordering::Relaxed),
          self.day.load(Ordering::Relaxed),
          self.quarter_tick.load(Ordering::Relaxed),
         self.tick.load(Ordering::Relaxed))
     }

}




#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PersonSnapshot {
    pub(crate) stats: StatsSnapshot,
    pub(crate) profile_picture: ProfilePictureSnapshot,
    pub(crate) person_id: u32,
    pub(crate) name: String,
    pub(crate) gender: String,

}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProfilePictureSnapshot {
    pub gender: String,
    pub category: i8,
    pub batch: i8,
    pub index: i8
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StatsSnapshot {
    // Cognition
    pub judgement: u16,
    pub creativity: u16,


    // Perception
    pub systems: u16,
    pub precision: u16,


    // Drive
    pub focus: u16,
    pub discipline: u16,


    // Social
    pub empathy: u16,
    pub communication: u16,

    // Defense
    pub resilience: u16,
    pub adaptability: u16,
}

impl From<Stats> for StatsSnapshot {
    fn from(s: Stats) -> Self {
        Self {
            judgement: s.judgement,
            creativity: s.creativity,
            systems: s.systems,
            precision: s.precision,
            focus: s.focus,
            discipline: s.discipline,
            empathy: s.empathy,
            communication: s.communication,
            resilience: s.resilience,
            adaptability: s.adaptability,
        }
    }
}