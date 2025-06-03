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

/// this is tha main integration state
#[derive(Debug)]
pub struct SnapshotState {
    pub tick: TickSnapshot,
    pub game_speed: Arc<SnapshotField<GameSpeedSnapshot>>,
    pub persons: Arc<DashMap<u32, PersonSnapshot>>,
    pub company: Arc<SnapshotField<CompanySnapshot>>,
    pub teams : Arc<DashMap<u32, TeamSnapshot>>,

}

impl Default for SnapshotState {
    fn default() -> Self {
        Self {
            tick: TickSnapshot::default(),
            game_speed: Arc::new(SnapshotField::from(GameSpeedSnapshot::default())),
            persons: Arc::new(DashMap::new()),
            company: Arc::new(SnapshotField::from(CompanySnapshot::default())),
            teams: Arc::new(DashMap::<u32, TeamSnapshot>::new()),
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

