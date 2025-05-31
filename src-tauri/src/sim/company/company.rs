use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    pub slogan: String,
}

pub struct PlayerControlled;