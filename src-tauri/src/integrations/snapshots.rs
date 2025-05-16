use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};
use crate::sim::person::components::{Gender, PersonId, ProfilePictureCategory};
use crate::sim::person::stats::Stats;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TickSnapshot(AtomicU64);

impl TickSnapshot {
    /// Updates the snapshot to match the given value
    pub fn set(&self, val: u64) {
        self.0.store(val, Ordering::Relaxed);
    }

    /// Reads the current snapshot value
    pub fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
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