use tauri::State;
use std::sync::{atomic::{AtomicU64, Ordering}, Arc};
use legion::system;
use crate::sim::resources::global::TickCounter;


#[derive(Clone, Debug, Default)]
pub struct AppState {// this is tha main integration state
    pub tick: Arc<AtomicU64>,
}

#[tauri::command]
pub fn get_tick(state: State<'_, Arc<AppState>>) -> u64 {
    state.tick.load(Ordering::Relaxed)
}




#[system]
pub fn update_tick(#[resource]tick_counter: &Arc<TickCounter>){
    tick_counter.tick.fetch_add(1, Ordering::Relaxed);
}



pub fn update_appstate(){
    //this updates appstate
}

// #[tauri::command]
// pub fn get_appstate(state: State<'_, AppState>) -> AppState {
//
// }