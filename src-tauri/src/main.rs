// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)] // For "never used"
#![allow(unused)] // For both "never used" and "never constructed"
#![deny(clippy::print_stdout)]
#![allow(clippy::dbg_macro)]

mod action_queues;
mod config;
mod constants;
mod db;
mod integrations;
mod macros;
mod master_data;
mod resources;
mod schedules;
mod sim;
mod simulation_thread;
use crate::resources::init::{initialize_emit_registries, initialize_non_shared_resources};
use crate::schedules::init::init_schedules;
use crate::sim::resources::global::{AssetBasePath, Dirty, TickCounter};
use crate::sim::systems::global::{increase_sim_tick_system, UsedProfilePictureRegistry};
use crate::simulation_thread::{run_simulation_thread, SimThreadConfig};

use legion::{Entity, Resources, Schedule, World};
use sim::systems::global::print_person_system;

use crate::integrations::systems::{
    push_company_to_integration_system, push_debug_displays_to_integration_system,
    push_game_speed_snapshots_system, push_needs_to_integration_system,
    push_persons_to_integration_system, push_teams_to_integration_system, tick_needs_system,
};
use crate::integrations::ui::{
    assign_person_to_team, exit_app, list_save_slots, new_sim, new_team, refresh_data, resume_sim,
    stop_sim, test_save_slots, unassign_team, AppContext,
};
use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};
use crate::sim::person::components::{PersonId, ProfilePicture};
use crate::sim::person::init::{
    emit_done_setup_event_system, generate_employees_system, init_company_system,
    load_global_skills_system, unset_first_run_flag_system, FirstRun,
};
use crate::sim::utils::logging::init_logging;
use crossbeam::queue::SegQueue;
use dashmap::{DashMap, DashSet};
use spin_sleep::{SpinSleeper, SpinStrategy};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{fmt, thread};

use tauri::{Emitter, Manager};
use tracing::{debug, info, trace};

use crate::integrations::queues::{
    handle_dispatch_queue_system, handle_sim_manager_dispatch_queue_system, QueueManager,
    UICommandQueues,
};
use crate::integrations::snapshots_emitter::snapshots_emitter::{
    run_snapshot_emitters_system, ExportFrequency, SnapshotCollectionEmitter, SnapshotEmitRegistry,
    SnapshotEmitterConfig, SnapshotFieldEmitter,
};
use action_queues::game_speed_manager::{
    decrease_speed, handle_game_speed_manager_queue_system, increase_speed, set_game_speed,
};
use action_queues::sim_manager;
use action_queues::sim_manager::{delete_all_entity_system, handle_new_game_manager_queue_system, handle_sim_manager_queue_system, reset_state_system, SimManager};

use crate::action_queues::sim_manager::{reset_snapshot_system, test_sim_manager_system};
use crate::db::init::{self, SavesDirectory};
use crate::integrations::events::{emit_app_event, AppEventType};
use crate::integrations::snapshots::company::CompanySnapshot;
use crate::integrations::snapshots::snapshots::SnapshotState;
use crate::sim::action::action::{decide_action_system, execute_action_system};
use crate::sim::ai::consideration::goal_selection_system;
use crate::sim::company::company::{Company, PlayerControlled};
use crate::sim::new_game::new_game::{
    get_company_presets, get_starting_employee_configs, CompanyPreset, CompanyPresetStatic,
    StartingEmployeesConfig,
};
use crate::sim::person::skills::SkillId;
use crate::sim::registries::registry::{GlobalSkillNameMap, Registry};
use crate::sim::team::components::TeamId;
use crate::sim::utils::debugging::clear_debug_display_system;
use crate::sim::utils::sim_reset::ResetRequest;
use crate::sim::utils::term::{bold, green, italic, red};
use action_queues::team_manager::{
    handle_team_assignment_queue_system, handle_team_manager_queue_system,
};
use parking_lot::{Mutex, RwLock};
use sim::utils::banner::print_banner;
use tauri::path::BaseDirectory;

