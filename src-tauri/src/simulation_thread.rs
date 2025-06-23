use std::{io, sync::{atomic::Ordering, Arc}, thread, time::{Duration, Instant}};
use std::io::BufRead;
use std::thread::sleep;
use legion::{Resources, World};
use parking_lot::RwLock;
use spin_sleep::{SpinSleeper, SpinStrategy};
use tracing::{error, info, trace};

use crate::{
    integrations::{
        queues::{QueueManager, UICommandQueues},
        snapshots::snapshots::SnapshotState,
        snapshots_emitter::snapshots_emitter::SnapshotEmitRegistry,
        ui::AppContext,
    },
    resources::init::initialize_non_shared_resources,
    schedules::init::init_schedules,
    sim::{
        game_speed::components::{GameSpeed, GameSpeedManager},
        person::init::FirstRun,
        resources::global::{AssetBasePath, TickCounter},
        utils::sim_reset::ResetRequest,
    },
};
use crate::action_queues::sim_manager::{SimManager, SimManagerCommand};
use crate::action_queues::team_manager::TeamManagerCommand;
use crate::db::constants::{db_keys, save_version};
use crate::db::error::SavesManagementError;
use crate::db::init::{SaveSlot, SaveSlotMetadata, SavesDirectory};
use crate::integrations::events::{emit_app_event, AppEventType};
use crate::integrations::queues::SimCommand;
use crate::integrations::queues::SimCommand::TeamManager;
use crate::sim::action::action::ActionIntent;
use crate::sim::company::company::{Company, PlayerControlled};
use crate::sim::persistence::persistence::{LoadGame, SavedEmployee};
use crate::sim::person::init::ShouldGenerateEmployees;
use crate::sim::resources::global::Dirty;
use crate::sim::sim_date::sim_date::SimDate;
use crate::sim::team::components::Team;
use crate::sim::utils::debugging::DebugDisplayComponent;

pub struct SimThreadConfig {
    pub app_handle: tauri::AppHandle,
    pub asset_base_path: std::path::PathBuf,
    pub reset_request: Arc<ResetRequest>,
    pub first_run: Arc<FirstRun>,
    pub sim_manager: Arc<SimManager>,
    pub queue_manager: QueueManager,
    pub sim_command_queues: Arc<UICommandQueues>,
    pub tick_counter: Arc<TickCounter>,
    pub game_speed: Arc<RwLock<GameSpeedManager>>,
    pub sim_snapshot_state: Arc<SnapshotState>,
    pub sim_snapshot_registry: Arc<SnapshotEmitRegistry>,
    pub saves_directory: Arc<SavesDirectory>,
}

