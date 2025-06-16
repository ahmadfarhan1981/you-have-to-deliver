use crate::action_queues::sim_manager::SimManager;
use dashmap::DashMap;
use std::sync::Arc;

use crate::integrations::queues::{ExposedQueue, SimCommand, UICommandQueues};
use crate::action_queues::sim_manager::SimManagerCommand;
use crate::sim::game_speed::components::GameSpeed;
use tauri::{AppHandle, State};
use tracing::info;
use tracing_subscriber::fmt::format;
use crate::integrations::snapshots_emitter::snapshots_emitter::SnapshotEmitRegistry;
use crate::action_queues::team_manager::{TeamAssignmentCommand, TeamManagerCommand};
use crate::db::init::{create_new_save_slot, scan_save_slots, SaveSlot, SavesDirectory};
use crate::sim::new_game::new_game::{CompanyPresetStatic, CompanyPreset, StartingEmployeesConfig};

#[derive(Clone)]
pub struct AppContext {
    pub app_handle: AppHandle,
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
pub fn new_sim(queues: State<'_, Arc<UICommandQueues>>, company: CompanyPreset, employee:StartingEmployeesConfig, slot_id: String) {
    queues.control.push(SimManagerCommand::StartSim{company, employee, slot_id});
}

#[tauri::command]
pub fn new_team(team_name: String, description: String, queues: State<'_, Arc<UICommandQueues>>) {
    queues.runtime.push(SimCommand::TeamManager(TeamManagerCommand::NewTeam {name:team_name, description }))
}

#[tauri::command]
pub fn assign_person_to_team(team_id: u32, person_id: u32, queues: State<'_, Arc<UICommandQueues>>) {
    queues.runtime.push(SimCommand::TeamAssignment(TeamAssignmentCommand::AddPersonToTeam {person_id, team_id }));
}

#[tauri::command]
pub fn unassign_team( person_id: u32, queues: State<'_, Arc<UICommandQueues>>) {
    queues.runtime.push(SimCommand::TeamAssignment(TeamAssignmentCommand::UnassignTeam {person_id }));
}


#[tauri::command]
pub fn refresh_data(app: AppHandle, emit_registry: State<'_, Arc<SnapshotEmitRegistry>>) {
    info!("Force refresh data called.");
    emit_registry.force_emit_all(&app);
}



#[tauri::command]
pub fn list_save_slots(saves_dir_state: tauri::State<Arc<SavesDirectory>>) -> Result<Vec<SaveSlot>, String> {
    scan_save_slots(&saves_dir_state) // Access the PathBuf via .0
        .map_err(|e| e.to_string())
}


#[tauri::command]
pub fn test_save_slots(saves_dir_state: tauri::State<Arc<SavesDirectory>>)  {
    create_new_save_slot(&saves_dir_state,
                         format!("{}{:?}","Save slot paan dev", std::time::SystemTime::now() ).as_str(),
    );

}

#[tauri::command]
pub async fn exit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}
// In your main.rs or setup function:
// .invoke_handler(tauri::generate_handler![list_save_slots])