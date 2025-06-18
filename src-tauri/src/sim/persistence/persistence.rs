use std::sync::Arc;

use crate::db::error;
use crate::db::init::SaveSlot;
use crate::integrations::snapshots::{company, person, team};
// Added for SavedEmployee
use crate::sim::ai::goap::CurrentGoal;
use crate::sim::company::company::{Company, PlayerControlled};
use crate::sim::person::components::{Person, PersonId, ProfilePicture};
use crate::sim::person::needs::{Energy, Hunger};
use crate::sim::person::personality_matrix::PersonalityMatrix;
use crate::sim::person::skills::SkillSet;
use crate::sim::person::stats::Stats;
use crate::sim::registries::registry::Registry;
use crate::sim::team::components::{Team, TeamId};
use bincode::{Decode, Encode};
use legion::world::SubWorld;
use legion::{query, system, Entity, IntoQuery, Query};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
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
}

#[system]
pub fn save_entity_state(
    world: &SubWorld,
    #[resource] current_save: &Arc<SaveSlot>,
    query: &mut Query<(
        &Person,
        &Stats,
        &ProfilePicture,
        &PersonalityMatrix,
        &SkillSet,
        &Energy,
        &Hunger,
        &CurrentGoal,
    )>,
    company_query: &mut Query<(&Company, &PlayerControlled)>,
    team_query: &mut Query<(&Team)>,
) {
    if current_save.is_empty {
        error!("No active save slot");
        return;
    }

    

    if let Some(db) = &current_save.handle {
        let mut collected_employees: Vec<SavedEmployee> = Vec::new(); // Example: collect them locally

        for (
            person,
            stats,
            profile_picture,
            personality_matrix,
            skill_set,
            energy,
            hunger,
            current_goal,
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
            };

            match bincode::encode_to_vec(saved_employee, bincode::config::standard()) {
                Ok(encoded_employee) => {
                    if let Err(e) = db.insert(
                        format!("employee{}", person.person_id.0),
                        encoded_employee,
                    ) {
                        error!("Failed to save employee {}: {}", person.person_id.0, e);
                        // Maybe collect failed saves and retry later?
                    }
                }
                Err(e) => {
                    error!("Failed to encode employee {}: {}", person.person_id.0, e);
                    // This is more serious - maybe indicates data corruption
                }
            }
        }
        for(company, _) in company_query.iter(world){
            match bincode::encode_to_vec(company, bincode::config::standard()) {
                Ok(enconded_company) => {
                    if let Err(e) = db.insert(
                        "company".to_string(),
                        enconded_company,
                    ) {
                        error!("Failed to save company: {}", e);
                        // Maybe collect failed saves and retry later?
                    }
                },
                Err(e) => {
                    error!("Failed to encode company {}: {}", company.name, e);
                    // This is more serious - maybe indicates data corruption
                }
            }
        }
         let teams:Vec<Team> = team_query.iter(world).map(|t| t.clone()).collect();
        match( bincode::encode_to_vec(teams, bincode::config::standard())){
            Ok(encoded_teams) => {if let Err(e) = db.insert(
                            "teams".to_string(),
                            encoded_teams,
                        ) {
                            error!("Failed to save company: {}", e);
                            // Maybe collect failed saves and retry later?
                        }},
            Err(_) => todo!(),
        }
    } else {
        error!("No active save slot");
        return;
    }

   

    
}


#[system]
pub fn sync_registry_from_person(
    world: &SubWorld,
    query: &mut Query<(
        &Person,
        &Entity,
    )>,
    #[resource]person_registry: &Arc<Registry<PersonId, Entity>>,
    
) {

    let x = query.iter(world).map(|(person, entity)| (person.person_id, *entity) );
    person_registry.repopulate_from_entities(x);

}


#[system]
pub fn sync_registry_from_team(
    world: &SubWorld,
    query: &mut Query<(
        &Team,
        &Entity,
    )>,
    #[resource]team_registry: &Arc<Registry<TeamId, Entity>>,
    
) {

    let x = query.iter(world).map(|(team, entity)| (team.team_id, *entity) );
    team_registry.repopulate_from_entities(x);

}