use dashmap::DashSet;
use serde::{Deserialize, Serialize};
use tracing::debug;
use crate::sim::person::components::{Person, PersonId};

#[derive(Debug, Serialize, Deserialize, Clone, Default, Eq, PartialEq, Hash, Copy)]
pub struct TeamId(pub u32);

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Team{
    pub team_id: TeamId,
    pub name: String,
    pub description: String,
    #[serde(skip)]//TODO manager recreating it after loading save
    members: DashSet<PersonId>
}
impl Team {
    /// Adds the Person to the team. Returns true if the Person was not already in the team.
    pub fn add_person(&mut self, person: &mut Person) -> bool{
        person.team = Some(self.team_id.clone());
        self.members.insert(person.person_id)
    }

    /// Creates a new `Team` instance.
    ///
    /// # Arguments
    ///
    /// * `team_id` - The unique id in u32
    /// * `name` - The name of the team.
    ///
    /// # Returns
    ///
    /// A new `Team` instance with the given `team_id` and `name`,
    /// and an empty set of members.
    pub fn new(id: u32, name: String, desc: String) -> Self {
        let team_id = TeamId(id);
        Team {
            team_id,
            name,
            description: desc,
            members: DashSet::new(), // Initialize members as an empty DashSet
        }
    }

    /// Removes the `Person` from this `Team`
    /// Clears the team assignment on the `Person` only if it was actually assigned to this `Team`
    pub fn remove_person(&mut self, person: &mut Person) {
        match self.members.remove(&person.person_id){
            None => {debug!("Person {:?} wasn't assigned to team {:?}. Skipping...", person.person_id, self.team_id);}
            Some(_) => {person.team = None}
        }
    }

    /// Checks if a provided PersonId is is the team
    pub fn is_member(&self, person: &PersonId) -> bool {
        self.members.contains(person)
    }

    /// Checks if a provided Person is is the team
    pub fn is_member_by_person(&self, person: &Person) -> bool {
        self.members.contains(&person.person_id)
    }
}
