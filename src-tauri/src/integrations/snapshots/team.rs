use serde::{Deserialize, Serialize};
use crate::sim::team::components::Team;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TeamSnapshot {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub members: Vec<u32>,
}

impl From<&Team> for TeamSnapshot {
    fn from(value: &Team) -> Self {
        Self{
            id: value.team_id.0,
            name: value.name.clone(),
            description: value.description.clone(),
            members: value.get_members_vec()
        }
    }
}

impl From<Team> for TeamSnapshot {
    fn from(value: Team) -> Self {
        let members = value.get_members_vec();
        Self{
            id: value.team_id.0,
            name: value.name,
            description: value.description,
            members
        }
    }
}

impl PartialEq<&Team> for TeamSnapshot {
    fn eq(&self, other: &&Team) -> bool {
        self.id == other.team_id.0
            && self.name == other.name
            && self.description == other.description
            && self.members == other.get_members_vec()
    }
}