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
use std::sync::Arc;

use crate::sim::new_game::new_game::{CompanyPreset, CompanyPresetStatic, StartingEmployeesConfig};
use crate::sim::person::init::FirstRun;
use crate::sim::registries::registry::Registry;
use arc_swap::ArcSwap;
use parking_lot::RwLock;
use std::time::Duration;
use tracing::field::debug;
use tracing::{debug, error, info, trace, warn};
use crate::integrations::snapshots::snapshots::SnapshotState;

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

        SimManagerCommand::ResetSim { .. } => {
            error!("Unexpected item in queue. ResetSim should be handled by new game queue")
            //reset is handled by new game manager queue
        }
        SimManagerCommand::StartSim { .. } => {
            error!("Unexpected item in queue. StartSim should be handled by new game queue")
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