use crate::integrations::snapshots::company::CompanySnapshot;
use crate::integrations::snapshots::debug_display::DebugDisplayEntrySnapshot;
use crate::integrations::snapshots::game_speed::GameSpeedSnapshot;
use crate::integrations::snapshots::person::PersonSnapshot;
use crate::integrations::snapshots::stress::StressSnapshot;
use crate::integrations::snapshots::stress_history::StressHistorySnapshot;
use crate::integrations::snapshots::team::TeamSnapshot;
use crate::integrations::snapshots::tick::TickSnapshot;
use crate::integrations::snapshots::working_hours::WorkingHoursSnapshot;
use crate::integrations::snapshots::thoughts::ThoughtsSnapshot;
use std::vec::Vec;
use crate::integrations::snapshots_emitter::snapshots_emitter::SnapshotField;
use crate::sim::calendar::availability::MonthlyAvailability;
use dashmap::DashMap;
use std::hash::Hash;
use std::sync::Arc;
use crate::sim::person::components::PersonId;
use crate::sim::team::components::TeamId;

/// this is tha main integration state
#[derive(Debug)]
pub struct SnapshotState {
    pub tick: TickSnapshot,
    pub game_speed: Arc<SnapshotField<GameSpeedSnapshot>>,
    pub persons: Arc<DashMap<PersonId, PersonSnapshot>>,
    pub company: Arc<SnapshotField<CompanySnapshot>>,
    pub teams : Arc<DashMap<TeamId, TeamSnapshot>>,
    pub debug_display: Arc<DashMap<PersonId, Vec<DebugDisplayEntrySnapshot>>>,
    pub stress_level: Arc<DashMap<PersonId, StressSnapshot>>,
    pub stress_history: Arc<DashMap<PersonId, StressHistorySnapshot>>,
    pub working_hours: Arc<DashMap<PersonId, WorkingHoursSnapshot>>,
    pub thoughts: Arc<DashMap<PersonId, ThoughtsSnapshot>>,

}

impl SnapshotState {
    pub fn reset(&self) {
        self.persons.clear();
        self.teams.clear();
        self.debug_display.clear();
        self.stress_level.clear();
        self.stress_history.clear();
        self.working_hours.clear();
        self.thoughts.clear();
    }
}

impl Default for SnapshotState {
    fn default() -> Self {
        Self {
            tick: TickSnapshot::default(),
            game_speed: Arc::new(SnapshotField::from(GameSpeedSnapshot::default())),
            persons: Arc::new(DashMap::<PersonId, PersonSnapshot>::new()),
            company: Arc::new(SnapshotField::from(CompanySnapshot::default())),
            teams: Arc::new(DashMap::<TeamId, TeamSnapshot>::new()),
            debug_display: Arc::new(DashMap::<PersonId, Vec<DebugDisplayEntrySnapshot>>::new()),
            stress_level: Arc::new(DashMap::<PersonId, StressSnapshot>::new()),
            stress_history: Arc::new(DashMap::<PersonId, StressHistorySnapshot>::new()),
            working_hours: Arc::new(DashMap::<PersonId, WorkingHoursSnapshot>::new()),
            thoughts: Arc::new(DashMap::<PersonId, ThoughtsSnapshot>::new()),
        }
    }
}

