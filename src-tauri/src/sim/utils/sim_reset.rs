use crate::action_queues::sim_manager::SimManager;
use crate::integrations::snapshots::snapshots::SnapshotState;
use legion::systems::CommandBuffer;
use legion::{system, Entity};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{debug, warn};

pub struct ResetRequest {
    pub should_reset: AtomicBool,
}
impl Default for ResetRequest {
    fn default() -> Self {
        Self { should_reset: AtomicBool::new(false) }
    }
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
pub fn reset_snapshot(#[resource] app_state: &Arc<SnapshotState>){
    app_state.persons.clear();
    app_state.teams.clear();
}