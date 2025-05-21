use crate::integrations::snapshots::{GameSpeedSnapshot, PersonSnapshot, SnapshotState, TickSnapshot};
use crate::sim::resources::global::SimManager;
use dashmap::DashMap;
use std::sync::Arc;
use tauri::{AppHandle, State};
use tracing::info;
use crate::integrations::queues::{ExposedQueue, SimCommand, UICommandQueues};
use crate::integrations::system_queues::sim_manager::{SimManagerCommand};
use crate::sim::game_speed::components::GameSpeed;

#[derive(Clone)]
pub struct AppContext {
    pub app_handle: AppHandle,
}


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
pub fn start_sim(queues: State<'_, Arc<UICommandQueues>>){
    queues.control.push(SimManagerCommand::StartSim)
}
#[tauri::command]
pub fn stop_sim(queues: State<'_, Arc<UICommandQueues>>){
    queues.control.push(SimManagerCommand::StopSim)
}
#[tauri::command]
pub fn resume_sim(queues: State<'_, Arc<UICommandQueues>>){
    info!("resume_sim");
    queues.control.push(SimManagerCommand::ResumeSim)
}

#[tauri::command]
pub fn new_sim(queues: State<'_, Arc<UICommandQueues>>){
    queues.control.push(SimManagerCommand::ResetSim)
}

