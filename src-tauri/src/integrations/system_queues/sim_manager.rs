use crate::integrations::queues::{QueueManager, UICommandQueues};
use crate::integrations::system_queues::shared::timed_dispatch;

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
use std::sync::Arc;
use std::sync::atomic::Ordering;

use std::time::Duration;
use parking_lot::RwLock;
use tracing::field::debug;
use tracing::{debug, error, info, trace, warn};
use crate::integrations::snapshots::SnapshotState;
use crate::sim::registries::registry::Registry;

#[derive(Default)]
pub enum SimManagerCommand {
    //Sim manager commands
    StartSim,
    #[default]
    StopSim,
    ResetSim,
    ResumeSim,
}
impl fmt::Debug for SimManagerCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimManagerCommand::StartSim => {
                write!(f, "StartSim")
            }
            SimManagerCommand::StopSim => {
                write!(f, "StopSim")
            }
            SimManagerCommand::ResetSim => {
                write!(f, "ResetSim")
            }
            SimManagerCommand::ResumeSim => {
                write!(f, "ResumeSim")
            }
        }
    }
}

#[system]
pub fn handle_sim_manager_queue(
    #[resource] queue_manager: &QueueManager,
    #[resource] sim_manager: &Arc<SimManager>,
) {
    trace!("Handling sim manager queue");
    let queue = &queue_manager.sim_manager;
    let dispatch_time_limit = Duration::from_millis(5);

    timed_dispatch(queue, dispatch_time_limit, |cmd| match cmd {
        SimManagerCommand::StartSim => {
            sim_manager.resume_sim();
        }
        SimManagerCommand::StopSim => {
            debug!("Sim manager stopped");
            sim_manager.pause_sim();
        }
        SimManagerCommand::ResumeSim => sim_manager.resume_sim(),

        SimManagerCommand::ResetSim => {
            error!("Unexpected item in queue. ResetSim should be handled by new game queue")
            //reset is handled by new game manager queue
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
    // println!("{:?}", entity);
    info!("{:?}", entity);
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
        SimManagerCommand::ResetSim => {
            info!("Resetting simulation...");

            debug!("Stopping sim...");
            if sim_manager.is_running() {
                sim_manager.pause_sim()
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

        _ => {}
    });
}