pub fn run_simulation_thread(config: SimThreadConfig) {
    let mut world = World::default();
    let mut resources = Resources::default();

    world.push(( ShouldGenerateEmployees(true), ));
    // Destructure config for use
    let SimThreadConfig {
        app_handle,
        asset_base_path,
        reset_request,
        first_run,
        sim_manager,
        queue_manager,
        sim_command_queues,
        tick_counter,
        game_speed,
        sim_snapshot_state,
        sim_snapshot_registry,
        saves_directory,
    } = config;

    resources.insert(Arc::new(AppContext { app_handle }));
    resources.insert(Arc::clone(&reset_request));
    resources.insert(Arc::clone(&first_run));
    resources.insert(Arc::clone(&sim_manager));
    resources.insert(queue_manager);
    resources.insert(Arc::clone(&sim_command_queues));
    resources.insert(Arc::clone(&tick_counter));
    resources.insert(Arc::clone(&game_speed));
    resources.insert(Arc::clone(&sim_snapshot_state));
    resources.insert(AssetBasePath(asset_base_path));
    resources.insert(Arc::clone(&sim_snapshot_registry));
    resources.insert(Arc::clone(&saves_directory));
    

    initialize_non_shared_resources(&mut resources);

    let mut game_schedules = init_schedules();
    let sleeper = SpinSleeper::new(0).with_spin_strategy(SpinStrategy::YieldThread);

    let load_game = Arc::new(LoadGame::default());
    resources.insert(Arc::clone(&load_game));
    
    // Clones for the loop
    let loop_sim_manager = Arc::clone(&sim_manager);
    let loop_reset_request = Arc::clone(&reset_request);
    let loop_first_run = Arc::clone(&first_run);
    let loop_game_speed = Arc::clone(&game_speed);
    let loop_tick_counter = Arc::clone(&tick_counter);
    let loop_load_game = Arc::clone(&load_game);
    let loop_snapshot_registry = Arc::clone(&sim_snapshot_registry);
    let loop_snapshot_state = Arc::clone(&sim_snapshot_state);

    #[cfg(debug_assertions)]
    {
        // --- Start of new code for console input ---
        let console_input_queues = Arc::clone(&sim_command_queues);
        thread::spawn(move || {
            println!("Console input thread started. Type commands and press Enter.");
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                match line {
                    Ok(input) => {
                        let trimmed_input = input.trim();
                        if trimmed_input.is_empty() {
                            continue;
                        }
                        println!("Console input: {}", trimmed_input);
                        // Example: Parse the input and push a command.
                        // You'll need to define how to parse `trimmed_input` into a `SimCommand`.
                        // For now, let's assume a simple command.
                        if trimmed_input == "team" {
                            console_input_queues.runtime.push(SimCommand::TeamManager(TeamManagerCommand::NewTeam {name:"Team Alpha".to_string(), description: "First team".to_string()}));
                            
                        }else if trimmed_input == "stop" {
                            console_input_queues.runtime.push(SimCommand::SimManager(SimManagerCommand::StopSim));
                        }else if trimmed_input== "resume" {
                            console_input_queues.control.push(SimManagerCommand::ResumeSim);
                        
                        }
                        
                        // Add more command parsing logic here
                    }
                    Err(error) => {
                        eprintln!("Error reading from console: {}", error);
                        break; // Exit thread on error
                    }
                }
            }
            println!("Console input thread finished.");
        });
        // --- End of new code for console input ---
    }
    
    loop {
        game_schedules
            .sim_manager_dispatch
            .execute(&mut world, &mut resources);
        game_schedules
            .sim_manager_reset
            .execute(&mut world, &mut resources);

        if loop_reset_request.should_reset.load(Ordering::Relaxed) {
            game_schedules
                .sim_manager_delete_world_entity
                .execute(&mut world, &mut resources);
            game_schedules
                .reset_state
                .execute(&mut world, &mut resources);
            game_schedules
                .pre_integration
                .execute(&mut world, &mut resources);
            game_schedules
                .integration
                .execute(&mut world, &mut resources);
            game_schedules
                .post_integration
                .execute(&mut world, &mut resources);
        }

        game_schedules
            .sim_manager
            .execute(&mut world, &mut resources);

        if loop_sim_manager.is_running() {
            if loop_load_game.should_load.load(Ordering::Relaxed) {
                //switch to save slot.
                let saves_directory = resources.get::<Arc<SavesDirectory>>().unwrap().clone();
                let slot = loop_load_game.slot_id.read().as_ref().unwrap().clone();

                let mut save_slot = SaveSlot{
                    slot_id: slot.clone(),
                    path: saves_directory.0.join(slot.clone()),
                    metadata: None,
                    is_empty: false,
                    handle: None,
                };
                match save_slot.ensure_db_handle_is_open(&saves_directory) {// populates the handle
                    Ok(_) => {}
                    Err(e) => {error!("Error when opening save slot. {:?}", e); return}
                }





                info!("Resetting the world...");
                world.clear();

                info!("Loading employees...");
                let Ok(Some(employee_list)) = save_slot.load_entry::<Vec<u32>>(db_keys::EMPLOYEES_LIST) else {return};
                let number_of_employees = employee_list.len();
                for employee_id in employee_list {
                    let Ok(Some(employee)) = save_slot.load_entry::<SavedEmployee>(format!("{}{}",db_keys::EMPLOYEE_PREFIX, employee_id).as_str()) else {return};
                    
                    info!("Loading employees: {:?}", employee);
                    world.push((
                        employee.person,
                        employee.stats,
                        employee.profile_picture,
                        employee.personality_matrix,
                        employee.hunger,
                        employee.energy,
                        employee.skill_set,
                        employee.stress_level,
                        employee.current_goal,
                        DebugDisplayComponent::default(),
                        PlayerControlled,
                        Dirty
                        ) );
                }

                info!("Loading teams...");
                let Ok(Some(team)) = save_slot.load_entry::<Vec<Team>>(db_keys::TEAMS) else {return};
                for team in team {
                    world.push((team, Dirty));
                }

                info!("Loading company...");
                let Ok(Some(company)) = save_slot.load_entry::<Company>(db_keys::COMPANY) else {return};
                let company_name = company.name.clone();
                world.push((company, PlayerControlled, Dirty));

                info!("Loading tick_counter...");
                let Ok(Some(tick_counter)) = save_slot.load_entry::<TickCounter>(db_keys::TICK_COUNTER) else {return};
                loop_tick_counter.update_from(&tick_counter);

                let Ok(Some(metadata)) = save_slot.load_entry::<SaveSlotMetadata>(db_keys::METADATA) else {return};
                save_slot.metadata = Some(metadata);

                
                //load data
                info!("Load game {:?}.", loop_load_game);
                loop_load_game.should_load.store(false, Ordering::Relaxed);
                let mut  slot = loop_load_game.slot_id.write();
                *slot = None;

                *loop_sim_manager.save_slot.lock() = Some(save_slot);
                
                game_schedules.load_game_schedule.execute(&mut world, &mut resources);// repopulate registries

                
                //clear snapshotshot state
                loop_snapshot_state.reset();
                //reset emitters
                loop_snapshot_registry.reset();
                //&DashMap<&'static str, u64> clear last update map
                
                let app_context = resources.get::<Arc<AppContext>>().unwrap() ;
                info!("emit_done_setup_event");
                emit_app_event(&app_context.app_handle, AppEventType::InitDone)
            }
            
            if loop_first_run.is_first_run() {
                game_schedules.startup.execute(&mut world, &mut resources);
            }
            let tick_start = Instant::now();

            game_schedules
                .dispatcher_queue
                .execute(&mut world, &mut resources);
            game_schedules
                .subsystem_command
                .execute(&mut world, &mut resources);

            let maybe_interval = loop_game_speed.read().current_interval();
            if maybe_interval.is_some() {
                game_schedules.sim.execute(&mut world, &mut resources);
            }

            game_schedules
                .pre_integration
                .execute(&mut world, &mut resources);
            game_schedules
                .integration
                .execute(&mut world, &mut resources);
            game_schedules
                .post_integration
                .execute(&mut world, &mut resources);

            let elapsed = tick_start.elapsed();
            match maybe_interval {
                Some(tick_duration) => {
                    trace!(
                        "tick duration {}ms, elapsed {}ms",
                        tick_duration.as_millis(),
                        elapsed.as_millis()
                    );
                    if elapsed < tick_duration {
                        sleeper.sleep(tick_duration - elapsed);
                    } else {
                        eprintln!(
                            "{:?} Tick lag: {:?}",
                            loop_tick_counter,
                            elapsed - tick_duration
                        );
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
}
