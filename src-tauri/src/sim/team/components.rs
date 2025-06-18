use std::task::Context;
use bincode::{Decode, Encode};
use dashmap::DashSet;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tracing::{debug, info};
use crate::sim::person::components::{Person, PersonId};
use crate::sim::utils::snapshots::convert_dashset_to_vec;

#[derive(Debug, Serialize, Deserialize, Clone, Default, Eq, PartialEq, Hash, Copy, Encode, Decode, PartialOrd, Ord)]
pub struct TeamId(pub u32);
impl Into<u32> for TeamId {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct Team {
    pub team_id: TeamId,
    pub name: String,
    pub description: String,
    members: DashSet<PersonId>,
}

impl Encode for Team {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        // Create temporary struct for encoding
        #[derive(Encode)]
        struct TeamBincode {
            team_id: TeamId,
            name: String,
            description: String,
            members: Vec<PersonId>,
        }

        let temp = TeamBincode {
            team_id: self.team_id,
            name: self.name.clone(),
            description: self.description.clone(),
            members: self.members.iter().map(|x| *x).collect(),
        };

        temp.encode(encoder)
    }
}

impl <Context> Decode<Context> for Team {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> core::result::Result<Self, bincode::error::DecodeError> {
        #[derive(Decode)]
        struct TeamBincode {
            team_id: TeamId,
            name: String,
            description: String,
            members: Vec<PersonId>,
        }

        let temp = TeamBincode::decode(decoder)?;

        Ok(Team {
            team_id: temp.team_id,
            name: temp.name,
            description: temp.description,
            members: temp.members.into_iter().collect(),
        })
    }
}

impl Team {
    /// Adds the Person to the team. Returns true if the Person was not already in the team.
    /// **NOTE** This does not remove the `Person` from their original `Team` if they are assigned somewhere else.
    /// Take care to manage state to ensure consistentcy.
    ///
    pub fn add_person(&mut self, person: &mut Person) -> bool {
        person.team = Some(self.team_id.clone());
        info!("Added person {} to team {:?}", person.name, person.team.unwrap());
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
        match self.members.remove(&person.person_id) {
            None => {
                debug!("Person {:?} wasn't assigned to team {:?}. Unexpected but continuing...", person.person_id, self.team_id);
                person.team = None
            }
            Some(_) => { person.team = None }
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
    
    pub fn get_members_vec(&self) -> Vec<u32> {
        let mut vec = self.members.iter().map(|id| id.0).collect::<Vec<u32>>();
        vec.sort_unstable();
        vec
    }
}

impl Serialize for Team {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Create temporary struct for serialization
        #[derive(Serialize)]
        struct TeamSerde {
            team_id: TeamId,
            name: String,
            description: String,
            members: Vec<PersonId>,
        }

        let temp = TeamSerde {
            team_id: self.team_id,
            name: self.name.clone(),
            description: self.description.clone(),
            members: self.members.iter().map(|x| *x).collect(),
        };

        temp.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Team {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TeamSerde {
            team_id: TeamId,
            name: String,
            description: String,
            members: Vec<PersonId>,
        }

        let temp = TeamSerde::deserialize(deserializer)?;

        Ok(Team {
            team_id: temp.team_id,
            name: temp.name,
            description: temp.description,
            members: temp.members.into_iter().collect(),
        })
    }
}