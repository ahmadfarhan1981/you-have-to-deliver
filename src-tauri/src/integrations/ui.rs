use crate::integrations::snapshots::{GameSpeedSnapshot, PersonSnapshot, SnapshotState, TickSnapshot};
use crate::sim::resources::global::SimManager;
use dashmap::DashMap;
use std::sync::Arc;
use tauri::State;
use tracing::info;
use crate::integrations::queues::{ExposedQueue, SimCommand};
use crate::integrations::system_queues::sim_manager::{SimManagerCommand};
use crate::sim::game_speed::components::GameSpeed;



#[tauri::command]
pub fn get_tick(state: State<'_, Arc<SnapshotState>>) -> (u16,u8,u8,u8,u64) {
    state.tick.get_date()
    //state.tick.get()
}

#[tauri::command]
pub fn get_persons(state: State<'_, Arc<SnapshotState>>) -> Vec<PersonSnapshot> {
    state.persons.iter().map(|e| e.value().clone()).collect()
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

