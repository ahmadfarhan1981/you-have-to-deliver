use crate::action_queues::shared::timed_dispatch;
use crate::integrations::queues::{QueueManager, UICommandQueues};

use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};
use crate::sim::person::components::{Person, PersonId, ProfilePicture};
use crate::sim::person::stats::Stats;
use crate::sim::resources::global::TickCounter;
use crate::sim::systems::global::UsedProfilePictureRegistry;
use crate::sim::team::components::TeamId;
use crate::sim::utils::sim_reset::ResetRequest;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{system, Entity, IntoQuery, Query, Resources, World};
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc};

use crate::sim::new_game::new_game::{CompanyPreset, CompanyPresetStatic, StartingEmployeesConfig};
use crate::sim::person::init::FirstRun;
use crate::sim::registries::registry::Registry;
use arc_swap::ArcSwap;
use parking_lot::{Mutex, RwLock};
use std::time::Duration;
use tracing::field::debug;
use tracing::{debug, error, info, trace, warn};
use crate::db::init::{create_new_save_slot, SaveSlot, SavesDirectory};
use crate::integrations::snapshots::snapshots::SnapshotState;
use crate::sim::persistence::persistence::LoadGame;

#[derive(Default, Debug)]
pub enum SimManagerCommand {
    //Sim manager commands
    StartSim {
        company: CompanyPreset,
        employee: StartingEmployeesConfig,
        slot_id: String,
    },
    #[default]
    StopSim,
    ResetSim {
        company: CompanyPreset,
        employee: StartingEmployeesConfig,
    },
    ResumeSim,
    LoadSim{slot_id:String},
}

// #[derive(Default, Debug)]
// pub enum NewGameCommand {
//     #[default]
//     StartSim{company:CompanyPreset,employee:StartingEmployeesConfig},
//     ResetSim{company:CompanyPreset,employee:StartingEmployeesConfig},
// }

#[system]
pub fn handle_sim_manager_queue(
    #[resource] queue_manager: &QueueManager,
    #[resource] sim_manager: &Arc<SimManager>,
) {
    trace!("Handling sim manager queue");
    let queue = &queue_manager.sim_manager;
    let dispatch_time_limit = Duration::from_millis(5);

    timed_dispatch(queue, dispatch_time_limit, |cmd| match cmd {
        SimManagerCommand::StopSim => {
            debug!("Sim manager stopped");
            sim_manager.pause_sim();
        }
        SimManagerCommand::ResumeSim => sim_manager.resume_sim(),

        SimManagerCommand::ResetSim { .. } => {//TODO split these to another sim command
            error!("Unexpected item in queue. ResetSim should be handled by new game queue")
            //reset is handled by new game manager queue
        }
        SimManagerCommand::StartSim { .. } => {
            error!("Unexpected item in queue. StartSim should be handled by new game queue")
        }
        SimManagerCommand::LoadSim { .. } => {
            error!("Unexpected item in queue. LoadSim should be handled by new game queue")
        }
    });
}

#[system]
pub fn handle_new_game_manager_queue(
    #[resource] queue_manager: &QueueManager,
    #[resource] sim_manager: &Arc<SimManager>,
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] first_run: &Arc<FirstRun>,
    #[resource] last_update_map: &Arc<dashmap::DashMap<&'static str, u64>>,
    #[resource] game_speed: &Arc<RwLock<GameSpeedManager>>,
    #[resource] used_profile_picture_registry: &UsedProfilePictureRegistry,
    #[resource] person_registry: &Arc<Registry<PersonId, Entity>>,
    #[resource] team_registry: &Arc<Registry<TeamId, Entity>>,
    #[resource] reset_request: &mut Arc<ResetRequest>,
    #[resource] command_queues: &Arc<UICommandQueues>,
    #[resource] saves_directory: &Arc<SavesDirectory>,
    #[resource] load_game: &Arc<LoadGame>,
) {
    trace!("Handle new game manager queue");

    let queue = &queue_manager.new_game_manager;
    let dispatch_time_limit = Duration::from_millis(5);

    timed_dispatch(queue, dispatch_time_limit, |cmd| match cmd {
        SimManagerCommand::ResetSim { employee, company } => {
            sim_manager.reset_sim(
                queue_manager,
                tick_counter,
                game_speed,
                used_profile_picture_registry,
                person_registry,
                team_registry,
                reset_request,
                command_queues,
                first_run,
                last_update_map,
                Some(company),
                Some(employee),
            );
        }
        SimManagerCommand::StartSim { company, employee, slot_id } => {

            let Ok((actual_slot_id, slot_metadata)) = create_new_save_slot(saves_directory, slot_id.as_str(), company.name.as_str()) else {
                error!("Failed to create new save slot");
                return;
            };
            let slot_path = saves_directory.0.join(actual_slot_id.clone());
            let save_slot = SaveSlot {
                slot_id:actual_slot_id,
                path: slot_path,
                metadata: Some(slot_metadata),
                is_empty: false,
                handle: None,
            };
            sim_manager.set_save_slot(save_slot);
            sim_manager.with_save_slot(|slot|{
                slot.ensure_db_handle_is_open(&saves_directory);
            });
            sim_manager.reset_sim(
                queue_manager,
                tick_counter,
                game_speed,
                used_profile_picture_registry,
                person_registry,
                team_registry,
                reset_request,
                command_queues,
                first_run,
                last_update_map,
                Some(company),
                Some(employee),
            );
        }
        SimManagerCommand::LoadSim { slot_id } => {
            // stop the sim and remove the current save slot, so dont accidentally writing to the previous save.
            sim_manager.pause_sim();
            *sim_manager.save_slot.lock() = None;
            
            
            load_game.should_load.store(true, Ordering::Relaxed);
            load_game.slot_id.write().replace(slot_id);

        }
        cmd => {
            error!("Unexpected item in game manager queue {:?}", cmd);
        }
    });
}

