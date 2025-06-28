use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Company {
    pub name: String,
    pub slogan: String,
}


pub struct WorkHourPolicy{
    daily_required_hours: u32,
    weekly_required_hours: u32,
    required_hours : u64,
    required_days: u8,
    
}

pub struct PlayerControlled;