fn print_startup_banner() {
    print_banner();
}


fn main() {
    init_logging();
    print_startup_banner();
    info!("Starting...");

    debug!("Debug log is {ENABLED}. Logs will be verbose. Use {log_settings} environment variable for normal operations.",ENABLED= red(&bold("ENABLED")), log_settings= green(&italic("RUST_LOG=info")));

    let snapshot_state = SnapshotState::default();
    let ui_snapshot_state = Arc::new(snapshot_state);
    let sim_snapshot_state = Arc::clone(&ui_snapshot_state); // Clone for ECS thread
    let main_snapshot_state = Arc::clone(&ui_snapshot_state); // Clone for ECS thread

    //Snapshot registry
    let mut snapshot_registry = initialize_emit_registries(&main_snapshot_state);

    let game_speed = Arc::new(RwLock::new(GameSpeedManager::default()));

    let mut command_queues = UICommandQueues::default();

    let queue_manager = QueueManager::new();
    command_queues.runtime = queue_manager.dispatch();
    command_queues.control = queue_manager.sim_manager_dispatch();

    // let ui_snapshot_state = Arc::new(snapshot_state);
    // let sim_snapshot_state = Arc::clone(&ui_snapshot_state); // Clone for ECS thread

    let ui_command_queues = Arc::new(command_queues);
    let sim_command_queues = Arc::clone(&ui_command_queues); // Clone for ECS thread

    // Used by person generation to prevent duplicate profile picture. no arc, only used in sim

    let tick_counter = Arc::new(TickCounter::default());

    let sim_manager = Arc::new(SimManager::default());
    let ui_sim_manager = Arc::clone(&sim_manager);

    let reset_request = Arc::new(ResetRequest::default());
    let first_run = Arc::new(FirstRun::default());
    let reset = Arc::clone(&reset_request);

    let sim_snapshot_registry = Arc::new(snapshot_registry);
    let ui_snapshot_registry = Arc::clone(&sim_snapshot_registry);

    // === Launch Tauri app ===
    tauri::Builder::default()
        .setup(|app| {
            info!("Tauri setup initiated.");
            let app_handle = app.handle().clone();
            let assets_path = app.path().resolve("assets", BaseDirectory::Resource)?;

            // Resolve the saves directory path
            let saves_dir_path = app
                .path()
                .resolve("saves", BaseDirectory::AppData)
                .expect("Failed to resolve saves directory path"); // Handle this error robustly

            // Ensure the directory exists
            if !saves_dir_path.exists() {
                std::fs::create_dir_all(&saves_dir_path).expect("Failed to create saves directory");
            }

            // Manage the path so commands can access it via tauri::State
            app.manage(SavesDirectory(saves_dir_path));

            // === Sim thread ===
            thread::spawn(move || {
                // All Arcs and owned values (like queue_manager) are moved into this closure.
                // They are then moved into SimThreadConfig.
                let sim_thread_config = SimThreadConfig {
                    app_handle, // This is app_handle.clone() from .setup()
                    asset_base_path: assets_path,
                    reset_request,
                    first_run,
                    sim_manager,
                    queue_manager, // queue_manager is moved here
                    sim_command_queues,
                    tick_counter,
                    game_speed,
                    sim_snapshot_state,
                    sim_snapshot_registry,
                };
                run_simulation_thread(sim_thread_config);
            });

            Ok(())
        })
        .manage(ui_snapshot_state)
        .manage(ui_command_queues)
        .manage(ui_sim_manager)
        .manage(ui_snapshot_registry)
        .invoke_handler(tauri::generate_handler![
            set_game_speed,
            increase_speed,
            decrease_speed,
            stop_sim,
            resume_sim,
            new_sim,
            new_team,
            assign_person_to_team,
            unassign_team,
            refresh_data,
            get_starting_employee_configs,
            get_company_presets,
            list_save_slots,
            test_save_slots,
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
