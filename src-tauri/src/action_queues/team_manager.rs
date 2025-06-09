use crate::action_queues::game_speed_manager::GameSpeedManagerCommand;
use crate::action_queues::shared::timed_dispatch;
use crate::integrations::queues::QueueManager;
use crate::integrations::queues::SimCommand::TeamManager;

use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};
use crate::sim::person::components::{Person, PersonId};
use crate::sim::registries::registry::Registry;
use crate::sim::resources::global::Dirty;
use crate::sim::team::components::{Team, TeamId};
use dashmap::DashSet;
use legion::systems::CommandBuffer;
use legion::world::{EntityAccessError, SubWorld};
use legion::{system, Entity, EntityStore, IntoQuery, Query, Write};
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, info, trace, warn};
use tracing_subscriber::fmt::time::uptime;
use crate::sim::team::utils::creat_new_team;

pub enum TeamManagerCommand {
    NewTeam {
        name: String,
        description: String,
    },
    EditTeam {
        id: u32,
        name: String,
        description: String,
    },
    RemoveTeam {
        id: u32,
    },
}

pub enum TeamAssignmentCommand {
    AddPersonToTeam { person_id: u32, team_id: u32 },
    UnassignTeam { person_id: u32 },
}

#[system]
pub fn handle_team_manager_queue(
    #[resource] queue_manager: &QueueManager,
    #[resource] team_registry: &Arc<Registry<TeamId, Entity>>,
    team_query: &mut Query<&Team>,
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    trace!("Handling team manager queue");
    let queue = &queue_manager.team_manager;
    let dispatch_time_limit = Duration::from_millis(5);

    timed_dispatch(queue, dispatch_time_limit, |cmd| match cmd {
        TeamManagerCommand::NewTeam { name, description } => {
            creat_new_team(team_registry, commands, name, description);
        }
        TeamManagerCommand::RemoveTeam { id } => {
            let teamId = TeamId(id);
            match team_registry.get_entity_from_id(&teamId) {
                None => {
                    warn!(
                        "Team with ID:{:?} not found for deleting. Skipping...",
                        teamId
                    )
                }
                Some(entity) => {
                    commands.remove(entity);
                }
            }
        }
        TeamManagerCommand::EditTeam {
            id,
            name,
            description,
        } => {
            let team_id = TeamId(id);
            match team_registry.get_entity_from_id(&team_id) {
                None => {
                    warn!(
                        "Team with ID:{:?} not found for editing. Skipping...",
                        team_id
                    )
                }
                Some(entity) => {
                    let updated_component = Team::new(id, name, description);
                    commands.add_component(entity, (updated_component, Dirty))
                }
            }
        }
    })
}



#[system]
pub fn handle_team_assignment_queue(
    #[resource] queue_manager: &QueueManager,
    #[resource] team_registry: &Arc<Registry<TeamId, Entity>>,
    #[resource] person_registry: &Arc<Registry<PersonId, Entity>>,
    team_query: &mut Query<&mut Team>,
    person_query: &mut Query<&mut Person>,
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    trace!("Handling team assignment queue");
    let queue = &queue_manager.team_assignment;
    let dispatch_time_limit = Duration::from_millis(5);

    timed_dispatch(queue, dispatch_time_limit, |cmd| match cmd {
        TeamAssignmentCommand::AddPersonToTeam { person_id, team_id } => {
            info!("Adding person {} to team {}", person_id, team_id);

            let (mut team_world, mut person_world) = world.split::<&mut Team>();

            let person_entity = match person_registry.get_entity_from_id(&PersonId(person_id)) {
                Some(entity) => entity,
                None => {
                    warn!(
                        "Can't find person entity with ID:{:?} when adding to team. Skipping...",
                        person_id
                    );
                    return;
                }
            };

            let mut person = match <&mut Person>::query().get_mut(&mut person_world, person_entity)
            {
                Ok(person) => person,
                Err(_) => {
                    warn!(
                        "Can't find person component with ID:{:?} when adding to team. Skipping...",
                        person_id
                    );
                    return; // Early exit if person component not found
                }
            };

            // If the person was on an old team, remove them from it first.
            if let Some(old_team) = person.team {
                trace!("Found existing team. Removing before adding to new team.");
                if let Some(team_entity) = team_registry.get_entity_from_id(&old_team) {
                    if let Ok(mut team_component) =
                        <&mut Team>::query().get_mut(&mut team_world, team_entity)
                    {
                        team_component.remove_person(person);
                        commands.add_component(team_entity, Dirty);
                    } else {
                        warn!("Can't find existing team component while trying to remove old team. Skipping...");
                    }
                } else {
                    warn!("Can't find existing team while trying to remove old team.. Skipping...");
                }
            }

            // Actual assignment
            if let Some(new_team) = team_registry.get_entity_from_id(&TeamId(team_id)) {
                if let Ok(mut team_component) =
                    <&mut Team>::query().get_mut(&mut team_world, new_team)
                {
                    info!("Im here yo");
                    team_component.add_person(person);
                    commands.add_component(person_entity, Dirty);
                    commands.add_component(new_team, Dirty);
                } else {
                    info!("No team component found while adding to new team. Skipping...");
                }
            } else {
                info!(
                    "No team entity found while adding to new team. Skipping... Registry {:?}",
                    team_registry
                );
            }
        }

        TeamAssignmentCommand::UnassignTeam { person_id } => {
            info!("Unassigning  {} from their assigned team", person_id);

            let (mut team_world, mut person_world) = world.split::<&mut Team>();

            let person_entity = match person_registry.get_entity_from_id(&PersonId(person_id)) {
                Some(entity) => entity,
                None => {
                    warn!(
                        "Can't find person entity with ID:{:?} when removing from team. Skipping...",
                        person_id
                    );
                    return;
                }
            };

            let mut person = match <&mut Person>::query().get_mut(&mut person_world, person_entity)
            {
                Ok(person) => person,
                Err(_) => {
                    warn!(
                        "Can't find person component with ID:{:?} when removing from team. Skipping...",
                        person_id
                    );
                    return; // Early exit if person component not found
                }
            };

            // If the person was on an old team, remove them from it first.
            if let Some(old_team) = person.team {
                trace!("Found existing team. Removing..."); // no need to check, we're just unassigning
                if let Some(team_entity) = team_registry.get_entity_from_id(&old_team) {
                    if let Ok(mut team_component) =
                        <&mut Team>::query().get_mut(&mut team_world, team_entity)
                    {
                        team_component.remove_person(person);
                        commands.add_component(team_entity, Dirty);
                        commands.add_component(person_entity, Dirty);
                    } else {
                        warn!("Can't find existing team component while trying to remove old team. Skipping...");
                    }
                } else {
                    warn!("Can't find existing team while trying to remove old team.. Skipping...");
                }
            }
        }
    })
}
