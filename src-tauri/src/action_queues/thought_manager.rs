use crate::action_queues::shared::timed_dispatch;
use crate::integrations::queues::QueueManager;
use crate::sim::person::thoughts::{ArchivedThoughts, Thought, Thoughts};
use crate::sim::person::components::{PersonId};
use crate::sim::registries::registry::Registry;
use crate::integrations::snapshots::thoughts::DirtyThought;
use legion::{system, world::SubWorld, systems::CommandBuffer, Entity, IntoQuery, Query};
use std::sync::Arc;
use std::time::Duration;
use tracing::{trace, info, warn};

pub enum ThoughtCommand {
    AddThought { person_id: u32, thought: Thought },
}

#[system]
pub fn handle_thought_command_queue(
    #[resource] queue_manager: &QueueManager,
    #[resource] person_registry: &Arc<Registry<PersonId, Entity>>,
    thought_query: &mut Query<(&mut Thoughts, &mut ArchivedThoughts)>,
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    trace!("Handling thought command queue");
    let queue = &queue_manager.thought_manager;
    let dispatch_time_limit = Duration::from_millis(5);

    timed_dispatch(queue, dispatch_time_limit, |cmd| match cmd {
        ThoughtCommand::AddThought { person_id, thought } => {
            info!("Adding thought to person {}", person_id);
            if let Some(entity) = person_registry.get_entity_from_id(&PersonId(person_id)) {
                if let Ok((mut thoughts, mut archived)) = thought_query.get_mut(world, entity) {
                    thoughts.add(thought, &mut archived);
                    commands.add_component(entity, DirtyThought);
                } else {
                    warn!("Cannot access thought components for person {}", person_id);
                }
            } else {
                warn!("Person id {} not found while adding thought", person_id);
            }
        }
    });
}
