// Added for SavedEmployee
use crate::action_queues::sim_manager::SimManager;
use crate::db::constants::{db_keys, save_version};
use crate::db::init::{SaveSlot, SaveSlotMetadata, SavesDirectory};
use crate::integrations::snapshots::{company, person, team};
use crate::integrations::snapshots::snapshots::SnapshotState;
use crate::integrations::snapshots_emitter::snapshots_emitter::SnapshotEmitRegistry;
use crate::sim::ai::goap::CurrentGoal;
use crate::sim::company::company::{Company, PlayerControlled};
use crate::sim::person::components::{Person, PersonId, ProfilePicture};
use crate::sim::person::thoughts::{Thoughts, ArchivedThoughts};
use crate::sim::person::morale::StressLevel;
use crate::sim::person::needs::{Energy, Hunger};
use crate::sim::person::personality_matrix::PersonalityMatrix;
use crate::sim::person::skills::SkillSet;
use crate::sim::person::stats::Stats;
use crate::sim::registries::registry::Registry;
use crate::sim::resources::global::TickCounter;
use crate::sim::sim_date::sim_date::SimDate;
use crate::sim::systems::global::UsedProfilePictureRegistry;
use crate::sim::team::components::{Team, TeamId};
use crate::schedules::init::GameSchedules;
use crate::utils::errors::LoadDataFromDBError;
use crate::utils::errors::{SavesManagementError, SavesManagementError::TimeError};
use bincode::error::EncodeError;
use bincode::{encode_to_vec, Decode, Encode};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{query, system, Entity, IntoQuery, Query, Resources, World};
use parking_lot::RwLock;
use rand_distr::num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::utils::acl::Commands;
use tracing::{error, info, warn};
use crate::sim::calendar::components::CalendarEvent;
// Added for logging

/// Represents the data of an employee that can be saved or transferred.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct SavedEmployee {
    pub person: Person,
    pub stats: Stats,
    pub profile_picture: ProfilePicture,
    pub personality_matrix: PersonalityMatrix,
    pub skill_set: SkillSet,
    pub energy: Energy,
    pub hunger: Hunger,
    pub current_goal: CurrentGoal,
    pub stress_level: StressLevel,
    pub thoughts: Thoughts,
    pub archived_thoughts: ArchivedThoughts,
}

#[derive(Debug)]
pub struct LoadGame{
    pub should_load: AtomicBool,
    pub slot_id: RwLock<Option<String>>,
}
impl LoadGame{
}
impl Default for LoadGame {
    fn default() -> Self {
        Self{
            should_load: AtomicBool::new(false),
            slot_id: RwLock::new(None),
        }
    }
}

#[system]
pub fn save_game_state(
    world: &SubWorld,
    #[resource] current_tick: &Arc<TickCounter>,
    #[resource] sim_manager: &Arc<SimManager>,
    #[resource] saves_directory: &Arc<SavesDirectory>,
    #[resource] used_profile_pictures: &UsedProfilePictureRegistry,
    query: &mut Query<(
        &Person,
        &Stats,
        &ProfilePicture,
        &PersonalityMatrix,
        &SkillSet,
        &Energy,
        &Hunger,
        &CurrentGoal,
        &StressLevel,
        Option<&Thoughts>,
        Option<&ArchivedThoughts>,
        &PlayerControlled,
    )>,
    company_query: &mut Query<(&Company, &PlayerControlled)>,
    team_query: &mut Query<(&Team)>,
    calendar_event_query: &mut Query<(&CalendarEvent)>,
) {
    if !sim_manager.has_save_slot() {
        warn!("No active save slot");
        return;
    }
    if sim_manager
        .with_save_slot(|slot| {
            if slot.is_empty {
                warn!("Empty save slot");
                true
            } else {
                false
            }
        })
        .unwrap_or(true)
    {
        return; // This exits the outer function
    }

    
    sim_manager.with_save_slot(|current_save| {
        current_save.ensure_db_handle_is_open(saves_directory);

        let mut employee_id_list: Vec<u32> = vec![]; 
        for (
            person,
            stats,
            profile_picture,
            personality_matrix,
            skill_set,
            energy,
            hunger,
            current_goal,
            stress_level,
            thoughts,
            archived_thoughts,
            _player_controlled,
        ) in query.iter(world)
        {
            let saved_employee = SavedEmployee {
                person: person.clone(),
                stats: stats.clone(),
                profile_picture: profile_picture.clone(),
                personality_matrix: personality_matrix.clone(),
                skill_set: skill_set.clone(),
                energy: energy.clone(),
                hunger: hunger.clone(),
                current_goal: current_goal.clone(),
                stress_level: stress_level.clone(),
                thoughts: thoughts.cloned().unwrap_or_default(),
                archived_thoughts: archived_thoughts.cloned().unwrap_or_default(),
            };

            current_save.save_entry(
                format!("{}{}", db_keys::EMPLOYEE_PREFIX, person.person_id.0).as_str(),
                &saved_employee,
            );
            employee_id_list.push(person.person_id.0);

        }
        current_save.save_entry(db_keys::EMPLOYEES_LIST, &employee_id_list );
        // Save the player-controlled company.
        // Currently, only the one player controlled company exists.
        //
        // Non player companies will be added later.
        // They will be stored using a different db key in future.
        for (company, _player_controlled) in company_query.iter(world) {
            current_save.save_entry(db_keys::COMPANY, company);
        }

        let teams: Vec<Team> = team_query.iter(world).map(|t| t.clone()).collect();
        current_save.save_entry(db_keys::TEAMS, &teams);

        let calendar_events: Vec<CalendarEvent> = calendar_event_query.iter(world).map(|t| t.clone()).collect();
        current_save.save_entry(db_keys::CALENDAR_EVENTS, &calendar_events);
        
        current_save.save_entry(db_keys::TICK_COUNTER, current_tick);

        let metadata = SaveSlotMetadata {
            name: current_save.metadata.clone().unwrap().name.clone(),
            employee_count: employee_id_list.len() as u32,
            sim_date: current_tick.current_date(),
            save_version: save_version::SAVE_VERSION.to_string(),
            last_saved_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        };
        current_save.save_entry(db_keys::METADATA, &metadata);
        
        current_save.save_entry(db_keys::USED_PROFILE_PICTURES, used_profile_pictures);
        
        
        
        // match current_save.load_entry::<TickCounter>(db_keys::TICK_COUNTER){
        //     Ok(Some(tick)) => {
        //         info!("Tick counter loaded {:?}", tick);
        //     }
        //     Ok(None)=>{info!("Nothing in db for tick counter")}
        //     Err(e) => {error!("Error loading tick counter: {:?}", e);}
        // }
        
    });
}

