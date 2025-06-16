use bincode::{Decode, Encode};
use legion::{system, Query};
use legion::world::SubWorld;
use serde::{Deserialize, Serialize}; // Added for SavedEmployee
use crate::sim::ai::goap::CurrentGoal;
use crate::sim::person::components::{Person, ProfilePicture};
use crate::sim::person::needs::{Energy, Hunger};
use crate::sim::person::personality_matrix::PersonalityMatrix;
use crate::sim::person::skills::SkillSet;
use crate::sim::person::stats::Stats;
use tracing::info; // Added for logging

/// Represents the data of an employee that can be saved or transferred.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct SavedEmployee {
    pub person: Person,
    pub stats: Stats,
    pub profile_picture: ProfilePicture,
    pub personality_matrix: PersonalityMatrix,
    pub skill_set: SkillSet,
    pub energy: Energy,
    pub hunger: Hunger,
    pub current_goal: CurrentGoal,
}

#[system]
pub fn process_person_data_explicit_query(
    world: &SubWorld,
    // Add #[resource] ResType if you need resources
    query: &mut Query<(
        &Person,
        &Stats,
        &ProfilePicture,
        &PersonalityMatrix,
        &SkillSet,
        &Energy,
        &Hunger,
        &CurrentGoal,
    )>,
    // You might want to pass a mutable resource here to store the collected employees
    // e.g., #[resource] mut saved_employee_data: &mut Vec<SavedEmployee>
) {
    let mut collected_employees: Vec<SavedEmployee> = Vec::new(); // Example: collect them locally

    for (person, stats, profile_picture, personality_matrix, skill_set, energy, hunger, current_goal) in query.iter(world) {
        let saved_employee = SavedEmployee {
            person: person.clone(),
            stats: stats.clone(),
            profile_picture: profile_picture.clone(),
            personality_matrix: personality_matrix.clone(),
            skill_set: skill_set.clone(),
            energy: energy.clone(),
            hunger: hunger.clone(),
            current_goal: current_goal.clone(),
        };
        info!("Processed and created SavedEmployee for person: {:?}", saved_employee.person.name);
        collected_employees.push(saved_employee);
    }

    // Here you would typically do something with `collected_employees`,
    // like saving them to a file, sending them over a network, or storing them in a resource.
    // For example, if you passed a mutable resource:
    // *saved_employee_data = collected_employees;
}