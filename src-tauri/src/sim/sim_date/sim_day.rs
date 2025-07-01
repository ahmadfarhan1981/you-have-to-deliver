use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};

use super::sim_date::SimDate;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Encode, Decode)]
pub struct SimDay {
    pub year: u16,
    pub week: u8,
    pub day: u8,
}

impl From<SimDate> for SimDay {
    fn from(date: SimDate) -> Self {
        Self { year: date.year, week: date.week, day: date.day }
    }
}

impl From<&SimDate> for SimDay {
    fn from(date: &SimDate) -> Self {
        Self { year: date.year, week: date.week, day: date.day }
    }
}
