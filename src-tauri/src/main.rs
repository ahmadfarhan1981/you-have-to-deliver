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
use std::time::Instant;

use crate::integrations::events::{handle_commands_system, SimCommand};
use crate::integrations::systems::{
    clear_person_list_system, print_person_list_system, push_persons_to_integration_system,
    push_tick_counter_to_integration_system,
};
use crate::integrations::ui::{decrease_speed, get_persons, get_tick, increase_speed, set_game_speed, AppState};
use crate::sim::game_speed::components::{GameSpeed, GameSpeedSettings};
use crate::sim::person::components::ProfilePicture;
use crate::sim::person::registry::PersonRegistry;
use crate::sim::person::systems::generate_employees_system;
use crossbeam::queue::SegQueue;
use dashmap::DashSet;
use spin_sleep::SpinSleeper;
use std::sync::{Arc, RwLock};
use tauri::Manager;

fn main() {
    // Create a properly shared AppState
    let mut app_state = AppState::default();
    let command_queue = Arc::new(SegQueue::<SimCommand>::new());
    app_state.command_queue = Arc::clone(&command_queue);

    let ui_app_state = Arc::new(app_state);
    // Clone for ECS thread
    let sim_app_state = Arc::clone(&ui_app_state);

    // Used by person generation to prevent duplicate profile picture. no arc, only used in sim
    let used_portrait = DashSet::<ProfilePicture>::new();

    let tick_counter = Arc::new(TickCounter::default());
    let game_speed = Arc::new(RwLock::new(GameSpeedSettings {
        game_speed: GameSpeed::Normal,
    }));

    // === Launch Tauri app ===
    tauri::Builder::default()
        .setup(|app| {
            let path = app
                .path()
                .resolve("assets", tauri::path::BaseDirectory::Resource)?;

            // === Sim thread ===
            thread::spawn(move || {
                let mut world = World::default();
                let mut resources = Resources::default();

                //queues
                resources.insert(Arc::clone(&command_queue));

                //tick counter
                resources.insert(tick_counter.clone());
                resources.insert(Arc::clone(&game_speed));

                // Insert Arc into resources so ECS systems can sync to it
                resources.insert(sim_app_state); // Insert the cloned Arc

                resources.insert(AssetBasePath(path));
                resources.insert(used_portrait);
                resources.insert(Arc::new(PersonRegistry::new()));

                let mut startup = Schedule::builder() // Startup schedule, runs once on startup. add run once systems here.
                    .add_system(generate_employees_system())
                    .build();

                let mut command_queue_loop = Schedule::builder() // Command queue handler, process all incoming command, runs first in the loop. doesnt stop when simulation is pause.
                    .add_system(handle_commands_system())
                    .build();


                let mut sim_loop = Schedule::builder() // Main game loop, add systems that runs per frame here.
                    .add_system(increase_sim_tick_system())
                    .add_system(print_person_system())
                    .build();

                //integration
                let mut pre_integration = Schedule::builder()
                    .add_system(clear_person_list_system())
                    .build();
                let mut integration_loop =
                    Schedule::builder() //Integration loop, add systems that updates the gui app state in this loop. this loop might run slower than the main loop
                        .add_system(push_tick_counter_to_integration_system())
                        .add_system(push_persons_to_integration_system())
                        .build();
                let mut post_integration = Schedule::builder()
                    .add_system(print_person_list_system())
                    .build();
                startup.execute(&mut world, &mut resources);

                let sleeper =
                    SpinSleeper::new(0).with_spin_strategy(spin_sleep::SpinStrategy::YieldThread); // prevents CPU burn

                loop {
                    let tick_start = Instant::now();

                    // Process UI queue or any pending commands (not shown here)
                    command_queue_loop.execute(&mut world, &mut resources);

                    let maybe_interval = game_speed.read().unwrap().current_interval();

                    // Main sim tick only if not paused
                    if let Some(_) = maybe_interval {
                        sim_loop.execute(&mut world, &mut resources);
                    }

                    // Always run integration so UI sees updates
                    pre_integration.execute(&mut world, &mut resources);
                    integration_loop.execute(&mut world, &mut resources);
                    post_integration.execute(&mut world, &mut resources);

                    let elapsed = tick_start.elapsed();

                    match maybe_interval {
                        Some(tick_duration) => {
                            if elapsed < tick_duration {
                                sleeper.sleep(tick_duration - elapsed);
                            } else {
                                eprintln!("Tick lag: {:?}", elapsed - tick_duration);
                                // Don’t sleep again — just loop immediately to catch up
                            }
                        }
                        None => {
                            eprintln!("Sim paused, sleeping normal tick...");
                            sleeper.sleep(GameSpeed::Normal.interval().unwrap());
                        }
                    }
                }
            });

            Ok(())
        })
        .manage(ui_app_state) // Pass the original Arc
        .invoke_handler(tauri::generate_handler![get_tick, get_persons,set_game_speed, increase_speed, decrease_speed])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
