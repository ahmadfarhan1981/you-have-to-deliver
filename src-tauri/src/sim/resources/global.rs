use crate::integrations::queues::{QueueManager, UICommandQueues};
use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};
use crate::sim::new_game::new_game::{CompanyPreset, StartingEmployeesConfig};
use crate::sim::person::components::PersonId;
use crate::sim::person::init::FirstRun;
use crate::sim::registries::registry::Registry;
use crate::sim::sim_date::sim_date::SimDate;
use crate::sim::systems::global::UsedProfilePictureRegistry;
use crate::sim::utils::sim_reset::ResetRequest;
use legion::Entity;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;
use tracing::{debug, info};

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

#[derive(Default, Debug)]
pub struct SimManager {
    running: AtomicBool,
    pub company_preset: RwLock<CompanyPreset>,
    pub employees_preset: RwLock<StartingEmployeesConfig>,
}
impl SimManager {
    pub fn resume_sim(&self) {
        self.running.store(true, Ordering::SeqCst);
    }
    pub fn pause_sim(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn new_sim(&self, company_preset: CompanyPreset, employee_preset: CompanyPreset) {
        self.pause_sim();
    }

    pub fn load_sim(&self) {}
    pub fn default() -> Self {
        Self {
            running: AtomicBool::new(false),
            company_preset: RwLock::new(CompanyPreset::default()),
            employees_preset: RwLock::new(StartingEmployeesConfig::default()),
        }
    }

    pub fn reset_sim(
        &self,
        queue_manager: &QueueManager,
        tick_counter: &Arc<TickCounter>,
        game_speed: &Arc<RwLock<GameSpeedManager>>,
        used_profile_picture_registry: &UsedProfilePictureRegistry,
        person_registry: &Arc<Registry<PersonId, Entity>>,
        reset_request: &mut Arc<ResetRequest>,
        command_queues: &Arc<UICommandQueues>,
        firs_run: &Arc<FirstRun>,
        last_update_map: &Arc<dashmap::DashMap<&'static str, u64>>,
        new_company_preset: Option<CompanyPreset>,
        new_employees_preset: Option<StartingEmployeesConfig>,
    ) {
        info!("Resetting simulation...");

        debug!("Stopping sim...");
        if self.is_running() {
            self.pause_sim()
        }
        last_update_map.clear();
        firs_run.mark_as_first_run();

        match new_company_preset {
            Some(new_company_preset) => {
                let mut company_preset = self.company_preset.write();
                *company_preset = new_company_preset;
            }
            None => {}
        }
        match new_employees_preset {
            Some(new_employees_preset) => {
                let mut employees_preset = self.employees_preset.write();
                *employees_preset = new_employees_preset;
            }
            None => {}
        }

        game_speed.write().set(GameSpeed::Normal);

        // debug!("Clearing entities...");
        //
        reset_request.should_reset.store(true, Ordering::Relaxed);

        debug!("Clearing resources...");

        debug!("Clearing command queue...");
        //dispatch queues
        command_queues.runtime.clear();
        command_queues.control.clear();

        //subsystem queues
        while queue_manager.sim_manager.queue.pop().is_some() {}
        while queue_manager.game_speed_manager.queue.pop().is_some() {}
        while queue_manager.sim_manager.queue.pop().is_some() {}

        debug!("Resetting tick counter...");
        tick_counter.reset();

        debug!("Resetting registries...");
        used_profile_picture_registry.used_profile_pictures.clear();
        person_registry.clear();
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Dirty;
