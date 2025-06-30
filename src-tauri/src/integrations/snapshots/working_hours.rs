use crate::sim::calendar::availability::{AvailabilityBitSet, MonthlyAvailability, YearMonth};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkingHoursSnapshot {
    pub person_id: u32,
    pub days: Vec<Vec<bool>>, // 28 days of 96 quarter ticks each
}

impl WorkingHoursSnapshot {
    pub fn from_month(
        person_id: u32,
        availability: &MonthlyAvailability,
        month: YearMonth,
    ) -> Option<Self> {
        availability.get_month(month).map(|detail| {
            let mut days = vec![vec![false; AvailabilityBitSet::TICKS_PER_DAY as usize]; 28];
            for day in 0..28u32 {
                for tick in 0..AvailabilityBitSet::TICKS_PER_DAY {
                    let month_tick = day * AvailabilityBitSet::TICKS_PER_DAY + tick;
                    // working_hours bitset uses 1 = outside working hours
                    let outside = !detail.working_hours.is_free(month_tick, 1);
                    days[day as usize][tick as usize] = !outside;
                }
            }
            Self { person_id, days }
        })
    }
}
