// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]
#![deny(clippy::print_stdout)]
#![allow(clippy::dbg_macro)]

mod config;
mod integrations;
mod macros;
mod master_data;
mod sim;

use crate::sim::resources::global::{AssetBasePath, SimManager, TickCounter};
use crate::sim::systems::global::{increase_sim_tick_system, UsedProfilePictureRegistry};

use legion::{Entity, Resources, Schedule, World};
use sim::systems::global::print_person_system;

use crate::integrations::systems::{
    push_game_speed_snapshots_system,
    push_persons_to_integration_system,
};
use crate::integrations::ui::{new_sim, resume_sim, start_sim, stop_sim, AppContext};
use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};
use crate::sim::person::components::{PersonId, ProfilePicture};
use crate::sim::person::init::{generate_employees_system, load_global_skills_system};
use crate::sim::utils::logging::init_logging;
use crossbeam::queue::SegQueue;
use dashmap::DashSet;
use spin_sleep::SpinSleeper;
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
use crate::integrations::snapshots::{PersonSnapshot, SnapshotField, SnapshotState};
use crate::integrations::snapshots_emitter::snapshots_emitter::{
    run_snapshot_emitters_system, ExportFrequency, SnapshotCollectionEmitter, SnapshotEmitRegistry,
    SnapshotEmitterConfig, SnapshotFieldEmitter,
};
use crate::integrations::system_queues::game_speed_manager::{
    decrease_speed, handle_game_speed_manager_queue_system, increase_speed, set_game_speed,
};
use crate::integrations::system_queues::sim_manager;
use crate::integrations::system_queues::sim_manager::{
    delete_all_entity_system, handle_new_game_manager_queue_system,
    handle_sim_manager_queue_system, reset_state_system,
};

use crate::sim::systems::banner::print_banner;
use crate::sim::utils::sim_reset::ResetRequest;
use crate::sim::utils::term::{bold, green, italic, red};
use parking_lot::{Mutex, RwLock};
use crate::sim::person::skills::SkillId;
use crate::sim::registries::registry::Registry;

fn print_startup_banner() {
    print_banner();
}

pub struct SimContext {
    pub world: Arc<Mutex<World>>,
    pub resources: Arc<Mutex<Resources>>,
}