pub fn load_game_state(
    world: &mut World,
    resources: &mut Resources,
    loop_load_game: &Arc<LoadGame>,
    loop_tick_counter: &Arc<TickCounter>,
    loop_sim_manager: &Arc<SimManager>,
    loop_snapshot_state: &Arc<SnapshotState>,
    loop_snapshot_registry: &Arc<SnapshotEmitRegistry>,
    game_schedules: &mut GameSchedules,
) -> Result<(), SavesManagementError> {
    use crate::integrations::events::{emit_app_event, AppEventType};
    use crate::integrations::ui::AppContext;
    use crate::sim::resources::global::Dirty;
    use crate::sim::utils::debugging::DebugDisplayComponent;
    use std::io;

    let saves_directory = resources
        .get::<Arc<SavesDirectory>>()
        .ok_or_else(|| SavesManagementError::Io(io::Error::new(io::ErrorKind::NotFound, "SavesDirectory resource not found")))?
        .clone();
    let slot = loop_load_game
        .slot_id
        .read()
        .as_ref()
        .ok_or_else(|| SavesManagementError::Io(io::Error::new(io::ErrorKind::NotFound, "Save slot ID not set")))?
        .clone();

    let mut save_slot = SaveSlot {
        slot_id: slot.clone(),
        path: saves_directory.0.join(slot.clone()),
        metadata: None,
        is_empty: false,
        handle: None,
    };

    save_slot.ensure_db_handle_is_open(&saves_directory)?;

    info!("Resetting the world...");
    world.clear();

    info!("Loading employees...");
    let employee_list = save_slot.load_entry::<Vec<u32>>(db_keys::EMPLOYEES_LIST)?;
    for employee_id in employee_list {
        let employee = save_slot.load_entry::<SavedEmployee>(&format!("{}{}", db_keys::EMPLOYEE_PREFIX, employee_id))?;
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
            employee.thoughts,
            employee.archived_thoughts,
            DebugDisplayComponent::default(),
            PlayerControlled,
            Dirty,
        ));
    }

    info!("Loading teams...");
    let teams = save_slot.load_entry::<Vec<Team>>(db_keys::TEAMS)?;
    for team in teams {
        world.push((team, Dirty));
    }

    info!("Loading calendar events");
    let calendar_events = save_slot.load_entry::<Vec<CalendarEvent>>(db_keys::CALENDAR_EVENTS)?;
    for calendar_event in calendar_events {
        world.push((calendar_event, Dirty));
    }

    info!("Loading company...");
    let company = save_slot.load_entry::<Company>(db_keys::COMPANY)?;
    world.push((company, PlayerControlled, Dirty));

    info!("Loading tick_counter...");
    let tick_counter = save_slot.load_entry::<TickCounter>(db_keys::TICK_COUNTER)?;
    loop_tick_counter.update_from(&tick_counter);

    let metadata = save_slot.load_entry::<SaveSlotMetadata>(db_keys::METADATA)?;
    save_slot.metadata = Some(metadata);

    info!("Load game {:?}.", loop_load_game);
    loop_load_game.should_load.store(false, Ordering::Relaxed);
    *loop_load_game.slot_id.write() = None;

    *loop_sim_manager.save_slot.lock() = Some(save_slot);

    game_schedules.load_game_schedule.execute(world, resources);

    loop_snapshot_state.reset();
    loop_snapshot_registry.reset();

    let app_context = resources
        .get::<Arc<AppContext>>()
        .ok_or_else(|| SavesManagementError::Io(io::Error::new(io::ErrorKind::NotFound, "AppContext resource not found")))?;
    info!("emit_done_setup_event");
    emit_app_event(&app_context.app_handle, AppEventType::InitDone);

    Ok(())
}




#[system]
pub fn sync_registry_from_person(
    world: &SubWorld,
    query: &mut Query<(&Person, &Entity)>,
    #[resource] person_registry: &Arc<Registry<PersonId, Entity>>,
) {
    info!("Syncing registry from person...");
    let x = query
        .iter(world)
        .map(|(person, entity)| (person.person_id, *entity));
    person_registry.repopulate_from_entities(x);
}

#[system]
pub fn sync_registry_from_team(
    world: &SubWorld,
    query: &mut Query<(&Team, &Entity)>,
    #[resource] team_registry: &Arc<Registry<TeamId, Entity>>,
) {
    info!("Syncing registry from team...");
    let x = query
        .iter(world)
        .map(|(team, entity)| (team.team_id, *entity));
    team_registry.repopulate_from_entities(x);
}
