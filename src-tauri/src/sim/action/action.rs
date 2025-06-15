use std::sync::Arc;
use legion::{system, Entity};
use legion::systems::CommandBuffer;
use tracing::info;
use crate::sim::ai::consideration::GoalName;
use crate::sim::ai::goap::CurrentGoal;
use crate::sim::person::components::Person;
use crate::sim::person::needs::{Energy, Hunger};
use crate::sim::project::project::ProjectId;
use crate::sim::resources::global::TickCounter;
use crate::sim::sim_date::sim_date::SimDate;

#[derive(Debug, Default)]
pub enum ActionType {
    GeneralWork,
    Work(ProjectId),
    #[default]
    Rest,
    Eat,
    Idle,
}

#[derive(Debug)]
pub struct ActionIntent {
    pub current: ActionType,
    pub started_at: Option<SimDate>,
}
impl From<ActionType> for ActionIntent {
    fn from(value: ActionType) -> Self {
        Self{
            current: value,
            started_at: None,
        }
    }
}

#[system(for_each)]
pub fn decide_action(
    #[resource] tick_counter: &Arc<TickCounter>,
    entity: &Entity,
    person: &Person,
    current_goal: &CurrentGoal,
    action: Option<&ActionIntent>,
    hunger: &Hunger,
    energy: &Energy,
    cmd: &mut CommandBuffer,
){

    match current_goal.0 {
        GoalName::Rest => {cmd.add_component(*entity,ActionIntent::from(ActionType::Rest));}
        GoalName::Eat => {cmd.add_component(*entity,ActionIntent::from(ActionType::Eat));}
        GoalName::DoNothing => { cmd.remove_component::<ActionIntent>(*entity);}
    }


}

#[system(for_each)]
pub fn execute_action(
    #[resource] tick_counter: &Arc<TickCounter>,
    entity: &Entity,
    person: &Person,
    action: &ActionIntent,
    hunger: &mut Hunger,
    energy: &mut Energy,
    cmd: &mut CommandBuffer,
) {
    info!("Executing {:?}", action);
    match action.current {
        ActionType::GeneralWork => {}
        ActionType::Work(_) => {}
        ActionType::Rest => {
            energy.level.increase(45);
        }
        ActionType::Eat => {
            hunger.level.increase(10);
        }
        ActionType::Idle => {
        }
    }
}