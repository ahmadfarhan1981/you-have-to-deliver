// Added for SavedEmployee
use crate::action_queues::sim_manager::SimManager;
use crate::db::constants::{db_keys, save_version};
use crate::db::init::{SaveSlot, SaveSlotMetadata, SavesDirectory};
use crate::integrations::snapshots::{company, person, team};
use crate::sim::ai::goap::CurrentGoal;
use crate::sim::company::company::{Company, PlayerControlled};
use crate::sim::person::components::{Person, PersonId, ProfilePicture};
use crate::sim::person::morale::StressLevel;
use crate::sim::person::needs::{Energy, Hunger};
use crate::sim::person::personality_matrix::PersonalityMatrix;
use crate::sim::person::skills::SkillSet;
use crate::sim::person::stats::Stats;
use crate::sim::registries::registry::Registry;
use crate::sim::resources::global::TickCounter;
use crate::sim::sim_date::sim_date::SimDate;
use crate::sim::team::components::{Team, TeamId};
use bincode::error::EncodeError;
use bincode::{encode_to_vec, Decode, Encode};
use legion::world::SubWorld;
use legion::{query, system, Entity, IntoQuery, Query, Resources, World};
use rand_distr::num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::{SystemTime, UNIX_EPOCH};
use legion::systems::CommandBuffer;
use parking_lot::RwLock;
use tauri::utils::acl::Commands;
use tracing::{error, info, warn};
use crate::utils::errors::SavesManagementError::TimeError;
use crate::sim::systems::global::UsedProfilePictureRegistry;
use crate::utils::errors::LoadDataFromDBError;
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
        &PlayerControlled,
    )>,
    company_query: &mut Query<(&Company, &PlayerControlled)>,
    team_query: &mut Query<(&Team)>,
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
    // #[resource] current_tick: &Arc<TickCounter>,
    // #[resource] sim_manager: &Arc<SimManager>,
    // #[resource] saves_directory: &Arc<SavesDirectory>,
    // #[resource] used_profile_pictures: &UsedProfilePictureRegistry,
    // commands: &mut CommandBuffer,
    // load: &mut Query<&Company>,
)
{
    // match resources.get::<&Arc< crate::action_queues::sim_manager::SimManager >>(){
    //     Some(sim_manager) => {
    //         if !sim_manager.has_save_slot() {
    //             warn!("No active save slot");
    //             return;
    //         }
    //
    //         sim_manager.with_save_slot(|current_save| {
    //             match resources.get_mut::<&Arc<TickCounter>>(){
    //                 Some(tick_counter) => {
    //                     let Ok(Some(saved_tick_counter)) = current_save.load_entry::<TickCounter>(db_keys::TICK_COUNTER) else {error!("Error loading tick counter"); return};
    //                     info!("Saved{:?}",saved_tick_counter);
    //                     info!("Current{:?}",tick_counter);
    //                     //tick_counter.update_from(&saved_tick_counter);
    //
    //
    //                 }
    //                 None => {}
    //
    //             }
    //         } );
    //     }
    //     None => {}
    //
    // }
    
    
    
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
