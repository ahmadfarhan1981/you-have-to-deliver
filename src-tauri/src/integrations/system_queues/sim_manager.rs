use std::sync::{Arc, RwLock};
use std::time::Duration;
use legion::system;
use tracing::debug;
use crate::integrations::queues::QueueManager;
use crate::integrations::system_queues::shared::timed_dispatch;
use crate::sim::game_speed::components::GameSpeedManager;
use crate::sim::resources::global::SimManager;

pub enum SimManagerCommand {
    //Sim manager commands
    StartSim,
    StopSim,
    ResetSim,
    ResumeSim,
}


pub struct SimManagerQueue{

}


#[system]
pub fn handle_sim_manager_queue(
    #[resource] queue_manager: &QueueManager,
    #[resource] sim_manager: &Arc<SimManager>,
) {
    debug!("Handling game speed manager queue");
    let queue = &queue_manager.sim_manager;
    let dispatch_time_limit = Duration::from_millis(5);

    timed_dispatch(queue, dispatch_time_limit, |cmd| match cmd {
        SimManagerCommand::StartSim=>{

        }
        SimManagerCommand::StopSim => {}
        SimManagerCommand::ResetSim => {}

        SimManagerCommand::ResumeSim => {
            sim_manager.resume_sim()
        }
    });
}