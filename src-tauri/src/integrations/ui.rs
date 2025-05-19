use crate::integrations::snapshots::{PersonSnapshot, TickSnapshot};
use crate::sim::resources::global::SimManager;
use dashmap::DashMap;
use std::sync::Arc;
use tauri::State;
use tracing::info;
use crate::integrations::queues::{ExposedQueue, SimCommand};
use crate::integrations::system_queues::sim_manager::{SimManagerCommand};

#[derive(Debug, Default)]
pub struct SnapshotState {// this is tha main integration state
    pub tick: TickSnapshot,
    pub persons: DashMap<u32, PersonSnapshot>,
    pub command_queue: ExposedQueue<SimCommand>,
    pub sim_manager_queue: ExposedQueue<SimManagerCommand>
}


#[tauri::command]
pub fn get_tick(state: State<'_, Arc<SnapshotState>>) -> u64 {
    state.tick.get()
}

#[tauri::command]
pub fn get_persons(state: State<'_, Arc<SnapshotState>>) -> Vec<PersonSnapshot> {
    state.persons.iter().map(|e| e.value().clone()).collect()
}


#[tauri::command]
pub fn new_game(state: State<'_, Arc<SnapshotState>>){
    // reset_simulation(
}


#[tauri::command]
pub fn start_sim(state: State<'_, Arc<SnapshotState>>){
    state.sim_manager_queue.push(SimManagerCommand::StartSim)
}
#[tauri::command]
pub fn stop_sim(state: State<'_, Arc<SnapshotState>>){
    state.sim_manager_queue.push(SimManagerCommand::StopSim)
}
#[tauri::command]
pub fn resume_sim(state: State<'_, Arc<SnapshotState>>){
    info!("resume_sim");
    state.sim_manager_queue.push(SimManagerCommand::ResumeSim)
}

#[tauri::command]
pub fn new_sim(state: State<'_, Arc<SnapshotState>>){
    state.sim_manager_queue.push(SimManagerCommand::ResetSim)
}

