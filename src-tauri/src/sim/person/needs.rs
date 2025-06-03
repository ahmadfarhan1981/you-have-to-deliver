use std::fmt;
use serde::{Deserialize, Serialize};
use crate::sim::globals::{BASE_ENERGY_DECAY_PER_TICK, BASE_HUNGER_DECAY_PER_TICK};
use crate::sim::person::stats::Stats;

pub struct NeedValue{
    pub value:u8,
    value_raw:f32,
}
impl NeedValue {
    fn new() -> NeedValue {
        NeedValue{value:100,value_raw:100000.0}
    }
    pub fn value(&self) -> u8{
        self.value
    }
    pub fn set_value(&mut self, value:u8){
        self.set_value_raw( value as f32 * 1000.0);
    }
    pub fn set_value_raw(&mut self,value:f32){
        self.value_raw = value;
        self.sync_from_raw();
    }
    pub fn increase_raw(&mut self, amount:f32) {
        self.value_raw += amount;
        self.sync_from_raw();
    }

    pub fn decrease_raw(&mut self, amount:f32) {
        self.value_raw -= amount;
        self.sync_from_raw();
    }
    fn sync_from_raw(&mut self){
        self.value = (self.value as f32 / 1000.0).floor() as u8;
    }


}

pub struct Energy{
    /// Current energy value on a 0-100 scale.
    /// 0 being dead tired and 100 being fully refreshed.
    level:NeedValue,
    pub personal_decay_modifier: f32
}

pub struct Hunger{
    /// Current energy value on a 0-100 scale.
    /// 0 being straving and 100 being fully satiated.
    level:NeedValue,
    pub personal_decay_modifier: f32

}
impl Hunger {
    pub fn level(&self) -> HungerLevel {
        match self.level.value() {
            0..=4 => HungerLevel::Starving,
            5..=19 => HungerLevel::VeryHungry,
            20..=39 => HungerLevel::Hungry,
            40..=59 => HungerLevel::Peckish,
            60..=79 => HungerLevel::ComfortablyFull,
            80..=94 => HungerLevel::WellFed,
            95..=100 => HungerLevel::CompletelySatiated,
            _ => unreachable!("Hunger value out of range"),
        }
    }

    /// Current energy value on a 0-100 scale.
    /// 0 being straving and 100 being fully satiated.
    pub fn value(&self) -> u8{
        self.level.value()
    }

    pub fn new()->Hunger{
        Hunger{level:NeedValue::new(),personal_decay_modifier:1.0}
    }
    pub fn tick (&mut self, stats: &Stats){
        //TODO take stats into account
        let base_decay_raw = BASE_HUNGER_DECAY_PER_TICK as f32 * 1000.0 ;
        let final_decay_raw = base_decay_raw * self.personal_decay_modifier;
        self.level.decrease_raw( final_decay_raw)
    }

}




#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnergyLevel {
    Drained,
    Exhausted,
    VeryTired,
    LowEnergy,
    ModeratelyAlert,
    WellRested,
    FullyEnergized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HungerLevel {
    Starving,
    VeryHungry,
    Hungry,
    Peckish,
    ComfortablyFull,
    WellFed,
    CompletelySatiated,
}

impl Energy {
    pub fn level(&self) -> EnergyLevel {
        match self.value() {
            0..=4 => EnergyLevel::Drained,
            5..=19 => EnergyLevel::Exhausted,
            20..=39 => EnergyLevel::VeryTired,
            40..=59 => EnergyLevel::LowEnergy,
            60..=79 => EnergyLevel::ModeratelyAlert,
            80..=94 => EnergyLevel::WellRested,
            95..=100 => EnergyLevel::FullyEnergized,
            _ => unreachable!("Energy value out of range"),
        }
    }

    pub fn value(&self) -> u8{
        self.level.value()
    }
    pub fn tick (&mut self, stats: &Stats){
        //TODO take stats into account
        let base_decay_raw = BASE_ENERGY_DECAY_PER_TICK as f32 * 1000.0;
        let final_decay_raw = base_decay_raw * self.personal_decay_modifier;
        self.level.decrease_raw( final_decay_raw)
    }

    pub fn new() -> Energy{
        Energy{level:NeedValue::new(),personal_decay_modifier:1.0}
    }


}



impl fmt::Display for EnergyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EnergyLevel::Drained => "Completely drained",
            EnergyLevel::Exhausted => "Exhausted",
            EnergyLevel::VeryTired => "Very tired",
            EnergyLevel::LowEnergy => "Low energy",
            EnergyLevel::ModeratelyAlert => "Moderately alert",
            EnergyLevel::WellRested => "Well rested",
            EnergyLevel::FullyEnergized => "Fully energized",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for HungerLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HungerLevel::Starving => "Starving",
            HungerLevel::VeryHungry => "Very hungry",
            HungerLevel::Hungry => "Hungry",
            HungerLevel::Peckish => "Peckish",
            HungerLevel::ComfortablyFull => "Comfortably full",
            HungerLevel::WellFed => "Well fed",
            HungerLevel::CompletelySatiated => "Completely satiated",
        };
        write!(f, "{}", s)
    }
}