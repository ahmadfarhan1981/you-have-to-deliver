use crate::action_queues::thought_manager::ThoughtCommand;
use crate::integrations::queues::QueueManager;
use crate::sim::person::components::Person;
use crate::sim::person::thoughts::{Thought, ThoughtContext};
use crate::sim::resources::global::TickCounter;
use legion::system;
use legion::Query;
use legion::world::SubWorld;
use std::sync::Arc;

/// Sends a reminder thought to all persons when the clock hits 9am.
#[system]
pub fn morning_thought_trigger(
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] queue_manager: &QueueManager,
    query: &mut Query<&Person>,
    world: &mut SubWorld,
) {
    // 9am corresponds to quarter_tick 37 (1-based indexing)
    let current_date = tick_counter.current_date();
    if current_date.quarter_tick == 37 {
        let thought_text = "Owh.. its 9am , time for work!".to_string();
        for person in query.iter(world) {
            queue_manager.thought_manager.queue.push(ThoughtCommand::AddThought {
                person_id: person.person_id.0,
                thought: Thought {
                    sim_date: current_date,
                    context: ThoughtContext::Event(thought_text.clone()),
                },
            });
        }
    }
}

