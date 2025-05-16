use tauri::State;
use std::sync::{atomic::{AtomicU64, Ordering}, Arc};
use dashmap::DashMap;
use legion::system;
use crate::integrations::snapshots::{PersonSnapshot, TickSnapshot};
use crate::sim::person::components::Person;
use crate::sim::resources::global::TickCounter;


#[derive(Debug, Default)]
pub struct AppState {// this is tha main integration state
    pub tick: TickSnapshot,
    pub persons: DashMap<u32, PersonSnapshot>,
}


#[tauri::command]
pub fn get_tick(state: State<'_, Arc<AppState>>) -> u64 {
    state.tick.get()
}

#[tauri::command]
pub fn get_persons(state: State<'_, Arc<AppState>>) -> Vec<PersonSnapshot> {
    state.persons.iter().map(|e| e.value().clone()).collect()
}




#[system]
pub fn update_tick(#[resource]tick_counter: &Arc<TickCounter>){
    tick_counter.tick();
}




