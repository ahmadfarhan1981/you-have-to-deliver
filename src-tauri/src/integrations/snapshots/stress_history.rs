use serde::{Deserialize, Serialize};
use crate::sim::person::morale::StressLevel;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StressHistorySnapshot {
    pub raw_input_history: Vec<f32>,
    pub felt_stress_history: Vec<f32>,

    pub average_raw_input: f32,
    pub average_felt_stress: f32,

    pub max_raw_input: f32,
    pub max_felt_stress: f32,
}

impl From<&StressLevel> for StressHistorySnapshot {
    fn from(stress: &StressLevel) -> Self {
        Self {
            raw_input_history: stress.raw_input_history.iter().copied().collect(),
            felt_stress_history: stress.felt_stress_history.iter().copied().collect(),

            average_raw_input: stress.average_raw_input(),
            average_felt_stress: stress.average_felt_stress(),

            max_raw_input: stress.raw_input_history.iter().copied().fold(0.0, f32::max),
            max_felt_stress: stress.felt_stress_history.iter().copied().fold(0.0, f32::max),
        }
    }
}

impl PartialEq<&StressLevel> for StressHistorySnapshot {
    fn eq(&self, other: &&StressLevel) -> bool {
        self == &StressHistorySnapshot::from(*other)
    }
}
