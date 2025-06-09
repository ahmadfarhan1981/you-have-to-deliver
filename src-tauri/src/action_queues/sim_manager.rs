use crate::action_queues::shared::timed_dispatch;
use crate::integrations::queues::{QueueManager, UICommandQueues};

use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};
use crate::sim::person::components::{Person, PersonId, ProfilePicture};
use crate::sim::person::stats::Stats;
use crate::sim::resources::global::{SimManager, TickCounter};
use crate::sim::systems::global::UsedProfilePictureRegistry;
use crate::sim::utils::sim_reset::ResetRequest;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{system, Entity, IntoQuery, Query, Resources, World};
use std::fmt;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::sim::new_game::new_game::{CompanyPreset, CompanyPresetStatic, StartingEmployeesConfig};
use crate::sim::person::init::FirstRun;
use crate::sim::registries::registry::Registry;
use arc_swap::ArcSwap;
use parking_lot::RwLock;
use std::time::Duration;
use tracing::field::debug;
use tracing::{debug, error, info, trace, warn};

#[derive(Default, Debug)]
pub enum SimManagerCommand {
    //Sim manager commands
    StartSim {
        company: CompanyPreset,
        employee: StartingEmployeesConfig,
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

#[system(for_each)]
pub fn delete_all_entity(
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    #[resource] sim_manager: &Arc<SimManager>,
    #[resource] reset_request: &Arc<ResetRequest>,
    // person: &Person
) {
    debug!("Clearing entities...");
    if !reset_request.should_reset.load(Ordering::Relaxed) {
        return;
    }

    if sim_manager.is_running() {
        warn!("Unexpected delete request when sim is running");
        return;
    }
    command_buffer.remove(*entity);
}

#[system]
pub fn reset_state(#[resource] reset_request: &mut Arc<ResetRequest>) {
    reset_request.should_reset.store(false, Ordering::Relaxed);
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
                reset_request,
                command_queues,
                first_run,
                last_update_map,
                Some(company),
                Some(employee),
            );
        }
        SimManagerCommand::StartSim { company, employee } => {
            sim_manager.reset_sim(
                queue_manager,
                tick_counter,
                game_speed,
                used_profile_picture_registry,
                person_registry,
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