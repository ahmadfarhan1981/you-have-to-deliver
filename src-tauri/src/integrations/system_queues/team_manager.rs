use std::sync::Arc;
use std::time::Duration;
use legion::{system, Entity, Query};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use parking_lot::RwLock;
use tracing::{debug, info, trace};
use crate::integrations::queues::QueueManager;
use crate::integrations::system_queues::game_speed_manager::GameSpeedManagerCommand;
use crate::integrations::system_queues::shared::timed_dispatch;
use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};
use crate::sim::person::components::PersonId;
use crate::sim::registries::registry::Registry;
use crate::sim::resources::global::Dirty;
use crate::sim::team::components::{Team, TeamId};


pub enum TeamManagerCommand{
    NewTeam{name:String, description:String},
    RemoveTeam{id:u32},
    AddPersonToTeam(PersonId, TeamId),
    RemovePersonFromTeam(PersonId, TeamId),
}

#[system]
pub fn handle_team_manager_queue(
    #[resource] queue_manager: &QueueManager,
    #[resource] team_registry: &Arc<Registry<TeamId,Entity>>,
    team_query: &mut Query<&Team>,
    world: &mut SubWorld,
    commands: &mut CommandBuffer,

) {
    trace!("Handling team manager queue");
    let queue = &queue_manager.team_manager;
    let dispatch_time_limit = Duration::from_millis(5);

    timed_dispatch(queue, dispatch_time_limit, |cmd| match cmd {
        TeamManagerCommand::AddPersonToTeam(_, _) => {}
        TeamManagerCommand::RemovePersonFromTeam(_, _) => {}
        TeamManagerCommand::NewTeam { name, description } => {
            let id = team_registry.generate_id();
            let team = Team::new(id, name.clone(), description.clone());
            info!("Adding new team {}", name);
            commands.push((team,Dirty));
        }
        TeamManagerCommand::RemoveTeam { id } => {
            let teamId = TeamId(id);
            match team_registry.get_entity_from_id(&teamId){
                None => {debug!("Team with ID:{:?} not found", teamId)}
                Some(entity) => {
                    commands.remove(entity);
                }
            }
        }
    })
}