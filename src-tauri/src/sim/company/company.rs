use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Company {
    pub name: String,
    pub slogan: String,
}

pub struct PlayerControlled;