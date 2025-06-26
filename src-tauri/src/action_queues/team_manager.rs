use crate::action_queues::game_speed_manager::GameSpeedManagerCommand;
use crate::action_queues::shared::timed_dispatch;
use crate::integrations::queues::QueueManager;
use crate::integrations::queues::SimCommand::TeamManager;

use crate::integrations::snapshots::{person, team};
use crate::integrations::ui::new_team;
use crate::sim;
use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};
use crate::sim::person::components::{Person, PersonId};
use crate::sim::person::utils;
use crate::sim::person::utils::PersonLookupResult;
use crate::sim::registries::registry::Registry;
use crate::sim::resources::global::Dirty;
use crate::sim::team::components::{Team, TeamId};
use crate::sim::team::utils::{creat_new_team, get_team_from_id};
use dashmap::DashSet;
use legion::systems::CommandBuffer;
use legion::world::{EntityAccessError, SubWorld};
use legion::{system, Entity, EntityStore, IntoQuery, Query, Write};
use parking_lot::RwLock;
use std::ops::Sub;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::fmt::time::uptime;

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
            let team_id = TeamId(id);
            match team_registry.get_entity_from_id(&team_id) {
                None => {
                    warn!(
                        "Team with ID:{:?} not found for deleting. Skipping...",
                        team_id
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

            let Some(mut person_lookup) = utils::get_person_from_id(person_id, &person_registry, &mut person_world)            else {
                error!("Cannot find person component");
                return;
            };
            
            let mut person=person_lookup.person;
            let person_entity = person_lookup.entity;
            // If the person was on an old team, remove them from it first.
            sim::team::utils::remove_person_from_current_team(
                person,
                person_entity,
                team_registry,
                &mut team_world,
                commands,
            );
            
            let Some(team_lookup) = get_team_from_id(team_id, team_registry, &mut team_world)  else { 
                error!("Cannot find team while adding person to team person:{}, team:{}", person_id, team_id);
                return;
            };
            person.team = Some(TeamId(team_id));
            team_lookup.team.add_person(person);
            commands.add_component(person_entity, Dirty);
            commands.add_component(team_lookup.entity, Dirty);
            
        }

        TeamAssignmentCommand::UnassignTeam { person_id } => {
            info!("Unassigning  {} from their assigned team", person_id);

            let (mut team_world, mut person_world) = world.split::<&mut Team>();

            let Some(mut person_lookup) =
                utils::get_person_from_id(person_id, &person_registry, &mut person_world)
            else {
                error!("Cannot find person component");
                return;
            };

            // If the person was on an old team, remove them from it first.
            sim::team::utils::remove_person_from_current_team(
                person_lookup.person,
                person_lookup.entity,
                team_registry,
                &mut team_world,
                commands,
            );
        }
    })
}

