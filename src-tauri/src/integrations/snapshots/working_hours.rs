use crate::sim::calendar::availability::{AvailabilityBitSet, MonthlyAvailability, YearMonth};
use crate::sim::sim_date::sim_day::SimDay;
use crate::sim::sim_date::sim_date::SimDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkingDaySnapshot {
    pub date: SimDay,
    pub slots: Vec<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkingHoursSnapshot {
    pub person_id: u32,
    pub days: Vec<WorkingDaySnapshot>,
}

impl WorkingHoursSnapshot {
    pub fn from_month(
        person_id: u32,
        availability: &MonthlyAvailability,
        month: YearMonth,
    ) -> Option<Self> {
        availability.get_month(month).map(|detail| {
            let mut days = Vec::new();
            for day_idx in 0..28u32 {
                let base_tick = day_idx * AvailabilityBitSet::TICKS_PER_DAY;
                let sim_date = SimDate::from_month_tick(month.year, month.month, base_tick);
                let sim_day = SimDay::from(&sim_date);
                let mut slots = vec![false; AvailabilityBitSet::TICKS_PER_DAY as usize];
                for tick in 0..AvailabilityBitSet::TICKS_PER_DAY {
                    let month_tick = base_tick + tick;
                    let outside = !detail.working_hours.is_free(month_tick, 1);
                    slots[tick as usize] = !outside;
                }
                days.push(WorkingDaySnapshot { date: sim_day, slots });
            }
            Self { person_id, days }
        })
    }
}
