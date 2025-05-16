// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]
mod config;
mod integrations;
mod macros;
mod sim;

use crate::sim::resources::global::{AssetBasePath, TickCounter};
use crate::sim::systems::global::increase_sim_tick_system;

use legion::{Resources, Schedule, World};
use sim::systems::global::print_person_system;

use std::thread;
use std::time::Duration;

use crate::integrations::systems::{clear_person_list_system, print_person_list_system, push_persons_to_integration_system, push_tick_counter_to_integration_system};
use crate::integrations::ui::{get_persons, get_tick, AppState};
use crate::sim::person::components::ProfilePicture;
use crate::sim::person::registry::PersonRegistry;
use crate::sim::person::systems::generate_employees_system;
use dashmap::DashSet;
use std::sync::Arc;
use tauri::Manager;

fn main() {
    // Create a properly shared AppState
    let ui_app_state = Arc::new(AppState::default());
    // Clone for ECS thread
    let sim_app_state = Arc::clone(&ui_app_state);

    // Used by person generation to prevent duplicate profile picture. no arc, only used in sim
    let used_portrait = DashSet::<ProfilePicture>::new();

    let tick_counter = Arc::new(TickCounter::default() );

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

                resources.insert(tick_counter.clone());

                // Insert Arc into resources so ECS systems can sync to it
                resources.insert(sim_app_state);  // Insert the cloned Arc

                resources.insert(AssetBasePath(path));
                resources.insert(used_portrait);
                resources.insert(Arc::new(PersonRegistry::new()));





                let mut startup = Schedule::builder() // Startup schedule, runs once on startup. add run once systems here.
                    .add_system(generate_employees_system())
                    .build();

                let mut sim_loop = Schedule::builder()// Main game loop, add systems that runs per frame here.
                    .add_system(increase_sim_tick_system())
                    .add_system(print_person_system())
                    .build();

                //integration
                let mut pre_integration = Schedule::builder()
                    .add_system(clear_person_list_system())
                    .build();
                let mut integration_loop = Schedule::builder()//Integration loop, add systems that updates the gui app state in this loop. this loop might run slower than the main loop
                    .add_system(push_tick_counter_to_integration_system())
                    .add_system(push_persons_to_integration_system())
                    .build();
                let mut post_integration = Schedule::builder()
                    .add_system(print_person_list_system())
                    .build();
                startup.execute(&mut world, &mut resources);
                loop {
                    sim_loop.execute(&mut world, &mut resources);// Execute main sim loop

                    pre_integration.execute(&mut world, &mut resources);
                    integration_loop.execute(&mut world, &mut resources);//execute the integration loop that syncs to the integration state
                    post_integration.execute(&mut world, &mut resources);
                    thread::sleep(Duration::from_millis(500));
                }
            });

            Ok(())
        })
        .manage(ui_app_state)  // Pass the original Arc
        .invoke_handler(tauri::generate_handler![get_tick,get_persons])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}