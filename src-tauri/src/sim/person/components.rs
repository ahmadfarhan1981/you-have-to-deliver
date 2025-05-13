
use serde::{Deserialize, Serialize};
use super::utils::{Gender, PersonId};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub person_id: PersonId,
    pub name: String,
    pub gender: Gender,
}


