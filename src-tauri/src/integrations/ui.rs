use tauri::State;
use std::sync::{atomic::{AtomicU64, Ordering}, Arc};

#[derive(Clone)]
pub struct AppState {
    pub tick: Arc<AtomicU64>,
}

#[tauri::command]
pub fn get_tick(state: State<'_, AppState>) -> u64 {
    state.tick.load(Ordering::Relaxed)
}
