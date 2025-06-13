use crate::sim::ai::goap::{CurrentGoal, EmployeeGoapFacts};
use crate::sim::person::needs::{Energy, Hunger, Needs};
use std::default::Default;
use legion::{system, Entity, IntoQuery};
use legion::world::SubWorld;
use tracing::info;
use crate::sim::person::components::Person;

// --- Score Context (The "all parameters" struct for Considerations) ---
// This bundles all the data a Consideration might need to calculate a score.
pub struct ScoreContext<'a> {
    pub goap_facts: &'a EmployeeGoapFacts, // For GOAP-related facts (e.g., task progress)
    pub needs: Needs
}

// --- Consideration Trait ---
// Defines the interface for all considerations.
// The score method now takes a ScoreContext.
pub trait Consideration: Send + Sync {
    fn score(&self, context: &ScoreContext) -> f32;
}

// --- Concrete Consideration Implementations ---

// Scores higher when energy is low (desire to rest)
pub struct EnergyConsideration;
impl Consideration for EnergyConsideration {
    fn score(&self, context: &ScoreContext) -> f32 {
        let score:f32 = 1.0 - (context.needs.energy.value() as f32 / 100.0);
        score.powf(2.0) // Exponentially higher score when very low
    }
}

// Scores higher when hungry
pub struct HungerConsideration;
impl Consideration for HungerConsideration {
    fn score(&self, context: &ScoreContext) -> f32 {
        let score:f32 = 1.0 - (context.needs.hunger.value() as f32 / 100.0);
        score.powf(2.0) // Exponentially higher score when very low
        //if context.needs.is_hungry { 1.0 } else { 0.0 }
    }
}
pub struct DefaultConsideration;
impl Consideration for DefaultConsideration {
    fn score(&self, context: &ScoreContext) -> f32 { 0.54f32 }
}

//
// // Scores higher when a task is assigned but not complete, modified by work ethic
// pub struct TaskProgressConsideration;
// impl Consideration for TaskProgressConsideration {
//     fn score(&self, context: &ScoreContext) -> f32 {
//         if !context.goap_facts.has_assigned_task || context.goap_facts.task_progress_u8 >= 100 {
//             return 0.0;
//         }
//         let base_score = 1.0 - (context.goap_facts.task_progress_u8 as f32 / 100.0);
//         base_score * context.personality.work_ethic
//     }
// }
//
// // Scores higher when stress is high (desire to reduce stress)
// pub struct StressConsideration;
// impl Consideration for StressConsideration {
//     fn score(&self, context: &ScoreContext) -> f32 {
//         (context.needs.stress_level_u8 as f32 / 100.0).powf(2.0) // Exponentially higher when very stressed
//     }
// }
//
// // Scores higher when morale is low (desire to boost morale)
// pub struct MoraleConsideration;
// impl Consideration for MoraleConsideration {
//     fn score(&self, context: &ScoreContext) -> f32 {
//         let base_score = 1.0 - (context.needs.morale_level_u8 as f32 / 100.0);
//         base_score * (1.0 - context.personality.social_preference) // Maybe introverts care less about external morale boosts
//     }
// }
//
// // Scores higher when social need is high (desire to socialize)
// pub struct SocialNeedConsideration;
// impl Consideration for SocialNeedConsideration {
//     fn score(&self, context: &ScoreContext) -> f32 {
//         (context.needs.social_need_level_u8 as f32 / 100.0).powf(2.0) * context.personality.social_preference
//     }
// }


// --- Goal Definition (Global Data / Resource) ---
// Defines a potential goal, its target GOAP state, and how its utility is scored.
pub struct GoalDefinition {
    pub name: &'static str,
    pub goap_target_facts: EmployeeGoapFacts, // The desired GOAP state
    pub considerations: Vec<(Box<dyn Consideration>, f32)>, // Considerations and their weights
}

impl GoalDefinition {
    // Calculates the overall utility score for this goal given an employee's context.
    pub fn calculate_utility(&self, context: &ScoreContext) -> f32 {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        for (consideration, weight) in &self.considerations {
            let score = consideration.score(context);
            total_score += score * weight;
            total_weight += weight;
        }

        if total_weight > 0.0 {
            total_score / total_weight
        } else {
            0.0 // Avoid division by zero if no considerations
        }
    }
}

