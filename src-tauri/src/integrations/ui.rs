use crate::integrations::snapshots::{
    GameSpeedSnapshot, PersonSnapshot, SnapshotState, TickSnapshot,
};
use crate::sim::resources::global::SimManager;
use dashmap::DashMap;
use std::sync::Arc;

use crate::integrations::queues::{ExposedQueue, SimCommand, UICommandQueues};
use crate::integrations::system_queues::sim_manager::SimManagerCommand;
use crate::sim::game_speed::components::GameSpeed;
use tauri::{AppHandle, State};
use tracing::info;

#[derive(Clone)]
pub struct AppContext {
    pub app_handle: AppHandle,
}
#[tauri::command]
pub fn start_sim(queues: State<'_, Arc<UICommandQueues>>) {
    queues.control.push(SimManagerCommand::StartSim)
}
#[tauri::command]
pub fn stop_sim(queues: State<'_, Arc<UICommandQueues>>) {
    queues.control.push(SimManagerCommand::StopSim)
}
#[tauri::command]
pub fn resume_sim(queues: State<'_, Arc<UICommandQueues>>) {
    info!("resume_sim");
    queues.control.push(SimManagerCommand::ResumeSim)
}

#[tauri::command]
pub fn new_sim(queues: State<'_, Arc<UICommandQueues>>) {
    queues.control.push(SimManagerCommand::ResetSim)
}
