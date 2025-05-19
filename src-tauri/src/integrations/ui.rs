use crate::integrations::snapshots::{PersonSnapshot, TickSnapshot};
use crate::sim::resources::global::SimManager;
use dashmap::DashMap;
use std::sync::Arc;
use tauri::State;
use crate::integrations::queues::DispatchQueue;

#[derive(Debug, Default)]
pub struct SnapshotState {// this is tha main integration state
    pub tick: TickSnapshot,
    pub persons: DashMap<u32, PersonSnapshot>,
    pub command_queue: DispatchQueue,
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
pub fn start_sim(sim_state: State<'_, Arc<SimManager>>){
    sim_state.resume_sim();
}
#[tauri::command]
pub fn stop_sim(sim_state: State<'_, Arc<SimManager>>){
    sim_state.pause_sim();
}