// --- Global List of All Goal Definitions (Legion Resource) ---
// This vector will be stored as a Legion resource and accessed by the goal selection system.
pub fn get_all_goal_definitions() -> Vec<GoalDefinition> {
    vec![
        GoalDefinition {
            name: "Rest",
            goap_target_facts: EmployeeGoapFacts::default(), // No specific fact change needed for rest (could be `at_home = true`)
            considerations: vec![
                (Box::new(EnergyConsideration), 1.0), // High weight for low energy
                // (Box::new(StressConsideration), 0.3), // Some weight for stress reduction
            ],
        },
        GoalDefinition {
            name: "Eat",
            goap_target_facts: EmployeeGoapFacts { has_food: false, ..Default::default() }, // Goal is to consume food
            considerations: vec![
                (Box::new(HungerConsideration), 1.0), // High weight for hunger
                // (Box::new(EnergyConsideration), 0.2), // Some weight for energy gain
            ],
        },
        GoalDefinition {
            name: "Do nothing",
            goap_target_facts: EmployeeGoapFacts::default(), // No specific fact change needed for rest (could be `at_home = true`)
            considerations: vec![
                (Box::new(DefaultConsideration), 1.0), // High weight for low energy
                // (Box::new(StressConsideration), 0.3), // Some weight for stress reduction
            ],
        },

        // GoalDefinition {
        //     name: "CompleteAssignedTask",
        //     goap_target_facts: EmployeeGoapFacts { has_assigned_task: true, task_progress_u8: 100, ..Default::default() },
        //     considerations: vec![
        //         // (Box::new(TaskProgressConsideration), 1.0), // Primary driver
        //         // (Box::new(StressConsideration), -0.5), // Completing task might reduce stress (negative score for high stress goal)
        //         (Box::new(EnergyConsideration), -0.2), // But it costs energy (negative score for low energy goal)
        //     ],
        // },
        // GoalDefinition {
        //     name: "Socialize",
        //     goap_target_facts: EmployeeGoapFacts::default(), // No specific fact change from social action
        //     considerations: vec![
        //         // (Box::new(SocialNeedConsideration), 1.0), // Primary driver
        //         // (Box::new(MoraleConsideration), 0.5),    // Boosts morale
        //         // (Box::new(StressConsideration), -0.2),   // Socializing might reduce stress
        //     ],
        // },
    ]
}

// --- System 2: Goal Selection System ---
// This system selects the most desirable goal for each employee based on their needs and personality.
#[system]
#[read_component(EmployeeGoapFacts)]
#[read_component(Energy)]
#[read_component(Hunger)]
#[read_component(Person)]
#[write_component(CurrentGoal)]
pub fn goal_selection(
    world: &mut SubWorld,

) {
    info!("Goal selection");
    let all_goal_definitions = get_all_goal_definitions();
    let mut query = <(Entity, &Person, Option<&EmployeeGoapFacts>, &Energy, &Hunger, Option<&mut CurrentGoal>)>::query();


    for (entity, person, goap_facts, energy, hunger, current_goal_option) in query.iter_mut(world) {

        let needs = Needs{
            energy: energy.clone(),
            hunger: hunger.clone(),
        };
        let facts = match goap_facts {
            None => {EmployeeGoapFacts::default()}
            Some(f) => {f.clone()}
        };
        // Construct the ScoreContext for the current employee
        let context = ScoreContext {
            goap_facts:&facts,
            needs,
        };

        let mut best_goal_name: Option<&'static str> = None;
        let mut highest_utility: f32 = -1.0; // Utility scores are 0-1, so -1 ensures any valid score is chosen

        for goal_def in all_goal_definitions.iter() {
            let utility = goal_def.calculate_utility(&context);

            // Simple highest utility wins (could add stickiness later)
            if utility > highest_utility {
                highest_utility = utility;
                best_goal_name = Some(goal_def.name);
                //current_goal.0 = goal_def.goap_target_facts.clone(); // Update the GOAP target facts
            }
        }
        info!("Employee {:?} selected Goal: {:?} (Utility: {:.2})", person, best_goal_name, highest_utility);
        // println!("  Target GOAP Facts: {:?}", current_goal.0.get_active_facts());
    }
}
