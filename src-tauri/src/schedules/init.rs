use crate::action_queues::game_speed_manager::handle_game_speed_manager_queue_system;
use crate::action_queues::sim_manager::{handle_new_game_manager_queue_system, handle_sim_manager_queue_system, test_sim_manager_system};
use crate::action_queues::team_manager::{handle_team_assignment_queue_system, handle_team_manager_queue_system};
use crate::action_queues::thought_manager::handle_thought_command_queue_system;
use crate::integrations::queues::{handle_dispatch_queue_system, handle_sim_manager_dispatch_queue_system};
use crate::integrations::snapshots_emitter::snapshots_emitter::run_snapshot_emitters_system;
use crate::integrations::systems::{push_company_to_integration_system, push_debug_displays_to_integration_system, push_game_speed_snapshots_system, push_needs_to_integration_system, push_persons_to_integration_system, push_stress_history_to_integration_system, push_stress_level_to_integration_system, push_teams_to_integration_system, tick_needs_system};
use crate::sim::action::action::{decide_action_system, execute_action_system};
use crate::sim::ai::consideration::goal_selection_system;
use crate::sim::calendar::systems::sync_registry_from_calendar_event_system;
use crate::sim::persistence::persistence::{save_game_state_system, sync_registry_from_person_system, sync_registry_from_team_system};
use crate::sim::person::init::{emit_done_setup_event_system, generate_employees_system, init_company_system, unset_first_run_flag_system};
use crate::sim::person::morale::{daily_stress_reset_system, update_stress_system};
use crate::sim::systems::global::{increase_sim_tick_system, print_person_system};
use crate::sim::systems::time_triggers::morning_thought_trigger_system;
use crate::sim::utils::debugging::clear_debug_display_system;
use crate::sim::utils::sim_reset::{delete_all_entity_system, reset_snapshot_system, reset_state_system};
use legion::systems::{Builder, ParallelRunnable};
use legion::Schedule;

pub struct GameSchedules {
    pub startup: Schedule,
    pub dispatcher_queue: Schedule,
    pub sim_manager_dispatch: Schedule,
    pub sim_manager: Schedule,
    pub sim_manager_reset: Schedule,
    pub sim_manager_delete_world_entity: Schedule,
    pub reset_state: Schedule,
    pub subsystem_command: Schedule,
    pub sim: Schedule,
    pub pre_integration: Schedule,
    pub integration: Schedule,
    pub post_integration: Schedule,
    pub load_game_schedule: Schedule,
}

pub fn init_schedules() -> GameSchedules {
    // Startup schedule, runs once on startup. add run once systems here.
    let startup = Schedule::builder()
        .add_system(init_company_system())
        .flush()
        .add_system(generate_employees_system())
        .flush()
        .add_system(unset_first_run_flag_system())
        // .flush()
        // .add_system(emit_done_setup_event_system())
        .build();


    // processes the command dispatch queues,  dispatch then sends to the resource profile specific queues.
    // sim manager runs outside the suspended kill switch
    let dispatcher_queue =
        Schedule::builder() // Command queue handler, process all incoming command, runs first in the loop. doesn't stop when simulation is pause.
            .add_system(handle_dispatch_queue_system())
            .build();

    let sim_manager_dispatch =
        Schedule::builder() // sim manager schedule, runs outside the killswitch
            .add_system(handle_sim_manager_dispatch_queue_system())
            .build();

    let sim_manager =
        Schedule::builder() // sim manager schedule, runs outside the killswitch
            .add_system(handle_sim_manager_queue_system())
            .build();

    let sim_manager_reset =
        Schedule::builder() // sim manager schedule, runs outside the killswitch
            .add_system(handle_new_game_manager_queue_system())
            .build();

    let sim_manager_delete_world_entity =
        Schedule::builder() // sim manager schedule, runs outside the killswitch
            .add_system(delete_all_entity_system())
            .build();

    let reset_state =
        Schedule::builder() // sim manager schedule, runs outside the killswitch
            .add_system(reset_snapshot_system())
            .flush()
            .add_system(reset_state_system())
            .flush()
            .add_system(emit_done_setup_event_system())
            .build();

    /// subsystem command system:
    /// processes the command that was dispatched from the dispatcher queues. uses different resource profiles
    let subsystem_command = Schedule::builder()
        .add_system(handle_game_speed_manager_queue_system())
        .add_system(handle_team_manager_queue_system())
        .add_system(handle_team_assignment_queue_system())
        .build();

    // main sim
    let sim = Schedule::builder() // Main game loop, add systems that runs per frame here.
        .add_system(clear_debug_display_system())
        .add_system(increase_sim_tick_system())
        .add_system(print_person_system())
        .add_system(morning_thought_trigger_system())
        .add_system(goal_selection_system())
        .add_system(update_stress_system())
        .add_system(daily_stress_reset_system())
        .add_system(tick_needs_system())
        .flush()
        .add_system(decide_action_system())
        .flush()
        .add_system(execute_action_system())
        .add_system(test_sim_manager_system())
        .add_system(save_game_state_system())
        .build();
    
    //integration, handles generating snapshots
    let pre_integration = Schedule::builder()
        .add_system(handle_thought_command_queue_system())
        .build();
    let integration =
        Schedule::builder() //Integration loop, add systems that updates the gui app state in this loop. this loop might run slower than the main loop
            .add_system(push_persons_to_integration_system())
            .add_system(push_game_speed_snapshots_system())
            .add_system(push_company_to_integration_system())
            .add_system(push_teams_to_integration_system())
            .add_system(push_needs_to_integration_system())
            .add_system(push_debug_displays_to_integration_system())
            .add_system(push_stress_level_to_integration_system())
            .add_system(push_stress_history_to_integration_system())
            .build();
    let post_integration = Schedule::builder()
        .add_system(run_snapshot_emitters_system())
        .build();
 
    let load_game_schedule = Schedule::builder()
        .add_system(sync_registry_from_person_system())
        .flush()
        .add_system(sync_registry_from_team_system())
        .flush()
        .add_system(sync_registry_from_calendar_event_system())
        .build();
    
    GameSchedules {
        startup,
        dispatcher_queue,
        sim_manager_dispatch,
        sim_manager,
        sim_manager_reset,
        sim_manager_delete_world_entity,
        reset_state,
        subsystem_command,
        sim,
        pre_integration,
        integration,
        post_integration,
        load_game_schedule
    }
}

pub fn build_schedule<T>( systems: Vec<T>, schedule: &mut Builder)
where T: ParallelRunnable + 'static{
    
    for sys in systems{
        schedule.add_system(sys);
    }
    
}