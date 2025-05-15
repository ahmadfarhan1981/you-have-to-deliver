use tauri::State;
use std::sync::{atomic::{AtomicU64, Ordering}, Arc};


#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub tick: Arc<AtomicU64>,
}

#[tauri::command]
pub fn get_tick(state: State<'_, Arc<AppState>>) -> u64 {
    state.tick.load(Ordering::Relaxed)
}

pub fn update_appstate(){
    //this updates appstate
}

// #[tauri::command]
// pub fn get_appstate(state: State<'_, AppState>) -> AppState {
//
// }