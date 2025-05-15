// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]
mod config;
mod integrations;
mod macros;
mod sim;

use crate::sim::resources::global::{AssetBasePath, TickCounter};
use crate::sim::systems::global::tick_counter_system;

use legion::{Resources, Schedule, World};
use sim::systems::global::print_person_system;

use std::thread;
use std::time::Duration;

use crate::integrations::ui::{get_tick, AppState};
use crate::sim::person::systems::generate_employees_system;
use crate::sim::person::components::ProfilePicture;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use dashmap::DashSet;
use tauri::Manager;
use crate::sim::person::registry::PersonRegistry;

fn main() {
    // Create a properly shared AppState
    let ui_app_state = Arc::new(AppState::default());

    // Clone for ECS thread
    let sim_app_state = Arc::clone(&ui_app_state);

    // Used by person generation to prevent duplicate profile picture
    let used_portrait = DashSet::<ProfilePicture>::new();

    // === Launch Tauri app ===
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let path = app
                .path()
                .resolve("assets", tauri::path::BaseDirectory::Resource)?;

            // === Sim thread ===
            thread::spawn(move || {
                let mut world = World::default();
                let mut resources = Resources::default();

                resources.insert(TickCounter { tick: 0 });

                // Insert Arc into resources so ECS systems can sync to it
                resources.insert(sim_app_state);  // Insert the cloned Arc
                resources.insert(AssetBasePath(path));
                resources.insert(used_portrait);
                resources.insert(Arc::new(PersonRegistry::new()));

                // Startup schedule, add run once systems here.
                let mut startup = Schedule::builder()
                    .add_system(generate_employees_system())
                    .build();

                // Tick schedule, add systems that runs per frame here.
                let mut schedule = Schedule::builder()
                    .add_system(tick_counter_system())
                    .add_system(print_person_system())
                    .build();

                startup.execute(&mut world, &mut resources);
                loop {
                    // Execute tick
                    schedule.execute(&mut world, &mut resources);

                    let tick = resources.get::<TickCounter>().unwrap().tick;

                    // Extract and sync tick
                    if let Some(arc) = resources.get::<Arc<AtomicU64>>() {
                        arc.store(tick, Ordering::Relaxed);
                    }

                    thread::sleep(Duration::from_millis(500));
                }
            });

            Ok(())
        })
        .manage(ui_app_state)  // Pass the original Arc
        .invoke_handler(tauri::generate_handler![get_tick])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}