use crate::sim::person::morale::StressLevel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StressSnapshot {
    pub person_id: u32,
    pub current: f32,
    pub daily_accumulator: f32,

    pub average_raw_input: f32,
    pub average_felt_stress: f32,

    pub max_recent_raw_input: f32,
    pub max_recent_felt_stress: f32,

    pub baseline_tolerance: f32,
    pub surge_tolerance: f32,
    pub recovery_rate: f32,
}
impl From<&StressLevel> for StressSnapshot {
    fn from(stress: &StressLevel) -> Self {
        Self {
            person_id: 0,
            current: stress.current,
            daily_accumulator: stress.daily_accumulator,

            average_raw_input: stress.average_raw_input(),
            average_felt_stress: stress.average_felt_stress(),

            max_recent_raw_input: stress.max_recent_input(7),
            max_recent_felt_stress: stress.max_recent_felt(7),

            baseline_tolerance: stress.baseline_tolerance,
            surge_tolerance: stress.surge_tolerance,
            recovery_rate: stress.recovery_rate,
        }
    }
}

impl PartialEq<&StressLevel> for StressSnapshot {
    fn eq(&self, other: &&StressLevel) -> bool {
        self == &StressSnapshot::from(*other)
    }
}


#[derive(Copy, Clone, Debug, Default)]
pub struct DirtyStress;
