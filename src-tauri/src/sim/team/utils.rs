use std::sync::Arc;
use legion::Entity;
use legion::systems::CommandBuffer;
use tracing::info;
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