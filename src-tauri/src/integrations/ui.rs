use crate::integrations::events::SimCommand;
use crate::integrations::snapshots::{PersonSnapshot, TickSnapshot};
use crate::sim::game_speed::components::GameSpeed;
use crossbeam::queue::SegQueue;
use dashmap::DashMap;
use std::sync::Arc;
use tauri::State;
use crate::sim::resources::global::SimManager;

#[derive(Debug, Default)]
pub struct SnapshotState {// this is tha main integration state
    pub tick: TickSnapshot,
    pub persons: DashMap<u32, PersonSnapshot>,
    pub command_queue: Arc<SegQueue<SimCommand>>,
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
pub fn set_game_speed(state: State<'_, Arc<SnapshotState>>, game_speed: GameSpeed){
    state.command_queue.push(SimCommand::SetGameSpeed(game_speed));
}

#[tauri::command]
pub fn increase_speed(state: State<'_, Arc<SnapshotState>>){
    state.command_queue.push(SimCommand::IncreaseGameSpeed());
}
#[tauri::command]
pub fn decrease_speed(state: State<'_, Arc<SnapshotState>>){
    state.command_queue.push(SimCommand::DecreaseGameSpeed());
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

