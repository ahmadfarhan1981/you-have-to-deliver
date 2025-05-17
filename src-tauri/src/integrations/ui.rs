use std::process::Command;
use tauri::State;
use std::sync::{atomic::{AtomicU64, Ordering}, Arc};
use crossbeam::queue::SegQueue;
use dashmap::DashMap;
use legion::system;
use crate::integrations::events::SimCommand;
use crate::integrations::snapshots::{PersonSnapshot, TickSnapshot};
use crate::sim::game_speed::components::GameSpeed;
use crate::sim::person::components::Person;
use crate::sim::resources::global::TickCounter;


#[derive(Debug, Default)]
pub struct AppState {// this is tha main integration state
    pub tick: TickSnapshot,
    pub persons: DashMap<u32, PersonSnapshot>,
    pub command_queue: Arc<SegQueue<SimCommand>>,
}


#[tauri::command]
pub fn get_tick(state: State<'_, Arc<AppState>>) -> u64 {
    state.tick.get()
}

#[tauri::command]
pub fn get_persons(state: State<'_, Arc<AppState>>) -> Vec<PersonSnapshot> {
    state.persons.iter().map(|e| e.value().clone()).collect()
}


#[tauri::command]
pub fn set_game_speed(state: State<'_, Arc<AppState>>, game_speed: GameSpeed){
    state.command_queue.push(SimCommand::SetGameSpeed(game_speed));
}

#[tauri::command]
pub fn increase_speed(state: State<'_, Arc<AppState>>){
    state.command_queue.push(SimCommand::IncreaseGameSpeed());
}
#[tauri::command]
pub fn decrease_speed(state: State<'_, Arc<AppState>>){
    state.command_queue.push(SimCommand::DecreaseGameSpeed());
}



#[system]
pub fn update_tick(#[resource]tick_counter: &Arc<TickCounter>){
    tick_counter.tick();
}




