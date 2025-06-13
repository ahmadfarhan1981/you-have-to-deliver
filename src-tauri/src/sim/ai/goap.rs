// src/ai/goap.rs
use std::collections::HashMap;
use legion::Entity;

// --- Employee GOAP Facts (Legion Component) ---
// This represents the employee's current beliefs about themselves and the world
// that are RELEVANT FOR GOAP ACTION PRECONDITIONS AND EFFECTS.
#[derive(Clone, Debug, PartialEq, Eq, Hash)] // Still needs Hash/Eq for A*
pub struct EmployeeGoapFacts {
    pub at_desk: bool,
    pub has_assigned_task: bool,
    pub task_progress_u8: u8, // 0-100 for percentage
    pub has_food: bool,
    // Add other facts directly changed by actions relevant for GOAP:
    // pub has_fixed_bug: bool,
    // pub knows_new_skill: bool,
}

impl Default for EmployeeGoapFacts {
    fn default() -> Self {
        EmployeeGoapFacts {
            at_desk: false,
            has_assigned_task: false,
            task_progress_u8: 0,
            has_food: false,
        }
    }
}

impl EmployeeGoapFacts {
    // Helper to check if this state satisfies a given goal state (partial match)
    pub fn satisfies_goal(&self, goal: &EmployeeGoapFacts) -> bool {
        let mut satisfied = true;
        // Compare only fields that are explicitly set/relevant in the 'goal' (non-default values)
        // Note: For numerical fields like task_progress_u8, "satisfies" might mean "at least"
        if goal.at_desk != EmployeeGoapFacts::default().at_desk && self.at_desk != goal.at_desk {
            satisfied = false;
        }
        if goal.has_assigned_task != EmployeeGoapFacts::default().has_assigned_task && self.has_assigned_task != goal.has_assigned_task {
            satisfied = false;
        }
        if goal.task_progress_u8 != EmployeeGoapFacts::default().task_progress_u8 && self.task_progress_u8 < goal.task_progress_u8 {
            satisfied = false;
        }
        if goal.has_food != EmployeeGoapFacts::default().has_food && self.has_food != goal.has_food {
            satisfied = false;
        }
        satisfied
    }

    // Helper to get all non-default facts as a HashMap (useful for debugging/display)
    pub fn get_active_facts(&self) -> HashMap<&'static str, String> {
        let default_state = EmployeeGoapFacts::default();
        let mut active_facts = HashMap::new();

        if self.at_desk != default_state.at_desk {
            active_facts.insert("at_desk", self.at_desk.to_string());
        }
        if self.has_assigned_task != default_state.has_assigned_task {
            active_facts.insert("has_assigned_task", self.has_assigned_task.to_string());
        }
        if self.task_progress_u8 != default_state.task_progress_u8 {
            active_facts.insert("task_progress_u8", self.task_progress_u8.to_string());
        }
        if self.has_food != default_state.has_food {
            active_facts.insert("has_food", self.has_food.to_string());
        }
        active_facts
    }
}

// --- Current Goal (Legion Component) ---
// The target EmployeeGoapFacts that the employee is currently trying to achieve.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CurrentGoal(pub EmployeeGoapFacts);

// --- Current Plan (Legion Component) ---
// A sequence of game actions to achieve the current goal.
#[derive(Clone, Debug, Default)]
pub struct CurrentPlan(pub Vec<EmployeeGameAction>);

// --- Employee Game Actions (Concrete Actions the employee can perform) ---
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EmployeeGameAction {
    GoToDesk,
    GoToCafeteria,
    BuyFood,
    EatFood,
    TakeBreak,
    CodeTask(Entity), // Task entity ID
    Socialize,
    // Add more specific game actions here
}

// --- Planner Action (Blueprint for GOAP planner) ---
// Defines preconditions, effects, and the associated game action.
pub struct PlannerAction {
    pub name: &'static str,
    pub cost: u32,
    pub check_planner_preconditions: fn(&EmployeeGoapFacts) -> bool,
    pub apply_planner_effects: fn(&mut EmployeeGoapFacts),
    pub game_action_type: EmployeeGameAction,
}

// --- Placeholder for all available planner actions ---
// In a real game, this might be loaded from data or dynamically generated.
pub fn get_all_planner_actions() -> Vec<PlannerAction> {
    vec![
        PlannerAction {
            name: "GoToDesk",
            cost: 2,
            check_planner_preconditions: |s| !s.at_desk,
            apply_planner_effects: |s| s.at_desk = true,
            game_action_type: EmployeeGameAction::GoToDesk,
        },
        PlannerAction {
            name: "GoToCafeteria",
            cost: 2,
            check_planner_preconditions: |s| s.at_desk, // Assume from desk to cafeteria
            apply_planner_effects: |s| s.at_desk = false,
            game_action_type: EmployeeGameAction::GoToCafeteria,
        },
        PlannerAction {
            name: "BuyFood",
            cost: 3,
            check_planner_preconditions: |s| !s.has_food && !s.at_desk, // Must be outside desk, assume in cafeteria
            apply_planner_effects: |s| s.has_food = true,
            game_action_type: EmployeeGameAction::BuyFood,
        },
        PlannerAction {
            name: "EatFood",
            cost: 1,
            check_planner_preconditions: |s| s.has_food,
            apply_planner_effects: |s| s.has_food = false,
            game_action_type: EmployeeGameAction::EatFood,
        },
        PlannerAction {
            name: "TakeBreak",
            cost: 2,
            check_planner_preconditions: |s| s.at_desk, // Assume breaks happen at desk
            apply_planner_effects: |_| {}, // No GOAP fact changes for a simple break
            game_action_type: EmployeeGameAction::TakeBreak,
        },
        // PlannerAction {
        //     name: "CodeTask",
        //     cost: 5, // Cost varies by task, simplified for now
        //     check_planner_preconditions: |s| s.at_desk && s.has_assigned_task && s.task_progress_u8 < 100,
        //     apply_planner_effects: |s| s.task_progress_u8 = 100, // Completes task
        //     game_action_type: EmployeeGameAction::CodeTask(Entity::new(0,0)), // Placeholder task entity
        // },
        PlannerAction {
            name: "Socialize",
            cost: 3,
            check_planner_preconditions: |s| !s.at_desk, // Assume social interaction outside desk
            apply_planner_effects: |_| {}, // No GOAP fact changes
            game_action_type: EmployeeGameAction::Socialize,
        },
    ]
}