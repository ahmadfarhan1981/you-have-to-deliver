use crate::sim::person::needs::{Energy, Hunger};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EnergySnapshot{
    pub level: u8,
    pub description: String,
}
impl From<&Energy> for EnergySnapshot{
    fn from(value: &Energy) -> Self {
        Self{
            level: value.value(),
            description: value.level().to_string(),
        }
    }
}
impl PartialEq<&Energy> for EnergySnapshot{
    fn eq(&self, other: &&Energy) -> bool {
        self.level == other.value()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HungerSnapshot{
    pub level: u8,
    pub description: String,
}
impl From<&Hunger> for HungerSnapshot{
    fn from(value: &Hunger) -> Self {
        Self{
            level: value.value(),
            description: value.level().to_string(),
        }
    }
}

impl PartialEq<&Hunger> for HungerSnapshot{
    fn eq(&self, other: &&Hunger) -> bool {
        self.level == other.value()
    }
}