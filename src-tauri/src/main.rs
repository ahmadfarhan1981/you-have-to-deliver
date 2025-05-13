// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod intergrations;
mod sim;


use crate::sim::systems::global::tick_counter_system;
use crate::sim::resources::global::TickCounter;

use legion::{Schedule, World, Resources};

use std::thread;
use std::time::Duration;
use tauri::State;

use crate::intergrations::ui::{get_tick, AppState};
use std::sync::{atomic::{AtomicU64, Ordering}, Arc};

// === Shared AppState ===



// === Tauri command: expose tick value ===



fn main() {


    let tick_shared = Arc::new(AtomicU64::new(0));
    // Clone for ECS thread
    let tick_clone = tick_shared.clone();

    // === Sim thread ===
    thread::spawn(move || {
        let mut world = World::default();
        let mut resources = Resources::default();

        resources.insert(TickCounter { tick: 0 });

        // Insert Arc into resources so ECS systems can sync to it
        resources.insert(tick_clone);

        let mut schedule = Schedule::builder()
            .add_system(tick_counter_system())
            .build();


        let mut startup = Schedule::builder()
            //.add_system(load_employee_system())
            .build();
        startup.execute(&mut world, &mut resources);
        loop {
            schedule.execute(&mut world, &mut resources);

            let tick = resources.get::<TickCounter>().unwrap().tick;

            // Extract and sync tick
            // let tick = resources.get::<TickCounter>().map(|t| t.tick).unwrap_or(0);
            if let Some(arc) = resources.get::<Arc<AtomicU64>>() {
                arc.store(tick, Ordering::Relaxed);
            }

            thread::sleep(Duration::from_millis(500));
        }
    });

    // === Launch Tauri app ===
    tauri::Builder::default()
        .manage(AppState { tick: tick_shared })
        .invoke_handler(tauri::generate_handler![get_tick])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
