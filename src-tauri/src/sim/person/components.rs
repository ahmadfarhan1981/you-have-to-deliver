
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub person_id: PersonId,
    pub name: String,
    pub gender: Gender,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct PersonId(pub u32);
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}
