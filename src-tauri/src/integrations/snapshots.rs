use crate::sim::person::stats::{Stat, Stats};
use crate::sim::resources::global::TickCounter;
use arc_swap::ArcSwap;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::sync::atomic::{AtomicU16, AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct SnapshotState {
    // this is tha main integration state
    pub tick: TickSnapshot,
    pub game_speed: SnapshotField<GameSpeedSnapshot>,
    pub persons: DashMap<u32, PersonSnapshot>,
}
#[derive(Debug, Default)]
pub struct SnapshotField<T> {
    pub value: ArcSwap<Arc<T>>,
}
impl<T> From<T> for SnapshotField<T> {
    fn from(value: T) -> Self {
        Self {
            value: ArcSwap::from_pointee(value.into()),
        }
    }
}

pub struct SnapshotCollection<K, V>
where
    K: Eq + Hash + Clone,
    V: Serialize + Clone,
{
    pub map: DashMap<K, V>,
}
impl<K, V> From<DashMap<K, V>> for SnapshotCollection<K, V>
where
    K: Eq + Hash + Clone,
    V: Serialize + Clone,
{
    fn from(map: DashMap<K, V>) -> Self {
        Self { map }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct GameSpeedSnapshot {
    pub tick: TickSnapshot,
    pub game_speed: AtomicU8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TickSnapshot {
    tick: AtomicU64,
    year: AtomicU16,
    week: AtomicU8,
    day: AtomicU8,
    quarter_tick: AtomicU8,
}

impl From<TickCounter> for TickSnapshot {
    fn from(value: TickCounter) -> Self {
        let (year, week, day, quarter_tick) = value.current_date();
        Self {
            tick: value.value().into(),
            year: year.into(),
            week: week.into(),
            day: day.into(),
            quarter_tick: quarter_tick.into(),
        }
    }
}

impl TickSnapshot {
    /// Updates the snapshot to match the given value
    pub fn set(&self, val: &TickCounter) {
        self.tick.store(val.value(), Ordering::Relaxed);

        let (year, week, day, quarter_tick) = val.current_date();
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
    pub index: i8,
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
            judgement: s.get_stat(Stat::Judgement),
            creativity: s.get_stat(Stat::Creativity),
            systems: s.get_stat(Stat::Systems),
            precision: s.get_stat(Stat::Precision),
            focus: s.get_stat(Stat::Focus),
            discipline: s.get_stat(Stat::Discipline),
            empathy: s.get_stat(Stat::Empathy),
            communication: s.get_stat(Stat::Communication),
            resilience: s.get_stat(Stat::Resilience),
            adaptability: s.get_stat(Stat::Adaptability),
        }
    }
}