// Implement Debug manually so Tauri is happy
impl fmt::Debug for SimContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimContext")
            .field("world", &"World")
            .field("resources", &"Resources")
            .finish()
    }
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
    let mut snapshot_registry = SnapshotEmitRegistry::new();
    let game_speed_snapshots_emitter = SnapshotFieldEmitter {
        field: Arc::new(main_snapshot_state.game_speed.clone()),
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: "game_speed_snapshot",
            last_sent_tick: Default::default(),
        },
    };

    let person_snapshots_emitter = SnapshotCollectionEmitter {
        map: Arc::clone(&main_snapshot_state.persons),
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: "persons_snapshot",
            last_sent_tick: Default::default(),
        },
    };
    snapshot_registry.register(game_speed_snapshots_emitter);
    snapshot_registry.register(person_snapshots_emitter);

    let gsm = GameSpeedManager {
        game_speed: GameSpeed::Normal,
    };
    let game_speed = Arc::new(RwLock::new(gsm));

    let mut command_queues = UICommandQueues::default();

    let queue_manager = QueueManager::new();
    command_queues.runtime = queue_manager.dispatch();
    command_queues.control = queue_manager.sim_manager_dispatch();

    // let ui_snapshot_state = Arc::new(snapshot_state);
    // let sim_snapshot_state = Arc::clone(&ui_snapshot_state); // Clone for ECS thread

    let ui_command_queues = Arc::new(command_queues);
    let sim_command_queues = Arc::clone(&ui_command_queues); // Clone for ECS thread

    // Used by person generation to prevent duplicate profile picture. no arc, only used in sim
    let used_portrait = UsedProfilePictureRegistry::default();

    let tick_counter = Arc::new(TickCounter::default());

    let sim_manager = Arc::new(SimManager::default());
    let ui_sim_manager = Arc::clone(&sim_manager);

    let reset_request = Arc::new(ResetRequest {
        should_reset: AtomicBool::new(false),
    });
    let reset = Arc::clone(&reset_request);

    // === Launch Tauri app ===
    tauri::Builder::default()
        .setup(|app| {
            info!("Setup");
            let app_handle = app.handle().clone();
            let path = app
                .path()
                .resolve("assets", tauri::path::BaseDirectory::Resource)?;

            // === Sim thread ===
            thread::spawn(move || {
                let mut world = World::default();
                let mut resources = Resources::default();

                resources.insert(Arc::new(AppContext { app_handle }));

                resources.insert(reset_request);
                resources.insert(Arc::clone(&sim_manager));

                //command queues related
                resources.insert(queue_manager);
                resources.insert(sim_command_queues);

                //tick counter
                resources.insert(tick_counter.clone());
                resources.insert(Arc::clone(&game_speed));

                resources.insert(sim_snapshot_state); // Insert the cloned Arc

                resources.insert(AssetBasePath(path));
                resources.insert(used_portrait);

                //registries
                // resources.insert(Arc::new(PersonRegistry::new()));
                resources.insert(Arc::new(Registry::<PersonId, Entity>::with_name("Person registry")));
                resources.insert(snapshot_registry);
                resources.insert(Arc::new(Registry::<SkillId, Entity>::with_name("Skill registry")));

                // Startup schedule, runs once on startup. add run once systems here.
                let mut startup = Schedule::builder()
                    .add_system(generate_employees_system())
                    .add_system(load_global_skills_system())
                    .build();

                // processes the command dispatch queues,  dispatch then sends to the resource profile specific queues.
                // sim manager runs outside the suspended kill switch
                let mut dispatcher_queue_schedule =
                    Schedule::builder() // Command queue handler, process all incoming command, runs first in the loop. doesn't stop when simulation is pause.
                        .add_system(handle_dispatch_queue_system())
                        .build();

                let mut sim_manager_dispatch_schedule =
                    Schedule::builder() // sim manager schedule, runs outside the killswitch
                        .add_system(handle_sim_manager_dispatch_queue_system())
                        .build();

                let mut sim_manager_schedule =
                    Schedule::builder() // sim manager schedule, runs outside the killswitch
                        .add_system(handle_sim_manager_queue_system())
                        .build();

                let mut sim_manager_reset_schedule =
                    Schedule::builder() // sim manager schedule, runs outside the killswitch
                        .add_system(handle_new_game_manager_queue_system())
                        .build();

                let mut sim_manager_delete_world_entity_schedule =
                    Schedule::builder() // sim manager schedule, runs outside the killswitch
                        .add_system(delete_all_entity_system())
                        .build();

                let mut reset_state_schedule =
                    Schedule::builder() // sim manager schedule, runs outside the killswitch
                        .add_system(reset_state_system())
                        .build();

                /// subsystem command system:
                /// processes the command that was dispatched from the dispatcher queues. uses different resource profiles
                let mut subsystem_command_schedule = Schedule::builder()
                    .add_system(handle_game_speed_manager_queue_system())
                    .build();

                // main sim
                let mut sim_schedule = Schedule::builder() // Main game loop, add systems that runs per frame here.
                    .add_system(increase_sim_tick_system())
                    .add_system(print_person_system())
                    .build();

                //integration, handles generating snapshots
                let mut pre_integration = Schedule::builder().build();
                let mut integration_schedule =
                    Schedule::builder() //Integration loop, add systems that updates the gui app state in this loop. this loop might run slower than the main loop
                        .add_system(push_persons_to_integration_system())
                        .add_system(push_game_speed_snapshots_system())
                        .build();
                let mut post_integration = Schedule::builder()
                    .add_system(run_snapshot_emitters_system())
                    .build();

                let sleeper =
                    SpinSleeper::new(0).with_spin_strategy(spin_sleep::SpinStrategy::YieldThread); // prevents CPU burn
                let state = Arc::clone(&sim_manager);

                //Tick the startup schedule
                startup.execute(&mut world, &mut resources);

                loop {
                    sim_manager_dispatch_schedule.execute(&mut world, &mut resources);
                    sim_manager_reset_schedule.execute(&mut world, &mut resources);
                    if reset.should_reset.load(Ordering::Relaxed) {
                        sim_manager_delete_world_entity_schedule
                            .execute(&mut world, &mut resources);
                        reset_state_schedule.execute(&mut world, &mut resources);
                        startup.execute(&mut world, &mut resources);

                        pre_integration.execute(&mut world, &mut resources);
                        integration_schedule.execute(&mut world, &mut resources);
                        post_integration.execute(&mut world, &mut resources);
                    }

                    sim_manager_schedule.execute(&mut world, &mut resources);

                    if state.is_running() {
                        let tick_start = Instant::now();

                        // Process SimCommand queue
                        dispatcher_queue_schedule.execute(&mut world, &mut resources);

                        subsystem_command_schedule.execute(&mut world, &mut resources);

                        // Main sim tick only if not paused. paused will return a None current interval
                        let maybe_interval = game_speed.read().current_interval();
                        if let Some(_) = maybe_interval {
                            sim_schedule.execute(&mut world, &mut resources);
                        }

                        // Always run integration so UI sees updates
                        pre_integration.execute(&mut world, &mut resources);
                        integration_schedule.execute(&mut world, &mut resources);
                        post_integration.execute(&mut world, &mut resources);

                        let elapsed = tick_start.elapsed();

                        match maybe_interval {
                            Some(tick_duration) => {
                                trace!(
                                    "tick duration {} elapse {}",
                                    tick_duration.as_millis(),
                                    elapsed.as_millis()
                                );
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
                    } else {
                        sleeper.sleep(Duration::from_millis(100));
                    }
                }
            });

            Ok(())
        })
        .manage(ui_snapshot_state)
        .manage(ui_command_queues)
        .manage(ui_sim_manager)
        .invoke_handler(tauri::generate_handler![
            set_game_speed,
            increase_speed,
            decrease_speed,
            start_sim,
            stop_sim,
            resume_sim,
            new_sim,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