#[system]
pub fn test_sim_manager(#[resource] sim_manager: &Arc<SimManager>){
    // info!("Sim manager: {:?}", sim_manager);

}

#[derive(Default, Debug)]
pub struct SimManager {
    running: AtomicBool,
    pub company_preset: RwLock<CompanyPreset>,
    pub employees_preset: RwLock<StartingEmployeesConfig>,
    pub save_slot: Mutex<Option<SaveSlot>>, // let's be conservative and use mutex for save slots
}

impl SimManager {
    pub fn resume_sim(&self) {
        self.running.store(true, Ordering::SeqCst);
    }
    pub fn pause_sim(&self) {
        self.with_save_slot(|slot|{
            slot.handle = None;
        });
        self.running.store(false, Ordering::SeqCst);
    }
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    // pub fn new_sim(&self, company_preset: CompanyPreset, employee_preset: CompanyPreset) {
    //     //self.pause_sim();
    // }

    pub fn load_sim(&self) {}
    pub fn default() -> Self {
        Self {
            running: AtomicBool::new(false),
            company_preset: RwLock::new(CompanyPreset::default()),
            employees_preset: RwLock::new(StartingEmployeesConfig::default()),
            save_slot: Mutex::new(None),
        }
    }

    pub fn reset_sim(
        &self,
        queue_manager: &QueueManager,
        tick_counter: &Arc<TickCounter>,
        game_speed: &Arc<RwLock<GameSpeedManager>>,
        used_profile_picture_registry: &UsedProfilePictureRegistry,
        person_registry: &Arc<Registry<PersonId, Entity>>,
        team_registry: &Arc<Registry<TeamId, Entity>>,
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
        command_queues.clear_queues();
        queue_manager.clear_queues();
        
        debug!("Resetting tick counter...");
        tick_counter.reset();

        debug!("Resetting registries...");
        used_profile_picture_registry.used_profile_pictures.clear();
        person_registry.clear();
        team_registry.clear();

    }
}

// SaveSlot related impl
impl SimManager {
    /// Sets the current save slot, replacing any existing one.
    ///
    /// # Example
    /// ```
    /// sim_manager.set_save_slot(my_slot);
    /// ```
    pub fn set_save_slot(&self, slot: SaveSlot) {
        let mut lock = self.save_slot.lock();
        *lock = Some(slot);
    }

    /// Clears the current save slot (sets it to `None`).
    ///
    /// # Example
    /// ```
    /// sim_manager.clear_save_slot();
    /// ```
    pub fn clear_save_slot(&self) {
        let mut lock = self.save_slot.lock();
        *lock = None;
    }

    /// Checks whether a save slot is currently loaded.
    ///
    /// # Example
    /// ```
    /// if sim_manager.has_save_slot() {
    ///     println!("Ready to save!");
    /// }
    /// ```
    pub fn has_save_slot(&self) -> bool {
        let lock = self.save_slot.lock();
        lock.is_some()
    }

    /// Runs a closure with **mutable access** to the current save slot if it exists.
    /// Returns `Some(result)` if a slot was present, or `None` if not.
    ///
    /// # Example
    /// ```
    /// sim_manager.with_save_slot(|slot| {
    ///     slot.is_empty = false;
    /// });
    /// ```
    pub fn with_save_slot<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut SaveSlot) -> R,
    {
        let mut lock = self.save_slot.lock();
        match &mut *lock {
            Some(slot) => Some(f(slot)),
            None => None,
        }
    }

    /// Runs a closure with **read-only access** to the current save slot if it exists.
    /// Returns `Some(result)` if a slot was present, or `None` if not.
    ///
    /// # Example
    /// ```
    /// let path = sim_manager.with_save_slot_ref(|slot| slot.path.clone());
    /// ```
    pub fn with_save_slot_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&SaveSlot) -> R,
    {
        let lock = self.save_slot.lock();
        match &*lock {
            Some(slot) => Some(f(slot)),
            None => None,
        }
    }

    /// Runs a **fallible** closure with mutable access to the current save slot.
    /// Returns `Ok(Some(result))` if a slot was present and the closure succeeded,
    /// `Ok(None)` if no slot was loaded, or `Err(e)` if the closure failed.
    ///
    /// # Example
    /// ```
    /// let result = sim_manager.try_with_save_slot(|slot| {
    ///     if slot.is_empty {
    ///         Err("Can't save: slot is empty")
    ///     } else {
    ///         Ok(slot.slot_id.clone())
    ///     }
    /// });
    /// ```
    pub fn try_with_save_slot<F, R, E>(&self, f: F) -> Result<Option<R>, E>
    where
        F: FnOnce(&mut SaveSlot) -> Result<R, E>,
    {
        let mut lock = self.save_slot.lock();
        match &mut *lock {
            Some(slot) => f(slot).map(Some),
            None => Ok(None),
        }
    }
}
