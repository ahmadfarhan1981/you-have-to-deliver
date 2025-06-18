use std::sync::Arc;
use legion::{Entity, IntoQuery};
use legion::systems::CommandBuffer;
use tracing::{info, trace, warn};
use legion::world::SubWorld;
use crate::sim::person::components::{Person, PersonId};
use crate::sim::person::utils::PersonLookupResult;
use crate::sim::registries::registry::Registry;
use crate::sim::resources::global::Dirty;
use crate::sim::team::components::{Team, TeamId};

pub fn creat_new_team(team_registry: &Arc<Registry<TeamId, Entity>>, commands: &mut CommandBuffer, name: String, description: String) {
    let id = team_registry.generate_id();
    let team = Team::new(id, name.clone(), description.clone());
    info!("Adding new team {}", name);
    let new_team = commands.push((team, Dirty));
    team_registry.insert(TeamId(id), new_team);
}

pub fn remove_person_from_current_team(
    person: &mut Person,
    person_entity: Entity,
    team_registry: &Arc<Registry<TeamId, Entity>>,
    team_world: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if let Some(old_team) = person.team {
        trace!("Found existing team. Removing..."); // no need to check, we're just unassigning
        if let Some(team_entity) = team_registry.get_entity_from_id(&old_team) {
            if let Ok(mut team_component) = <&mut Team>::query().get_mut(team_world, team_entity) {
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

#[derive(Debug)]
pub struct TeamLookupResult<'a> {
    pub entity: Entity,
    pub team: &'a mut Team,
}
pub fn get_team_from_id<'a>(
    team_id: u32,
    team_registry: &Arc<Registry<TeamId, Entity>>,
    team_world: &'a mut SubWorld,
)-> Option<TeamLookupResult<'a>> {
    if let Some(entity) = team_registry.get_entity_from_id(&TeamId(team_id)) {
        if let Ok(mut team) =  <&mut Team>::query().get_mut(team_world, entity)
        {
            Some(TeamLookupResult { entity, team })
        } else {
            warn!("No team component found while looking for {}. Skipping...", team_id);
            return None
        }
    } else {
        warn!("No team entity found in registry while looking for teamId:{}. Skipping...", team_id);
        return None
    }
}
