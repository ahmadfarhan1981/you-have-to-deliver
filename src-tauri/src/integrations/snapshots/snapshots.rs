use crate::integrations::snapshots::company::CompanySnapshot;
use crate::integrations::snapshots::game_speed::GameSpeedSnapshot;
use crate::integrations::snapshots::person::PersonSnapshot;
use crate::integrations::snapshots::team::TeamSnapshot;
use crate::integrations::snapshots::tick::TickSnapshot;
use arc_swap::ArcSwap;
use dashmap::DashMap;
use serde::Serialize;
use std::hash::Hash;
use std::sync::Arc;
use crate::integrations::snapshots::debug_display::DebugDisplayEntrySnapshot;
use crate::integrations::snapshots::stress::StressSnapshot;
use crate::integrations::snapshots::stress_history::StressHistorySnapshot;
use crate::sim::person::morale::StressLevel;

/// this is tha main integration state
#[derive(Debug)]
pub struct SnapshotState {
    pub tick: TickSnapshot,
    pub game_speed: Arc<SnapshotField<GameSpeedSnapshot>>,
    pub persons: Arc<DashMap<u32, PersonSnapshot>>,
    pub company: Arc<SnapshotField<CompanySnapshot>>,
    pub teams : Arc<DashMap<u32, TeamSnapshot>>,
    pub debug_display: Arc<DashMap<u32, Vec<DebugDisplayEntrySnapshot>>>,
    pub stress_level: Arc<DashMap<u32, StressSnapshot>>,
    pub stress_history: Arc<DashMap<u32, StressHistorySnapshot>>,

}

impl SnapshotState {
    pub fn reset(&self) {
        self.persons.clear();
        self.teams.clear();
        self.debug_display.clear();
        self.stress_level.clear();
        self.stress_history.clear();
    }
}

impl Default for SnapshotState {
    fn default() -> Self {
        Self {
            tick: TickSnapshot::default(),
            game_speed: Arc::new(SnapshotField::from(GameSpeedSnapshot::default())),
            persons: Arc::new(DashMap::new()),
            company: Arc::new(SnapshotField::from(CompanySnapshot::default())),
            teams: Arc::new(DashMap::<u32, TeamSnapshot>::new()),
            debug_display: Arc::new(DashMap::<u32, Vec<DebugDisplayEntrySnapshot>>::new()),
            stress_level: Arc::new(DashMap::<u32, StressSnapshot>::new()),
            stress_history: Arc::new(DashMap::<u32, StressHistorySnapshot>::new()),
        }
    }
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

impl<T> Clone for SnapshotField<T> {
    fn clone(&self) -> Self {
        Self {
            value: ArcSwap::from(self.value.load_full()),
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

